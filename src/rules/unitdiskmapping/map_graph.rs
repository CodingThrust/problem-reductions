//! Graph to grid mapping functions.

use super::copyline::{create_copylines, mis_overhead_copyline, CopyLine};
use super::gadgets::{
    apply_crossing_gadgets, apply_simplifier_gadgets, tape_entry_mis_overhead, SquarePattern,
    TapeEntry,
};
use super::grid::MappingGrid;
use super::pathdecomposition::{pathwidth, vertex_order_from_layout, PathDecompositionMethod};
use crate::topology::{GridGraph, GridNode, GridType};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Default spacing for square lattice mapping.
pub const SQUARE_SPACING: usize = 4;
/// Default padding for square lattice mapping.
pub const SQUARE_PADDING: usize = 2;

const DEFAULT_SPACING: usize = SQUARE_SPACING;
const DEFAULT_PADDING: usize = SQUARE_PADDING;
const SQUARE_UNIT_RADIUS: f64 = 1.5;

/// Result of mapping a graph to a grid graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingResult {
    /// The resulting grid graph.
    pub grid_graph: GridGraph<i32>,
    /// Copy lines used in the mapping.
    pub lines: Vec<CopyLine>,
    /// Padding used.
    pub padding: usize,
    /// Spacing used.
    pub spacing: usize,
    /// MIS overhead from the mapping.
    pub mis_overhead: i32,
    /// Tape entries recording gadget applications (for unapply during solution extraction).
    pub tape: Vec<TapeEntry>,
    /// Doubled cells (where two copy lines overlap) for map_config_back.
    #[serde(default)]
    pub doubled_cells: HashSet<(usize, usize)>,
}

impl MappingResult {
    /// Map a configuration back from grid to original graph.
    ///
    /// This follows Julia's exact algorithm:
    /// 1. Convert flat grid config to 2D matrix
    /// 2. Unapply gadgets in reverse order (modifying config matrix)
    /// 3. Extract vertex configs from copyline locations
    ///
    /// # Arguments
    /// * `grid_config` - Configuration on the grid graph (0 = not selected, 1 = selected)
    ///
    /// # Returns
    /// A vector where `result[v]` is 1 if vertex `v` is selected, 0 otherwise.
    pub fn map_config_back(&self, grid_config: &[usize]) -> Vec<usize> {
        // Step 1: Convert flat config to 2D matrix
        let (rows, cols) = self.grid_graph.size();
        let mut config_2d = vec![vec![0usize; cols]; rows];

        for (idx, node) in self.grid_graph.nodes().iter().enumerate() {
            let row = node.row as usize;
            let col = node.col as usize;
            if row < rows && col < cols {
                config_2d[row][col] = grid_config.get(idx).copied().unwrap_or(0);
            }
        }

        // Step 2: Unapply gadgets in reverse order
        unapply_gadgets(&self.tape, &mut config_2d);

        // Step 3: Extract vertex configs from copylines
        map_config_copyback(
            &self.lines,
            self.padding,
            self.spacing,
            &config_2d,
            &self.doubled_cells,
        )
    }

    // NOTE: map_config_back_via_centers has been moved to ksg::MappingResult.
    // This old implementation is kept for backward compatibility but deprecated.
    // Use ksg::MappingResult::map_config_back instead.

