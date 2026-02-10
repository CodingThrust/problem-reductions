//! Weighted triangular lattice gadgets with WeightedTri prefix.
//!
//! This module contains gadget definitions for triangular lattice mapping.
//! All gadgets use weighted mode (weight 2 for standard nodes).

use super::super::grid::{CellState, MappingGrid};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Cell type for source matrix pattern matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCell {
    Empty,
    Occupied,
    Connected,
}

/// Tape entry recording a weighted triangular gadget application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedTriTapeEntry {
    /// Index of the gadget in the ruleset (0-12).
    pub gadget_idx: usize,
    /// Row where gadget was applied.
    pub row: usize,
    /// Column where gadget was applied.
    pub col: usize,
}

/// Trait for weighted triangular lattice gadgets.
///
/// Note: source_graph returns explicit edges (like Julia's simplegraph),
/// while mapped_graph locations should use unit disk edges.
#[allow(dead_code)]
#[allow(clippy::type_complexity)]
pub trait WeightedTriangularGadget {
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
        let connected_set: HashSet<usize> = if self.is_connected() {
            self.connected_nodes().into_iter().collect()
        } else {
            HashSet::new()
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

/// Weighted triangular cross gadget - matches Julia's Cross gadget with weights.
///
/// This uses the same structure as Julia's base Cross gadget, with all nodes
/// having weight 2 (the standard weighted mode).
/// mis_overhead = base_overhead * 2 = -1 * 2 = -2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedTriCross<const CON: bool>;

impl WeightedTriangularGadget for WeightedTriCross<true> {
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
        // Note: Julia has duplicate (2,2) at indices 2 and 6
        let locs = vec![
            (2, 1),
            (2, 2),
            (2, 3),
            (2, 4),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 2),
        ];
        // Julia: g = simplegraph([(1,2), (2,3), (3,4), (5,6), (6,7), (7,8), (8,9), (9,10), (1,5)])
        // 0-indexed: [(0,1), (1,2), (2,3), (4,5), (5,6), (6,7), (7,8), (8,9), (0,4)]
        let edges = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 8),
            (8, 9),
            (0, 4),
        ];
        // Julia: pins = [1,5,10,4] -> 0-indexed: [0,4,9,3]
        let pins = vec![0, 4, 9, 3];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2), (2,1), (2,2), (2,3), (1,4), (3,3), (4,2), (4,3), (5,1), (6,1), (6,2)])
        let locs = vec![
            (1, 2),
            (2, 1),
            (2, 2),
            (2, 3),
            (1, 4),
            (3, 3),
            (4, 2),
            (4, 3),
            (5, 1),
            (6, 1),
            (6, 2),
        ];
        // Julia: pins = [2,1,11,5] -> 0-indexed: [1,0,10,4]
        let pins = vec![1, 0, 10, 4];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        1
    }

    fn connected_nodes(&self) -> Vec<usize> {
        // Julia: connected_nodes = [1,5] (1-indexed, keep as-is for source_matrix)
        vec![1, 5]
    }

    fn source_weights(&self) -> Vec<i32> {
        // Julia: sw = [2,2,2,2,2,2,2,2,2,2]
        vec![2; 10]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        // Julia: mw = [3,2,3,3,2,2,2,2,2,2,2]
        vec![3, 2, 3, 3, 2, 2, 2, 2, 2, 2, 2]
    }
}

impl WeightedTriangularGadget for WeightedTriCross<false> {
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
        // Note: Julia has duplicate (2,4) at indices 3 and 7
        let locs = vec![
            (2, 2),
            (2, 3),
            (2, 4),
            (2, 5),
            (2, 6),
            (1, 4),
            (2, 4),
            (3, 4),
            (4, 4),
            (5, 4),
            (6, 4),
            (2, 1),
        ];
        // Julia: g = simplegraph([(1,2), (2,3), (3,4), (4,5), (6,7), (7,8), (8,9), (9,10), (10,11), (12,1)])
        // 0-indexed: [(0,1), (1,2), (2,3), (3,4), (5,6), (6,7), (7,8), (8,9), (9,10), (11,0)]
        let edges = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (5, 6),
            (6, 7),
            (7, 8),
            (8, 9),
            (9, 10),
            (11, 0),
        ];
        // Julia: pins = [12,6,11,5] -> 0-indexed: [11,5,10,4]
        let pins = vec![11, 5, 10, 4];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,4), (2,2), (2,3), (2,4), (2,5), (2,6), (3,2), (3,3), (3,4), (3,5), (4,2), (4,3), (5,2), (6,3), (6,4), (2,1)])
        let locs = vec![
            (1, 4),
            (2, 2),
            (2, 3),
            (2, 4),
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
        // Julia: pins = [16,1,15,6] -> 0-indexed: [15,0,14,5]
        let pins = vec![15, 0, 14, 5];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        3
    }

    fn source_weights(&self) -> Vec<i32> {
        vec![2; 12]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        vec![3, 3, 2, 4, 2, 2, 2, 4, 3, 2, 2, 2, 2, 2, 2, 2]
    }
}

