//! Mapping functions for weighted triangular lattice.
//!
//! This module provides functions to map arbitrary graphs to weighted triangular
//! lattice grid graphs using the copy-line technique.

use super::super::copyline::{create_copylines, CopyLine};
use super::super::grid::MappingGrid;
use super::super::ksg::mapping::MappingResult;
use super::super::ksg::KsgTapeEntry as TapeEntry;
use super::super::pathdecomposition::{
    pathwidth, vertex_order_from_layout, PathDecompositionMethod,
};
use super::gadgets::{apply_crossing_gadgets, apply_simplifier_gadgets, tape_entry_mis_overhead};
use crate::rules::unitdiskmapping::ksg::mapping::GridKind;

/// Spacing between copy lines on triangular lattice.
pub const SPACING: usize = 6;

/// Padding around the grid for triangular lattice.
pub const PADDING: usize = 2;

/// Calculate crossing point for two copylines on triangular lattice.
fn crossat(
    copylines: &[CopyLine],
    v: usize,
    w: usize,
    spacing: usize,
    padding: usize,
) -> (usize, usize) {
    let line_v = &copylines[v];
    let line_w = &copylines[w];

    // Use vslot to determine order
    let (line_first, line_second) = if line_v.vslot < line_w.vslot {
        (line_v, line_w)
    } else {
        (line_w, line_v)
    };

    let hslot = line_first.hslot;
    let max_vslot = line_second.vslot;

    // 0-indexed coordinates (subtract 1 from Julia's 1-indexed formula)
    let row = (hslot - 1) * spacing + 1 + padding; // 0-indexed
    let col = (max_vslot - 1) * spacing + padding; // 0-indexed

    (row, col)
}

/// Map a graph to a weighted triangular lattice grid graph using optimal path decomposition.
///
/// This is the main entry point for triangular lattice mapping. It uses
/// automatic path decomposition (exact for ≤30 vertices, greedy for larger).
///
/// # Arguments
/// * `num_vertices` - Number of vertices in the original graph
/// * `edges` - Edge list as (u, v) pairs
///
/// # Returns
/// A `MappingResult` containing the grid graph and mapping metadata.
///
/// # Panics
/// Panics if `num_vertices == 0`.
///
/// # Example
/// ```rust
/// use problemreductions::rules::unitdiskmapping::triangular::mapping::map_weighted;
/// use problemreductions::topology::Graph;
///
/// let edges = vec![(0, 1), (1, 2)];
/// let result = map_weighted(3, &edges);
/// assert!(result.to_triangular_subgraph().num_vertices() > 0);
/// ```
pub fn map_weighted(num_vertices: usize, edges: &[(usize, usize)]) -> MappingResult {
    map_weighted_with_method(num_vertices, edges, PathDecompositionMethod::Auto)
}

/// Map a graph to weighted triangular lattice using a specific path decomposition method.
///
/// # Arguments
/// * `num_vertices` - Number of vertices in the original graph
/// * `edges` - Edge list as (u, v) pairs
/// * `method` - Path decomposition method to use
///
/// # Returns
/// A `MappingResult` containing the grid graph and mapping metadata.
pub fn map_weighted_with_method(
    num_vertices: usize,
    edges: &[(usize, usize)],
    method: PathDecompositionMethod,
) -> MappingResult {
    let layout = pathwidth(num_vertices, edges, method);
    let vertex_order = vertex_order_from_layout(&layout);
    map_weighted_with_order(num_vertices, edges, &vertex_order)
}

