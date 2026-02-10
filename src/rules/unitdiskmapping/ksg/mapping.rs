//! KSG (King's SubGraph) mapping functions for graphs to grid graphs.
//!
//! This module provides functions to map arbitrary graphs to King's SubGraph
//! (8-connected grid graphs). It supports both unweighted and weighted mapping modes.

use super::super::copyline::{create_copylines, mis_overhead_copyline, CopyLine};
use super::super::grid::MappingGrid;
use super::super::pathdecomposition::{
    pathwidth, vertex_order_from_layout, PathDecompositionMethod,
};
use super::gadgets::{
    apply_crossing_gadgets, apply_simplifier_gadgets, tape_entry_mis_overhead, KsgPattern,
    KsgTapeEntry,
};
use super::gadgets_weighted::{
    apply_weighted_crossing_gadgets, apply_weighted_simplifier_gadgets,
    weighted_tape_entry_mis_overhead, WeightedKsgPattern, WeightedKsgTapeEntry,
};
use super::{PADDING, SPACING};
use crate::topology::{GridGraph, GridNode, GridType};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Unit radius for KSG square lattice grid graphs.
const KSG_UNIT_RADIUS: f64 = 1.5;

/// Result of mapping a graph to a KSG grid graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingResult<T = KsgTapeEntry> {
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
    pub tape: Vec<T>,
    /// Doubled cells (where two copy lines overlap) for map_config_back.
    #[serde(default)]
    pub doubled_cells: HashSet<(usize, usize)>,
}

impl<T> MappingResult<T> {
    /// Get the grid graph size.
    pub fn grid_size(&self) -> (usize, usize) {
        self.grid_graph.size()
    }

    /// Get the number of vertices in the original graph.
    pub fn num_original_vertices(&self) -> usize {
        self.lines.len()
    }

    /// Print a configuration on the grid, highlighting selected nodes.
    ///
    /// Characters:
    /// - `.` = empty cell (no grid node at this position)
    /// - `*` = selected node (config != 0)
    /// - `o` = unselected node (config == 0)
    pub fn print_config(&self, config: &[Vec<usize>]) {
        print!("{}", self.format_config(config));
    }

    /// Format a 2D configuration as a string.
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
                        "*"
                    } else {
                        "o"
                    }
                } else {
                    "."
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
    pub fn print_config_flat(&self, config: &[usize]) {
        print!("{}", self.format_config_flat(config));
    }

    /// Format a flat configuration vector as a string.
    pub fn format_config_flat(&self, config: &[usize]) -> String {
        self.grid_graph.format_with_config(Some(config), false)
    }
}

impl MappingResult<KsgTapeEntry> {
    /// Map a configuration back from grid to original graph.
    ///
    /// This follows the algorithm:
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

    /// Map a configuration back from grid to original graph using center locations.
    pub fn map_config_back_via_centers(&self, grid_config: &[usize]) -> Vec<usize> {
        // Build a position to node index map
        let mut pos_to_idx: HashMap<(usize, usize), usize> = HashMap::new();
        for (idx, node) in self.grid_graph.nodes().iter().enumerate() {
            if let (Ok(row), Ok(col)) = (usize::try_from(node.row), usize::try_from(node.col)) {
                pos_to_idx.insert((row, col), idx);
            }
        }

        // Get traced center locations (after gadget transformations)
        let centers = trace_centers(self);
        let num_vertices = centers.len();
        let mut result = vec![0usize; num_vertices];

        // Read config at each center location
        for (vertex, &(row, col)) in centers.iter().enumerate() {
            if let Some(&node_idx) = pos_to_idx.get(&(row, col)) {
                result[vertex] = grid_config.get(node_idx).copied().unwrap_or(0);
            }
        }

        result
    }
}

impl MappingResult<WeightedKsgTapeEntry> {
    /// Map a configuration back from grid to original graph (weighted version).
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
        unapply_weighted_gadgets(&self.tape, &mut config_2d);

        // Step 3: Extract vertex configs from copylines
        map_config_copyback(
            &self.lines,
            self.padding,
            self.spacing,
            &config_2d,
            &self.doubled_cells,
        )
    }
}

impl<T> fmt::Display for MappingResult<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid_graph)
    }
}