/// Weighted triangular turn gadget - matches Julia's TriTurn gadget.
///
/// Julia TriTurn (from triangular.jl):
/// - size = (3, 4)
/// - cross_location = (2, 2)
/// - 4 source nodes, 4 mapped nodes
/// - mis_overhead = -2 (weighted)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedTriTurn;

impl WeightedTriangularGadget for WeightedTriTurn {
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
        let locs = vec![(1, 2), (2, 2), (2, 3), (2, 4)];
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        // Julia: pins = [1,4] -> 0-indexed: [0,3]
        let pins = vec![0, 3];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2), (2,2), (3,3), (2,4)])
        let locs = vec![(1, 2), (2, 2), (3, 3), (2, 4)];
        // Julia: pins = [1,4] -> 0-indexed: [0,3]
        let pins = vec![0, 3];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }

    fn source_weights(&self) -> Vec<i32> {
        vec![2; 4]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        vec![2; 4]
    }
}

/// Weighted triangular branch gadget - matches Julia's Branch gadget with weights.
///
/// Julia Branch:
/// - size = (5, 4)
/// - cross_location = (3, 2)
/// - 8 source nodes, 6 mapped nodes
/// - mis_overhead = -1 (base), -2 (weighted)
/// - For weighted mode: source node 4 has weight 3, mapped node 2 has weight 3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedTriBranch;

impl WeightedTriangularGadget for WeightedTriBranch {
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
        let locs = vec![
            (1, 2),
            (2, 2),
            (2, 3),
            (2, 4),
            (3, 3),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 2),
        ];
        // Julia: g = simplegraph([(1,2), (2,3), (3, 4), (3,5), (5,6), (6,7), (7,8), (8,9)])
        // 0-indexed: [(0,1), (1,2), (2,3), (2,4), (4,5), (5,6), (6,7), (7,8)]
        let edges = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (2, 4),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 8),
        ];
        // Julia: pins = [1, 4, 9] -> 0-indexed: [0, 3, 8]
        let pins = vec![0, 3, 8];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2),(2,2),(2,4),(3,3),(4,2),(4,3),(5,1),(6,1),(6,2)])
        let locs = vec![
            (1, 2),
            (2, 2),
            (2, 4),
            (3, 3),
            (4, 2),
            (4, 3),
            (5, 1),
            (6, 1),
            (6, 2),
        ];
        // Julia: pins = [1,3,9] -> 0-indexed: [0,2,8]
        let pins = vec![0, 2, 8];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }

    fn source_weights(&self) -> Vec<i32> {
        // Julia: sw = [2,2,3,2,2,2,2,2,2]
        vec![2, 2, 3, 2, 2, 2, 2, 2, 2]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        // Julia: mw = [2,2,2,3,2,2,2,2,2]
        vec![2, 2, 2, 3, 2, 2, 2, 2, 2]
    }
}

/// Weighted triangular T-connection left gadget - matches Julia's TCon gadget with weights.
///
/// Julia TCon:
/// - size = (3, 4)
/// - cross_location = (2, 2)
/// - 4 source nodes, 4 mapped nodes, 3 pins
/// - connected_nodes = [1, 2] -> [0, 1]
/// - mis_overhead = 0 (both base and weighted)
/// - For weighted mode: source node 2 has weight 1, mapped node 2 has weight 1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedTriTConLeft;

