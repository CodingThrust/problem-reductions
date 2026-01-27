//! Gadgets for resolving crossings in grid graph embeddings.
//!
//! A gadget transforms a pattern in the source graph to an equivalent pattern
//! in the mapped graph, preserving MIS properties. Gadgets are the building
//! blocks for resolving crossings when copy-lines intersect.
//!
//! This implementation matches Julia's UnitDiskMapping.jl gadgets.jl

use super::grid::{CellState, MappingGrid};
use serde::{Deserialize, Serialize};

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
pub trait Pattern: Clone + std::fmt::Debug {
    /// Size of the gadget pattern (rows, cols).
    fn size(&self) -> (usize, usize);

    /// Cross location within the gadget (1-indexed like Julia).
    fn cross_location(&self) -> (usize, usize);

    /// Whether this gadget involves connected nodes (edge markers).
    fn is_connected(&self) -> bool;

    /// Whether this is a Cross-type gadget where is_connected affects pattern matching.
    /// Cross<false> should NOT match at Connected cells, while Cross<true> should.
    /// For non-Cross gadgets, this returns false and Connected cells always match Occupied.
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
            let r = loc.0 - 1; // Convert to 0-indexed
            let c = loc.1 - 1;
            if r < rows && c < cols {
                if matrix[r][c] == PatternCell::Empty {
                    matrix[r][c] = PatternCell::Occupied;
                } else {
                    matrix[r][c] = PatternCell::Doubled;
                }
            }
        }

        // Mark connected nodes
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
    fn mapped_entry_to_compact(&self) -> std::collections::HashMap<usize, usize>;

    /// Source entry to configurations for solution mapping back.
    fn source_entry_to_configs(&self) -> std::collections::HashMap<usize, Vec<Vec<bool>>>;
}

/// Check if a pattern matches at position (i, j) in the grid.
/// i, j are 0-indexed row/col offsets.
///
/// Note: Connected cells are treated as Occupied for matching purposes,
/// since they represent occupied cells with edge markers.
pub fn pattern_matches<P: Pattern>(pattern: &P, grid: &MappingGrid, i: usize, j: usize) -> bool {
    let source = pattern.source_matrix();
    let (m, n) = pattern.size();
    let is_cross = pattern.is_cross_gadget();
    let is_connected = pattern.is_connected();

    for r in 0..m {
        for c in 0..n {
            let grid_r = i + r;
            let grid_c = j + c;

            let expected = source[r][c];
            let actual = safe_get_pattern_cell(grid, grid_r, grid_c);

            // For Cross gadgets specifically:
            // - Cross<true> (connected) can match at Connected cells
            // - Cross<false> (unconnected) should NOT match at Connected cells
            // For all other gadgets, Connected cells are treated as Occupied.
            let matches = match (expected, actual) {
                // Exact match
                (a, b) if a == b => true,
                // Connected grid cell matching Occupied pattern:
                // - For Cross gadgets: only match if the gadget is the connected variant
                // - For non-Cross gadgets: always match (Connected ≈ Occupied)
                (PatternCell::Occupied, PatternCell::Connected) => !is_cross || is_connected,
                // Occupied grid cell matching Connected pattern
                (PatternCell::Connected, PatternCell::Occupied) => !is_cross || is_connected,
                // Otherwise no match
                _ => false,
            };

            if !matches {
                return false;
            }
        }
    }
    true
}

/// Check if unmapped pattern matches (for unapply verification).
#[allow(dead_code)]
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

// ============================================================================
// Crossing Gadgets - matching Julia's gadgets.jl exactly
// ============================================================================

/// Crossing gadget for resolving two crossing copy-lines.
///
/// `Cross<true>`: connected crossing (edges share a vertex), size (3,3)
/// `Cross<false>`: disconnected crossing, size (4,5)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cross<const CON: bool>;

impl Pattern for Cross<true> {
    fn size(&self) -> (usize, usize) {
        (3, 3)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn is_cross_gadget(&self) -> bool {
        true
    }

    fn connected_nodes(&self) -> Vec<usize> {
        vec![0, 5] // indices in source_graph locations
    }

    // Julia: locs = Node.([(2,1), (2,2), (2,3), (1,2), (2,2), (3,2)])
    // Note: (2,2) appears twice (crossing point)
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(2, 1), (2, 2), (2, 3), (1, 2), (2, 2), (3, 2)];
        let edges = vec![(0, 1), (1, 2), (3, 4), (4, 5), (0, 5)];
        let pins = vec![0, 3, 5, 2]; // [1,4,6,3] in Julia (1-indexed)
        (locs, edges, pins)
    }

    // Julia: locs = Node.([(2,1), (2,2), (2,3), (1,2), (3,2)])
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(2, 1), (2, 2), (2, 3), (1, 2), (3, 2)];
        let pins = vec![0, 3, 4, 2];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    fn mapped_entry_to_compact(&self) -> std::collections::HashMap<usize, usize> {
        // From Julia's extracting_results.jl
        [
            (5, 5),
            (12, 12),
            (8, 0),
            (1, 0),
            (0, 0),
            (6, 6),
            (11, 11),
            (9, 9),
            (14, 14),
            (3, 3),
            (7, 7),
            (4, 0),
            (13, 13),
            (15, 15),
            (2, 0),
            (10, 10),
        ]
        .into_iter()
        .collect()
    }

    fn source_entry_to_configs(&self) -> std::collections::HashMap<usize, Vec<Vec<bool>>> {
        // Simplified - returns empty for invalid configs
        let mut map = std::collections::HashMap::new();
        map.insert(0, vec![vec![false, true, false, false, true, false]]);
        map.insert(1, vec![vec![true, false, false, false, true, false]]);
        map.insert(3, vec![vec![true, false, false, true, false, false]]);
        map.insert(4, vec![vec![false, true, false, false, false, true]]);
        map.insert(6, vec![vec![false, true, false, true, false, true]]);
        map.insert(8, vec![vec![false, false, true, false, true, false]]);
        map.insert(9, vec![vec![true, false, true, false, true, false]]);
        map.insert(10, vec![vec![false, false, true, true, false, false]]);
        map.insert(11, vec![vec![true, false, true, true, false, false]]);
        map.insert(12, vec![vec![false, false, true, false, false, true]]);
        map.insert(14, vec![vec![false, false, true, true, false, true]]);
        // 5, 7, 13, 15 have empty configs (invalid boundary combinations)
        map.insert(5, vec![]);
        map.insert(7, vec![]);
        map.insert(13, vec![]);
        map.insert(15, vec![]);
        map.insert(2, vec![vec![false, true, false, true, false, false]]);
        map
    }
}