    /// Print a configuration on the grid, highlighting selected nodes.
    ///
    /// This is equivalent to Julia's `print_config(res, c)` where `c` is a 2D
    /// configuration matrix indexed by grid coordinates.
    ///
    /// Characters (matching Julia exactly):
    /// - `⋅` = empty cell (no grid node at this position)
    /// - `●` = selected node (config != 0)
    /// - `○` = unselected node (config == 0)
    /// - Each cell is followed by a space
    ///
    /// # Arguments
    ///
    /// * `config` - A 2D configuration where `config[row][col] = 1` means the node is selected.
    ///   The matrix should have dimensions matching the grid size.
    ///
    /// # Example
    ///
    /// ```
    /// use problemreductions::rules::unitdiskmapping::map_graph;
    ///
    /// let edges = vec![(0, 1), (1, 2)];
    /// let result = map_graph(3, &edges);
    ///
    /// // Create a config matrix (rows x cols)
    /// let (rows, cols) = result.grid_graph.size();
    /// let config = vec![vec![0; cols]; rows];
    /// result.print_config(&config);
    /// ```
    pub fn print_config(&self, config: &[Vec<usize>]) {
        print!("{}", self.format_config(config));
    }

    /// Format a 2D configuration as a string matching Julia's print_config format.
    ///
    /// Characters (matching Julia exactly):
    /// - `⋅` = empty cell (no grid node at this position)
    /// - `●` = selected node (config != 0)
    /// - `○` = unselected node (config == 0)
    /// - Each cell is followed by a space
    pub fn format_config(&self, config: &[Vec<usize>]) -> String {
        let (rows, cols) = self.grid_graph.size();

        // Build position to node index map
        let mut pos_to_node: HashMap<(i32, i32), usize> = HashMap::new();
        for (idx, node) in self.grid_graph.nodes().iter().enumerate() {
            pos_to_node.insert((node.row, node.col), idx);
        }

        let mut lines = Vec::new();

        for r in 0..rows {
            let mut line = String::new();
            for c in 0..cols {
                let is_selected = config
                    .get(r)
                    .and_then(|row| row.get(c))
                    .copied()
                    .unwrap_or(0)
                    > 0;
                let has_node = pos_to_node.contains_key(&(r as i32, c as i32));

                let s = if has_node {
                    if is_selected {
                        "●"
                    } else {
                        "○"
                    }
                } else if is_selected {
                    // Julia would error here, but we just ignore
                    "⋅"
                } else {
                    "⋅"
                };
                line.push_str(s);
                line.push(' ');
            }
            // Remove trailing space
            line.pop();
            lines.push(line);
        }

        lines.join("\n")
    }

    /// Print a flat configuration vector on the grid.
    ///
    /// This is an alternative to `print_config` when the configuration is a flat
    /// vector indexed by node order rather than a 2D matrix.
    ///
    /// # Arguments
    ///
    /// * `config` - A flat configuration vector where `config[i] = 1` means node `i` is selected.
    pub fn print_config_flat(&self, config: &[usize]) {
        print!("{}", self.format_config_flat(config));
    }

    /// Format a flat configuration vector as a string.
    pub fn format_config_flat(&self, config: &[usize]) -> String {
        self.grid_graph.format_with_config(Some(config), false)
    }
}

impl fmt::Display for MappingResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid_graph)
    }
}