impl WeightedTriangularGadget for WeightedTriTConLeft {
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
        let locs = vec![(1, 2), (2, 1), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2)];
        // Julia: g = simplegraph([(1,2), (1,3), (3,4), (4,5), (5,6), (6,7)])
        // 0-indexed: [(0,1), (0,2), (2,3), (3,4), (4,5), (5,6)]
        let edges = vec![(0, 1), (0, 2), (2, 3), (3, 4), (4, 5), (5, 6)];
        // Julia: pins = [1,2,7] -> 0-indexed: [0,1,6]
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
        // Julia: pins = [1,2,11] -> 0-indexed: [0,1,10]
        let pins = vec![0, 1, 10];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        4
    }

    fn connected_nodes(&self) -> Vec<usize> {
        // Julia: connected_nodes = [1,2] (1-indexed, keep as-is for source_matrix)
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

/// Weighted triangular T-connection down gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedTriTConDown;

impl WeightedTriangularGadget for WeightedTriTConDown {
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
        // 0-indexed: [(0,1), (1,2), (0,3)]
        let locs = vec![(2, 1), (2, 2), (2, 3), (3, 2)];
        let edges = vec![(0, 1), (1, 2), (0, 3)];
        // Julia: pins = [1,4,3] -> 0-indexed: [0,3,2]
        let pins = vec![0, 3, 2];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(2,2), (3,1), (3,2), (3,3)])
        let locs = vec![(2, 2), (3, 1), (3, 2), (3, 3)];
        // Julia: pins = [2,3,4] -> 0-indexed: [1,2,3]
        let pins = vec![1, 2, 3];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }

    fn connected_nodes(&self) -> Vec<usize> {
        // Julia: connected_nodes = [1, 4] (1-indexed, keep as-is for source_matrix)
        vec![1, 4]
    }

    fn source_weights(&self) -> Vec<i32> {
        vec![2, 2, 2, 1]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        vec![2, 2, 3, 2]
    }
}

/// Weighted triangular T-connection up gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedTriTConUp;

impl WeightedTriangularGadget for WeightedTriTConUp {
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
        // 0-indexed: [(0,1), (1,2), (2,3)]
        let locs = vec![(1, 2), (2, 1), (2, 2), (2, 3)];
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        // Julia: pins = [2,1,4] -> 0-indexed: [1,0,3]
        let pins = vec![1, 0, 3];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2), (2,1), (2,2), (2,3)])
        let locs = vec![(1, 2), (2, 1), (2, 2), (2, 3)];
        // Julia: pins = [2,1,4] -> 0-indexed: [1,0,3]
        let pins = vec![1, 0, 3];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }

    fn connected_nodes(&self) -> Vec<usize> {
        // Julia: connected_nodes = [1, 2] (1-indexed, keep as-is for source_matrix)
        vec![1, 2]
    }

    fn source_weights(&self) -> Vec<i32> {
        vec![1, 2, 2, 2]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        vec![3, 2, 2, 2]
    }
}

/// Weighted triangular trivial turn left gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedTriTrivialTurnLeft;

impl WeightedTriangularGadget for WeightedTriTrivialTurnLeft {
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
        // Julia: locs = Node.([(1,2), (2,1)])
        let locs = vec![(1, 2), (2, 1)];
        let edges = vec![(0, 1)];
        let pins = vec![0, 1];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2),(2,1)])
        let locs = vec![(1, 2), (2, 1)];
        let pins = vec![0, 1];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }

    fn connected_nodes(&self) -> Vec<usize> {
        // Julia: connected_nodes = [1, 2] (1-indexed, keep as-is for source_matrix)
        vec![1, 2]
    }

    fn source_weights(&self) -> Vec<i32> {
        vec![1, 1]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        vec![1, 1]
    }
}

/// Weighted triangular trivial turn right gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedTriTrivialTurnRight;

impl WeightedTriangularGadget for WeightedTriTrivialTurnRight {
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
        // Julia: locs = Node.([(1,1), (2,2)])
        let locs = vec![(1, 1), (2, 2)];
        let edges = vec![(0, 1)];
        let pins = vec![0, 1];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(2,1),(2,2)])
        let locs = vec![(2, 1), (2, 2)];
        let pins = vec![0, 1];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }

    fn connected_nodes(&self) -> Vec<usize> {
        // Julia: connected_nodes = [1, 2] (1-indexed, keep as-is for source_matrix)
        vec![1, 2]
    }

    fn source_weights(&self) -> Vec<i32> {
        vec![1, 1]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        vec![1, 1]
    }
}