impl Pattern for Cross<false> {
    fn size(&self) -> (usize, usize) {
        (4, 5)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 3)
    }

    fn is_connected(&self) -> bool {
        false
    }

    fn is_cross_gadget(&self) -> bool {
        true
    }

    // Julia: locs = Node.([(2,1), (2,2), (2,3), (2,4), (2,5), (1,3), (2,3), (3,3), (4,3)])
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![
            (2, 1),
            (2, 2),
            (2, 3),
            (2, 4),
            (2, 5),
            (1, 3),
            (2, 3),
            (3, 3),
            (4, 3),
        ];
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (5, 6), (6, 7), (7, 8)];
        let pins = vec![0, 5, 8, 4]; // [1,6,9,5] in Julia
        (locs, edges, pins)
    }

    // Julia: locs = Node.([(2,1), (2,2), (2,3), (2,4), (2,5), (1,3), (3,3), (4,3), (3,2), (3,4)])
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![
            (2, 1),
            (2, 2),
            (2, 3),
            (2, 4),
            (2, 5),
            (1, 3),
            (3, 3),
            (4, 3),
            (3, 2),
            (3, 4),
        ];
        let pins = vec![0, 5, 7, 4];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    fn mapped_entry_to_compact(&self) -> std::collections::HashMap<usize, usize> {
        [
            (5, 4),
            (12, 4),
            (8, 0),
            (1, 0),
            (0, 0),
            (6, 0),
            (11, 11),
            (9, 9),
            (14, 2),
            (3, 2),
            (7, 2),
            (4, 4),
            (13, 13),
            (15, 11),
            (2, 2),
            (10, 2),
        ]
        .into_iter()
        .collect()
    }

    fn source_entry_to_configs(&self) -> std::collections::HashMap<usize, Vec<Vec<bool>>> {
        let mut map = std::collections::HashMap::new();
        // From Julia's extracting_results.jl - simplified version
        map.insert(
            0,
            vec![
                vec![
                    false, true, false, true, false, false, false, true, false,
                ],
                vec![
                    false, true, false, true, false, false, true, false, false,
                ],
            ],
        );
        map.insert(
            2,
            vec![vec![
                false, true, false, true, false, true, false, true, false,
            ]],
        );
        map.insert(
            4,
            vec![vec![
                false, true, false, true, false, false, true, false, true,
            ]],
        );
        map.insert(
            9,
            vec![
                vec![true, false, true, false, true, false, false, true, false],
                vec![true, false, true, false, true, false, true, false, false],
            ],
        );
        map.insert(
            11,
            vec![vec![
                true, false, true, false, true, true, false, true, false,
            ]],
        );
        map.insert(
            13,
            vec![vec![
                true, false, true, false, true, false, true, false, true,
            ]],
        );
        // Fill others with reasonable defaults
        for i in [1, 3, 5, 6, 7, 8, 10, 12, 14, 15] {
            if !map.contains_key(&i) {
                map.insert(i, vec![]);
            }
        }
        map
    }
}

/// Turn gadget for 90-degree turns in copy-lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Turn;

