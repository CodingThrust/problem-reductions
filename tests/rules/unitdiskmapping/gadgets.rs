//! Tests for gadget properties (src/rules/mapping/gadgets.rs and triangular gadgets).

use super::common::{solve_weighted_mis, triangular_edges};
use problemreductions::rules::unitdiskmapping::{
    Branch, BranchFix, Cross, EndTurn, Pattern, TCon, TriBranch, TriBranchFix, TriBranchFixB,
    TriCross, TriEndTurn, TriTConDown, TriTConUp, TriTrivialTurnLeft, TriTrivialTurnRight, TriTurn,
    TriWTurn, TriangularGadget, TrivialTurn, Turn, WTurn,
};

// === Square Gadget Tests ===

#[test]
fn test_cross_disconnected_gadget() {
    let gadget = Cross::<false>;
    let (locs, edges, pins) = gadget.source_graph();

    assert!(!locs.is_empty());
    assert!(pins.len() >= 2);
    assert!(edges.iter().all(|&(a, b)| a < locs.len() && b < locs.len()));
}

#[test]
fn test_cross_connected_gadget() {
    let gadget = Cross::<true>;
    let (locs, _, pins) = gadget.source_graph();

    assert!(!locs.is_empty());
    assert!(pins.len() >= 2);
}

#[test]
fn test_turn_gadget() {
    let gadget = Turn;
    let (locs, edges, pins) = gadget.source_graph();

    assert!(!locs.is_empty());
    assert!(!pins.is_empty());
    assert!(edges.iter().all(|&(a, b)| a < locs.len() && b < locs.len()));
}

#[test]
fn test_wturn_gadget() {
    let gadget = WTurn;
    let (locs, _, pins) = gadget.source_graph();

    assert!(!locs.is_empty());
    assert!(!pins.is_empty());
}

#[test]
fn test_branch_gadget() {
    let gadget = Branch;
    let (locs, edges, pins) = gadget.source_graph();

    assert!(!locs.is_empty());
    assert!(!pins.is_empty());
    assert!(edges.iter().all(|&(a, b)| a < locs.len() && b < locs.len()));
}

#[test]
fn test_branch_fix_gadget() {
    let gadget = BranchFix;
    let (locs, _, pins) = gadget.source_graph();

    assert!(!locs.is_empty());
    assert!(!pins.is_empty());
}

#[test]
fn test_tcon_gadget() {
    let gadget = TCon;
    let (locs, _, pins) = gadget.source_graph();

    assert!(!locs.is_empty());
    assert!(!pins.is_empty());
}

#[test]
fn test_trivial_turn_gadget() {
    let gadget = TrivialTurn;
    let (locs, _, pins) = gadget.source_graph();

    assert!(!locs.is_empty());
    assert!(!pins.is_empty());
}

#[test]
fn test_end_turn_gadget() {
    let gadget = EndTurn;
    let (locs, _, pins) = gadget.source_graph();

    assert!(!locs.is_empty());
    assert!(!pins.is_empty());
}

#[test]
fn test_all_gadgets_have_valid_pins() {
    // Test Cross<true>
    let (source_locs, _, source_pins) = Cross::<true>.source_graph();
    let (mapped_locs, mapped_pins) = Cross::<true>.mapped_graph();
    assert!(source_pins.iter().all(|&p| p < source_locs.len()));
    assert!(mapped_pins.iter().all(|&p| p < mapped_locs.len()));
    assert_eq!(source_pins.len(), mapped_pins.len());

    // Test Cross<false>
    let (source_locs, _, source_pins) = Cross::<false>.source_graph();
    let (mapped_locs, mapped_pins) = Cross::<false>.mapped_graph();
    assert!(source_pins.iter().all(|&p| p < source_locs.len()));
    assert!(mapped_pins.iter().all(|&p| p < mapped_locs.len()));
    assert_eq!(source_pins.len(), mapped_pins.len());

    // Test Turn
    let (source_locs, _, source_pins) = Turn.source_graph();
    let (mapped_locs, mapped_pins) = Turn.mapped_graph();
    assert!(source_pins.iter().all(|&p| p < source_locs.len()));
    assert!(mapped_pins.iter().all(|&p| p < mapped_locs.len()));
    assert_eq!(source_pins.len(), mapped_pins.len());

    // Test WTurn
    let (source_locs, _, source_pins) = WTurn.source_graph();
    let (mapped_locs, mapped_pins) = WTurn.mapped_graph();
    assert!(source_pins.iter().all(|&p| p < source_locs.len()));
    assert!(mapped_pins.iter().all(|&p| p < mapped_locs.len()));
    assert_eq!(source_pins.len(), mapped_pins.len());

    // Test Branch
    let (source_locs, _, source_pins) = Branch.source_graph();
    let (mapped_locs, mapped_pins) = Branch.mapped_graph();
    assert!(source_pins.iter().all(|&p| p < source_locs.len()));
    assert!(mapped_pins.iter().all(|&p| p < mapped_locs.len()));
    assert_eq!(source_pins.len(), mapped_pins.len());

    // Test BranchFix
    let (source_locs, _, source_pins) = BranchFix.source_graph();
    let (mapped_locs, mapped_pins) = BranchFix.mapped_graph();
    assert!(source_pins.iter().all(|&p| p < source_locs.len()));
    assert!(mapped_pins.iter().all(|&p| p < mapped_locs.len()));
    assert_eq!(source_pins.len(), mapped_pins.len());

    // Test TCon
    let (source_locs, _, source_pins) = TCon.source_graph();
    let (mapped_locs, mapped_pins) = TCon.mapped_graph();
    assert!(source_pins.iter().all(|&p| p < source_locs.len()));
    assert!(mapped_pins.iter().all(|&p| p < mapped_locs.len()));
    assert_eq!(source_pins.len(), mapped_pins.len());

    // Test TrivialTurn
    let (source_locs, _, source_pins) = TrivialTurn.source_graph();
    let (mapped_locs, mapped_pins) = TrivialTurn.mapped_graph();
    assert!(source_pins.iter().all(|&p| p < source_locs.len()));
    assert!(mapped_pins.iter().all(|&p| p < mapped_locs.len()));
    assert_eq!(source_pins.len(), mapped_pins.len());

    // Test EndTurn
    let (source_locs, _, source_pins) = EndTurn.source_graph();
    let (mapped_locs, mapped_pins) = EndTurn.mapped_graph();
    assert!(source_pins.iter().all(|&p| p < source_locs.len()));
    assert!(mapped_pins.iter().all(|&p| p < mapped_locs.len()));
    assert_eq!(source_pins.len(), mapped_pins.len());
}