/// Extract original vertex configurations from copyline locations.
/// Julia: `map_config_copyback!(ug, c)`
///
/// For each copyline, count selected nodes handling doubled cells specially:
/// - For doubled cells (from grid state, not weight): count 1 if value is 2, or if value is 1 and both neighbors are 0
/// - For regular cells: just add the value
/// - Result is `count - (len(locs) / 2)`
///
/// This works after gadgets have been unapplied, so copyline locations
/// are intact in the config matrix.
pub fn map_config_copyback(
    lines: &[CopyLine],
    padding: usize,
    spacing: usize,
    config: &[Vec<usize>],
    doubled_cells: &HashSet<(usize, usize)>,
) -> Vec<usize> {
    let mut result = vec![0usize; lines.len()];

    for line in lines {
        let locs = line.copyline_locations(padding, spacing);
        let n = locs.len();
        let mut count = 0i32;

        for (iloc, &(row, col, weight)) in locs.iter().enumerate() {
            let ci = config
                .get(row)
                .and_then(|r| r.get(col))
                .copied()
                .unwrap_or(0);

            // Check if this cell is doubled in the grid (two copylines overlap here)
            if doubled_cells.contains(&(row, col)) {
                // Doubled cell - handle specially like Julia
                if ci == 2 {
                    count += 1;
                } else if ci == 1 {
                    // Check if both neighbors are 0
                    let prev_zero = if iloc > 0 {
                        let (pr, pc, _) = locs[iloc - 1];
                        config.get(pr).and_then(|r| r.get(pc)).copied().unwrap_or(0) == 0
                    } else {
                        true
                    };
                    let next_zero = if iloc + 1 < n {
                        let (nr, nc, _) = locs[iloc + 1];
                        config.get(nr).and_then(|r| r.get(nc)).copied().unwrap_or(0) == 0
                    } else {
                        true
                    };
                    if prev_zero && next_zero {
                        count += 1;
                    }
                }
                // ci == 0: count += 0 (nothing)
            } else if weight >= 1 {
                // Regular non-empty cell
                count += ci as i32;
            }
            // weight == 0 or empty: skip (error in Julia, we just skip)
        }

        // Subtract overhead: MIS overhead for copyline is len/2
        let overhead = (n / 2) as i32;
        // Result is count - overhead, clamped to non-negative
        result[line.vertex] = (count - overhead).max(0) as usize;
    }

    result
}

/// Unapply gadgets from tape in reverse order, converting mapped configs to source configs.
/// Julia: `unapply_gadgets!(ug, tape, configurations)`
///
/// # Arguments
/// * `tape` - Vector of TapeEntry recording applied gadgets
/// * `config` - 2D config matrix (modified in place)
pub fn unapply_gadgets(tape: &[TapeEntry], config: &mut Vec<Vec<usize>>) {
    // Iterate tape in REVERSE order
    for entry in tape.iter().rev() {
        if let Some(pattern) = SquarePattern::from_tape_idx(entry.pattern_idx) {
            pattern.map_config_back(entry.row, entry.col, config);
        }
    }
}

/// Trace center locations through square lattice gadget transformations.
///
/// This follows Julia's approach: start with center locations, then apply
/// move_center for each gadget in the tape.
///
/// For square lattice:
/// - Crossing gadgets don't move centers (source_centers = mapped_centers)
/// - DanglingLeg simplifier: source_center = (2,2), mapped_center = (4,2)
///
/// Returns traced center locations sorted by vertex index.
pub fn trace_centers_square(result: &MappingResult) -> Vec<(usize, usize)> {
    // Initial center locations with (0, 1) offset (matching Julia)
    let mut centers: Vec<(usize, usize)> = result
        .lines
        .iter()
        .map(|line| {
            let (row, col) = line.center_location(result.padding, result.spacing);
            (row, col + 1) // Julia adds (0, 1) offset
        })
        .collect();

    // Apply gadget transformations from tape
    for entry in &result.tape {
        let pattern_idx = entry.pattern_idx;
        let gi = entry.row;
        let gj = entry.col;

        // Get gadget size and center mapping
        // pattern_idx < 100: crossing gadgets (don't move centers)
        // pattern_idx >= 100: simplifier gadgets (DanglingLeg with rotations)
        if pattern_idx >= 100 {
            // DanglingLeg variants
            let simplifier_idx = pattern_idx - 100;
            let (m, n, source_center, mapped_center) = match simplifier_idx {
                0 => (4, 3, (2, 2), (4, 2)), // DanglingLeg (no rotation)
                1 => (3, 4, (2, 2), (2, 4)), // Rotated 90° clockwise
                2 => (4, 3, (3, 2), (1, 2)), // Rotated 180°
                3 => (3, 4, (2, 3), (2, 1)), // Rotated 270°
                4 => (4, 3, (2, 2), (4, 2)), // Reflected X (same as original for vertical)
                5 => (4, 3, (2, 2), (4, 2)), // Reflected Y (same as original for vertical)
                _ => continue,
            };

            // Check each center and apply transformation if within gadget bounds
            for center in centers.iter_mut() {
                let (ci, cj) = *center;

                // Check if center is within gadget bounds (1-indexed)
                if ci >= gi && ci < gi + m && cj >= gj && cj < gj + n {
                    // Local coordinates (1-indexed)
                    let local_i = ci - gi + 1;
                    let local_j = cj - gj + 1;

                    // Check if this matches the source center
                    if local_i == source_center.0 && local_j == source_center.1 {
                        // Move to mapped center
                        *center = (gi + mapped_center.0 - 1, gj + mapped_center.1 - 1);
                    }
                }
            }
        }
        // Crossing gadgets (pattern_idx < 100) don't move centers
    }

    // Sort by vertex index and return
    let mut indexed: Vec<_> = result
        .lines
        .iter()
        .enumerate()
        .map(|(idx, line)| (line.vertex, centers[idx]))
        .collect();
    indexed.sort_by_key(|(v, _)| *v);
    indexed.into_iter().map(|(_, c)| c).collect()
}