impl Pattern for Turn {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (3, 2)
    }

    fn is_connected(&self) -> bool {
        false
    }

    // Julia: locs = Node.([(1,2), (2,2), (3,2), (3,3), (3,4)])
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 2), (3, 2), (3, 3), (3, 4)];
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
        let pins = vec![0, 4]; // [1,5] in Julia
        (locs, edges, pins)
    }

    // Julia: locs = Node.([(1,2), (2,3), (3,4)])
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 3), (3, 4)];
        let pins = vec![0, 2];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    fn mapped_entry_to_compact(&self) -> std::collections::HashMap<usize, usize> {
        [(0, 0), (2, 0), (3, 3), (1, 0)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> std::collections::HashMap<usize, Vec<Vec<bool>>> {
        let mut map = std::collections::HashMap::new();
        map.insert(0, vec![vec![false, true, false, true, false]]);
        map.insert(
            1,
            vec![
                vec![true, false, true, false, false],
                vec![true, false, false, true, false],
            ],
        );
        map.insert(
            2,
            vec![
                vec![false, true, false, false, true],
                vec![false, false, true, false, true],
            ],
        );
        map.insert(3, vec![vec![true, false, true, false, true]]);
        map
    }
}

/// W-shaped turn gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WTurn;

impl Pattern for WTurn {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        false
    }

    // Julia: locs = Node.([(2,3), (2,4), (3,2),(3,3),(4,2)])
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(2, 3), (2, 4), (3, 2), (3, 3), (4, 2)];
        let edges = vec![(0, 1), (0, 3), (2, 3), (2, 4)];
        let pins = vec![1, 4]; // [2,5] in Julia
        (locs, edges, pins)
    }

    // Julia: locs = Node.([(2,4),(3,3),(4,2)])
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(2, 4), (3, 3), (4, 2)];
        let pins = vec![0, 2];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    fn mapped_entry_to_compact(&self) -> std::collections::HashMap<usize, usize> {
        [(0, 0), (2, 0), (3, 3), (1, 0)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> std::collections::HashMap<usize, Vec<Vec<bool>>> {
        let mut map = std::collections::HashMap::new();
        map.insert(0, vec![vec![true, false, true, false, false]]);
        map.insert(
            1,
            vec![
                vec![false, true, false, true, false],
                vec![false, true, true, false, false],
            ],
        );
        map.insert(
            2,
            vec![
                vec![false, false, false, true, true],
                vec![true, false, false, false, true],
            ],
        );
        map.insert(3, vec![vec![false, true, false, true, true]]);
        map
    }
}

/// Branch gadget for T-junctions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Branch;

impl Pattern for Branch {
    fn size(&self) -> (usize, usize) {
        (5, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (3, 2)
    }

    fn is_connected(&self) -> bool {
        false
    }

    // Julia: locs = Node.([(1,2), (2,2), (3,2),(3,3),(3,4),(4,3),(4,2),(5,2)])
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![
            (1, 2),
            (2, 2),
            (3, 2),
            (3, 3),
            (3, 4),
            (4, 3),
            (4, 2),
            (5, 2),
        ];
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (3, 5), (5, 6), (6, 7)];
        let pins = vec![0, 4, 7]; // [1,5,8] in Julia
        (locs, edges, pins)
    }

    // Julia: locs = Node.([(1,2), (2,3), (3,2),(3,4),(4,3),(5,2)])
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 3), (3, 2), (3, 4), (4, 3), (5, 2)];
        let pins = vec![0, 3, 5];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    fn mapped_entry_to_compact(&self) -> std::collections::HashMap<usize, usize> {
        [
            (0, 0),
            (4, 0),
            (5, 5),
            (6, 6),
            (2, 0),
            (7, 7),
            (3, 3),
            (1, 0),
        ]
        .into_iter()
        .collect()
    }

    fn source_entry_to_configs(&self) -> std::collections::HashMap<usize, Vec<Vec<bool>>> {
        let mut map = std::collections::HashMap::new();
        map.insert(
            0,
            vec![vec![
                false, true, false, true, false, false, true, false,
            ]],
        );
        map.insert(
            3,
            vec![
                vec![true, false, true, false, true, false, true, false],
                vec![true, false, true, false, true, true, false, false],
            ],
        );
        map.insert(
            5,
            vec![vec![
                true, false, true, false, false, true, false, true,
            ]],
        );
        map.insert(
            6,
            vec![
                vec![false, false, true, false, true, true, false, true],
                vec![false, true, false, false, true, true, false, true],
            ],
        );
        map.insert(
            7,
            vec![vec![
                true, false, true, false, true, true, false, true,
            ]],
        );
        for i in [1, 2, 4] {
            map.insert(i, vec![]);
        }
        map
    }
}

/// Branch fix gadget for simplifying branches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchFix;

impl Pattern for BranchFix {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        false
    }

    // Julia: locs = Node.([(1,2), (2,2), (2,3),(3,3),(3,2),(4,2)])
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 2), (2, 3), (3, 3), (3, 2), (4, 2)];
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)];
        let pins = vec![0, 5]; // [1,6] in Julia
        (locs, edges, pins)
    }

    // Julia: locs = Node.([(1,2),(2,2),(3,2),(4,2)])
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 2), (3, 2), (4, 2)];
        let pins = vec![0, 3];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    fn mapped_entry_to_compact(&self) -> std::collections::HashMap<usize, usize> {
        [(0, 0), (2, 2), (3, 1), (1, 1)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> std::collections::HashMap<usize, Vec<Vec<bool>>> {
        let mut map = std::collections::HashMap::new();
        map.insert(
            0,
            vec![
                vec![false, true, false, true, false, false],
                vec![false, true, false, false, true, false],
                vec![false, false, true, false, true, false],
            ],
        );
        map.insert(1, vec![vec![true, false, true, false, true, false]]);
        map.insert(2, vec![vec![false, true, false, true, false, true]]);
        map.insert(
            3,
            vec![
                vec![true, false, false, true, false, true],
                vec![true, false, true, false, false, true],
            ],
        );
        map
    }
}

/// T-connection gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TCon;

impl Pattern for TCon {
    fn size(&self) -> (usize, usize) {
        (3, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn connected_nodes(&self) -> Vec<usize> {
        vec![0, 1]
    }

    // Julia: locs = Node.([(1,2), (2,1), (2,2),(3,2)])
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 1), (2, 2), (3, 2)];
        let edges = vec![(0, 1), (0, 2), (2, 3)];
        let pins = vec![0, 1, 3]; // [1,2,4] in Julia
        (locs, edges, pins)
    }

    // Julia: locs = Node.([(1,2),(2,1),(2,3),(3,2)])
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 1), (2, 3), (3, 2)];
        let pins = vec![0, 1, 3];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }

    fn mapped_entry_to_compact(&self) -> std::collections::HashMap<usize, usize> {
        [
            (0, 0),
            (4, 0),
            (5, 5),
            (6, 6),
            (2, 2),
            (7, 7),
            (3, 3),
            (1, 0),
        ]
        .into_iter()
        .collect()
    }

    fn source_entry_to_configs(&self) -> std::collections::HashMap<usize, Vec<Vec<bool>>> {
        let mut map = std::collections::HashMap::new();
        map.insert(0, vec![vec![false, false, true, false]]);
        map.insert(1, vec![vec![true, false, false, false]]);
        map.insert(2, vec![vec![false, true, true, false]]);
        map.insert(4, vec![vec![false, false, false, true]]);
        map.insert(5, vec![vec![true, false, false, true]]);
        map.insert(6, vec![vec![false, true, false, true]]);
        map.insert(3, vec![]);
        map.insert(7, vec![]);
        map
    }
}

/// Trivial turn gadget for simple diagonal turns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrivialTurn;