/// Map a graph to weighted triangular lattice with specific vertex ordering.
///
/// This is the most flexible mapping function, allowing custom vertex ordering
/// for cases where a specific layout is desired.
///
/// # Arguments
/// * `num_vertices` - Number of vertices in the original graph
/// * `edges` - Edge list as (u, v) pairs
/// * `vertex_order` - Custom vertex ordering
///
/// # Returns
/// A `MappingResult` containing the grid graph and mapping metadata.
///
/// # Panics
/// Panics if `num_vertices == 0` or if any edge vertex is not in `vertex_order`.
pub fn map_weighted_with_order(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> MappingResult {
    assert!(num_vertices > 0, "num_vertices must be > 0");

    let spacing = SPACING;
    let padding = PADDING;

    let copylines = create_copylines(num_vertices, edges, vertex_order);

    // Calculate grid dimensions
    // Julia formula: N = (n-1)*col_spacing + 2 + 2*padding
    //                M = nrow*row_spacing + 2 + 2*padding
    // where nrow = max(hslot, vstop) and n = num_vertices
    let max_hslot = copylines.iter().map(|l| l.hslot).max().unwrap_or(1);
    let max_vstop = copylines.iter().map(|l| l.vstop).max().unwrap_or(1);

    let rows = max_hslot.max(max_vstop) * spacing + 2 + 2 * padding;
    // Use (num_vertices - 1) for cols, matching Julia's (n-1) formula
    let cols = (num_vertices - 1) * spacing + 2 + 2 * padding;

    let mut grid = MappingGrid::with_padding(rows, cols, spacing, padding);

    // Add copy line nodes using triangular dense locations
    // (includes the endpoint node for triangular weighted mode)
    for line in &copylines {
        for (row, col, weight) in line.copyline_locations_triangular(padding, spacing) {
            grid.add_node(row, col, weight as i32);
        }
    }

    // Mark edge connections at crossing points
    for &(u, v) in edges {
        let u_line = &copylines[u];
        let v_line = &copylines[v];

        let (smaller_line, larger_line) = if u_line.vslot < v_line.vslot {
            (u_line, v_line)
        } else {
            (v_line, u_line)
        };

        let (row, col) = crossat(
            &copylines,
            smaller_line.vertex,
            larger_line.vertex,
            spacing,
            padding,
        );

        // Mark connected cells at crossing point
        if col > 0 {
            grid.connect(row, col - 1);
        }
        if row > 0 && grid.is_occupied(row - 1, col) {
            grid.connect(row - 1, col);
        } else if row + 1 < grid.size().0 && grid.is_occupied(row + 1, col) {
            grid.connect(row + 1, col);
        }
    }

    // Apply crossing gadgets (iterates ALL pairs, not just edges)
    let mut triangular_tape = apply_crossing_gadgets(&mut grid, &copylines, spacing, padding);

    // Apply simplifier gadgets (weighted DanglingLeg pattern)
    // Julia's triangular mode uses: weighted.(default_simplifier_ruleset(UnWeighted()))
    // which applies the weighted DanglingLeg pattern to reduce grid complexity.
    let simplifier_tape = apply_simplifier_gadgets(&mut grid, 10);
    triangular_tape.extend(simplifier_tape);

    // Calculate MIS overhead from copylines using the dedicated function
    // which matches Julia's mis_overhead_copyline(TriangularWeighted(), ...)
    let copyline_overhead: i32 = copylines
        .iter()
        .map(|line| super::super::copyline::mis_overhead_copyline_triangular(line, spacing))
        .sum();

    // Add gadget overhead (crossing gadgets + simplifiers)
    let gadget_overhead: i32 = triangular_tape.iter().map(tape_entry_mis_overhead).sum();
    let mis_overhead = copyline_overhead + gadget_overhead;

    // Convert triangular tape entries to generic tape entries
    let tape: Vec<TapeEntry> = triangular_tape
        .into_iter()
        .map(|entry| TapeEntry {
            pattern_idx: entry.gadget_idx,
            row: entry.row,
            col: entry.col,
        })
        .collect();

    // Extract doubled cells before extracting positions
    let doubled_cells = grid.doubled_cells();

    // Extract positions and weights from occupied cells
    let (positions, node_weights): (Vec<(i32, i32)>, Vec<i32>) = grid
        .occupied_coords()
        .into_iter()
        .filter_map(|(row, col)| {
            grid.get(row, col)
                .map(|cell| ((row as i32, col as i32), cell.weight()))
        })
        .filter(|&(_, w)| w > 0)
        .unzip();

    MappingResult {
        positions,
        node_weights,
        grid_dimensions: grid.size(),
        kind: GridKind::Triangular,
        lines: copylines,
        padding,
        spacing,
        mis_overhead,
        tape,
        doubled_cells,
    }
}

/// Get the weighted triangular crossing ruleset.
///
/// This returns the list of weighted triangular gadgets used for resolving
/// crossings in the mapping process. Matches Julia's `crossing_ruleset_triangular_weighted`.
///
/// # Returns
/// A vector of `WeightedTriangularGadget` enum variants.
pub fn weighted_ruleset() -> Vec<super::super::weighted::WeightedTriangularGadget> {
    super::super::weighted::triangular_weighted_ruleset()
}

/// Trace center locations through gadget transformations.
///
/// Returns the final center location for each original vertex after all
/// gadget transformations have been applied.
///
/// This matches Julia's `trace_centers` function which:
/// 1. Gets initial center locations with (0, 1) offset
/// 2. Applies `move_center` for each gadget in the tape
///
/// # Arguments
/// * `result` - The mapping result from `map_weighted`
///
/// # Returns
/// A vector of (row, col) positions for each original vertex.
pub fn trace_centers(result: &MappingResult) -> Vec<(usize, usize)> {
    super::super::weighted::trace_centers(result)
}

/// Map source vertex weights to grid graph weights.
///
/// This function takes weights for each original vertex and maps them to
/// the corresponding nodes in the grid graph.
///
/// # Arguments
/// * `result` - The mapping result from `map_weighted`
/// * `source_weights` - Weights for each original vertex (should be in [0, 1])
///
/// # Returns
/// A vector of weights for each node in the grid graph.
///
/// # Panics
/// Panics if any weight is outside the range [0, 1] or if the number of
/// weights doesn't match the number of vertices.
pub fn map_weights(result: &MappingResult, source_weights: &[f64]) -> Vec<f64> {
    super::super::weighted::map_weights(result, source_weights)
}

#[cfg(test)]
#[path = "../../../unit_tests/rules/unitdiskmapping/triangular/mapping.rs"]
mod tests;