// === Triangular Gadget Tests ===

#[test]
fn test_triangular_gadgets_have_valid_pins() {
    fn check_tri_gadget<G: TriangularGadget>(gadget: G, name: &str) {
        let (source_locs, _, source_pins) = gadget.source_graph();
        let (mapped_locs, mapped_pins) = gadget.mapped_graph();

        assert!(
            source_pins.iter().all(|&p| p < source_locs.len()),
            "{}: source pins should be valid indices",
            name
        );
        assert!(
            mapped_pins.iter().all(|&p| p < mapped_locs.len()),
            "{}: mapped pins should be valid indices",
            name
        );
    }

    check_tri_gadget(TriCross::<true>, "TriCross<true>");
    check_tri_gadget(TriCross::<false>, "TriCross<false>");
    check_tri_gadget(TriTurn, "TriTurn");
    check_tri_gadget(TriWTurn, "TriWTurn");
    check_tri_gadget(TriBranch, "TriBranch");
    check_tri_gadget(TriBranchFix, "TriBranchFix");
    check_tri_gadget(TriBranchFixB, "TriBranchFixB");
    check_tri_gadget(TriTConUp, "TriTConUp");
    check_tri_gadget(TriTConDown, "TriTConDown");
    check_tri_gadget(TriTrivialTurnLeft, "TriTrivialTurnLeft");
    check_tri_gadget(TriTrivialTurnRight, "TriTrivialTurnRight");
    check_tri_gadget(TriEndTurn, "TriEndTurn");
}

// === Weighted MIS Equivalence Tests ===

#[test]
fn test_triturn_mis_equivalence() {
    // TriTurn is already weighted (WeightedTriTurn)
    let gadget = TriTurn;
    let (src_locs, src_edges, src_pins) = gadget.source_graph();
    let (map_locs, map_pins) = gadget.mapped_graph();

    let mut src_weights: Vec<i32> = gadget.source_weights().to_vec();
    let mut map_weights: Vec<i32> = gadget.mapped_weights().to_vec();
    for &p in &src_pins {
        src_weights[p] -= 1;
    }
    for &p in &map_pins {
        map_weights[p] -= 1;
    }

    let map_edges = triangular_edges(&map_locs, 1.1);

    let src_mis = solve_weighted_mis(src_locs.len(), &src_edges, &src_weights);
    let map_mis = solve_weighted_mis(map_locs.len(), &map_edges, &map_weights);

    let expected = gadget.mis_overhead();
    let actual = map_mis - src_mis;

    assert_eq!(
        actual, expected,
        "TriTurn: expected overhead {}, got {} (src={}, map={})",
        expected, actual, src_mis, map_mis
    );
}

#[test]
fn test_tribranch_mis_equivalence() {
    // TriBranch is already weighted (WeightedTriBranch)
    let gadget = TriBranch;
    let (src_locs, src_edges, src_pins) = gadget.source_graph();
    let (map_locs, map_pins) = gadget.mapped_graph();

    let mut src_weights: Vec<i32> = gadget.source_weights().to_vec();
    let mut map_weights: Vec<i32> = gadget.mapped_weights().to_vec();
    for &p in &src_pins {
        src_weights[p] -= 1;
    }
    for &p in &map_pins {
        map_weights[p] -= 1;
    }

    let map_edges = triangular_edges(&map_locs, 1.1);

    let src_mis = solve_weighted_mis(src_locs.len(), &src_edges, &src_weights);
    let map_mis = solve_weighted_mis(map_locs.len(), &map_edges, &map_weights);

    let expected = gadget.mis_overhead();
    let actual = map_mis - src_mis;

    assert_eq!(
        actual, expected,
        "TriBranch: expected overhead {}, got {} (src={}, map={})",
        expected, actual, src_mis, map_mis
    );
}