impl Pattern for TrivialTurn {
    fn size(&self) -> (usize, usize) {
        (2, 2)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn connected_nodes(&self) -> Vec<usize> {
        vec![0, 1]
    }

    // Julia: locs = Node.([(1,2), (2,1)])
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 1)];
        let edges = vec![(0, 1)];
        let pins = vec![0, 1];
        (locs, edges, pins)
    }

    // Julia: locs = Node.([(1,2),(2,1)])
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 1)];
        let pins = vec![0, 1];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }

    fn mapped_entry_to_compact(&self) -> std::collections::HashMap<usize, usize> {
        [(0, 0), (2, 2), (3, 3), (1, 1)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> std::collections::HashMap<usize, Vec<Vec<bool>>> {
        let mut map = std::collections::HashMap::new();
        map.insert(0, vec![vec![false, false]]);
        map.insert(1, vec![vec![true, false]]);
        map.insert(2, vec![vec![false, true]]);
        map.insert(3, vec![]);
        map
    }
}

/// End turn gadget for line terminations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndTurn;

impl Pattern for EndTurn {
    fn size(&self) -> (usize, usize) {
        (3, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        false
    }

    // Julia: locs = Node.([(1,2), (2,2), (2,3)])
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 2), (2, 3)];
        let edges = vec![(0, 1), (1, 2)];
        let pins = vec![0]; // [1] in Julia
        (locs, edges, pins)
    }

    // Julia: locs = Node.([(1,2)])
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2)];
        let pins = vec![0];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    fn mapped_entry_to_compact(&self) -> std::collections::HashMap<usize, usize> {
        [(0, 0), (1, 1)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> std::collections::HashMap<usize, Vec<Vec<bool>>> {
        let mut map = std::collections::HashMap::new();
        map.insert(
            0,
            vec![vec![false, false, true], vec![false, true, false]],
        );
        map.insert(1, vec![vec![true, false, true]]);
        map
    }
}

/// Alternate branch fix gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchFixB;

impl Pattern for BranchFixB {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        false
    }

    // Julia: locs = Node.([(2,3),(3,2),(3,3),(4,2)])
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(2, 3), (3, 2), (3, 3), (4, 2)];
        let edges = vec![(0, 2), (1, 2), (1, 3)];
        let pins = vec![0, 3]; // [1,4] in Julia
        (locs, edges, pins)
    }

    // Julia: locs = Node.([(3,2),(4,2)])
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(3, 2), (4, 2)];
        let pins = vec![0, 1];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    fn mapped_entry_to_compact(&self) -> std::collections::HashMap<usize, usize> {
        [(0, 0), (2, 2), (3, 3), (1, 1)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> std::collections::HashMap<usize, Vec<Vec<bool>>> {
        let mut map = std::collections::HashMap::new();
        map.insert(
            0,
            vec![vec![false, false, true, false], vec![false, true, false, false]],
        );
        map.insert(1, vec![vec![true, true, false, false]]);
        map.insert(2, vec![vec![false, false, true, true]]);
        map.insert(3, vec![vec![true, false, false, true]]);
        map
    }
}

// ============================================================================
// Rotated and Reflected Gadgets
// ============================================================================

/// A rotated version of a gadget.
#[derive(Debug, Clone)]
pub struct RotatedGadget<G: Pattern> {
    pub gadget: G,
    /// Number of 90-degree clockwise rotations (0-3).
    pub n: usize,
}

impl<G: Pattern> RotatedGadget<G> {
    pub fn new(gadget: G, n: usize) -> Self {
        Self { gadget, n: n % 4 }
    }
}

fn rotate90(loc: (i32, i32)) -> (i32, i32) {
    (-loc.1, loc.0)
}

fn rotate_around_center(loc: (usize, usize), center: (usize, usize), n: usize) -> (i32, i32) {
    let mut dx = loc.0 as i32 - center.0 as i32;
    let mut dy = loc.1 as i32 - center.1 as i32;
    for _ in 0..n {
        let (nx, ny) = rotate90((dx, dy));
        dx = nx;
        dy = ny;
    }
    (center.0 as i32 + dx, center.1 as i32 + dy)
}

impl<G: Pattern> Pattern for RotatedGadget<G> {
    fn size(&self) -> (usize, usize) {
        let (m, n) = self.gadget.size();
        if self.n % 2 == 0 {
            (m, n)
        } else {
            (n, m)
        }
    }

    fn cross_location(&self) -> (usize, usize) {
        let center = self.gadget.cross_location();
        let (m, n) = self.gadget.size();

        // Calculate rotated cross location
        let rotated = rotate_around_center(center, center, self.n);

        // Calculate offset to keep pattern in positive coordinates
        let corners = [(1, 1), (m, n)];
        let rotated_corners: Vec<_> = corners
            .iter()
            .map(|&c| rotate_around_center(c, center, self.n))
            .collect();

        let min_r = rotated_corners.iter().map(|c| c.0).min().unwrap();
        let min_c = rotated_corners.iter().map(|c| c.1).min().unwrap();

        let offset_r = 1 - min_r;
        let offset_c = 1 - min_c;

        (
            (rotated.0 + offset_r) as usize,
            (rotated.1 + offset_c) as usize,
        )
    }

    fn is_connected(&self) -> bool {
        self.gadget.is_connected()
    }

    fn is_cross_gadget(&self) -> bool {
        self.gadget.is_cross_gadget()
    }

    fn connected_nodes(&self) -> Vec<usize> {
        self.gadget.connected_nodes()
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let (locs, edges, pins) = self.gadget.source_graph();
        let center = self.gadget.cross_location();
        let (m, n) = self.gadget.size();

        // Calculate offset
        let corners = [(1usize, 1usize), (m, n)];
        let rotated_corners: Vec<_> = corners
            .iter()
            .map(|&c| rotate_around_center(c, center, self.n))
            .collect();

        let min_r = rotated_corners.iter().map(|c| c.0).min().unwrap();
        let min_c = rotated_corners.iter().map(|c| c.1).min().unwrap();

        let offset_r = 1 - min_r;
        let offset_c = 1 - min_c;

        let new_locs: Vec<_> = locs
            .into_iter()
            .map(|loc| {
                let rotated = rotate_around_center(loc, center, self.n);
                (
                    (rotated.0 + offset_r) as usize,
                    (rotated.1 + offset_c) as usize,
                )
            })
            .collect();

        (new_locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let (locs, pins) = self.gadget.mapped_graph();
        let center = self.gadget.cross_location();
        let (m, n) = self.gadget.size();

        // Calculate offset
        let corners = [(1usize, 1usize), (m, n)];
        let rotated_corners: Vec<_> = corners
            .iter()
            .map(|&c| rotate_around_center(c, center, self.n))
            .collect();

        let min_r = rotated_corners.iter().map(|c| c.0).min().unwrap();
        let min_c = rotated_corners.iter().map(|c| c.1).min().unwrap();

        let offset_r = 1 - min_r;
        let offset_c = 1 - min_c;

        let new_locs: Vec<_> = locs
            .into_iter()
            .map(|loc| {
                let rotated = rotate_around_center(loc, center, self.n);
                (
                    (rotated.0 + offset_r) as usize,
                    (rotated.1 + offset_c) as usize,
                )
            })
            .collect();

        (new_locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        self.gadget.mis_overhead()
    }

    fn mapped_entry_to_compact(&self) -> std::collections::HashMap<usize, usize> {
        self.gadget.mapped_entry_to_compact()
    }

    fn source_entry_to_configs(&self) -> std::collections::HashMap<usize, Vec<Vec<bool>>> {
        self.gadget.source_entry_to_configs()
    }
}

/// Mirror axis for reflection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mirror {
    X,
    Y,
    Diag,
    OffDiag,
}

/// A reflected version of a gadget.
#[derive(Debug, Clone)]
pub struct ReflectedGadget<G: Pattern> {
    pub gadget: G,
    pub mirror: Mirror,
}

impl<G: Pattern> ReflectedGadget<G> {
    pub fn new(gadget: G, mirror: Mirror) -> Self {
        Self { gadget, mirror }
    }
}

fn reflect(loc: (i32, i32), mirror: Mirror) -> (i32, i32) {
    match mirror {
        Mirror::X => (loc.0, -loc.1),
        Mirror::Y => (-loc.0, loc.1),
        Mirror::Diag => (-loc.1, -loc.0),
        Mirror::OffDiag => (loc.1, loc.0),
    }
}

fn reflect_around_center(loc: (usize, usize), center: (usize, usize), mirror: Mirror) -> (i32, i32) {
    let dx = loc.0 as i32 - center.0 as i32;
    let dy = loc.1 as i32 - center.1 as i32;
    let (nx, ny) = reflect((dx, dy), mirror);
    (center.0 as i32 + nx, center.1 as i32 + ny)
}

impl<G: Pattern> Pattern for ReflectedGadget<G> {
    fn size(&self) -> (usize, usize) {
        let (m, n) = self.gadget.size();
        match self.mirror {
            Mirror::X | Mirror::Y => (m, n),
            Mirror::Diag | Mirror::OffDiag => (n, m),
        }
    }

    fn cross_location(&self) -> (usize, usize) {
        let center = self.gadget.cross_location();
        let (m, n) = self.gadget.size();

        let reflected = reflect_around_center(center, center, self.mirror);

        let corners = [(1, 1), (m, n)];
        let reflected_corners: Vec<_> = corners
            .iter()
            .map(|&c| reflect_around_center(c, center, self.mirror))
            .collect();

        let min_r = reflected_corners.iter().map(|c| c.0).min().unwrap();
        let min_c = reflected_corners.iter().map(|c| c.1).min().unwrap();

        let offset_r = 1 - min_r;
        let offset_c = 1 - min_c;

        (
            (reflected.0 + offset_r) as usize,
            (reflected.1 + offset_c) as usize,
        )
    }

    fn is_connected(&self) -> bool {
        self.gadget.is_connected()
    }

    fn is_cross_gadget(&self) -> bool {
        self.gadget.is_cross_gadget()
    }

    fn connected_nodes(&self) -> Vec<usize> {
        self.gadget.connected_nodes()
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let (locs, edges, pins) = self.gadget.source_graph();
        let center = self.gadget.cross_location();
        let (m, n) = self.gadget.size();

        let corners = [(1usize, 1usize), (m, n)];
        let reflected_corners: Vec<_> = corners
            .iter()
            .map(|&c| reflect_around_center(c, center, self.mirror))
            .collect();

        let min_r = reflected_corners.iter().map(|c| c.0).min().unwrap();
        let min_c = reflected_corners.iter().map(|c| c.1).min().unwrap();

        let offset_r = 1 - min_r;
        let offset_c = 1 - min_c;

        let new_locs: Vec<_> = locs
            .into_iter()
            .map(|loc| {
                let reflected = reflect_around_center(loc, center, self.mirror);
                (
                    (reflected.0 + offset_r) as usize,
                    (reflected.1 + offset_c) as usize,
                )
            })
            .collect();

        (new_locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let (locs, pins) = self.gadget.mapped_graph();
        let center = self.gadget.cross_location();
        let (m, n) = self.gadget.size();

        let corners = [(1usize, 1usize), (m, n)];
        let reflected_corners: Vec<_> = corners
            .iter()
            .map(|&c| reflect_around_center(c, center, self.mirror))
            .collect();

        let min_r = reflected_corners.iter().map(|c| c.0).min().unwrap();
        let min_c = reflected_corners.iter().map(|c| c.1).min().unwrap();

        let offset_r = 1 - min_r;
        let offset_c = 1 - min_c;

        let new_locs: Vec<_> = locs
            .into_iter()
            .map(|loc| {
                let reflected = reflect_around_center(loc, center, self.mirror);
                (
                    (reflected.0 + offset_r) as usize,
                    (reflected.1 + offset_c) as usize,
                )
            })
            .collect();

        (new_locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        self.gadget.mis_overhead()
    }

    fn mapped_entry_to_compact(&self) -> std::collections::HashMap<usize, usize> {
        self.gadget.mapped_entry_to_compact()
    }

    fn source_entry_to_configs(&self) -> std::collections::HashMap<usize, Vec<Vec<bool>>> {
        self.gadget.source_entry_to_configs()
    }
}

// ============================================================================
// Simplifier Patterns
// ============================================================================

/// Dangling leg simplifier - removes 3-node dangling chains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DanglingLeg;

impl Pattern for DanglingLeg {
    fn size(&self) -> (usize, usize) {
        (4, 3)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2) // center
    }

    fn is_connected(&self) -> bool {
        false
    }

    // Source: 3-node vertical line at column 2
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(2, 2), (3, 2), (4, 2)];
        let edges = vec![(0, 1), (1, 2)];
        let pins = vec![2]; // bottom node is boundary
        (locs, edges, pins)
    }

    // Mapped: single node
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(4, 2)];
        let pins = vec![0];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -1
    }

    fn mapped_entry_to_compact(&self) -> std::collections::HashMap<usize, usize> {
        [(0, 0), (1, 1)].into_iter().collect()
    }

    fn source_entry_to_configs(&self) -> std::collections::HashMap<usize, Vec<Vec<bool>>> {
        let mut map = std::collections::HashMap::new();
        map.insert(
            0,
            vec![vec![true, false, false], vec![false, true, false]],
        );
        map.insert(1, vec![vec![true, false, true]]);
        map
    }
}

