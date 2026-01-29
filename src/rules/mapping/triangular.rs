//! Triangular lattice mapping support.

use super::copyline::create_copylines;
use super::gadgets::TapeEntry;
use super::grid::MappingGrid;
use super::map_graph::MappingResult;
use super::pathdecomposition::{pathwidth, vertex_order_from_layout, PathDecompositionMethod};
use crate::topology::{GridGraph, GridNode, GridType};
use serde::{Deserialize, Serialize};

const TRIANGULAR_SPACING: usize = 6;
const TRIANGULAR_PADDING: usize = 2;
const TRIANGULAR_UNIT_RADIUS: f64 = 1.1;

/// Tape entry recording a triangular gadget application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangularTapeEntry {
    /// Index of the gadget in the ruleset (0-12).
    pub gadget_idx: usize,
    /// Row where gadget was applied.
    pub row: usize,
    /// Column where gadget was applied.
    pub col: usize,
}

/// Calculate crossing point for two copylines on triangular lattice.
fn crossat_triangular(
    copylines: &[super::copyline::CopyLine],
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

    let row = (hslot - 1) * spacing + 2 + padding;
    let col = (max_vslot - 1) * spacing + 1 + padding;

    (row, col)
}

/// Cell type for source matrix pattern matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCell {
    Empty,
    Occupied,
    Connected,
}

/// Trait for triangular lattice gadgets (simplified interface).
///
/// Note: source_graph returns explicit edges (like Julia's simplegraph),
/// while mapped_graph locations should use unit disk edges.
#[allow(dead_code)]
pub trait TriangularGadget {
    fn size(&self) -> (usize, usize);
    fn cross_location(&self) -> (usize, usize);
    fn is_connected(&self) -> bool;
    /// Returns (locations, edges, pins) - edges are explicit, not unit disk.
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>);
    /// Returns (locations, pins) - use unit disk for edges on triangular lattice.
    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>);
    fn mis_overhead(&self) -> i32;

    /// Returns 1-indexed node indices that should be Connected (matching Julia).
    fn connected_nodes(&self) -> Vec<usize> {
        vec![]
    }

    /// Returns source node weights. Default is weight 2 for all nodes.
    fn source_weights(&self) -> Vec<i32> {
        let (locs, _, _) = self.source_graph();
        vec![2; locs.len()]
    }

    /// Returns mapped node weights. Default is weight 2 for all nodes.
    fn mapped_weights(&self) -> Vec<i32> {
        let (locs, _) = self.mapped_graph();
        vec![2; locs.len()]
    }

    /// Generate source matrix for pattern matching.
    /// Returns SourceCell::Connected for nodes in connected_nodes() when is_connected() is true.
    fn source_matrix(&self) -> Vec<Vec<SourceCell>> {
        let (rows, cols) = self.size();
        let (locs, _, _) = self.source_graph();
        let mut matrix = vec![vec![SourceCell::Empty; cols]; rows];

        // Build set of connected node indices (1-indexed in Julia)
        let connected_set: std::collections::HashSet<usize> = if self.is_connected() {
            self.connected_nodes().into_iter().collect()
        } else {
            std::collections::HashSet::new()
        };

        for (idx, (r, c)) in locs.iter().enumerate() {
            if *r > 0 && *c > 0 && *r <= rows && *c <= cols {
                let cell_type = if connected_set.contains(&(idx + 1)) {
                    SourceCell::Connected
                } else {
                    SourceCell::Occupied
                };
                matrix[r - 1][c - 1] = cell_type;
            }
        }
        matrix
    }

    /// Generate mapped matrix for gadget application.
    fn mapped_matrix(&self) -> Vec<Vec<bool>> {
        let (rows, cols) = self.size();
        let (locs, _) = self.mapped_graph();
        let mut matrix = vec![vec![false; cols]; rows];
        for (r, c) in locs {
            if r > 0 && c > 0 && r <= rows && c <= cols {
                matrix[r - 1][c - 1] = true;
            }
        }
        matrix
    }
}

