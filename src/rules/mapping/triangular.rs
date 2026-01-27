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
#[allow(dead_code)]
pub trait TriangularGadget {
    fn size(&self) -> (usize, usize);
    fn cross_location(&self) -> (usize, usize);
    fn is_connected(&self) -> bool;
    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>);
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

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
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
        let pins = vec![0, 4, 9, 3];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
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

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
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
        let pins = vec![11, 5, 10, 4];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
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

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
        let locs = vec![(1, 2), (2, 2), (2, 3), (2, 4)];
        let pins = vec![0, 3];
        (locs, pins)
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

    fn source_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
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
        let pins = vec![0, 3, 8];
        (locs, pins)
    }

    fn mapped_graph(&self) -> (Vec<(usize, usize)>, Vec<usize>) {
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
        let pins = vec![0, 2, 8];
        (locs, pins)
    }

    fn mis_overhead(&self) -> i32 {
        0
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
        let (_, pins) = TriangularGadget::source_graph(&turn);
        assert_eq!(pins.len(), 2);
    }

    #[test]
    fn test_triangular_branch_gadget() {
        let branch = TriBranch;
        assert_eq!(TriangularGadget::size(&branch), (6, 4));
        assert_eq!(TriangularGadget::mis_overhead(&branch), 0);
        let (_, pins) = TriangularGadget::source_graph(&branch);
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
            let (source_locs, source_pins) = gadget.source_graph();
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