/// Weighted triangular end turn gadget - matches Julia's EndTurn gadget with weights.
///
/// Julia EndTurn:
/// - size = (3, 4)
/// - cross_location = (2, 2)
/// - 3 source nodes, 1 mapped node, 1 pin
/// - mis_overhead = -1 (base), -2 (weighted)
/// - For weighted mode: source node 3 has weight 1, mapped node 1 has weight 1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedTriEndTurn;

impl WeightedTriangularGadget for WeightedTriEndTurn {
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
        // Julia: pins = [1] -> 0-indexed: [0]
        let pins = vec![0];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2)])
        let locs = vec![(1, 2)];
        // Julia: pins = [1] -> 0-indexed: [0]
        let pins = vec![0];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -2
    }

    fn source_weights(&self) -> Vec<i32> {
        vec![2, 2, 1]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        vec![1]
    }
}

/// Weighted triangular W-turn gadget - matches Julia's WTurn gadget with weights.
///
/// Julia WTurn:
/// - size = (4, 4)
/// - cross_location = (2, 2)
/// - 5 source nodes, 3 mapped nodes
/// - mis_overhead = -1 (base), -2 (weighted)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedTriWTurn;

impl WeightedTriangularGadget for WeightedTriWTurn {
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
        let locs = vec![(2, 3), (2, 4), (3, 2), (3, 3), (4, 2)];
        // Julia: g = simplegraph([(1,2), (1,4), (3,4),(3,5)])
        // 0-indexed: [(0,1), (0,3), (2,3), (2,4)]
        let edges = vec![(0, 1), (0, 3), (2, 3), (2, 4)];
        // Julia: pins = [2, 5] -> 0-indexed: [1, 4]
        let pins = vec![1, 4];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,4), (2,3), (3,2), (3,3), (4,2)])
        let locs = vec![(1, 4), (2, 3), (3, 2), (3, 3), (4, 2)];
        // Julia: pins = [1, 5] -> 0-indexed: [0, 4]
        let pins = vec![0, 4];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
    }

    fn source_weights(&self) -> Vec<i32> {
        vec![2; 5]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        vec![2; 5]
    }
}

/// Weighted triangular branch fix gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedTriBranchFix;

impl WeightedTriangularGadget for WeightedTriBranchFix {
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
        // 0-indexed: [(0,1), (1,2), (2,3), (3,4), (4,5)]
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)];
        // Julia: pins = [1, 6] -> 0-indexed: [0, 5]
        let pins = vec![0, 5];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(1,2),(2,2),(3,2),(4,2)])
        let locs = vec![(1, 2), (2, 2), (3, 2), (4, 2)];
        // Julia: pins = [1, 4] -> 0-indexed: [0, 3]
        let pins = vec![0, 3];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -2
    }

    fn source_weights(&self) -> Vec<i32> {
        vec![2; 6]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        vec![2; 4]
    }
}

/// Weighted triangular branch fix B gadget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedTriBranchFixB;

impl WeightedTriangularGadget for WeightedTriBranchFixB {
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
        // 0-indexed: [(0,2), (1,2), (1,3)]
        let edges = vec![(0, 2), (1, 2), (1, 3)];
        // Julia: pins = [1, 4] -> 0-indexed: [0, 3]
        let pins = vec![0, 3];
        (locs, edges, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        // Julia: locs = Node.([(3,2),(4,2)])
        let locs = vec![(3, 2), (4, 2)];
        // Julia: pins = [1, 2] -> 0-indexed: [0, 1]
        let pins = vec![0, 1];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        -2
    }

    fn source_weights(&self) -> Vec<i32> {
        vec![2; 4]
    }

    fn mapped_weights(&self) -> Vec<i32> {
        vec![2; 2]
    }
}

// ============================================================================
// Pattern MaximumMatching and Application Functions
// ============================================================================