/// Triangular cross gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriCross<const CON: bool>;

impl TriangularGadget for TriCross<true> {
    fn size(&self) -> (usize, usize) {
        (6, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(2,1), (2,2), (2,3), (2,4), (1,2), (2,2), (3,2), (4,2), (5,2), (6,2)])
        // Julia: g = simplegraph([(1,2), (2,3), (3,4), (5,6), (6,7), (7,8), (8,9), (9,10), (1,5)])
        // Note: Julia is 1-indexed, Rust is 0-indexed
        let locs = vec![
            (2, 1), // 0
            (2, 2), // 1
            (2, 3), // 2
            (2, 4), // 3
            (1, 2), // 4
            (2, 2), // 5 (duplicate of 1)
            (3, 2), // 6
            (4, 2), // 7
            (5, 2), // 8
            (6, 2), // 9
        ];
        // Convert Julia 1-indexed edges to 0-indexed
        let edges = vec![
            (0, 1), // (1,2)
            (1, 2), // (2,3)
            (2, 3), // (3,4)
            (4, 5), // (5,6)
            (5, 6), // (6,7)
            (6, 7), // (7,8)
            (7, 8), // (8,9)
            (8, 9), // (9,10)
            (0, 4), // (1,5)
        ];
        let pins = vec![0, 4, 9, 3]; // Julia: [1,5,10,4] -> 0-indexed: [0,4,9,3]
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2), (2,1), (2,2), (2,3), (1,4), (3,3), (4,2), (4,3), (5,1), (6,1), (6,2)])
        // Julia: pins = [2,1,11,5] -> 0-indexed: [1,0,10,4]
        let locs = vec![
            (1, 2), // 0
            (2, 1), // 1
            (2, 2), // 2
            (2, 3), // 3
            (1, 4), // 4
            (3, 3), // 5
            (4, 2), // 6
            (4, 3), // 7
            (5, 1), // 8
            (6, 1), // 9
            (6, 2), // 10
        ];
        let pins = vec![1, 0, 10, 4];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        1
    }

    fn connected_nodes(&self) -> Vec<usize> {
        // Julia: connected_nodes(::TriCross{true}) = [1,5]
        vec![1, 5]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        // Julia: mw = [3,2,3,3,2,2,2,2,2,2,2]
        vec![3, 2, 3, 3, 2, 2, 2, 2, 2, 2, 2]
    }
}

impl TriangularGadget for TriCross<false> {
    fn size(&self) -> (usize, usize) {
        (6, 6)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 4)
    }

    fn is_connected(&self) -> bool {
        false
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(2,2), (2,3), (2,4), (2,5), (2,6), (1,4), (2,4), (3,4), (4,4), (5,4), (6,4), (2,1)])
        // Julia: g = simplegraph([(1,2), (2,3), (3,4), (4,5), (6,7), (7,8), (8,9), (9,10), (10,11), (12,1)])
        // Julia: pins = [12,6,11,5] -> 0-indexed: [11,5,10,4]
        let locs = vec![
            (2, 2), // 0
            (2, 3), // 1
            (2, 4), // 2
            (2, 5), // 3
            (2, 6), // 4
            (1, 4), // 5
            (2, 4), // 6 (duplicate of 2)
            (3, 4), // 7
            (4, 4), // 8
            (5, 4), // 9
            (6, 4), // 10
            (2, 1), // 11
        ];
        // Convert Julia 1-indexed edges to 0-indexed
        let edges = vec![
            (0, 1),  // (1,2)
            (1, 2),  // (2,3)
            (2, 3),  // (3,4)
            (3, 4),  // (4,5)
            (5, 6),  // (6,7)
            (6, 7),  // (7,8)
            (7, 8),  // (8,9)
            (8, 9),  // (9,10)
            (9, 10), // (10,11)
            (11, 0), // (12,1)
        ];
        let pins = vec![11, 5, 10, 4];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,4), (2,2), (2,3), (2,4), (2,5), (2,6), (3,2), (3,3), (3,4), (3,5), (4,2), (4,3), (5,2), (6,3), (6,4), (2,1)])
        // Julia: pins = [16,1,15,6] -> 0-indexed: [15,0,14,5]
        let locs = vec![
            (1, 4), // 0
            (2, 2), // 1
            (2, 3), // 2
            (2, 4), // 3
            (2, 5),
            (2, 6),
            (3, 2),
            (3, 3),
            (3, 4),
            (3, 5),
            (4, 2),
            (4, 3),
            (5, 2),
            (6, 3),
            (6, 4),
            (2, 1),
        ];
        let pins = vec![15, 0, 14, 5];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        3
    }

    fn mapped_weights(&self) -> Vec<i32> {
        // Julia: mw = [3,3,2,4,2,2,2,4,3,2,2,2,2,2,2,2]
        vec![3, 3, 2, 4, 2, 2, 2, 4, 3, 2, 2, 2, 2, 2, 2, 2]
    }
}