/// Extract original vertex configurations from copyline locations.
///
/// For each copyline, count selected nodes handling doubled cells specially:
/// - For doubled cells: count 1 if value is 2, or if value is 1 and both neighbors are 0
/// - For regular cells: just add the value
/// - Result is `count - (len(locs) / 2)`
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
                // Doubled cell - handle specially
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
            // weight == 0 or empty: skip
        }

        // Subtract overhead: MIS overhead for copyline is len/2
        let overhead = (n / 2) as i32;
        // Result is count - overhead, clamped to non-negative
        result[line.vertex] = (count - overhead).max(0) as usize;
    }

    result
}

/// Unapply gadgets from tape in reverse order, converting mapped configs to source configs.
pub fn unapply_gadgets(tape: &[KsgTapeEntry], config: &mut [Vec<usize>]) {
    // Iterate tape in REVERSE order
    for entry in tape.iter().rev() {
        if let Some(pattern) = KsgPattern::from_tape_idx(entry.pattern_idx) {
            pattern.map_config_back(entry.row, entry.col, config);
        }
    }
}

/// Unapply weighted gadgets from tape in reverse order.
pub fn unapply_weighted_gadgets(tape: &[WeightedKsgTapeEntry], config: &mut [Vec<usize>]) {
    // Iterate tape in REVERSE order
    for entry in tape.iter().rev() {
        if let Some(pattern) = WeightedKsgPattern::from_tape_idx(entry.pattern_idx) {
            pattern.map_config_back(entry.row, entry.col, config);
        }
    }
}