/// Check if a weighted triangular gadget pattern matches at position (i, j) in the grid.
/// i, j are 0-indexed row/col offsets (pattern top-left corner).
///
/// For weighted triangular mode, this also checks that weights match the expected
/// source_weights from the gadget. This matches Julia's behavior where WeightedGadget
/// source matrices include weights and match() uses == comparison.
#[allow(clippy::needless_range_loop)]
fn pattern_matches<G: WeightedTriangularGadget>(
    gadget: &G,
    grid: &MappingGrid,
    i: usize,
    j: usize,
) -> bool {
    let source = gadget.source_matrix();
    let (m, n) = gadget.size();

    // First pass: check cell states (empty/occupied/connected)
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

    // Second pass: check weights for weighted triangular mode
    // Julia's WeightedGadget stores source_weights and match() compares cells including weight
    let (locs, _, _) = gadget.source_graph();
    let weights = gadget.source_weights();

    for (idx, (loc_r, loc_c)) in locs.iter().enumerate() {
        // source_graph locations are 1-indexed, convert to grid position
        let grid_r = i + loc_r - 1;
        let grid_c = j + loc_c - 1;
        let expected_weight = weights[idx];

        if let Some(cell) = grid.get(grid_r, grid_c) {
            if cell.weight() != expected_weight {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

/// Apply a weighted triangular gadget pattern at position (i, j).
/// i, j are 0-indexed row/col offsets (pattern top-left corner).
#[allow(clippy::needless_range_loop)]
fn apply_gadget<G: WeightedTriangularGadget>(
    gadget: &G,
    grid: &mut MappingGrid,
    i: usize,
    j: usize,
) {
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
    // locs are 1-indexed within the pattern's bounding box
    let (locs, _) = gadget.mapped_graph();
    let weights = gadget.mapped_weights();
    for (idx, (r, c)) in locs.iter().enumerate() {
        if *r > 0 && *c > 0 && *r <= m && *c <= n {
            let weight = weights.get(idx).copied().unwrap_or(2);
            // Convert 1-indexed pattern pos to 0-indexed grid pos
            grid.add_node(i + r - 1, j + c - 1, weight);
        }
    }
}

/// Try to match and apply a weighted triangular gadget at the crossing point.
fn try_match_gadget(
    grid: &mut MappingGrid,
    cross_row: usize,
    cross_col: usize,
) -> Option<WeightedTriTapeEntry> {
    // Macro to reduce repetition
    macro_rules! try_gadget {
        ($gadget:expr, $idx:expr) => {{
            let g = $gadget;
            let (cr, cc) = g.cross_location();
            if cross_row >= cr && cross_col >= cc {
                let x = cross_row - cr + 1;
                let y = cross_col - cc + 1;
                if pattern_matches(&g, grid, x, y) {
                    apply_gadget(&g, grid, x, y);
                    return Some(WeightedTriTapeEntry {
                        gadget_idx: $idx,
                        row: x,
                        col: y,
                    });
                }
            }
        }};
    }

    // Try gadgets in order (matching Julia's triangular_crossing_ruleset)
    // WeightedTriCross<true> must be tried BEFORE WeightedTriCross<false> because it's more specific
    // (requires Connected cells). If we try WeightedTriCross<false> first, it will match
    // even when there are Connected cells since it doesn't check for them.
    try_gadget!(WeightedTriCross::<true>, 1);
    try_gadget!(WeightedTriCross::<false>, 0);
    try_gadget!(WeightedTriTConLeft, 2);
    try_gadget!(WeightedTriTConUp, 3);
    try_gadget!(WeightedTriTConDown, 4);
    try_gadget!(WeightedTriTrivialTurnLeft, 5);
    try_gadget!(WeightedTriTrivialTurnRight, 6);
    try_gadget!(WeightedTriEndTurn, 7);
    try_gadget!(WeightedTriTurn, 8);
    try_gadget!(WeightedTriWTurn, 9);
    try_gadget!(WeightedTriBranchFix, 10);
    try_gadget!(WeightedTriBranchFixB, 11);
    try_gadget!(WeightedTriBranch, 12);

    None
}

/// Calculate crossing point for two copylines on triangular lattice.
fn crossat(
    copylines: &[super::super::copyline::CopyLine],
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

/// Apply all weighted triangular crossing gadgets to resolve crossings.
/// Returns the tape of applied gadgets.
///
/// This matches Julia's `apply_crossing_gadgets!` which iterates ALL pairs (i,j)
/// and tries to match patterns at each crossing point.
pub fn apply_crossing_gadgets(
    grid: &mut MappingGrid,
    copylines: &[super::super::copyline::CopyLine],
    spacing: usize,
    padding: usize,
) -> Vec<WeightedTriTapeEntry> {
    let mut tape = Vec::new();
    let mut processed = HashSet::new();
    let n = copylines.len();

    // Iterate ALL pairs (matching Julia's for j=1:n, for i=1:n)
    for j in 0..n {
        for i in 0..n {
            let (cross_row, cross_col) = crossat(copylines, i, j, spacing, padding);

            // Skip if this crossing point has already been processed
            // (avoids double-applying trivial gadgets for symmetric pairs like (i,j) and (j,i))
            if processed.contains(&(cross_row, cross_col)) {
                continue;
            }

            // Try each gadget in the ruleset at this crossing point
            if let Some(entry) = try_match_gadget(grid, cross_row, cross_col) {
                tape.push(entry);
                processed.insert((cross_row, cross_col));
            }
        }
    }

    tape
}

/// Apply simplifier gadgets to the weighted triangular grid.
/// This matches Julia's `apply_simplifier_gadgets!` for TriangularWeighted mode.
///
/// The weighted DanglingLeg pattern matches 3 nodes in a line where:
/// - The end node (closest to center) has weight 1
/// - The other two nodes have weight 2
///   After simplification, only 1 node remains with weight 1.
#[allow(dead_code)]
pub fn apply_simplifier_gadgets(
    grid: &mut MappingGrid,
    nrepeat: usize,
) -> Vec<WeightedTriTapeEntry> {
    let mut tape = Vec::new();
    let (rows, cols) = grid.size();

    for _ in 0..nrepeat {
        // Try all 4 directions at each position
        // Pattern functions handle bounds checking internally
        for j in 0..cols {
            for i in 0..rows {
                // Down pattern (4x3): needs i+3 < rows, j+2 < cols
                if try_apply_dangling_leg_down(grid, i, j) {
                    tape.push(WeightedTriTapeEntry {
                        gadget_idx: 100, // DanglingLeg down
                        row: i,
                        col: j,
                    });
                }
                // Up pattern (4x3): needs i+3 < rows, j+2 < cols
                if try_apply_dangling_leg_up(grid, i, j) {
                    tape.push(WeightedTriTapeEntry {
                        gadget_idx: 101, // DanglingLeg up
                        row: i,
                        col: j,
                    });
                }
                // Right pattern (3x4): needs i+2 < rows, j+3 < cols
                if try_apply_dangling_leg_right(grid, i, j) {
                    tape.push(WeightedTriTapeEntry {
                        gadget_idx: 102, // DanglingLeg right
                        row: i,
                        col: j,
                    });
                }
                // Left pattern (3x4): needs i+2 < rows, j+3 < cols
                if try_apply_dangling_leg_left(grid, i, j) {
                    tape.push(WeightedTriTapeEntry {
                        gadget_idx: 103, // DanglingLeg left
                        row: i,
                        col: j,
                    });
                }
            }
        }
    }

    tape
}

/// Try to apply DanglingLeg pattern going downward.
/// Julia pattern (4 rows x 3 cols, 0-indexed at (i,j)):
///   . . .    <- row i: empty, empty, empty
///   . o .    <- row i+1: empty, occupied(w=1), empty  [dangling end]
///   . @ .    <- row i+2: empty, occupied(w=2), empty
///   . @ .    <- row i+3: empty, occupied(w=2), empty
/// After: only node at (i+3, j+1) remains with weight 1
#[allow(dead_code)]
fn try_apply_dangling_leg_down(grid: &mut MappingGrid, i: usize, j: usize) -> bool {
    let (rows, cols) = grid.size();

    // Need at least 4 rows and 3 cols from position (i, j)
    if i + 3 >= rows || j + 2 >= cols {
        return false;
    }

    // Helper to check if cell at (row, col) is empty
    let is_empty = |row: usize, col: usize| -> bool { !grid.is_occupied(row, col) };

    // Helper to check if cell has specific weight
    let has_weight = |row: usize, col: usize, w: i32| -> bool {
        grid.get(row, col).is_some_and(|c| c.weight() == w)
    };

    // Row i (row 1 of pattern): all 3 cells must be empty
    if !is_empty(i, j) || !is_empty(i, j + 1) || !is_empty(i, j + 2) {
        return false;
    }

    // Row i+1 (row 2): empty, occupied(w=1), empty
    if !is_empty(i + 1, j) || !has_weight(i + 1, j + 1, 1) || !is_empty(i + 1, j + 2) {
        return false;
    }

    // Row i+2 (row 3): empty, occupied(w=2), empty
    if !is_empty(i + 2, j) || !has_weight(i + 2, j + 1, 2) || !is_empty(i + 2, j + 2) {
        return false;
    }

    // Row i+3 (row 4): empty, occupied(w=2), empty
    if !is_empty(i + 3, j) || !has_weight(i + 3, j + 1, 2) || !is_empty(i + 3, j + 2) {
        return false;
    }

    // Apply transformation: remove top 2 nodes, bottom node gets weight 1
    grid.set(i + 1, j + 1, CellState::Empty);
    grid.set(i + 2, j + 1, CellState::Empty);
    grid.set(i + 3, j + 1, CellState::Occupied { weight: 1 });

    true
}

/// Try to apply DanglingLeg pattern going upward (180 rotation of down).
/// Julia pattern (4 rows x 3 cols, 0-indexed at (i,j)):
///   . @ .    <- row i: empty, occupied(w=2), empty [base]
///   . @ .    <- row i+1: empty, occupied(w=2), empty
///   . o .    <- row i+2: empty, occupied(w=1), empty [dangling end]
///   . . .    <- row i+3: empty, empty, empty
/// After: only node at (i, j+1) remains with weight 1
#[allow(dead_code)]
fn try_apply_dangling_leg_up(grid: &mut MappingGrid, i: usize, j: usize) -> bool {
    let (rows, cols) = grid.size();

    // Need at least 4 rows and 3 cols from position (i, j)
    if i + 3 >= rows || j + 2 >= cols {
        return false;
    }

    let is_empty = |row: usize, col: usize| -> bool { !grid.is_occupied(row, col) };

    let has_weight = |row: usize, col: usize, w: i32| -> bool {
        grid.get(row, col).is_some_and(|c| c.weight() == w)
    };

    // Row i: empty, occupied(w=2), empty
    if !is_empty(i, j) || !has_weight(i, j + 1, 2) || !is_empty(i, j + 2) {
        return false;
    }

    // Row i+1: empty, occupied(w=2), empty
    if !is_empty(i + 1, j) || !has_weight(i + 1, j + 1, 2) || !is_empty(i + 1, j + 2) {
        return false;
    }

    // Row i+2: empty, occupied(w=1), empty [dangling end]
    if !is_empty(i + 2, j) || !has_weight(i + 2, j + 1, 1) || !is_empty(i + 2, j + 2) {
        return false;
    }

    // Row i+3: all 3 cells must be empty
    if !is_empty(i + 3, j) || !is_empty(i + 3, j + 1) || !is_empty(i + 3, j + 2) {
        return false;
    }

    // Apply transformation: remove dangling end and middle, base gets weight 1
    grid.set(i + 1, j + 1, CellState::Empty);
    grid.set(i + 2, j + 1, CellState::Empty);
    grid.set(i, j + 1, CellState::Occupied { weight: 1 });

    true
}

/// Try to apply DanglingLeg pattern going right (90 rotation of down).
/// Julia pattern (3 rows x 4 cols, 0-indexed at (i,j)):
///   . . . .    <- row i: all empty
///   @ @ o .    <- row i+1: occupied(w=2), occupied(w=2), occupied(w=1), empty
///   . . . .    <- row i+2: all empty
/// After: only node at (i+1, j) remains with weight 1
#[allow(dead_code)]
fn try_apply_dangling_leg_right(grid: &mut MappingGrid, i: usize, j: usize) -> bool {
    let (rows, cols) = grid.size();

    // Need at least 3 rows and 4 cols from position (i, j)
    if i + 2 >= rows || j + 3 >= cols {
        return false;
    }

    let is_empty = |row: usize, col: usize| -> bool { !grid.is_occupied(row, col) };

    let has_weight = |row: usize, col: usize, w: i32| -> bool {
        grid.get(row, col).is_some_and(|c| c.weight() == w)
    };

    // Row i: all 4 cells must be empty
    if !is_empty(i, j) || !is_empty(i, j + 1) || !is_empty(i, j + 2) || !is_empty(i, j + 3) {
        return false;
    }

    // Row i+1: occupied(w=2), occupied(w=2), occupied(w=1), empty
    if !has_weight(i + 1, j, 2)
        || !has_weight(i + 1, j + 1, 2)
        || !has_weight(i + 1, j + 2, 1)
        || !is_empty(i + 1, j + 3)
    {
        return false;
    }

    // Row i+2: all 4 cells must be empty
    if !is_empty(i + 2, j)
        || !is_empty(i + 2, j + 1)
        || !is_empty(i + 2, j + 2)
        || !is_empty(i + 2, j + 3)
    {
        return false;
    }

    // Apply transformation: remove dangling and middle, base gets weight 1
    grid.set(i + 1, j + 1, CellState::Empty);
    grid.set(i + 1, j + 2, CellState::Empty);
    grid.set(i + 1, j, CellState::Occupied { weight: 1 });

    true
}

/// Try to apply DanglingLeg pattern going left (270 rotation of down).
/// Julia pattern (3 rows x 4 cols, 0-indexed at (i,j)):
///   . . . .    <- row i: all empty
///   . o @ @    <- row i+1: empty, occupied(w=1), occupied(w=2), occupied(w=2)
///   . . . .    <- row i+2: all empty
/// After: only node at (i+1, j+3) remains with weight 1
#[allow(dead_code)]
fn try_apply_dangling_leg_left(grid: &mut MappingGrid, i: usize, j: usize) -> bool {
    let (rows, cols) = grid.size();

    // Need at least 3 rows and 4 cols from position (i, j)
    if i + 2 >= rows || j + 3 >= cols {
        return false;
    }

    let is_empty = |row: usize, col: usize| -> bool { !grid.is_occupied(row, col) };

    let has_weight = |row: usize, col: usize, w: i32| -> bool {
        grid.get(row, col).is_some_and(|c| c.weight() == w)
    };

    // Row i: all 4 cells must be empty
    if !is_empty(i, j) || !is_empty(i, j + 1) || !is_empty(i, j + 2) || !is_empty(i, j + 3) {
        return false;
    }

    // Row i+1: empty, occupied(w=1), occupied(w=2), occupied(w=2)
    if !is_empty(i + 1, j)
        || !has_weight(i + 1, j + 1, 1)
        || !has_weight(i + 1, j + 2, 2)
        || !has_weight(i + 1, j + 3, 2)
    {
        return false;
    }

    // Row i+2: all 4 cells must be empty
    if !is_empty(i + 2, j)
        || !is_empty(i + 2, j + 1)
        || !is_empty(i + 2, j + 2)
        || !is_empty(i + 2, j + 3)
    {
        return false;
    }

    // Apply transformation: remove dangling and middle, base gets weight 1
    grid.set(i + 1, j + 1, CellState::Empty);
    grid.set(i + 1, j + 2, CellState::Empty);
    grid.set(i + 1, j + 3, CellState::Occupied { weight: 1 });

    true
}

/// Get MIS overhead for a weighted triangular tape entry.
/// For triangular mode, crossing gadgets use their native overhead,
/// but simplifiers (DanglingLeg) use weighted overhead = unweighted * 2.
/// Julia: mis_overhead(w::WeightedGadget) = mis_overhead(w.gadget) * 2
pub fn tape_entry_mis_overhead(entry: &WeightedTriTapeEntry) -> i32 {
    match entry.gadget_idx {
        0 => WeightedTriCross::<false>.mis_overhead(),
        1 => WeightedTriCross::<true>.mis_overhead(),
        2 => WeightedTriTConLeft.mis_overhead(),
        3 => WeightedTriTConUp.mis_overhead(),
        4 => WeightedTriTConDown.mis_overhead(),
        5 => WeightedTriTrivialTurnLeft.mis_overhead(),
        6 => WeightedTriTrivialTurnRight.mis_overhead(),
        7 => WeightedTriEndTurn.mis_overhead(),
        8 => WeightedTriTurn.mis_overhead(),
        9 => WeightedTriWTurn.mis_overhead(),
        10 => WeightedTriBranchFix.mis_overhead(),
        11 => WeightedTriBranchFixB.mis_overhead(),
        12 => WeightedTriBranch.mis_overhead(),
        // Simplifier gadgets (100+): weighted overhead = -1 * 2 = -2
        idx if idx >= 100 => -2,
        _ => 0,
    }
}