/// Triangular turn gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriTurn;

impl TriangularGadget for TriTurn {
    fn size(&self) -> (usize, usize) {
        (3, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        false
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2), (2,2), (2,3), (2,4)])
        // Julia: g = simplegraph([(1,2), (2,3), (3,4)])
        // Julia: pins = [1,4] -> 0-indexed: [0,3]
        let locs = vec![(1, 2), (2, 2), (2, 3), (2, 4)];
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let pins = vec![0, 3];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 2), (3, 3), (2, 4)];
        let pins = vec![0, 3];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }
}

/// Triangular branch gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriBranch;

impl TriangularGadget for TriBranch {
    fn size(&self) -> (usize, usize) {
        (6, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        false
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2),(2,2),(2,3),(2,4),(3,3),(3,2),(4,2),(5,2),(6,2)])
        // Julia: g = simplegraph([(1,2), (2,3), (3, 4), (3,5), (5,6), (6,7), (7,8), (8,9)])
        // Julia: pins = [1, 4, 9] -> 0-indexed: [0, 3, 8]
        let locs = vec![
            (1, 2), // 0
            (2, 2), // 1
            (2, 3), // 2
            (2, 4), // 3
            (3, 3), // 4
            (3, 2), // 5
            (4, 2), // 6
            (5, 2), // 7
            (6, 2), // 8
        ];
        // Convert Julia 1-indexed edges to 0-indexed
        let edges = vec![
            (0, 1), // (1,2)
            (1, 2), // (2,3)
            (2, 3), // (3,4)
            (2, 4), // (3,5)
            (4, 5), // (5,6)
            (5, 6), // (6,7)
            (6, 7), // (7,8)
            (7, 8), // (8,9)
        ];
        let pins = vec![0, 3, 8];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2),(2,2),(2,4),(3,3),(4,2),(4,3),(5,1),(6,1),(6,2)])
        // Julia: pins = [1,3,9] -> 0-indexed: [0,2,8]
        let locs = vec![
            (1, 2), // 0
            (2, 2), // 1
            (2, 4), // 2
            (3, 3), // 3
            (4, 2), // 4
            (4, 3), // 5
            (5, 1), // 6
            (6, 1), // 7
            (6, 2), // 8
        ];
        let pins = vec![0, 2, 8];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }
}

/// Triangular T-connection left gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriTConLeft;

