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
use crate::rules::unitdiskmapping::{mapping_integer_overflow, mapping_invalid};
use crate::rules::ReductionError;
use crate::topology::{Graph, KingsSubgraph, TriangularSubgraph};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// The kind of grid lattice used in a mapping result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridKind {
    /// Square lattice (King's SubGraph connectivity, radius 1.5).
    Kings,
    /// Triangular lattice (radius 1.1).
    Triangular,
}

/// Result of mapping a graph to a grid graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingResult<T = KsgTapeEntry> {
    /// Integer grid positions (row, col) for each node.
    pub positions: Vec<(i64, i64)>,
    /// Weight of each node.
    pub node_weights: Vec<i64>,
    /// Grid dimensions (rows, cols).
    pub grid_dimensions: (usize, usize),
    /// The kind of grid lattice.
    pub kind: GridKind,
    /// Copy lines used in the mapping.
    pub lines: Vec<CopyLine>,
    /// Padding used.
    pub padding: usize,
    /// Spacing used.
    pub spacing: usize,
    /// MIS overhead from the mapping.
    pub mis_overhead: i64,
    /// Tape entries recording gadget applications (for unapply during solution extraction).
    pub tape: Vec<T>,
    /// Doubled cells (where two copy lines overlap) for map_config_back.
    #[serde(default)]
    pub doubled_cells: HashSet<(usize, usize)>,
}

impl<T> MappingResult<T> {
    /// Get the number of vertices in the original graph.
    pub fn num_original_vertices(&self) -> usize {
        self.lines.len()
    }

    /// Compute edges based on grid kind.
    pub fn edges(&self) -> Vec<(usize, usize)> {
        match self.kind {
            GridKind::Kings => self.to_kings_subgraph().edges(),
            GridKind::Triangular => self.to_triangular_subgraph().edges(),
        }
    }