// ============================================================================
// Crossing ruleset and apply functions
// ============================================================================

/// A tape entry recording a gadget application.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TapeEntry {
    /// Index of the pattern in the ruleset.
    pub pattern_idx: usize,
    /// Position where pattern was applied.
    pub row: usize,
    pub col: usize,
}

/// The default crossing ruleset for square lattice.
/// Matches Julia's crossing_ruleset exactly.
#[allow(dead_code)]
pub fn crossing_ruleset_indices() -> Vec<usize> {
    // Returns indices into the full pattern list
    // 0: Cross<false>
    // 1: Turn
    // 2: WTurn
    // 3: Branch
    // 4: BranchFix
    // 5: TCon
    // 6: TrivialTurn
    // 7: RotatedGadget(TCon, 1)
    // 8: ReflectedGadget(Cross<true>, Y)
    // 9: ReflectedGadget(TrivialTurn, Y)
    // 10: BranchFixB
    // 11: EndTurn
    // 12: ReflectedGadget(RotatedGadget(TCon, 1), Y)
    (0..13).collect()
}

/// Apply all crossing gadgets to the grid.
/// Returns the modified grid and a tape of applied gadgets.
pub fn apply_crossing_gadgets(
    grid: &mut MappingGrid,
    copylines: &[super::copyline::CopyLine],
) -> Vec<TapeEntry> {
    use std::collections::HashSet;

    let mut tape = Vec::new();
    let mut processed = HashSet::new();
    let n = copylines.len();

    // Iterate through all pairs of vertices (like Julia's for j=1:n, for i=1:n)
    for j in 0..n {
        for i in 0..n {
            // Calculate cross position using actual copyline hslot values
            // Julia: crossat(ug, i, j) looks up lines by vertex ID and uses hslot
            let (cross_row, cross_col) = crossat(grid, copylines, i, j);

            // Skip if this crossing point was already processed
            // (prevents duplicate gadget applications at same location)
            if processed.contains(&(cross_row, cross_col)) {
                continue;
            }

            // Try each pattern in the ruleset
            if let Some((pattern_idx, row, col)) =
                try_match_and_apply_crossing(grid, cross_row, cross_col)
            {
                tape.push(TapeEntry {
                    pattern_idx,
                    row,
                    col,
                });
                // Mark this crossing point as processed
                processed.insert((cross_row, cross_col));
            }
        }
    }

    tape
}