impl TriangularGadget for TriTConLeft {
    fn size(&self) -> (usize, usize) {
        (6, 5)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2), (2,1), (2,2), (3,2), (4,2), (5,2), (6,2)])
        // Julia: g = simplegraph([(1,2), (1,3), (3,4), (4,5), (5,6), (6,7)])
        let locs = vec![(1, 2), (2, 1), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2)];
        let edges = vec![(0, 1), (0, 2), (2, 3), (3, 4), (4, 5), (5, 6)];
        let pins = vec![0, 1, 6];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2), (2,1), (2,2), (2,3), (2,4), (3,3), (4,2), (4,3), (5,1), (6,1), (6,2)])
        let locs = vec![
            (1, 2),
            (2, 1),
            (2, 2),
            (2, 3),
            (2, 4),
            (3, 3),
            (4, 2),
            (4, 3),
            (5, 1),
            (6, 1),
            (6, 2),
        ];
        let pins = vec![0, 1, 10];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        // Julia: mis_overhead(::TriTCon_left) = 4
        4
    }

    fn connected_nodes(&self) -> Vec<usize> {
        // Julia: connected_nodes(::TriTCon_left) = [1, 2]
        vec![1, 2]
    }

    fn source_weights(&self) -> Vec<i32> {
        // Julia: sw = [2,1,2,2,2,2,2]
        vec![2, 1, 2, 2, 2, 2, 2]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        // Julia: mw = [3,2,3,3,1,3,2,2,2,2,2]
        vec![3, 2, 3, 3, 1, 3, 2, 2, 2, 2, 2]
    }
}

/// Triangular T-connection down gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriTConDown;

impl TriangularGadget for TriTConDown {
    fn size(&self) -> (usize, usize) {
        (3, 3)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(2,1), (2,2), (2,3), (3,2)])
        // Julia: g = simplegraph([(1,2), (2,3), (1,4)])
        let locs = vec![(2, 1), (2, 2), (2, 3), (3, 2)];
        let edges = vec![(0, 1), (1, 2), (0, 3)];
        let pins = vec![0, 3, 2];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(2,2), (3,1), (3,2), (3,3)])
        let locs = vec![(2, 2), (3, 1), (3, 2), (3, 3)];
        let pins = vec![1, 2, 3];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }
}

/// Triangular T-connection up gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriTConUp;

impl TriangularGadget for TriTConUp {
    fn size(&self) -> (usize, usize) {
        (3, 3)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2), (2,1), (2,2), (2,3)])
        // Julia: g = simplegraph([(1,2), (2,3), (3,4)])
        let locs = vec![(1, 2), (2, 1), (2, 2), (2, 3)];
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let pins = vec![1, 0, 3];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2), (2,1), (2,2), (2,3)])
        let locs = vec![(1, 2), (2, 1), (2, 2), (2, 3)];
        let pins = vec![1, 0, 3];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }
}

/// Triangular trivial turn left gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriTrivialTurnLeft;

impl TriangularGadget for TriTrivialTurnLeft {
    fn size(&self) -> (usize, usize) {
        (2, 2)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 1)];
        let edges = vec![(0, 1)];
        let pins = vec![0, 1];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 1)];
        let pins = vec![0, 1];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }

    fn mapped_weights(&self) -> Vec<i32> {
        // Julia: m1 = [1, 2] for TrivialTurn, both nodes have weight 1
        vec![1, 1]
    }
}

/// Triangular trivial turn right gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriTrivialTurnRight;

impl TriangularGadget for TriTrivialTurnRight {
    fn size(&self) -> (usize, usize) {
        (2, 2)
    }

    fn cross_location(&self) -> (usize, usize) {
        (1, 2)
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 1), (2, 2)];
        let edges = vec![(0, 1)];
        let pins = vec![0, 1];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(2, 1), (2, 2)];
        let pins = vec![0, 1];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }

    fn mapped_weights(&self) -> Vec<i32> {
        // Julia: m1 = [1, 2] for TrivialTurn, both nodes have weight 1
        vec![1, 1]
    }
}

/// Triangular end turn gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriEndTurn;