/// Internal function that creates both the mapping grid and copylines.
fn embed_graph_internal(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> Option<(MappingGrid, Vec<CopyLine>)> {
    if num_vertices == 0 {
        return None;
    }

    let spacing = DEFAULT_SPACING;
    let padding = DEFAULT_PADDING;

    let copylines = create_copylines(num_vertices, edges, vertex_order);

    // Calculate grid dimensions - matching Julia's ugrid formula:
    // N = (n-1)*col_spacing + 2 + 2*padding (columns)
    // M = nrow*row_spacing + 2 + 2*padding (rows, where nrow = max_hslot)
    let max_hslot = copylines.iter().map(|l| l.hslot).max().unwrap_or(1);

    let rows = max_hslot * spacing + 2 + 2 * padding;
    let cols = (num_vertices - 1) * spacing + 2 + 2 * padding;

    let mut grid = MappingGrid::with_padding(rows, cols, spacing, padding);

    // Add copy line nodes using dense locations (all cells along the L-shape)
    for line in &copylines {
        for (row, col, weight) in line.copyline_locations(padding, spacing) {
            grid.add_node(row, col, weight as i32);
        }
    }

    // Mark edge connections
    // Copylines are indexed by vertex ID (copylines[v] = copyline for vertex v)
    // Julia's crossat uses hslot from the line with smaller position (vslot)
    for &(u, v) in edges {
        let u_line = &copylines[u];
        let v_line = &copylines[v];

        // Julia's crossat uses: minmax(i,j) then lines[i].hslot (smaller position) for row,
        // and j (larger position) for col
        let (smaller_line, larger_line) = if u_line.vslot < v_line.vslot {
            (u_line, v_line)
        } else {
            (v_line, u_line)
        };
        let (row, col) = grid.cross_at(smaller_line.vslot, larger_line.vslot, smaller_line.hslot);

        // Mark connected cells
        if col > 0 {
            grid.connect(row, col - 1);
        }
        if row > 0 && grid.is_occupied(row - 1, col) {
            grid.connect(row - 1, col);
        } else if row + 1 < grid.size().0 && grid.is_occupied(row + 1, col) {
            grid.connect(row + 1, col);
        }
    }

    Some((grid, copylines))
}

/// Embed a graph into a mapping grid.
///
/// # Panics
///
/// Panics if any edge vertex is not found in `vertex_order`.
pub fn embed_graph(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> Option<MappingGrid> {
    embed_graph_internal(num_vertices, edges, vertex_order).map(|(grid, _)| grid)
}

/// Map a graph to a grid graph using optimal path decomposition (MinhThiTrick).
///
/// This uses the branch-and-bound algorithm to find the optimal vertex ordering
/// that minimizes the grid size.
pub fn map_graph(num_vertices: usize, edges: &[(usize, usize)]) -> MappingResult {
    map_graph_with_method(num_vertices, edges, PathDecompositionMethod::MinhThiTrick)
}

/// Map a graph using a specific path decomposition method.
///
/// # Arguments
/// * `num_vertices` - Number of vertices in the graph
/// * `edges` - List of edges as (u, v) pairs
/// * `method` - The path decomposition method to use for vertex ordering
///
/// # Example
/// ```
/// use problemreductions::rules::unitdiskmapping::{map_graph_with_method, PathDecompositionMethod};
///
/// let edges = vec![(0, 1), (1, 2)];
/// // Use greedy method for faster (but potentially suboptimal) results
/// let result = map_graph_with_method(3, &edges, PathDecompositionMethod::greedy());
/// ```
pub fn map_graph_with_method(
    num_vertices: usize,
    edges: &[(usize, usize)],
    method: PathDecompositionMethod,
) -> MappingResult {
    let layout = pathwidth(num_vertices, edges, method);
    // Julia reverses the vertex order from pathwidth result
    let vertex_order = vertex_order_from_layout(&layout);
    map_graph_with_order(num_vertices, edges, &vertex_order)
}

/// Map a graph with a specific vertex ordering.
///
/// # Panics
///
/// Panics if `num_vertices == 0`.
pub fn map_graph_with_order(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> MappingResult {
    let spacing = DEFAULT_SPACING;
    let padding = DEFAULT_PADDING;

    let (mut grid, copylines) = embed_graph_internal(num_vertices, edges, vertex_order)
        .expect("Failed to embed graph: num_vertices must be > 0");

    // Extract doubled cells BEFORE applying gadgets
    // Julia restores grid state with unapply!(gadget), but we just save it beforehand
    let doubled_cells = grid.doubled_cells();

    // Apply crossing gadgets to resolve line intersections
    let crossing_tape = apply_crossing_gadgets(&mut grid, &copylines);

    // Apply simplifier gadgets to clean up the grid
    let simplifier_tape = apply_simplifier_gadgets(&mut grid, 2);

    // Combine tape entries
    let mut tape = crossing_tape;
    tape.extend(simplifier_tape);

    // Calculate MIS overhead from copylines
    let copyline_overhead: i32 = copylines
        .iter()
        .map(|line| mis_overhead_copyline(line, spacing, padding) as i32)
        .sum();

    // Add MIS overhead from gadgets
    let gadget_overhead: i32 = tape.iter().map(tape_entry_mis_overhead).sum();
    let mis_overhead = copyline_overhead + gadget_overhead;

    // Convert to GridGraph
    let nodes: Vec<GridNode<i32>> = grid
        .occupied_coords()
        .into_iter()
        .filter_map(|(row, col)| {
            grid.get(row, col)
                .map(|cell| GridNode::new(row as i32, col as i32, cell.weight()))
        })
        .filter(|n| n.weight > 0)
        .collect();

    let grid_graph = GridGraph::new(GridType::Square, grid.size(), nodes, SQUARE_UNIT_RADIUS);

    MappingResult {
        grid_graph,
        lines: copylines,
        padding,
        spacing,
        mis_overhead,
        tape,
        doubled_cells,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::Graph;

    #[test]
    fn test_embed_graph_path() {
        // Path graph: 0-1-2
        let edges = vec![(0, 1), (1, 2)];
        let result = embed_graph(3, &edges, &[0, 1, 2]);

        assert!(result.is_some());
        let grid = result.unwrap();
        assert!(!grid.occupied_coords().is_empty());
    }

    #[test]
    fn test_map_graph_triangle() {
        // Triangle graph
        let edges = vec![(0, 1), (1, 2), (0, 2)];
        let result = map_graph(3, &edges);

        assert!(result.grid_graph.num_vertices() > 0);
        assert!(result.mis_overhead >= 0);
    }

    #[test]
    fn debug_path_graph_overhead() {
        use super::super::gadgets::{
            apply_crossing_gadgets, apply_simplifier_gadgets, pattern_matches, DanglingLeg,
            RotatedGadget,
        };

        // Path graph: 0-1-2 - Julia gives MIS overhead = 2
        let edges = vec![(0, 1), (1, 2)];

        // Step by step like Julia
        let spacing = DEFAULT_SPACING;
        let padding = DEFAULT_PADDING;
        let layout = super::super::pathdecomposition::pathwidth(
            3,
            &edges,
            super::super::pathdecomposition::PathDecompositionMethod::MinhThiTrick,
        );
        let vertex_order = super::super::pathdecomposition::vertex_order_from_layout(&layout);

        let (mut grid, copylines) = embed_graph_internal(3, &edges, &vertex_order).unwrap();

        println!("=== Copylines ===");
        for line in &copylines {
            let locs = line.copyline_locations(padding, spacing);
            let overhead = locs.len() / 2;
            println!(
                "  Line vertex={}: vslot={}, hslot={}, vstart={}, vstop={}, hstop={}, locs={}, overhead={}",
                line.vertex, line.vslot, line.hslot, line.vstart, line.vstop, line.hstop, locs.len(), overhead
            );
        }

        println!("\n=== After embed ===");
        println!("Occupied cells: {}", grid.occupied_coords().len());
        for (row, col) in grid.occupied_coords() {
            if let Some(cell) = grid.get(row, col) {
                println!("  ({}, {}) weight={}", row, col, cell.weight());
            }
        }

        // Check all crossing points
        println!("\n=== Crossing points ===");
        for j in 1..=copylines.len() {
            for i in 1..=copylines.len() {
                let (cross_row, cross_col) = grid.cross_at(i, j, i.min(j));
                println!("  cross_at({}, {}) = ({}, {})", i, j, cross_row, cross_col);
            }
        }

        println!("\n=== After crossing gadgets ===");
        let crossing_tape = apply_crossing_gadgets(&mut grid, &copylines);
        println!("Crossing tape entries: {}", crossing_tape.len());
        for entry in &crossing_tape {
            println!(
                "  Tape: pattern_idx={}, pos=({}, {})",
                entry.pattern_idx, entry.row, entry.col
            );
        }
        println!("Occupied cells: {}", grid.occupied_coords().len());
        for (row, col) in grid.occupied_coords() {
            if let Some(cell) = grid.get(row, col) {
                println!("  ({}, {}) weight={}", row, col, cell.weight());
            }
        }

        // Check for DanglingLeg matches before simplifier
        println!("\n=== DanglingLeg pattern matching ===");
        let dl = DanglingLeg;
        let (rows, cols) = grid.size();
        let mut dl_matches = 0;
        for i in 0..rows {
            for j in 0..cols {
                if pattern_matches(&dl, &grid, i, j) {
                    println!("  DanglingLeg matches at ({}, {})", i, j);
                    dl_matches += 1;
                }
            }
        }
        println!("Total DanglingLeg matches: {}", dl_matches);

        // Check rotated versions
        for rot in 0..4 {
            let rotated = RotatedGadget::new(DanglingLeg, rot);
            let mut count = 0;
            for i in 0..rows {
                for j in 0..cols {
                    if pattern_matches(&rotated, &grid, i, j) {
                        count += 1;
                    }
                }
            }
            if count > 0 {
                println!("  RotatedGadget(DanglingLeg, {}) matches: {}", rot, count);
            }
        }

        println!("\n=== After simplifier gadgets ===");
        let simplifier_tape = apply_simplifier_gadgets(&mut grid, 2);
        println!("Simplifier tape entries: {}", simplifier_tape.len());
        println!("Occupied cells: {}", grid.occupied_coords().len());

        // Final result
        let result = map_graph(3, &edges);
        println!("\n=== Final result ===");
        println!("Grid vertices: {}", result.grid_graph.num_vertices());
        println!("Grid edges: {}", result.grid_graph.edges().len());
        println!("MIS overhead: {}", result.mis_overhead);

        // Julia: 7 vertices, 6 edges, overhead 2
        assert!(result.grid_graph.num_vertices() > 0);
    }

    #[test]
    fn test_mapping_result_config_back() {
        let edges = vec![(0, 1)];
        let result = map_graph(2, &edges);

        // Create a dummy config
        let config: Vec<usize> = vec![0; result.grid_graph.num_vertices()];
        let original = result.map_config_back(&config);

        assert_eq!(original.len(), 2);
    }

    #[test]
    fn test_map_config_copyback_simple() {
        // Create a simple copyline
        let line = CopyLine {
            vertex: 0,
            vslot: 1,
            hslot: 1,
            vstart: 1,
            vstop: 1,
            hstop: 3,
        };
        let lines = vec![line];

        // Create config with some nodes selected
        let locs = lines[0].copyline_locations(2, 4);
        let (rows, cols) = (20, 20);
        let mut config = vec![vec![0; cols]; rows];

        // Select all nodes in copyline
        for &(row, col, _) in &locs {
            if row < rows && col < cols {
                config[row][col] = 1;
            }
        }

        let doubled_cells = HashSet::new();
        let result = map_config_copyback(&lines, 2, 4, &config, &doubled_cells);

        // count = len(locs) (all selected with ci=1), overhead = len/2
        // result = count - overhead = n - n/2 ≈ n/2
        let n = locs.len();
        let overhead = n / 2;
        let expected = n - overhead;
        assert_eq!(result[0], expected);
    }

    #[test]
    fn test_map_config_copyback_multiple_vertices() {
        // Create two copylines for different vertices
        let line0 = CopyLine {
            vertex: 0,
            vslot: 1,
            hslot: 1,
            vstart: 1,
            vstop: 1,
            hstop: 2,
        };
        let line1 = CopyLine {
            vertex: 1,
            vslot: 2,
            hslot: 1,
            vstart: 1,
            vstop: 1,
            hstop: 2,
        };
        let lines = vec![line0, line1];

        let (rows, cols) = (20, 20);
        let mut config = vec![vec![0; cols]; rows];

        // Select all nodes for vertex 0, none for vertex 1
        let locs0 = lines[0].copyline_locations(2, 4);
        for &(row, col, _) in &locs0 {
            if row < rows && col < cols {
                config[row][col] = 1;
            }
        }

        let doubled_cells = HashSet::new();
        let result = map_config_copyback(&lines, 2, 4, &config, &doubled_cells);

        // Vertex 0: all selected, result = n - n/2 ≈ n/2
        let n0 = locs0.len();
        let expected0 = n0 - (n0 / 2);
        assert_eq!(result[0], expected0);

        // Vertex 1: none selected, count = 0 <= overhead, so result = 0
        assert_eq!(result[1], 0);
    }

    #[test]
    fn test_map_config_copyback_partial_selection() {
        // Create a copyline
        let line = CopyLine {
            vertex: 0,
            vslot: 1,
            hslot: 1,
            vstart: 1,
            vstop: 2,
            hstop: 2,
        };
        let lines = vec![line];

        let locs = lines[0].copyline_locations(2, 4);
        let (rows, cols) = (20, 20);
        let mut config = vec![vec![0; cols]; rows];

        // Select only half the nodes
        let half = locs.len() / 2;
        for &(row, col, _) in locs.iter().take(half) {
            if row < rows && col < cols {
                config[row][col] = 1;
            }
        }

        let doubled_cells = HashSet::new();
        let result = map_config_copyback(&lines, 2, 4, &config, &doubled_cells);

        // count = half, overhead = len/2
        // result = half - len/2 = 0 (since half == len/2)
        let overhead = locs.len() / 2;
        let expected = half.saturating_sub(overhead);
        assert_eq!(result[0], expected);
    }
}
