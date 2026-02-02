//! Shared traits for gadget patterns in unit disk mapping.
//!
//! This module provides the core `Pattern` trait and helper functions
//! used by both King's SubGraph (KSG) and triangular lattice gadgets.

use super::grid::{CellState, MappingGrid};
use std::collections::HashMap;

/// Cell type in pattern matching.
/// Matches Julia's cell types: empty (0), occupied (1), doubled (2), connected with edge markers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PatternCell {
    #[default]
    Empty,
    Occupied,
    Doubled,
    Connected,
}

/// A gadget pattern that transforms source configurations to mapped configurations.
#[allow(clippy::type_complexity)]
pub trait Pattern: Clone + std::fmt::Debug {
    /// Size of the gadget pattern (rows, cols).
    fn size(&self) -> (usize, usize);

    /// Cross location within the gadget (1-indexed like Julia).
    fn cross_location(&self) -> (usize, usize);

    /// Whether this gadget involves connected nodes (edge markers).
    fn is_connected(&self) -> bool;

    /// Whether this is a Cross-type gadget where is_connected affects pattern matching.
    fn is_cross_gadget(&self) -> bool {
        false
    }

    /// Connected node indices (for gadgets with edge markers).
    fn connected_nodes(&self) -> Vec<usize> {
        vec![]
    }

    /// Source graph: (locations as (row, col), edges, pin_indices).
    /// Locations are 1-indexed to match Julia.
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>);

    /// Mapped graph: (locations as (row, col), pin_indices).
    /// Locations are 1-indexed to match Julia.
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>);

    /// MIS overhead when applying this gadget.
    fn mis_overhead(&self) -> i32;

    /// Weights for each node in source graph (for weighted mode).
    /// Default: all nodes have weight 2 (Julia's default for weighted gadgets).
    fn source_weights(&self) -> Vec<i32> {
        let (locs, _, _) = self.source_graph();
        vec![2; locs.len()]
    }

    /// Weights for each node in mapped graph (for weighted mode).
    /// Default: all nodes have weight 2 (Julia's default for weighted gadgets).
    fn mapped_weights(&self) -> Vec<i32> {
        let (locs, _) = self.mapped_graph();
        vec![2; locs.len()]
    }

    /// Generate source matrix for pattern matching.
    fn source_matrix(&self) -> Vec<Vec<PatternCell>> {
        let (rows, cols) = self.size();
        let (locs, _, _) = self.source_graph();
        let mut matrix = vec![vec![PatternCell::Empty; cols]; rows];

        for loc in &locs {
            let r = loc.0 - 1;
            let c = loc.1 - 1;
            if r < rows && c < cols {
                if matrix[r][c] == PatternCell::Empty {
                    matrix[r][c] = PatternCell::Occupied;
                } else {
                    matrix[r][c] = PatternCell::Doubled;
                }
            }
        }

        if self.is_connected() {
            for &idx in &self.connected_nodes() {
                if idx < locs.len() {
                    let loc = locs[idx];
                    let r = loc.0 - 1;
                    let c = loc.1 - 1;
                    if r < rows && c < cols {
                        matrix[r][c] = PatternCell::Connected;
                    }
                }
            }
        }

        matrix
    }

    /// Generate mapped matrix.
    fn mapped_matrix(&self) -> Vec<Vec<PatternCell>> {
        let (rows, cols) = self.size();
        let (locs, _) = self.mapped_graph();
        let mut matrix = vec![vec![PatternCell::Empty; cols]; rows];

        for loc in &locs {
            let r = loc.0 - 1;
            let c = loc.1 - 1;
            if r < rows && c < cols {
                if matrix[r][c] == PatternCell::Empty {
                    matrix[r][c] = PatternCell::Occupied;
                } else {
                    matrix[r][c] = PatternCell::Doubled;
                }
            }
        }

        matrix
    }

    /// Entry-to-compact mapping for configuration extraction.
    fn mapped_entry_to_compact(&self) -> HashMap<usize, usize>;

    /// Source entry to configurations for solution mapping back.
    fn source_entry_to_configs(&self) -> HashMap<usize, Vec<Vec<bool>>>;
}

/// Check if a pattern matches at position (i, j) in the grid.
/// Uses strict equality matching like Julia's Base.match.
#[allow(clippy::needless_range_loop)]
pub fn pattern_matches<P: Pattern>(pattern: &P, grid: &MappingGrid, i: usize, j: usize) -> bool {
    let source = pattern.source_matrix();
    let (m, n) = pattern.size();

    for r in 0..m {
        for c in 0..n {
            let grid_r = i + r;
            let grid_c = j + c;

            let expected = source[r][c];
            let actual = safe_get_pattern_cell(grid, grid_r, grid_c);

            if expected != actual {
                return false;
            }
        }
    }
    true
}

fn safe_get_pattern_cell(grid: &MappingGrid, row: usize, col: usize) -> PatternCell {
    let (rows, cols) = grid.size();
    if row >= rows || col >= cols {
        return PatternCell::Empty;
    }
    match grid.get(row, col) {
        Some(CellState::Empty) => PatternCell::Empty,
        Some(CellState::Occupied { .. }) => PatternCell::Occupied,
        Some(CellState::Doubled { .. }) => PatternCell::Doubled,
        Some(CellState::Connected { .. }) => PatternCell::Connected,
        None => PatternCell::Empty,
    }
}

/// Apply a gadget pattern at position (i, j).
#[allow(clippy::needless_range_loop)]
pub fn apply_gadget<P: Pattern>(pattern: &P, grid: &mut MappingGrid, i: usize, j: usize) {
    let mapped = pattern.mapped_matrix();
    let (m, n) = pattern.size();

    for r in 0..m {
        for c in 0..n {
            let grid_r = i + r;
            let grid_c = j + c;

            let cell = mapped[r][c];
            let state = match cell {
                PatternCell::Empty => CellState::Empty,
                PatternCell::Occupied => CellState::Occupied { weight: 1 },
                PatternCell::Doubled => CellState::Doubled { weight: 2 },
                PatternCell::Connected => CellState::Connected { weight: 1 },
            };
            grid.set(grid_r, grid_c, state);
        }
    }
}

/// Unapply a gadget pattern at position (i, j).
#[allow(clippy::needless_range_loop)]
pub fn unapply_gadget<P: Pattern>(pattern: &P, grid: &mut MappingGrid, i: usize, j: usize) {
    let source = pattern.source_matrix();
    let (m, n) = pattern.size();

    for r in 0..m {
        for c in 0..n {
            let grid_r = i + r;
            let grid_c = j + c;

            let cell = source[r][c];
            let state = match cell {
                PatternCell::Empty => CellState::Empty,
                PatternCell::Occupied => CellState::Occupied { weight: 1 },
                PatternCell::Doubled => CellState::Doubled { weight: 2 },
                PatternCell::Connected => CellState::Connected { weight: 1 },
            };
            grid.set(grid_r, grid_c, state);
        }
    }
}