impl TriangularGadget for TriEndTurn {
    fn size(&self) -> (usize, usize) {
        (3, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        false
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2), (2,2), (2,3)])
        // Julia: g = simplegraph([(1,2), (2,3)])
        let locs = vec![(1, 2), (2, 2), (2, 3)];
        let edges = vec![(0, 1), (1, 2)];
        let pins = vec![0];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2)];
        let pins = vec![0];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -2
    }

    fn mapped_weights(&self) -> Vec<i32> {
        // Julia: m1 = [1] for EndTurn, first mapped node has weight 1
        vec![1]
    }
}

/// Triangular W-turn gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriWTurn;

impl TriangularGadget for TriWTurn {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        false
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(2,3), (2,4), (3,2),(3,3),(4,2)])
        // Julia: g = simplegraph([(1,2), (1,4), (3,4),(3,5)])
        let locs = vec![(2, 3), (2, 4), (3, 2), (3, 3), (4, 2)];
        let edges = vec![(0, 1), (0, 3), (2, 3), (2, 4)];
        let pins = vec![1, 4];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,4), (2,3), (3,2), (3,3), (4,2)])
        let locs = vec![(1, 4), (2, 3), (3, 2), (3, 3), (4, 2)];
        let pins = vec![0, 4];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }
}

/// Triangular branch fix gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriBranchFix;

impl TriangularGadget for TriBranchFix {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        false
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2), (2,2), (2,3),(3,3),(3,2),(4,2)])
        // Julia: g = simplegraph([(1,2), (2,3), (3,4),(4,5), (5,6)])
        let locs = vec![(1, 2), (2, 2), (2, 3), (3, 3), (3, 2), (4, 2)];
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)];
        let pins = vec![0, 5];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2),(2,2),(3,2),(4,2)])
        let locs = vec![(1, 2), (2, 2), (3, 2), (4, 2)];
        let pins = vec![0, 3];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -2
    }
}

/// Triangular branch fix B gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriBranchFixB;

impl TriangularGadget for TriBranchFixB {
    fn size(&self) -> (usize, usize) {
        (4, 4)
    }

    fn cross_location(&self) -> (usize, usize) {
        (2, 2)
    }

    fn is_connected(&self) -> bool {
        false
    }

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(2,3),(3,2),(3,3),(4,2)])
        // Julia: g = simplegraph([(1,3), (2,3), (2,4)])
        let locs = vec![(2, 3), (3, 2), (3, 3), (4, 2)];
        let edges = vec![(0, 2), (1, 2), (1, 3)];
        let pins = vec![0, 3];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(3,2),(4,2)])
        let locs = vec![(3, 2), (4, 2)];
        let pins = vec![0, 1];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -2
    }

    fn mapped_weights(&self) -> Vec<i32> {
        // Julia: m1 = [1] for BranchFixB, meaning first mapped node has weight 1
        vec![1, 2]
    }
}

/// Check if a triangular gadget pattern matches at position (i, j) in the grid.
/// i, j are 0-indexed row/col offsets.
fn pattern_matches_triangular<G: TriangularGadget>(
    gadget: &G,
    grid: &MappingGrid,
    i: usize,
    j: usize,
) -> bool {
    use super::grid::CellState;

    let source = gadget.source_matrix();
    let (m, n) = gadget.size();

    for r in 0..m {
        for c in 0..n {
            let grid_r = i + r;
            let grid_c = j + c;
            let expected = source[r][c];
            let actual = grid.get(grid_r, grid_c);

            match expected {
                SourceCell::Empty => {
                    // Grid cell should be empty
                    if actual.map(|c| !c.is_empty()).unwrap_or(false) {
                        return false;
                    }
                }
                SourceCell::Occupied => {
                    // Grid cell should be occupied (but not necessarily connected)
                    if !actual.map(|c| !c.is_empty()).unwrap_or(false) {
                        return false;
                    }
                }
                SourceCell::Connected => {
                    // Grid cell should be Connected specifically
                    match actual {
                        Some(CellState::Connected { .. }) => {}
                        _ => return false,
                    }
                }
            }
        }
    }
    true
}