/// Calculate the crossing point for two copylines.
/// This matches Julia's crossat function.
///
/// Julia's crossat uses the position in the lines array (which is ordered by vertex_order).
/// Our copylines are indexed by vertex ID, so we use vslot (which is the position in vertex_order)
/// to determine which line is "first" (smaller vslot = earlier in vertex_order).
fn crossat(
    grid: &MappingGrid,
    copylines: &[super::copyline::CopyLine],
    v: usize,
    w: usize,
) -> (usize, usize) {
    // Get the copylines for vertices v and w
    let line_v = copylines.get(v);
    let line_w = copylines.get(w);

    match (line_v, line_w) {
        (Some(lv), Some(lw)) => {
            // Use vslot to determine order (vslot = position in vertex_order)
            // The line with smaller vslot came first in vertex_order
            let (line_first, line_second) = if lv.vslot < lw.vslot {
                (lv, lw)
            } else {
                (lw, lv)
            };

            // Use hslot from the line that came first (smaller vslot)
            let hslot = line_first.hslot;
            // Use the larger vslot for column calculation
            let max_vslot = line_second.vslot;

            let spacing = grid.spacing();
            let padding = grid.padding();

            let row = (hslot - 1) * spacing + 2 + padding;
            let col = (max_vslot - 1) * spacing + 1 + padding;

            (row, col)
        }
        _ => (0, 0), // Invalid - should not happen
    }
}

