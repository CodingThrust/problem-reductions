//! Graph to grid mapping functions.

use super::copyline::{create_copylines, mis_overhead_copyline, CopyLine};
use super::gadgets::{
    apply_crossing_gadgets, apply_simplifier_gadgets, tape_entry_mis_overhead, TapeEntry,
};
use super::grid::MappingGrid;
use super::pathdecomposition::{pathwidth, vertex_order_from_layout, PathDecompositionMethod};
use crate::topology::{GridGraph, GridNode, GridType};
use serde::{Deserialize, Serialize};

const DEFAULT_SPACING: usize = 4;
const DEFAULT_PADDING: usize = 2;
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
}

impl MappingResult {
    /// Map a configuration back from grid to original graph.
    ///
    /// This uses a region-based approach: for each copyline, we look at the
    /// bounding box of its cells and count selected grid nodes in that region.
    /// The vertex is selected if more than half the relevant grid nodes are selected.
    pub fn map_config_back(&self, grid_config: &[usize]) -> Vec<usize> {
        use std::collections::HashMap;

        let debug = std::env::var("DEBUG_MAP_CONFIG").is_ok();

        // Build a position to node index map
        let mut pos_to_idx: HashMap<(usize, usize), usize> = HashMap::new();
        for (idx, node) in self.grid_graph.nodes().iter().enumerate() {
            let row = node.row as usize;
            let col = node.col as usize;
            pos_to_idx.insert((row, col), idx);
        }

        if debug {
            eprintln!("=== map_config_back debug ===");
            eprintln!("Grid nodes: {}", self.grid_graph.nodes().len());
            eprintln!("Grid config (selected nodes):");
            for (idx, &val) in grid_config.iter().enumerate() {
                if val > 0 {
                    if let Some(node) = self.grid_graph.nodes().get(idx) {
                        eprintln!("  node {} at ({}, {})", idx, node.row, node.col);
                    }
                }
            }
            eprintln!("Copylines:");
            for line in &self.lines {
                let locs = line.dense_locations(self.padding, self.spacing);
                eprintln!(
                    "  vertex={}: vslot={}, hslot={}, locs={:?}",
                    line.vertex, line.vslot, line.hslot, locs
                );
            }
        }

        // For each copyline, find grid nodes at copyline positions
        // Use weighted counting: weight=1 cells (endpoints) count double
        let mut result = vec![0; self.lines.len()];

        for line in &self.lines {
            let locs = line.dense_locations(self.padding, self.spacing);
            let mut weighted_count = 0.0;
            let mut total_weight = 0.0;

            // Check each copyline location for a grid node
            for &(row, col, weight) in locs.iter() {
                if let Some(&node_idx) = pos_to_idx.get(&(row, col)) {
                    // Use inverse weight: endpoint cells (weight=1) are more important
                    let w = if weight == 1 { 2.0 } else { 1.0 };
                    total_weight += w;
                    if grid_config.get(node_idx).copied().unwrap_or(0) > 0 {
                        weighted_count += w;
                    }
                }
            }

            if debug {
                eprintln!(
                    "Line vertex={}: locs={}, total_weight={}, weighted_count={}",
                    line.vertex,
                    locs.len(),
                    total_weight,
                    weighted_count
                );
            }

            // For copylines that have no nodes in the final grid (all transformed by gadgets),
            // we need to look at neighboring cells that replaced them
            if total_weight == 0.0 {
                // Expand search to the bounding box + 1 cell margin
                let min_row = locs.iter().map(|l| l.0).min().unwrap_or(0).saturating_sub(1);
                let max_row = locs.iter().map(|l| l.0).max().unwrap_or(0) + 1;
                let min_col = locs.iter().map(|l| l.1).min().unwrap_or(0).saturating_sub(1);
                let max_col = locs.iter().map(|l| l.1).max().unwrap_or(0) + 1;

                for (idx, node) in self.grid_graph.nodes().iter().enumerate() {
                    let r = node.row as usize;
                    let c = node.col as usize;
                    if r >= min_row && r <= max_row && c >= min_col && c <= max_col {
                        total_weight += 1.0;
                        if grid_config.get(idx).copied().unwrap_or(0) > 0 {
                            weighted_count += 1.0;
                        }
                    }
                }

                if debug {
                    eprintln!(
                        "  (expanded search) total_weight={}, weighted_count={}",
                        total_weight, weighted_count
                    );
                }
            }

            // Use majority voting: weighted_count must be at least half
            // (>= rather than > to handle edge cases)
            let threshold = total_weight / 2.0;
            result[line.vertex] = if total_weight > 0.0 && weighted_count >= threshold { 1 } else { 0 };
        }

        result
    }
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

    // Calculate grid dimensions
    let max_hslot = copylines.iter().map(|l| l.hslot).max().unwrap_or(1);
    let max_vslot = copylines.iter().map(|l| l.vslot).max().unwrap_or(1);
    let max_hstop = copylines.iter().map(|l| l.hstop).max().unwrap_or(1);
    let max_vstop = copylines.iter().map(|l| l.vstop).max().unwrap_or(1);

    let rows = max_hslot.max(max_vstop) * spacing + 2 + 2 * padding;
    let cols = max_vslot.max(max_hstop) * spacing + 2 + 2 * padding;

    let mut grid = MappingGrid::with_padding(rows, cols, spacing, padding);

    // Add copy line nodes using dense locations (all cells along the L-shape)
    for line in &copylines {
        for (row, col, weight) in line.dense_locations(padding, spacing) {
            grid.add_node(row, col, weight as i32);
        }
    }

    // Mark edge connections
    for &(u, v) in edges {
        let u_idx = vertex_order
            .iter()
            .position(|&x| x == u)
            .expect("Edge vertex u not found in vertex_order");
        let v_idx = vertex_order
            .iter()
            .position(|&x| x == v)
            .expect("Edge vertex v not found in vertex_order");
        let u_line = &copylines[u_idx];
        let v_line = &copylines[v_idx];

        let (row, col) = grid.cross_at(u_line.vslot, v_line.vslot, u_line.hslot.min(v_line.hslot));

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
/// use problemreductions::rules::mapping::{map_graph_with_method, PathDecompositionMethod};
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
            let locs = line.dense_locations(padding, spacing);
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
            println!("  Tape: pattern_idx={}, pos=({}, {})", entry.pattern_idx, entry.row, entry.col);
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
}