/// Apply a triangular gadget pattern at position (i, j).
fn apply_triangular_gadget<G: TriangularGadget>(
    gadget: &G,
    grid: &mut MappingGrid,
    i: usize,
    j: usize,
) {
    use super::grid::CellState;

    let source = gadget.source_matrix();
    let (m, n) = gadget.size();

    // First, clear source pattern cells (any non-empty cell)
    for r in 0..m {
        for c in 0..n {
            if source[r][c] != SourceCell::Empty {
                grid.set(i + r, j + c, CellState::Empty);
            }
        }
    }

    // Then, add mapped pattern cells with proper weights
    let (locs, _) = gadget.mapped_graph();
    let weights = gadget.mapped_weights();
    for (idx, (r, c)) in locs.iter().enumerate() {
        if *r > 0 && *c > 0 && *r <= m && *c <= n {
            let weight = weights.get(idx).copied().unwrap_or(2);
            grid.add_node(i + r - 1, j + c - 1, weight);
        }
    }
}

/// Apply all triangular crossing gadgets to resolve crossings.
/// Returns the tape of applied gadgets.
///
/// This matches Julia's `apply_crossing_gadgets!` which iterates ALL pairs (i,j)
/// and tries to match patterns at each crossing point.
pub fn apply_triangular_crossing_gadgets(
    grid: &mut MappingGrid,
    copylines: &[super::copyline::CopyLine],
    spacing: usize,
    padding: usize,
) -> Vec<TriangularTapeEntry> {
    use std::collections::HashSet;

    let mut tape = Vec::new();
    let mut processed = HashSet::new();
    let n = copylines.len();

    // Iterate ALL pairs (matching Julia's for j=1:n, for i=1:n)
    for j in 0..n {
        for i in 0..n {
            let (cross_row, cross_col) = crossat_triangular(copylines, i, j, spacing, padding);

            // Skip if this crossing point has already been processed
            // (avoids double-applying trivial gadgets for symmetric pairs like (i,j) and (j,i))
            if processed.contains(&(cross_row, cross_col)) {
                continue;
            }

            // Try each gadget in the ruleset at this crossing point
            if let Some(entry) = try_match_triangular_gadget(grid, cross_row, cross_col) {
                tape.push(entry);
                processed.insert((cross_row, cross_col));
            }
        }
    }

    tape
}

/// Try to match and apply a triangular gadget at the crossing point.
fn try_match_triangular_gadget(
    grid: &mut MappingGrid,
    cross_row: usize,
    cross_col: usize,
) -> Option<TriangularTapeEntry> {
    // Macro to reduce repetition
    macro_rules! try_gadget {
        ($gadget:expr, $idx:expr) => {{
            let g = $gadget;
            let (cr, cc) = g.cross_location();
            if cross_row >= cr && cross_col >= cc {
                let x = cross_row - cr + 1;
                let y = cross_col - cc + 1;
                if pattern_matches_triangular(&g, grid, x, y) {
                    apply_triangular_gadget(&g, grid, x, y);
                    return Some(TriangularTapeEntry {
                        gadget_idx: $idx,
                        row: x,
                        col: y,
                    });
                }
            }
        }};
    }

    // Try gadgets in order (matching Julia's triangular_crossing_ruleset)
    // TriCross<true> must be tried BEFORE TriCross<false> because it's more specific
    // (requires Connected cells). If we try TriCross<false> first, it will match
    // even when there are Connected cells since it doesn't check for them.
    try_gadget!(TriCross::<true>, 1);
    try_gadget!(TriCross::<false>, 0);
    try_gadget!(TriTConLeft, 2);
    try_gadget!(TriTConUp, 3);
    try_gadget!(TriTConDown, 4);
    try_gadget!(TriTrivialTurnLeft, 5);
    try_gadget!(TriTrivialTurnRight, 6);
    try_gadget!(TriEndTurn, 7);
    try_gadget!(TriTurn, 8);
    try_gadget!(TriWTurn, 9);
    try_gadget!(TriBranchFix, 10);
    try_gadget!(TriBranchFixB, 11);
    try_gadget!(TriBranch, 12);

    None
}