/// Try to match and apply a crossing gadget at the given position.
/// Returns the pattern index and position if successful.
fn try_match_and_apply_crossing(
    grid: &mut MappingGrid,
    cross_row: usize,
    cross_col: usize,
) -> Option<(usize, usize, usize)> {
    // Try Cross<false> (most common)
    let cross_false = Cross::<false>;
    let cl = Pattern::cross_location(&cross_false);
    if cross_row >= cl.0 && cross_col >= cl.1 {
        let x = cross_row - cl.0 + 1;
        let y = cross_col - cl.1 + 1;
        if pattern_matches(&cross_false, grid, x, y) {
            apply_gadget(&cross_false, grid, x, y);
            return Some((0, x, y));
        }
    }

    // Try Turn
    let turn = Turn;
    let cl = Pattern::cross_location(&turn);
    if cross_row >= cl.0 && cross_col >= cl.1 {
        let x = cross_row - cl.0 + 1;
        let y = cross_col - cl.1 + 1;
        if pattern_matches(&turn, grid, x, y) {
            apply_gadget(&turn, grid, x, y);
            return Some((1, x, y));
        }
    }

    // Try WTurn
    let wturn = WTurn;
    let cl = Pattern::cross_location(&wturn);
    if cross_row >= cl.0 && cross_col >= cl.1 {
        let x = cross_row - cl.0 + 1;
        let y = cross_col - cl.1 + 1;
        if pattern_matches(&wturn, grid, x, y) {
            apply_gadget(&wturn, grid, x, y);
            return Some((2, x, y));
        }
    }

    // Try Branch
    let branch = Branch;
    let cl = Pattern::cross_location(&branch);
    if cross_row >= cl.0 && cross_col >= cl.1 {
        let x = cross_row - cl.0 + 1;
        let y = cross_col - cl.1 + 1;
        if pattern_matches(&branch, grid, x, y) {
            apply_gadget(&branch, grid, x, y);
            return Some((3, x, y));
        }
    }

    // Try BranchFix
    let branchfix = BranchFix;
    let cl = Pattern::cross_location(&branchfix);
    if cross_row >= cl.0 && cross_col >= cl.1 {
        let x = cross_row - cl.0 + 1;
        let y = cross_col - cl.1 + 1;
        if pattern_matches(&branchfix, grid, x, y) {
            apply_gadget(&branchfix, grid, x, y);
            return Some((4, x, y));
        }
    }

    // Try TCon
    let tcon = TCon;
    let cl = Pattern::cross_location(&tcon);
    if cross_row >= cl.0 && cross_col >= cl.1 {
        let x = cross_row - cl.0 + 1;
        let y = cross_col - cl.1 + 1;
        if pattern_matches(&tcon, grid, x, y) {
            apply_gadget(&tcon, grid, x, y);
            return Some((5, x, y));
        }
    }

    // Try TrivialTurn
    let trivialturn = TrivialTurn;
    let cl = Pattern::cross_location(&trivialturn);
    if cross_row >= cl.0 && cross_col >= cl.1 {
        let x = cross_row - cl.0 + 1;
        let y = cross_col - cl.1 + 1;
        if pattern_matches(&trivialturn, grid, x, y) {
            apply_gadget(&trivialturn, grid, x, y);
            return Some((6, x, y));
        }
    }

    // Try RotatedGadget(TCon, 1)
    let rotated_tcon = RotatedGadget::new(TCon, 1);
    let cl = Pattern::cross_location(&rotated_tcon);
    if cross_row >= cl.0 && cross_col >= cl.1 {
        let x = cross_row - cl.0 + 1;
        let y = cross_col - cl.1 + 1;
        if pattern_matches(&rotated_tcon, grid, x, y) {
            apply_gadget(&rotated_tcon, grid, x, y);
            return Some((7, x, y));
        }
    }

    // Try ReflectedGadget(Cross<true>, Y)
    let reflected_cross = ReflectedGadget::new(Cross::<true>, Mirror::Y);
    let cl = Pattern::cross_location(&reflected_cross);
    if cross_row >= cl.0 && cross_col >= cl.1 {
        let x = cross_row - cl.0 + 1;
        let y = cross_col - cl.1 + 1;
        if pattern_matches(&reflected_cross, grid, x, y) {
            apply_gadget(&reflected_cross, grid, x, y);
            return Some((8, x, y));
        }
    }

    // Try ReflectedGadget(TrivialTurn, Y)
    let reflected_trivial = ReflectedGadget::new(TrivialTurn, Mirror::Y);
    let cl = Pattern::cross_location(&reflected_trivial);
    if cross_row >= cl.0 && cross_col >= cl.1 {
        let x = cross_row - cl.0 + 1;
        let y = cross_col - cl.1 + 1;
        if pattern_matches(&reflected_trivial, grid, x, y) {
            apply_gadget(&reflected_trivial, grid, x, y);
            return Some((9, x, y));
        }
    }

    // Try BranchFixB
    let branchfixb = BranchFixB;
    let cl = Pattern::cross_location(&branchfixb);
    if cross_row >= cl.0 && cross_col >= cl.1 {
        let x = cross_row - cl.0 + 1;
        let y = cross_col - cl.1 + 1;
        if pattern_matches(&branchfixb, grid, x, y) {
            apply_gadget(&branchfixb, grid, x, y);
            return Some((10, x, y));
        }
    }

    // Try EndTurn
    let endturn = EndTurn;
    let cl = Pattern::cross_location(&endturn);
    if cross_row >= cl.0 && cross_col >= cl.1 {
        let x = cross_row - cl.0 + 1;
        let y = cross_col - cl.1 + 1;
        if pattern_matches(&endturn, grid, x, y) {
            apply_gadget(&endturn, grid, x, y);
            return Some((11, x, y));
        }
    }

    // Try ReflectedGadget(RotatedGadget(TCon, 1), Y)
    let reflected_rotated_tcon = ReflectedGadget::new(RotatedGadget::new(TCon, 1), Mirror::Y);
    let cl = Pattern::cross_location(&reflected_rotated_tcon);
    if cross_row >= cl.0 && cross_col >= cl.1 {
        let x = cross_row - cl.0 + 1;
        let y = cross_col - cl.1 + 1;
        if pattern_matches(&reflected_rotated_tcon, grid, x, y) {
            apply_gadget(&reflected_rotated_tcon, grid, x, y);
            return Some((12, x, y));
        }
    }

    None
}

