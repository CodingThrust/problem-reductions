//! Triangular lattice mapping support.

use super::copyline::create_copylines;
use super::grid::MappingGrid;
use super::map_graph::MappingResult;
use super::pathdecomposition::{pathwidth, vertex_order_from_layout, PathDecompositionMethod};
use crate::topology::{GridGraph, GridNode, GridType};
use serde::{Deserialize, Serialize};

const TRIANGULAR_SPACING: usize = 6;
const TRIANGULAR_PADDING: usize = 2;
const TRIANGULAR_UNIT_RADIUS: f64 = 1.1;

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
        4
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
    let max_hslot = copylines.iter().map(|l| l.hslot).max().unwrap_or(1);
    let max_vslot = copylines.iter().map(|l| l.vslot).max().unwrap_or(1);
    let max_hstop = copylines.iter().map(|l| l.hstop).max().unwrap_or(1);
    let max_vstop = copylines.iter().map(|l| l.vstop).max().unwrap_or(1);

    let rows = max_hslot.max(max_vstop) * spacing + 2 + 2 * padding;
    let cols = max_vslot.max(max_hstop) * spacing + 2 + 2 * padding;

    let mut grid = MappingGrid::with_padding(rows, cols, spacing, padding);

    // Add copy line nodes
    for line in &copylines {
        for (row, col, weight) in line.locations(padding, spacing) {
            grid.add_node(row, col, weight as i32);
        }
    }

    // Calculate MIS overhead
    let mis_overhead: i32 = copylines
        .iter()
        .map(|line| {
            let row_overhead = (line.hslot.saturating_sub(line.vstart)) * spacing
                + (line.vstop.saturating_sub(line.hslot)) * spacing;
            let col_overhead = if line.hstop > line.vslot {
                (line.hstop - line.vslot) * spacing - 2
            } else {
                0
            };
            (row_overhead + col_overhead) as i32
        })
        .sum();

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
        tape: Vec::new(), // Triangular lattice uses different gadgets
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