/// Get MIS overhead for a triangular tape entry.
pub fn triangular_tape_entry_mis_overhead(entry: &TriangularTapeEntry) -> i32 {
    match entry.gadget_idx {
        0 => TriCross::<false>.mis_overhead(),
        1 => TriCross::<true>.mis_overhead(),
        2 => TriTConLeft.mis_overhead(),
        3 => TriTConUp.mis_overhead(),
        4 => TriTConDown.mis_overhead(),
        5 => TriTrivialTurnLeft.mis_overhead(),
        6 => TriTrivialTurnRight.mis_overhead(),
        7 => TriEndTurn.mis_overhead(),
        8 => TriTurn.mis_overhead(),
        9 => TriWTurn.mis_overhead(),
        10 => TriBranchFix.mis_overhead(),
        11 => TriBranchFixB.mis_overhead(),
        12 => TriBranch.mis_overhead(),
        _ => 0,
    }
}

/// Map a graph to a triangular lattice grid graph using optimal path decomposition.
///
/// # Panics
/// Panics if `num_vertices == 0`.
pub fn map_graph_triangular(num_vertices: usize, edges: &[(usize, usize)]) -> MappingResult {
    map_graph_triangular_with_method(num_vertices, edges, PathDecompositionMethod::MinhThiTrick)
}

/// Map a graph to triangular lattice using a specific path decomposition method.
pub fn map_graph_triangular_with_method(
    num_vertices: usize,
    edges: &[(usize, usize)],
    method: PathDecompositionMethod,
) -> MappingResult {
    let layout = pathwidth(num_vertices, edges, method);
    let vertex_order = vertex_order_from_layout(&layout);
    map_graph_triangular_with_order(num_vertices, edges, &vertex_order)
}