/// Get MIS overhead for a tape entry.
pub fn tape_entry_mis_overhead(entry: &TapeEntry) -> i32 {
    match entry.pattern_idx {
        0 => Pattern::mis_overhead(&Cross::<false>),
        1 => Pattern::mis_overhead(&Turn),
        2 => Pattern::mis_overhead(&WTurn),
        3 => Pattern::mis_overhead(&Branch),
        4 => Pattern::mis_overhead(&BranchFix),
        5 => Pattern::mis_overhead(&TCon),
        6 => Pattern::mis_overhead(&TrivialTurn),
        7 => Pattern::mis_overhead(&RotatedGadget::new(TCon, 1)),
        8 => Pattern::mis_overhead(&ReflectedGadget::new(Cross::<true>, Mirror::Y)),
        9 => Pattern::mis_overhead(&ReflectedGadget::new(TrivialTurn, Mirror::Y)),
        10 => Pattern::mis_overhead(&BranchFixB),
        11 => Pattern::mis_overhead(&EndTurn),
        12 => Pattern::mis_overhead(&ReflectedGadget::new(RotatedGadget::new(TCon, 1), Mirror::Y)),
        // Simplifier patterns (100+) - DanglingLeg and its rotations/reflections
        idx if idx >= 100 => Pattern::mis_overhead(&DanglingLeg),
        _ => 0,
    }
}

/// Apply simplifier gadgets to the grid.
pub fn apply_simplifier_gadgets(
    grid: &mut MappingGrid,
    nrepeat: usize,
) -> Vec<TapeEntry> {
    let mut tape = Vec::new();
    let (rows, cols) = grid.size();

    // Get all rotations and reflections of DanglingLeg
    let patterns = rotated_and_reflected_danglinleg();

    for _ in 0..nrepeat {
        for (pattern_idx, pattern) in patterns.iter().enumerate() {
            for j in 0..cols {
                for i in 0..rows {
                    if pattern_matches_boxed(pattern.as_ref(), grid, i, j) {
                        apply_gadget_boxed(pattern.as_ref(), grid, i, j);
                        tape.push(TapeEntry {
                            pattern_idx: 100 + pattern_idx, // Offset to distinguish from crossing gadgets
                            row: i,
                            col: j,
                        });
                    }
                }
            }
        }
    }

    tape
}

fn rotated_and_reflected_danglinleg() -> Vec<Box<dyn PatternBoxed>> {
    vec![
        Box::new(DanglingLeg),
        Box::new(RotatedGadget::new(DanglingLeg, 1)),
        Box::new(RotatedGadget::new(DanglingLeg, 2)),
        Box::new(RotatedGadget::new(DanglingLeg, 3)),
        Box::new(ReflectedGadget::new(DanglingLeg, Mirror::X)),
        Box::new(ReflectedGadget::new(DanglingLeg, Mirror::Y)),
    ]
}

/// Helper trait for boxing patterns.
pub trait PatternBoxed: std::fmt::Debug {
    fn size(&self) -> (usize, usize);
    fn cross_location(&self) -> (usize, usize);
    fn is_connected(&self) -> bool;
    fn source_matrix(&self) -> Vec<Vec<PatternCell>>;
    fn mapped_matrix(&self) -> Vec<Vec<PatternCell>>;
    fn mis_overhead(&self) -> i32;
}

impl<P: Pattern> PatternBoxed for P {
    fn size(&self) -> (usize, usize) {
        Pattern::size(self)
    }

    fn cross_location(&self) -> (usize, usize) {
        Pattern::cross_location(self)
    }

    fn is_connected(&self) -> bool {
        Pattern::is_connected(self)
    }

    fn source_matrix(&self) -> Vec<Vec<PatternCell>> {
        Pattern::source_matrix(self)
    }

    fn mapped_matrix(&self) -> Vec<Vec<PatternCell>> {
        Pattern::mapped_matrix(self)
    }

    fn mis_overhead(&self) -> i32 {
        Pattern::mis_overhead(self)
    }
}

fn pattern_matches_boxed(pattern: &dyn PatternBoxed, grid: &MappingGrid, i: usize, j: usize) -> bool {
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

fn apply_gadget_boxed(pattern: &dyn PatternBoxed, grid: &mut MappingGrid, i: usize, j: usize) {
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

// ============================================================================
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

        // Check that node at (1,2) is occupied (0-indexed: row 0, col 1)
        assert_eq!(matrix[0][1], PatternCell::Occupied);
        // Check that (0,0) is empty
        assert_eq!(matrix[0][0], PatternCell::Empty);
    }

    #[test]
    fn test_mapped_matrix_generation() {
        let turn = Turn;
        let matrix = Pattern::mapped_matrix(&turn);
        assert_eq!(matrix.len(), 4);
        assert_eq!(matrix[0].len(), 4);

        // Mapped graph has 3 nodes: (1,2), (2,3), (3,4)
        // 0-indexed: (0,1), (1,2), (2,3)
        assert_eq!(matrix[0][1], PatternCell::Occupied);
        assert_eq!(matrix[1][2], PatternCell::Occupied);
        assert_eq!(matrix[2][3], PatternCell::Occupied);
    }

    #[test]
    fn test_rotated_gadget() {
        let tcon = TCon;
        let rotated = RotatedGadget::new(tcon, 1);

        // Original TCon is 3x4, rotated 90 degrees should be 4x3
        assert_eq!(Pattern::size(&rotated), (4, 3));
    }

    #[test]
    fn test_reflected_gadget() {
        let cross = Cross::<true>;
        let reflected = ReflectedGadget::new(cross, Mirror::Y);

        // Cross<true> is 3x3, reflection should keep same size
        assert_eq!(Pattern::size(&reflected), (3, 3));
    }

    #[test]
    fn test_dangling_leg_simplifier() {
        let leg = DanglingLeg;
        assert_eq!(Pattern::size(&leg), (4, 3));
        assert_eq!(Pattern::mis_overhead(&leg), -1);
    }
}
