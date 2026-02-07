use super::*;
use crate::topology::Graph;

#[test]
fn test_triangular_cross_gadget() {
    // Julia: Base.size(::TriCross{true}) = (6, 4)
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
    // Julia: TriCross{true} - size (6,4), cross (2,2), overhead 1
    let cross = TriCross::<true>;
    assert_eq!(TriangularGadget::size(&cross), (6, 4));
    assert_eq!(TriangularGadget::cross_location(&cross), (2, 2));
    assert!(TriangularGadget::is_connected(&cross));
    assert_eq!(TriangularGadget::mis_overhead(&cross), 1);
}

#[test]
fn test_triangular_cross_disconnected_gadget() {
    // Julia: TriCross{false} - size (6,6), cross (2,4), overhead 3
    let cross = TriCross::<false>;
    assert_eq!(TriangularGadget::size(&cross), (6, 6));
    assert_eq!(TriangularGadget::cross_location(&cross), (2, 4));
    assert!(!TriangularGadget::is_connected(&cross));
    assert_eq!(TriangularGadget::mis_overhead(&cross), 3);
}

#[test]
fn test_triangular_turn_gadget() {
    // Julia: TriTurn - size (3,4), cross (2,2), overhead 0
    let turn = TriTurn;
    assert_eq!(TriangularGadget::size(&turn), (3, 4));
    assert_eq!(TriangularGadget::mis_overhead(&turn), 0);
    let (_, _, pins) = TriangularGadget::source_graph(&turn);
    assert_eq!(pins.len(), 2);
}

#[test]
fn test_triangular_branch_gadget() {
    // Julia: TriBranch - size (6,4), cross (2,2), overhead 0
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