#[test]
fn test_tricross_connected_weighted_mis_equivalence() {
    // TriCross is already weighted (WeightedTriCross)
    let gadget = TriCross::<true>;
    let (source_locs, source_edges, source_pins) = gadget.source_graph();
    let (mapped_locs, mapped_pins) = gadget.mapped_graph();

    let mut src_weights: Vec<i32> = gadget.source_weights().to_vec();
    let mut map_weights: Vec<i32> = gadget.mapped_weights().to_vec();
    for &p in &source_pins {
        src_weights[p] -= 1;
    }
    for &p in &mapped_pins {
        map_weights[p] -= 1;
    }

    let mapped_edges = triangular_edges(&mapped_locs, 1.1);

    let source_mis = solve_weighted_mis(source_locs.len(), &source_edges, &src_weights);
    let mapped_mis = solve_weighted_mis(mapped_locs.len(), &mapped_edges, &map_weights);

    let expected_overhead = gadget.mis_overhead();
    let actual_overhead = mapped_mis - source_mis;

    assert_eq!(
        actual_overhead, expected_overhead,
        "TriCross<true> weighted: expected overhead {}, got {} (src={}, map={})",
        expected_overhead, actual_overhead, source_mis, mapped_mis
    );
}

#[test]
fn test_tricross_disconnected_weighted_mis_equivalence() {
    // TriCross is already weighted (WeightedTriCross)
    let gadget = TriCross::<false>;
    let (source_locs, source_edges, source_pins) = gadget.source_graph();
    let (mapped_locs, mapped_pins) = gadget.mapped_graph();

    let mut src_weights: Vec<i32> = gadget.source_weights().to_vec();
    let mut map_weights: Vec<i32> = gadget.mapped_weights().to_vec();
    for &p in &source_pins {
        src_weights[p] -= 1;
    }
    for &p in &mapped_pins {
        map_weights[p] -= 1;
    }

    let mapped_edges = triangular_edges(&mapped_locs, 1.1);

    let source_mis = solve_weighted_mis(source_locs.len(), &source_edges, &src_weights);
    let mapped_mis = solve_weighted_mis(mapped_locs.len(), &mapped_edges, &map_weights);

    let expected_overhead = gadget.mis_overhead();
    let actual_overhead = mapped_mis - source_mis;

    assert_eq!(
        actual_overhead, expected_overhead,
        "TriCross<false> weighted: expected overhead {}, got {} (src={}, map={})",
        expected_overhead, actual_overhead, source_mis, mapped_mis
    );
}

#[test]
fn test_all_triangular_weighted_gadgets_mis_equivalence() {
    // Triangular gadgets are already weighted (WeightedTri* prefix)
    // So we directly use their source_weights() and mapped_weights() methods
    fn test_gadget<G: TriangularGadget + Copy>(gadget: G, name: &str) {
        let (src_locs, src_edges, src_pins) = gadget.source_graph();
        let (map_locs, map_pins) = gadget.mapped_graph();

        let mut src_weights: Vec<i32> = gadget.source_weights().to_vec();
        let mut map_weights: Vec<i32> = gadget.mapped_weights().to_vec();
        for &p in &src_pins {
            src_weights[p] -= 1;
        }
        for &p in &map_pins {
            map_weights[p] -= 1;
        }

        let map_edges = triangular_edges(&map_locs, 1.1);

        let src_mis = solve_weighted_mis(src_locs.len(), &src_edges, &src_weights);
        let map_mis = solve_weighted_mis(map_locs.len(), &map_edges, &map_weights);

        let expected = gadget.mis_overhead();
        let actual = map_mis - src_mis;

        assert_eq!(
            actual, expected,
            "{}: expected overhead {}, got {} (src={}, map={})",
            name, expected, actual, src_mis, map_mis
        );
    }

    test_gadget(TriTurn, "TriTurn");
    test_gadget(TriBranch, "TriBranch");
    test_gadget(TriCross::<true>, "TriCross<true>");
    test_gadget(TriCross::<false>, "TriCross<false>");
    test_gadget(TriTConDown, "TriTConDown");
    test_gadget(TriTConUp, "TriTConUp");
    test_gadget(TriTrivialTurnLeft, "TriTrivialTurnLeft");
    test_gadget(TriTrivialTurnRight, "TriTrivialTurnRight");
    test_gadget(TriEndTurn, "TriEndTurn");
    test_gadget(TriWTurn, "TriWTurn");
    test_gadget(TriBranchFix, "TriBranchFix");
    test_gadget(TriBranchFixB, "TriBranchFixB");
}
