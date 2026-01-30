//! Gadgets for resolving crossings in grid graph embeddings.
//!
//! A gadget transforms a pattern in the source graph to an equivalent pattern
//! in the mapped graph, preserving MIS properties. Gadgets are the building
//! blocks for resolving crossings when copy-lines intersect.
//!
//! This module provides the core `Pattern` trait and helper functions.
//! Specific gadget implementations are in submodules:
//! - `gadgets_unweighted`: Square lattice unweighted gadgets

use super::grid::{CellState, MappingGrid};
use std::collections::HashMap;

// Re-export all gadget types from gadgets_unweighted (declared in mod.rs)
pub use super::gadgets_unweighted::{
    apply_crossing_gadgets, apply_simplifier_gadgets, crossing_ruleset_indices,
    tape_entry_mis_overhead, Branch, BranchFix, BranchFixB, Cross, DanglingLeg, EndTurn, Mirror,
    PatternBoxed, ReflectedGadget, RotatedGadget, SquarePattern, TCon, TapeEntry, TrivialTurn,
    Turn, WTurn,
};

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

/// Compute binary boundary config from pin values in the mapped graph.
#[allow(dead_code)]
pub fn mapped_boundary_config<P: Pattern>(pattern: &P, config: &[usize]) -> usize {
    let (_, pins) = pattern.mapped_graph();
    let mut result = 0usize;
    for (i, &pin_idx) in pins.iter().enumerate() {
        if pin_idx < config.len() && config[pin_idx] > 0 {
            result |= 1 << i;
        }
    }
    result
}

/// Map configuration back through a single gadget.
pub fn map_config_back_pattern<P: Pattern>(
    pattern: &P,
    gi: usize,
    gj: usize,
    config: &mut Vec<Vec<usize>>,
) {
    let (m, n) = pattern.size();
    let (mapped_locs, mapped_pins) = pattern.mapped_graph();
    let (source_locs, _, _) = pattern.source_graph();

    // Step 1: Extract config at mapped locations
    let mapped_config: Vec<usize> = mapped_locs
        .iter()
        .map(|&(r, c)| {
            let row = gi + r - 1;
            let col = gj + c - 1;
            config.get(row).and_then(|row_vec| row_vec.get(col)).copied().unwrap_or(0)
        })
        .collect();

    // Step 2: Compute boundary config
    let bc = {
        let mut result = 0usize;
        for (i, &pin_idx) in mapped_pins.iter().enumerate() {
            if pin_idx < mapped_config.len() && mapped_config[pin_idx] > 0 {
                result |= 1 << i;
            }
        }
        result
    };

    // Step 3: Look up source config
    let d1 = pattern.mapped_entry_to_compact();
    let d2 = pattern.source_entry_to_configs();

    let compact = d1.get(&bc).copied();
    debug_assert!(compact.is_some(), "Boundary config {} not found in mapped_entry_to_compact", bc);
    let compact = compact.unwrap_or(0);

    let source_configs = d2.get(&compact).cloned();
    debug_assert!(source_configs.is_some(), "Compact {} not found in source_entry_to_configs", compact);
    let source_configs = source_configs.unwrap_or_default();

    debug_assert!(!source_configs.is_empty(), "Empty source configs for compact {}.", compact);
    let new_config = if source_configs.is_empty() {
        vec![false; source_locs.len()]
    } else {
        source_configs[0].clone()
    };

    // Step 4: Clear gadget area
    for row in gi..gi + m {
        for col in gj..gj + n {
            if let Some(row_vec) = config.get_mut(row) {
                if let Some(cell) = row_vec.get_mut(col) {
                    *cell = 0;
                }
            }
        }
    }

    // Step 5: Write source config
    for (k, &(r, c)) in source_locs.iter().enumerate() {
        let row = gi + r - 1;
        let col = gj + c - 1;
        if let Some(rv) = config.get_mut(row) {
            if let Some(cv) = rv.get_mut(col) {
                *cv += if new_config.get(k).copied().unwrap_or(false) { 1 } else { 0 };
            }
        }
    }
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

/// Check if unmapped pattern matches (for unapply verification).
#[allow(dead_code, clippy::needless_range_loop)]
pub fn pattern_unmatches<P: Pattern>(pattern: &P, grid: &MappingGrid, i: usize, j: usize) -> bool {
    let mapped = pattern.mapped_matrix();
    let (m, n) = pattern.size();

    for r in 0..m {
        for c in 0..n {
            let grid_r = i + r;
            let grid_c = j + c;

            let expected = mapped[r][c];
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_gadget_size() {
        let cross = Cross::<false>;
        assert_eq!(Pattern::size(&cross), (4, 5));

        let cross_con = Cross::<true>;
        assert_eq!(Pattern::size(&cross_con), (3, 3));
    }

    #[test]
    fn test_turn_gadget() {
        let turn = Turn;
        assert_eq!(Pattern::size(&turn), (4, 4));
        let (locs, _, pins) = Pattern::source_graph(&turn);
        assert_eq!(pins.len(), 2);
        assert!(!locs.is_empty());
    }

    #[test]
    fn test_gadget_mis_overhead() {
        assert_eq!(Pattern::mis_overhead(&Cross::<false>), -1);
        assert_eq!(Pattern::mis_overhead(&Cross::<true>), -1);
        assert_eq!(Pattern::mis_overhead(&Turn), -1);
        assert_eq!(Pattern::mis_overhead(&TCon), 0);
        assert_eq!(Pattern::mis_overhead(&TrivialTurn), 0);
    }

    #[test]
    fn test_source_matrix_generation() {
        let turn = Turn;
        let matrix = Pattern::source_matrix(&turn);
        assert_eq!(matrix.len(), 4);
        assert_eq!(matrix[0].len(), 4);
        assert_eq!(matrix[0][1], PatternCell::Occupied);
        assert_eq!(matrix[0][0], PatternCell::Empty);
    }

    #[test]
    fn test_mapped_matrix_generation() {
        let turn = Turn;
        let matrix = Pattern::mapped_matrix(&turn);
        assert_eq!(matrix.len(), 4);
        assert_eq!(matrix[0].len(), 4);
        assert_eq!(matrix[0][1], PatternCell::Occupied);
        assert_eq!(matrix[1][2], PatternCell::Occupied);
        assert_eq!(matrix[2][3], PatternCell::Occupied);
    }

    #[test]
    fn test_rotated_gadget() {
        let tcon = TCon;
        let rotated = RotatedGadget::new(tcon, 1);
        assert_eq!(Pattern::size(&rotated), (4, 3));
    }

    #[test]
    fn test_reflected_gadget() {
        let cross = Cross::<true>;
        let reflected = ReflectedGadget::new(cross, Mirror::Y);
        assert_eq!(Pattern::size(&reflected), (3, 3));
    }

    #[test]
    fn test_dangling_leg_simplifier() {
        let leg = DanglingLeg;
        assert_eq!(Pattern::size(&leg), (4, 3));
        assert_eq!(Pattern::mis_overhead(&leg), -1);
    }
}