    /// Compute the number of edges based on grid kind.
    pub fn num_edges(&self) -> usize {
        match self.kind {
            GridKind::Kings => self.to_kings_subgraph().num_edges(),
            GridKind::Triangular => self.to_triangular_subgraph().num_edges(),
        }
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
        let (rows, cols) = self.grid_dimensions;

        // Build position to node index map
        let mut pos_to_node: HashMap<(i64, i64), usize> = HashMap::new();
        for (idx, &(r, c)) in self.positions.iter().enumerate() {
            pos_to_node.insert((r, c), idx);
        }

        let mut lines = Vec::new();

        for r in 0..rows {
            let row = i64::try_from(r).expect("mapping grid rows are validated against i64");
            let mut line = String::new();
            for c in 0..cols {
                let is_selected = config
                    .get(r)
                    .and_then(|row| row.get(c))
                    .copied()
                    .unwrap_or(0)
                    > 0;
                let column =
                    i64::try_from(c).expect("mapping grid columns are validated against i64");
                let has_node = pos_to_node.contains_key(&(row, column));

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
        self.format_grid_with_config(Some(config))
    }

    /// Create a [`KingsSubgraph`] from this mapping result, extracting positions
    /// and discarding weights.
    pub fn to_kings_subgraph(&self) -> KingsSubgraph {
        KingsSubgraph::new(self.positions.clone())
    }

    /// Create a [`TriangularSubgraph`] from this mapping result, extracting positions
    /// and discarding weights.
    pub fn to_triangular_subgraph(&self) -> TriangularSubgraph {
        TriangularSubgraph::new(self.positions.clone())
    }

    /// Format the grid, optionally with a configuration overlay.
    ///
    /// Without config: shows weight values (single-char) or `●` for multi-char weights.
    /// With config: shows `●` for selected nodes, `○` for unselected.
    /// Empty cells show `⋅`.
    fn format_grid_with_config(&self, config: Option<&[usize]>) -> String {
        if self.positions.is_empty() {
            return String::from("(empty grid graph)");
        }

        let (rows, cols) = self.grid_dimensions;

        let mut pos_to_idx: HashMap<(i64, i64), usize> = HashMap::new();
        for (idx, &(r, c)) in self.positions.iter().enumerate() {
            pos_to_idx.insert((r, c), idx);
        }

        let mut lines = Vec::new();

        for r in 0..rows {
            let r = i64::try_from(r).expect("mapping grid rows are validated against i64");
            let mut line = String::new();
            for c in 0..cols {
                let c = i64::try_from(c).expect("mapping grid columns are validated against i64");
                let s = if let Some(&idx) = pos_to_idx.get(&(r, c)) {
                    if let Some(cfg) = config {
                        if cfg.get(idx).copied().unwrap_or(0) > 0 {
                            "●".to_string()
                        } else {
                            "○".to_string()
                        }
                    } else {
                        let w = self.node_weights[idx];
                        let ws = format!("{}", w);
                        if ws.len() == 1 {
                            ws
                        } else {
                            "●".to_string()
                        }
                    }
                } else {
                    "⋅".to_string()
                };
                line.push_str(&s);
                line.push(' ');
            }
            line.pop();
            lines.push(line);
        }

        lines.join("\n")
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
    pub fn map_config_back(
        &self,
        grid_config: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        self.map_config_back_internal(grid_config)
            .map_err(|error| crate::rules::ExtractionError::invalid(error.to_string()))
    }

    fn map_config_back_internal(
        &self,
        grid_config: &[usize],
    ) -> Result<Vec<usize>, ReductionError> {
        if grid_config.len() != self.positions.len() {
            return Err(mapping_invalid(
                "grid configuration length must match the mapped vertex count",
            ));
        }
        // Step 1: Convert flat config to 2D matrix
        let (rows, cols) = self.grid_dimensions;
        let mut config_2d = vec![vec![0usize; cols]; rows];

        for (idx, &(row, col)) in self.positions.iter().enumerate() {
            let row = usize::try_from(row)
                .map_err(|_| mapping_invalid("mapping result contains a negative grid row"))?;
            let col = usize::try_from(col)
                .map_err(|_| mapping_invalid("mapping result contains a negative grid column"))?;
            if row >= rows || col >= cols {
                return Err(mapping_invalid(
                    "mapping result contains a position outside its grid dimensions",
                ));
            }
            config_2d[row][col] = grid_config[idx];
        }

        // Step 2: Unapply gadgets in reverse order
        unapply_gadgets(&self.tape, &mut config_2d)?;

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

impl MappingResult<WeightedKsgTapeEntry> {
    /// Map a configuration back from grid to original graph (weighted version).
    pub fn map_config_back(
        &self,
        grid_config: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        self.map_config_back_internal(grid_config)
            .map_err(|error| crate::rules::ExtractionError::invalid(error.to_string()))
    }

    fn map_config_back_internal(
        &self,
        grid_config: &[usize],
    ) -> Result<Vec<usize>, ReductionError> {
        if grid_config.len() != self.positions.len() {
            return Err(mapping_invalid(
                "grid configuration length must match the mapped vertex count",
            ));
        }
        // Step 1: Convert flat config to 2D matrix
        let (rows, cols) = self.grid_dimensions;
        let mut config_2d = vec![vec![0usize; cols]; rows];

        for (idx, &(row, col)) in self.positions.iter().enumerate() {
            let row = usize::try_from(row)
                .map_err(|_| mapping_invalid("mapping result contains a negative grid row"))?;
            let col = usize::try_from(col)
                .map_err(|_| mapping_invalid("mapping result contains a negative grid column"))?;
            if row >= rows || col >= cols {
                return Err(mapping_invalid(
                    "mapping result contains a position outside its grid dimensions",
                ));
            }
            config_2d[row][col] = grid_config[idx];
        }

        // Step 2: Unapply gadgets in reverse order
        unapply_weighted_gadgets(&self.tape, &mut config_2d)?;

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
        write!(f, "{}", self.format_grid_with_config(None))
    }
}

/// Extract original vertex configurations from copyline locations.
///
/// For each copyline, count selected nodes handling doubled cells specially:
/// - For doubled cells: count 1 if value is 2, or if value is 1 and both neighbors are 0
/// - For regular cells: just add the value
/// - Result is `count - (len(locs) / 2)`
pub(crate) fn map_config_copyback(
    lines: &[CopyLine],
    padding: usize,
    spacing: usize,
    config: &[Vec<usize>],
    doubled_cells: &HashSet<(usize, usize)>,
) -> Result<Vec<usize>, ReductionError> {
    let mut result = vec![0usize; lines.len()];

    for line in lines {
        let locs = line.copyline_locations(padding, spacing);
        let n = locs.len();
        let mut count = 0i64;

        for (iloc, &(row, col, weight)) in locs.iter().enumerate() {
            let ci = config
                .get(row)
                .and_then(|r| r.get(col))
                .copied()
                .ok_or(mapping_invalid(
                    "copy line lies outside the configuration grid",
                ))?;

            // Check if this cell is doubled in the grid (two copylines overlap here)
            if doubled_cells.contains(&(row, col)) {
                // Doubled cell - handle specially
                if ci == 2 {
                    count = count
                        .checked_add(1)
                        .ok_or(mapping_integer_overflow("summing copy-back values"))?;
                } else if ci == 1 {
                    // Check if both neighbors are 0
                    let prev_zero =
                        if iloc > 0 {
                            let (pr, pc, _) = locs[iloc - 1];
                            config.get(pr).and_then(|r| r.get(pc)).copied().ok_or(
                                mapping_invalid(
                                    "copy-line neighbor lies outside the configuration grid",
                                ),
                            )? == 0
                        } else {
                            true
                        };
                    let next_zero =
                        if iloc + 1 < n {
                            let (nr, nc, _) = locs[iloc + 1];
                            config.get(nr).and_then(|r| r.get(nc)).copied().ok_or(
                                mapping_invalid(
                                    "copy-line neighbor lies outside the configuration grid",
                                ),
                            )? == 0
                        } else {
                            true
                        };
                    if prev_zero && next_zero {
                        count = count
                            .checked_add(1)
                            .ok_or(mapping_integer_overflow("summing copy-back values"))?;
                    }
                }
                // ci == 0: count += 0 (nothing)
            } else if weight >= 1 {
                // Regular non-empty cell
                let value = i64::try_from(ci)
                    .map_err(|_| mapping_integer_overflow("converting a copy-back value to i64"))?;
                count = count
                    .checked_add(value)
                    .ok_or(mapping_integer_overflow("summing copy-back values"))?;
            }
            // weight == 0 or empty: skip
        }

        // Subtract overhead: MIS overhead for copyline is len/2
        let overhead = i64::try_from(n / 2)
            .map_err(|_| mapping_integer_overflow("converting copy-back overhead to i64"))?;
        // Result is count - overhead, clamped to non-negative
        let adjusted = count
            .checked_sub(overhead)
            .ok_or(mapping_integer_overflow("subtracting copy-back overhead"))?;
        let adjusted = adjusted.max(0);
        result[line.vertex] = usize::try_from(adjusted)
            .map_err(|_| mapping_integer_overflow("converting a copy-back result to usize"))?;
    }

    Ok(result)
}

/// Unapply gadgets from tape in reverse order, converting mapped configs to source configs.
pub(crate) fn unapply_gadgets(
    tape: &[KsgTapeEntry],
    config: &mut [Vec<usize>],
) -> Result<(), ReductionError> {
    // Iterate tape in REVERSE order
    for entry in tape.iter().rev() {
        let pattern = KsgPattern::from_tape_idx(entry.pattern_idx).ok_or(mapping_invalid(
            "tape contains an unknown unweighted KSG gadget index",
        ))?;
        pattern.map_config_back(entry.row, entry.col, config)?;
    }
    Ok(())
}

/// Unapply weighted gadgets from tape in reverse order.
pub(crate) fn unapply_weighted_gadgets(
    tape: &[WeightedKsgTapeEntry],
    config: &mut [Vec<usize>],
) -> Result<(), ReductionError> {
    // Iterate tape in REVERSE order
    for entry in tape.iter().rev() {
        let pattern = WeightedKsgPattern::from_tape_idx(entry.pattern_idx).ok_or(
            mapping_invalid("tape contains an unknown weighted KSG gadget index"),
        )?;
        pattern.map_config_back(entry.row, entry.col, config)?;
    }
    Ok(())
}

/// Internal function that creates both the mapping grid and copylines.
fn embed_graph_internal(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> Result<(MappingGrid, Vec<CopyLine>), ReductionError> {
    if num_vertices == 0 {
        return Err(mapping_invalid("num_vertices must be positive"));
    }

    let copylines = create_copylines(num_vertices, edges, vertex_order)?;

    // Calculate grid dimensions
    let max_hslot = copylines.iter().map(|l| l.hslot).max().unwrap_or(1);

    let padding_twice = PADDING
        .checked_mul(2)
        .ok_or(mapping_integer_overflow("computing grid padding"))?;
    let extent = |slots: usize| {
        slots
            .checked_mul(SPACING)
            .and_then(|value| value.checked_add(2))
            .and_then(|value| value.checked_add(padding_twice))
            .ok_or(mapping_integer_overflow("computing grid dimensions"))
    };
    let rows = extent(max_hslot)?;
    let cols = extent(num_vertices - 1)?;

    let mut grid = MappingGrid::with_padding(rows, cols, SPACING, PADDING);

    // Add copy line nodes using dense locations (all cells along the L-shape)
    for line in &copylines {
        for (row, col, weight) in line.copyline_locations(PADDING, SPACING) {
            let weight = i64::try_from(weight)
                .map_err(|_| mapping_integer_overflow("converting a grid weight to i64"))?;
            grid.add_node(row, col, weight);
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

    Ok((grid, copylines))
}

/// Embed a graph into a mapping grid.
///
/// # Errors
///
/// Returns [`ReductionError`] if the vertex order, graph, or generated dimensions are invalid.
#[cfg(test)]
pub(crate) fn embed_graph(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> Result<MappingGrid, ReductionError> {
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
) -> Result<MappingResult<KsgTapeEntry>, ReductionError> {
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
) -> Result<MappingResult<KsgTapeEntry>, ReductionError> {
    let layout = pathwidth(num_vertices, edges, method);
    let vertex_order = vertex_order_from_layout(&layout);
    map_unweighted_with_order(num_vertices, edges, &vertex_order)
}

/// Map a graph with a specific vertex ordering (unweighted).
///
/// # Errors
///
/// Returns [`ReductionError`] if the vertex order, graph, or generated dimensions are invalid.
pub fn map_unweighted_with_order(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> Result<MappingResult<KsgTapeEntry>, ReductionError> {
    let (mut grid, copylines) = embed_graph_internal(num_vertices, edges, vertex_order)?;

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
    let copyline_overhead = copylines.iter().try_fold(0_i64, |total, line| {
        total
            .checked_add(mis_overhead_copyline(line, SPACING, PADDING)?)
            .ok_or(mapping_integer_overflow("summing copy-line MIS overhead"))
    })?;

    // Add MIS overhead from gadgets
    let gadget_overhead = tape.iter().try_fold(0_i64, |total, entry| {
        total
            .checked_add(tape_entry_mis_overhead(entry)?)
            .ok_or(mapping_integer_overflow("summing gadget MIS overhead"))
    })?;
    let mis_overhead = copyline_overhead
        .checked_add(gadget_overhead)
        .ok_or(mapping_integer_overflow("computing total MIS overhead"))?;

    if grid.has_unresolved_cells() {
        return Err(mapping_invalid(
            "mapping left doubled or connected cells unresolved",
        ));
    }

    // Extract positions from occupied cells.
    // In unweighted mode, all node weights are 1 — matching Julia's behavior where
    // `node(::Type{<:UnWeightedNode}, i, j, w) = Node(i, j)` ignores the weight parameter.
    let positions: Vec<(i64, i64)> = grid
        .occupied_coords()
        .into_iter()
        .filter_map(|(row, col)| {
            grid.get(row, col)
                .filter(|cell| cell.weight() > 0)
                .map(|_| {
                    Ok((
                        i64::try_from(row).map_err(|_| {
                            mapping_integer_overflow("converting a grid row to i64")
                        })?,
                        i64::try_from(col).map_err(|_| {
                            mapping_integer_overflow("converting a grid column to i64")
                        })?,
                    ))
                })
        })
        .collect::<Result<_, ReductionError>>()?;
    let node_weights = vec![1i64; positions.len()];

    Ok(MappingResult {
        positions,
        node_weights,
        grid_dimensions: grid.size(),
        kind: GridKind::Kings,
        lines: copylines,
        padding: PADDING,
        spacing: SPACING,
        mis_overhead,
        tape,
        doubled_cells,
    })
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
) -> Result<MappingResult<WeightedKsgTapeEntry>, ReductionError> {
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
) -> Result<MappingResult<WeightedKsgTapeEntry>, ReductionError> {
    let layout = pathwidth(num_vertices, edges, method);
    let vertex_order = vertex_order_from_layout(&layout);
    map_weighted_with_order(num_vertices, edges, &vertex_order)
}

/// Map a graph with a specific vertex ordering (weighted).
///
/// # Errors
///
/// Returns [`ReductionError`] if the vertex order, graph, or generated dimensions are invalid.
pub fn map_weighted_with_order(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> Result<MappingResult<WeightedKsgTapeEntry>, ReductionError> {
    let (mut grid, copylines) = embed_graph_internal(num_vertices, edges, vertex_order)?;

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
    let copyline_overhead = copylines.iter().try_fold(0_i64, |total, line| {
        let overhead = mis_overhead_copyline(line, SPACING, PADDING)?;
        let overhead = overhead.checked_mul(2).ok_or(mapping_integer_overflow(
            "doubling weighted copy-line MIS overhead",
        ))?;
        total.checked_add(overhead).ok_or(mapping_integer_overflow(
            "summing weighted copy-line MIS overhead",
        ))
    })?;

    // Add MIS overhead from weighted gadgets
    let gadget_overhead = tape.iter().try_fold(0_i64, |total, entry| {
        total
            .checked_add(weighted_tape_entry_mis_overhead(entry)?)
            .ok_or(mapping_integer_overflow(
                "summing weighted gadget MIS overhead",
            ))
    })?;
    let mis_overhead =
        copyline_overhead
            .checked_add(gadget_overhead)
            .ok_or(mapping_integer_overflow(
                "computing total weighted MIS overhead",
            ))?;

    if grid.has_unresolved_cells() {
        return Err(mapping_invalid(
            "weighted mapping left doubled or connected cells unresolved",
        ));
    }

    // Extract positions and weights from occupied cells
    let positions_and_weights = grid
        .occupied_coords()
        .into_iter()
        .filter_map(|(row, col)| {
            grid.get(row, col)
                .filter(|cell| cell.weight() > 0)
                .map(|cell| {
                    Ok((
                        (
                            i64::try_from(row).map_err(|_| {
                                mapping_integer_overflow("converting a grid row to i64")
                            })?,
                            i64::try_from(col).map_err(|_| {
                                mapping_integer_overflow("converting a grid column to i64")
                            })?,
                        ),
                        cell.weight(),
                    ))
                })
        })
        .collect::<Result<Vec<_>, ReductionError>>()?;
    let (positions, node_weights): (Vec<_>, Vec<_>) = positions_and_weights.into_iter().unzip();

    Ok(MappingResult {
        positions,
        node_weights,
        grid_dimensions: grid.size(),
        kind: GridKind::Kings,
        lines: copylines,
        padding: PADDING,
        spacing: SPACING,
        mis_overhead,
        tape,
        doubled_cells,
    })
}

#[cfg(test)]
#[path = "../../../unit_tests/rules/unitdiskmapping/ksg/mapping.rs"]
mod tests;