/// Trace center locations through KSG square lattice gadget transformations.
///
/// Returns traced center locations sorted by vertex index.
pub fn trace_centers(result: &MappingResult<KsgTapeEntry>) -> Vec<(usize, usize)> {
    // Initial center locations with (0, 1) offset
    let mut centers: Vec<(usize, usize)> = result
        .lines
        .iter()
        .map(|line| {
            let (row, col) = line.center_location(result.padding, result.spacing);
            (row, col + 1) // Add (0, 1) offset
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
                1 => (3, 4, (2, 2), (2, 4)), // Rotated 90 clockwise
                2 => (4, 3, (3, 2), (1, 2)), // Rotated 180
                3 => (3, 4, (2, 3), (2, 1)), // Rotated 270
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

    let copylines = create_copylines(num_vertices, edges, vertex_order);

    // Calculate grid dimensions
    let max_hslot = copylines.iter().map(|l| l.hslot).max().unwrap_or(1);

    let rows = max_hslot * SPACING + 2 + 2 * PADDING;
    let cols = (num_vertices - 1) * SPACING + 2 + 2 * PADDING;

    let mut grid = MappingGrid::with_padding(rows, cols, SPACING, PADDING);

    // Add copy line nodes using dense locations (all cells along the L-shape)
    for line in &copylines {
        for (row, col, weight) in line.copyline_locations(PADDING, SPACING) {
            grid.add_node(row, col, weight as i32);
        }
    }

    // Mark edge connections
    for &(u, v) in edges {
        let u_line = &copylines[u];
        let v_line = &copylines[v];

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

// ============================================================================
// Unweighted Mapping Functions
// ============================================================================

/// Map a graph to a KSG grid graph using automatic path decomposition.
///
/// Uses exact branch-and-bound for small graphs (≤30 vertices) and greedy for larger.
pub fn map_unweighted(
    num_vertices: usize,
    edges: &[(usize, usize)],
) -> MappingResult<KsgTapeEntry> {
    map_unweighted_with_method(num_vertices, edges, PathDecompositionMethod::Auto)
}

/// Map a graph using a specific path decomposition method (unweighted).
///
/// # Arguments
/// * `num_vertices` - Number of vertices in the graph
/// * `edges` - List of edges as (u, v) pairs
/// * `method` - The path decomposition method to use for vertex ordering
pub fn map_unweighted_with_method(
    num_vertices: usize,
    edges: &[(usize, usize)],
    method: PathDecompositionMethod,
) -> MappingResult<KsgTapeEntry> {
    let layout = pathwidth(num_vertices, edges, method);
    let vertex_order = vertex_order_from_layout(&layout);
    map_unweighted_with_order(num_vertices, edges, &vertex_order)
}

/// Map a graph with a specific vertex ordering (unweighted).
///
/// # Panics
///
/// Panics if `num_vertices == 0`.
pub fn map_unweighted_with_order(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> MappingResult<KsgTapeEntry> {
    let (mut grid, copylines) = embed_graph_internal(num_vertices, edges, vertex_order)
        .expect("Failed to embed graph: num_vertices must be > 0");

    // Extract doubled cells BEFORE applying gadgets
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
        .map(|line| mis_overhead_copyline(line, SPACING, PADDING) as i32)
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

    let grid_graph = GridGraph::new(GridType::Square, grid.size(), nodes, KSG_UNIT_RADIUS);

    MappingResult {
        grid_graph,
        lines: copylines,
        padding: PADDING,
        spacing: SPACING,
        mis_overhead,
        tape,
        doubled_cells,
    }
}

// ============================================================================
// Weighted Mapping Functions
// ============================================================================

/// Map a graph to a KSG grid graph using optimal path decomposition (weighted mode).
///
/// Weighted mode uses gadgets with appropriate weight values that preserve
/// the MWIS (Maximum Weight Independent Set) correspondence.
pub fn map_weighted(
    num_vertices: usize,
    edges: &[(usize, usize)],
) -> MappingResult<WeightedKsgTapeEntry> {
    map_weighted_with_method(num_vertices, edges, PathDecompositionMethod::Auto)
}

/// Map a graph using a specific path decomposition method (weighted).
///
/// # Arguments
/// * `num_vertices` - Number of vertices in the graph
/// * `edges` - List of edges as (u, v) pairs
/// * `method` - The path decomposition method to use for vertex ordering
pub fn map_weighted_with_method(
    num_vertices: usize,
    edges: &[(usize, usize)],
    method: PathDecompositionMethod,
) -> MappingResult<WeightedKsgTapeEntry> {
    let layout = pathwidth(num_vertices, edges, method);
    let vertex_order = vertex_order_from_layout(&layout);
    map_weighted_with_order(num_vertices, edges, &vertex_order)
}

/// Map a graph with a specific vertex ordering (weighted).
///
/// # Panics
///
/// Panics if `num_vertices == 0`.
pub fn map_weighted_with_order(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> MappingResult<WeightedKsgTapeEntry> {
    let (mut grid, copylines) = embed_graph_internal(num_vertices, edges, vertex_order)
        .expect("Failed to embed graph: num_vertices must be > 0");

    // Extract doubled cells BEFORE applying gadgets
    let doubled_cells = grid.doubled_cells();

    // Apply weighted crossing gadgets to resolve line intersections
    let crossing_tape = apply_weighted_crossing_gadgets(&mut grid, &copylines);

    // Apply weighted simplifier gadgets to clean up the grid
    let simplifier_tape = apply_weighted_simplifier_gadgets(&mut grid, 2);

    // Combine tape entries
    let mut tape = crossing_tape;
    tape.extend(simplifier_tape);

    // Calculate MIS overhead from copylines (weighted: multiply by 2)
    let copyline_overhead: i32 = copylines
        .iter()
        .map(|line| mis_overhead_copyline(line, SPACING, PADDING) as i32 * 2)
        .sum();

    // Add MIS overhead from weighted gadgets
    let gadget_overhead: i32 = tape.iter().map(weighted_tape_entry_mis_overhead).sum();
    let mis_overhead = copyline_overhead + gadget_overhead;

    // Convert to GridGraph with weights
    let nodes: Vec<GridNode<i32>> = grid
        .occupied_coords()
        .into_iter()
        .filter_map(|(row, col)| {
            grid.get(row, col)
                .map(|cell| GridNode::new(row as i32, col as i32, cell.weight()))
        })
        .filter(|n| n.weight > 0)
        .collect();

    let grid_graph = GridGraph::new(GridType::Square, grid.size(), nodes, KSG_UNIT_RADIUS);

    MappingResult {
        grid_graph,
        lines: copylines,
        padding: PADDING,
        spacing: SPACING,
        mis_overhead,
        tape,
        doubled_cells,
    }
}

#[cfg(test)]
#[path = "../../../unit_tests/rules/unitdiskmapping/ksg/mapping.rs"]
mod tests;