/// Map a graph to triangular lattice with specific vertex ordering.
///
/// # Panics
/// Panics if `num_vertices == 0` or if any edge vertex is not in `vertex_order`.
pub fn map_graph_triangular_with_order(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> MappingResult {
    assert!(num_vertices > 0, "num_vertices must be > 0");

    let spacing = TRIANGULAR_SPACING;
    let padding = TRIANGULAR_PADDING;

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
        for (row, col, weight) in line.dense_locations_triangular(padding, spacing) {
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

        let (row, col) = crossat_triangular(&copylines, smaller_line.vertex, larger_line.vertex, spacing, padding);

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
    let triangular_tape = apply_triangular_crossing_gadgets(&mut grid, &copylines, spacing, padding);

    // Calculate MIS overhead from copylines using the dedicated function
    // which matches Julia's mis_overhead_copyline(TriangularWeighted(), ...)
    let copyline_overhead: i32 = copylines
        .iter()
        .map(|line| super::copyline::mis_overhead_copyline_triangular(line, spacing))
        .sum();

    // Add gadget overhead
    let gadget_overhead: i32 = triangular_tape
        .iter()
        .map(triangular_tape_entry_mis_overhead)
        .sum();
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

    // Convert to GridGraph with triangular type
    let nodes: Vec<GridNode<i32>> = grid
        .occupied_coords()
        .into_iter()
        .filter_map(|(row, col)| {
            grid.get(row, col)
                .map(|cell| GridNode::new(row as i32, col as i32, cell.weight()))
        })
        .filter(|n| n.weight > 0)
        .collect();

    let grid_graph = GridGraph::new(
        GridType::Triangular {
            offset_even_cols: true,
        },
        grid.size(),
        nodes,
        TRIANGULAR_UNIT_RADIUS,
    );

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
    fn test_triangular_cross_gadget() {
        let cross = TriCross::<true>;
        assert_eq!(cross.size(), (6, 4));
    }

    #[test]
    fn test_map_graph_triangular() {
        let edges = vec![(0, 1), (1, 2)];
        let result = map_graph_triangular(3, &edges);

        assert!(result.grid_graph.num_vertices() > 0);
        assert!(matches!(
            result.grid_graph.grid_type(),
            GridType::Triangular { .. }
        ));
    }

    #[test]
    fn test_triangular_cross_connected_gadget() {
        let cross = TriCross::<true>;
        assert_eq!(TriangularGadget::size(&cross), (6, 4));
        assert_eq!(TriangularGadget::cross_location(&cross), (2, 2));
        assert!(TriangularGadget::is_connected(&cross));
        assert_eq!(TriangularGadget::mis_overhead(&cross), 1);
    }

    #[test]
    fn test_triangular_cross_disconnected_gadget() {
        let cross = TriCross::<false>;
        assert_eq!(TriangularGadget::size(&cross), (6, 6));
        assert_eq!(TriangularGadget::cross_location(&cross), (2, 4));
        assert!(!TriangularGadget::is_connected(&cross));
        assert_eq!(TriangularGadget::mis_overhead(&cross), 3);
    }

    #[test]
    fn test_triangular_turn_gadget() {
        let turn = TriTurn;
        assert_eq!(TriangularGadget::size(&turn), (3, 4));
        assert_eq!(TriangularGadget::mis_overhead(&turn), 0);
        let (_, _, pins) = TriangularGadget::source_graph(&turn);
        assert_eq!(pins.len(), 2);
    }

    #[test]
    fn test_triangular_branch_gadget() {
        let branch = TriBranch;
        assert_eq!(TriangularGadget::size(&branch), (6, 4));
        assert_eq!(TriangularGadget::mis_overhead(&branch), 0);
        let (_, _, pins) = TriangularGadget::source_graph(&branch);
        assert_eq!(pins.len(), 3);
    }

    #[test]
    fn test_map_graph_triangular_with_order() {
        let edges = vec![(0, 1), (1, 2)];
        let order = vec![2, 1, 0];
        let result = map_graph_triangular_with_order(3, &edges, &order);

        assert!(result.grid_graph.num_vertices() > 0);
        assert_eq!(result.spacing, TRIANGULAR_SPACING);
        assert_eq!(result.padding, TRIANGULAR_PADDING);
    }

    #[test]
    fn test_map_graph_triangular_single_vertex() {
        let edges: Vec<(usize, usize)> = vec![];
        let result = map_graph_triangular(1, &edges);

        assert!(result.grid_graph.num_vertices() > 0);
    }

    #[test]
    #[should_panic(expected = "num_vertices must be > 0")]
    fn test_map_graph_triangular_zero_vertices_panics() {
        let edges: Vec<(usize, usize)> = vec![];
        map_graph_triangular(0, &edges);
    }

    #[test]
    fn test_triangular_gadgets_have_valid_pins() {
        // Verify pin indices are within bounds for each gadget
        fn check_gadget<G: TriangularGadget>(gadget: &G, name: &str) {
            let (source_locs, _, source_pins) = gadget.source_graph();
            let (mapped_locs, mapped_pins) = gadget.mapped_graph();

            for &pin in &source_pins {
                assert!(
                    pin < source_locs.len(),
                    "{}: Source pin {} out of bounds (len={})",
                    name,
                    pin,
                    source_locs.len()
                );
            }

            for &pin in &mapped_pins {
                assert!(
                    pin < mapped_locs.len(),
                    "{}: Mapped pin {} out of bounds (len={})",
                    name,
                    pin,
                    mapped_locs.len()
                );
            }
        }

        check_gadget(&TriCross::<true>, "TriCross<true>");
        check_gadget(&TriCross::<false>, "TriCross<false>");
        check_gadget(&TriTurn, "TriTurn");
        check_gadget(&TriBranch, "TriBranch");
    }
}
