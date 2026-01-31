//! Tests for gadget properties (src/rules/mapping/gadgets.rs and triangular gadgets).

use super::common::{solve_weighted_mis, triangular_edges};
use problemreductions::rules::unitdiskmapping::{
    Branch, BranchFix, Cross, EndTurn, Pattern, TCon, TriBranch, TriBranchFix, TriBranchFixB,
    TriCross, TriEndTurn, TriTConDown, TriTConUp, TriTrivialTurnLeft, TriTrivialTurnRight, TriTurn,
    TriWTurn, TriangularGadget, TrivialTurn, Turn, WTurn, WeightedKsgBranch, WeightedKsgBranchFix,
    WeightedKsgBranchFixB, WeightedKsgCross, WeightedKsgDanglingLeg, WeightedKsgEndTurn,
    WeightedKsgTCon, WeightedKsgTrivialTurn, WeightedKsgTurn, WeightedKsgWTurn,
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

// === KSG Weighted Gadget Tests ===

/// Generate King's SubGraph (KSG) edges for square lattice.
/// KSG includes both axis-aligned and diagonal neighbors within distance sqrt(2).
fn ksg_edges(locs: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for (i, &(r1, c1)) in locs.iter().enumerate() {
        for (j, &(r2, c2)) in locs.iter().enumerate() {
            if i < j {
                let dr = (r1 as i32 - r2 as i32).abs();
                let dc = (c1 as i32 - c2 as i32).abs();
                // KSG: neighbors at distance <= sqrt(2) => dr,dc each <= 1
                if dr <= 1 && dc <= 1 {
                    edges.push((i, j));
                }
            }
        }
    }
    edges
}

#[test]
fn test_weighted_ksg_cross_connected_mis_equivalence() {
    let gadget = WeightedKsgCross::<true>;
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

    let map_edges = ksg_edges(&map_locs);

    let src_mis = solve_weighted_mis(src_locs.len(), &src_edges, &src_weights);
    let map_mis = solve_weighted_mis(map_locs.len(), &map_edges, &map_weights);

    let expected = gadget.mis_overhead();
    let actual = map_mis - src_mis;

    assert_eq!(
        actual, expected,
        "WeightedKsgCross<true>: expected overhead {}, got {} (src={}, map={})",
        expected, actual, src_mis, map_mis
    );
}

#[test]
fn test_weighted_ksg_cross_disconnected_mis_equivalence() {
    let gadget = WeightedKsgCross::<false>;
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

    let map_edges = ksg_edges(&map_locs);

    let src_mis = solve_weighted_mis(src_locs.len(), &src_edges, &src_weights);
    let map_mis = solve_weighted_mis(map_locs.len(), &map_edges, &map_weights);

    let expected = gadget.mis_overhead();
    let actual = map_mis - src_mis;

    assert_eq!(
        actual, expected,
        "WeightedKsgCross<false>: expected overhead {}, got {} (src={}, map={})",
        expected, actual, src_mis, map_mis
    );
}

#[test]
fn test_weighted_ksg_turn_mis_equivalence() {
    let gadget = WeightedKsgTurn;
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

    let map_edges = ksg_edges(&map_locs);

    let src_mis = solve_weighted_mis(src_locs.len(), &src_edges, &src_weights);
    let map_mis = solve_weighted_mis(map_locs.len(), &map_edges, &map_weights);

    let expected = gadget.mis_overhead();
    let actual = map_mis - src_mis;

    assert_eq!(
        actual, expected,
        "WeightedKsgTurn: expected overhead {}, got {} (src={}, map={})",
        expected, actual, src_mis, map_mis
    );
}

#[test]
fn test_weighted_ksg_wturn_mis_equivalence() {
    let gadget = WeightedKsgWTurn;
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

    let map_edges = ksg_edges(&map_locs);

    let src_mis = solve_weighted_mis(src_locs.len(), &src_edges, &src_weights);
    let map_mis = solve_weighted_mis(map_locs.len(), &map_edges, &map_weights);

    let expected = gadget.mis_overhead();
    let actual = map_mis - src_mis;

    assert_eq!(
        actual, expected,
        "WeightedKsgWTurn: expected overhead {}, got {} (src={}, map={})",
        expected, actual, src_mis, map_mis
    );
}

#[test]
fn test_weighted_ksg_branch_mis_equivalence() {
    let gadget = WeightedKsgBranch;
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

    let map_edges = ksg_edges(&map_locs);

    let src_mis = solve_weighted_mis(src_locs.len(), &src_edges, &src_weights);
    let map_mis = solve_weighted_mis(map_locs.len(), &map_edges, &map_weights);

    let expected = gadget.mis_overhead();
    let actual = map_mis - src_mis;

    assert_eq!(
        actual, expected,
        "WeightedKsgBranch: expected overhead {}, got {} (src={}, map={})",
        expected, actual, src_mis, map_mis
    );
}

#[test]
fn test_weighted_ksg_branchfix_mis_equivalence() {
    let gadget = WeightedKsgBranchFix;
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

    let map_edges = ksg_edges(&map_locs);

    let src_mis = solve_weighted_mis(src_locs.len(), &src_edges, &src_weights);
    let map_mis = solve_weighted_mis(map_locs.len(), &map_edges, &map_weights);

    let expected = gadget.mis_overhead();
    let actual = map_mis - src_mis;

    assert_eq!(
        actual, expected,
        "WeightedKsgBranchFix: expected overhead {}, got {} (src={}, map={})",
        expected, actual, src_mis, map_mis
    );
}

#[test]
fn test_weighted_ksg_tcon_mis_equivalence() {
    let gadget = WeightedKsgTCon;
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

    let map_edges = ksg_edges(&map_locs);

    let src_mis = solve_weighted_mis(src_locs.len(), &src_edges, &src_weights);
    let map_mis = solve_weighted_mis(map_locs.len(), &map_edges, &map_weights);

    let expected = gadget.mis_overhead();
    let actual = map_mis - src_mis;

    assert_eq!(
        actual, expected,
        "WeightedKsgTCon: expected overhead {}, got {} (src={}, map={})",
        expected, actual, src_mis, map_mis
    );
}

#[test]
fn test_weighted_ksg_trivialturn_mis_equivalence() {
    let gadget = WeightedKsgTrivialTurn;
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

    let map_edges = ksg_edges(&map_locs);

    let src_mis = solve_weighted_mis(src_locs.len(), &src_edges, &src_weights);
    let map_mis = solve_weighted_mis(map_locs.len(), &map_edges, &map_weights);

    let expected = gadget.mis_overhead();
    let actual = map_mis - src_mis;

    assert_eq!(
        actual, expected,
        "WeightedKsgTrivialTurn: expected overhead {}, got {} (src={}, map={})",
        expected, actual, src_mis, map_mis
    );
}

#[test]
fn test_weighted_ksg_endturn_mis_equivalence() {
    let gadget = WeightedKsgEndTurn;
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

    let map_edges = ksg_edges(&map_locs);

    let src_mis = solve_weighted_mis(src_locs.len(), &src_edges, &src_weights);
    let map_mis = solve_weighted_mis(map_locs.len(), &map_edges, &map_weights);

    let expected = gadget.mis_overhead();
    let actual = map_mis - src_mis;

    assert_eq!(
        actual, expected,
        "WeightedKsgEndTurn: expected overhead {}, got {} (src={}, map={})",
        expected, actual, src_mis, map_mis
    );
}

#[test]
fn test_weighted_ksg_branchfixb_mis_equivalence() {
    let gadget = WeightedKsgBranchFixB;
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

    let map_edges = ksg_edges(&map_locs);

    let src_mis = solve_weighted_mis(src_locs.len(), &src_edges, &src_weights);
    let map_mis = solve_weighted_mis(map_locs.len(), &map_edges, &map_weights);

    let expected = gadget.mis_overhead();
    let actual = map_mis - src_mis;

    assert_eq!(
        actual, expected,
        "WeightedKsgBranchFixB: expected overhead {}, got {} (src={}, map={})",
        expected, actual, src_mis, map_mis
    );
}

#[test]
fn test_weighted_ksg_danglinleg_mis_equivalence() {
    let gadget = WeightedKsgDanglingLeg;
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

    let map_edges = ksg_edges(&map_locs);

    let src_mis = solve_weighted_mis(src_locs.len(), &src_edges, &src_weights);
    let map_mis = solve_weighted_mis(map_locs.len(), &map_edges, &map_weights);

    let expected = gadget.mis_overhead();
    let actual = map_mis - src_mis;

    assert_eq!(
        actual, expected,
        "WeightedKsgDanglingLeg: expected overhead {}, got {} (src={}, map={})",
        expected, actual, src_mis, map_mis
    );
}

/// Test all KSG weighted gadgets have valid graph structure
#[test]
fn test_all_ksg_weighted_gadgets_valid_structure() {
    fn check_gadget<G: Pattern + Copy>(gadget: G, name: &str) {
        let (src_locs, src_edges, src_pins) = gadget.source_graph();
        let (map_locs, map_pins) = gadget.mapped_graph();
        let src_weights = gadget.source_weights();
        let map_weights = gadget.mapped_weights();

        assert!(
            !src_locs.is_empty(),
            "{}: source should have locations",
            name
        );
        assert!(
            !map_locs.is_empty(),
            "{}: mapped should have locations",
            name
        );
        assert!(
            src_edges.iter().all(|&(a, b)| a < src_locs.len() && b < src_locs.len()),
            "{}: source edges should be valid",
            name
        );
        assert!(
            src_pins.iter().all(|&p| p < src_locs.len()),
            "{}: source pins should be valid",
            name
        );
        assert!(
            map_pins.iter().all(|&p| p < map_locs.len()),
            "{}: mapped pins should be valid",
            name
        );
        assert_eq!(
            src_weights.len(),
            src_locs.len(),
            "{}: source weights should match locations",
            name
        );
        assert_eq!(
            map_weights.len(),
            map_locs.len(),
            "{}: mapped weights should match locations",
            name
        );
    }

    check_gadget(WeightedKsgCross::<true>, "WeightedKsgCross<true>");
    check_gadget(WeightedKsgCross::<false>, "WeightedKsgCross<false>");
    check_gadget(WeightedKsgTurn, "WeightedKsgTurn");
    check_gadget(WeightedKsgWTurn, "WeightedKsgWTurn");
    check_gadget(WeightedKsgBranch, "WeightedKsgBranch");
    check_gadget(WeightedKsgBranchFix, "WeightedKsgBranchFix");
    check_gadget(WeightedKsgBranchFixB, "WeightedKsgBranchFixB");
    check_gadget(WeightedKsgTCon, "WeightedKsgTCon");
    check_gadget(WeightedKsgTrivialTurn, "WeightedKsgTrivialTurn");
    check_gadget(WeightedKsgEndTurn, "WeightedKsgEndTurn");
    check_gadget(WeightedKsgDanglingLeg, "WeightedKsgDanglingLeg");
}

/// Test all KSG weighted gadgets MIS equivalence in one test
#[test]
fn test_all_ksg_weighted_gadgets_mis_equivalence() {
    fn test_gadget<G: Pattern + Copy>(gadget: G, name: &str) {
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

        let map_edges = ksg_edges(&map_locs);

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

    test_gadget(WeightedKsgCross::<true>, "WeightedKsgCross<true>");
    test_gadget(WeightedKsgCross::<false>, "WeightedKsgCross<false>");
    test_gadget(WeightedKsgTurn, "WeightedKsgTurn");
    test_gadget(WeightedKsgWTurn, "WeightedKsgWTurn");
    test_gadget(WeightedKsgBranch, "WeightedKsgBranch");
    test_gadget(WeightedKsgBranchFix, "WeightedKsgBranchFix");
    test_gadget(WeightedKsgBranchFixB, "WeightedKsgBranchFixB");
    test_gadget(WeightedKsgTCon, "WeightedKsgTCon");
    test_gadget(WeightedKsgTrivialTurn, "WeightedKsgTrivialTurn");
    test_gadget(WeightedKsgEndTurn, "WeightedKsgEndTurn");
    test_gadget(WeightedKsgDanglingLeg, "WeightedKsgDanglingLeg");
}
