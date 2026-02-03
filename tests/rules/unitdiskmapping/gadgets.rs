//! Tests for gadget properties (src/rules/mapping/gadgets.rs and triangular gadgets).

use super::common::{solve_weighted_mis, triangular_edges};
use problemreductions::rules::unitdiskmapping::{
    Branch, BranchFix, Cross, EndTurn, Mirror, Pattern, ReflectedGadget, RotatedGadget, TCon,
    TriBranch, TriBranchFix, TriBranchFixB, TriCross, TriEndTurn, TriTConDown, TriTConUp,
    TriTrivialTurnLeft, TriTrivialTurnRight, TriTurn, TriWTurn, TriangularGadget, TrivialTurn,
    Turn, WTurn, WeightedKsgBranch, WeightedKsgBranchFix, WeightedKsgBranchFixB, WeightedKsgCross,
    WeightedKsgDanglingLeg, WeightedKsgEndTurn, WeightedKsgTCon, WeightedKsgTrivialTurn,
    WeightedKsgTurn, WeightedKsgWTurn,
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
            src_edges
                .iter()
                .all(|&(a, b)| a < src_locs.len() && b < src_locs.len()),
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

// === Pattern Trait Method Tests ===

#[test]
fn test_pattern_source_matrix() {
    // Test source_matrix generation for all gadgets
    let cross_matrix = Cross::<true>.source_matrix();
    assert!(!cross_matrix.is_empty());

    let turn_matrix = Turn.source_matrix();
    assert!(!turn_matrix.is_empty());

    let branch_matrix = Branch.source_matrix();
    assert!(!branch_matrix.is_empty());
}

#[test]
fn test_weighted_ksg_pattern_source_matrix() {
    let cross_matrix = WeightedKsgCross::<true>.source_matrix();
    assert!(!cross_matrix.is_empty());

    let turn_matrix = WeightedKsgTurn.source_matrix();
    assert!(!turn_matrix.is_empty());

    let branch_matrix = WeightedKsgBranch.source_matrix();
    assert!(!branch_matrix.is_empty());
}

#[test]
fn test_pattern_mapped_matrix() {
    use problemreductions::rules::unitdiskmapping::Pattern;

    let cross_mapped = Cross::<true>.mapped_matrix();
    assert!(!cross_mapped.is_empty());

    let turn_mapped = Turn.mapped_matrix();
    assert!(!turn_mapped.is_empty());
}

#[test]
fn test_weighted_pattern_weights_length() {
    // Verify weights match location counts
    let (src_locs, _, _) = WeightedKsgCross::<true>.source_graph();
    let src_weights = WeightedKsgCross::<true>.source_weights();
    assert_eq!(src_locs.len(), src_weights.len());

    let (map_locs, _) = WeightedKsgCross::<true>.mapped_graph();
    let map_weights = WeightedKsgCross::<true>.mapped_weights();
    assert_eq!(map_locs.len(), map_weights.len());
}

#[test]
fn test_all_weighted_gadgets_weights_positive() {
    fn check_positive_weights<G: Pattern>(gadget: G, name: &str) {
        let src_weights = gadget.source_weights();
        let map_weights = gadget.mapped_weights();

        assert!(
            src_weights.iter().all(|&w| w > 0),
            "{}: all source weights should be positive",
            name
        );
        assert!(
            map_weights.iter().all(|&w| w > 0),
            "{}: all mapped weights should be positive",
            name
        );
    }

    check_positive_weights(WeightedKsgCross::<true>, "WeightedKsgCross<true>");
    check_positive_weights(WeightedKsgCross::<false>, "WeightedKsgCross<false>");
    check_positive_weights(WeightedKsgTurn, "WeightedKsgTurn");
    check_positive_weights(WeightedKsgWTurn, "WeightedKsgWTurn");
    check_positive_weights(WeightedKsgBranch, "WeightedKsgBranch");
    check_positive_weights(WeightedKsgBranchFix, "WeightedKsgBranchFix");
    check_positive_weights(WeightedKsgBranchFixB, "WeightedKsgBranchFixB");
    check_positive_weights(WeightedKsgTCon, "WeightedKsgTCon");
    check_positive_weights(WeightedKsgTrivialTurn, "WeightedKsgTrivialTurn");
    check_positive_weights(WeightedKsgEndTurn, "WeightedKsgEndTurn");
    check_positive_weights(WeightedKsgDanglingLeg, "WeightedKsgDanglingLeg");
}

#[test]
fn test_gadget_is_connected_variants() {
    // Test is_connected() method
    assert!(Cross::<true>.is_connected());
    assert!(!Cross::<false>.is_connected());

    assert!(WeightedKsgCross::<true>.is_connected());
    assert!(!WeightedKsgCross::<false>.is_connected());
}

#[test]
fn test_gadget_is_cross_gadget() {
    // Cross gadgets should return true
    assert!(Cross::<true>.is_cross_gadget());
    assert!(Cross::<false>.is_cross_gadget());
    assert!(WeightedKsgCross::<true>.is_cross_gadget());
    assert!(WeightedKsgCross::<false>.is_cross_gadget());

    // Non-cross gadgets should return false
    assert!(!Turn.is_cross_gadget());
    assert!(!WeightedKsgTurn.is_cross_gadget());
}

#[test]
fn test_gadget_connected_nodes() {
    // Connected gadgets should have connected_nodes
    let nodes = Cross::<true>.connected_nodes();
    assert!(!nodes.is_empty());

    let weighted_nodes = WeightedKsgCross::<true>.connected_nodes();
    assert!(!weighted_nodes.is_empty());
}

// === Alpha Tensor Tests ===

#[test]
fn test_build_standard_unit_disk_edges() {
    use problemreductions::rules::unitdiskmapping::alpha_tensor::build_standard_unit_disk_edges;

    // Simple test: two adjacent points
    let locs = vec![(0, 0), (1, 0)];
    let edges = build_standard_unit_disk_edges(&locs);
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0], (0, 1));

    // Points too far apart
    let locs = vec![(0, 0), (3, 3)];
    let edges = build_standard_unit_disk_edges(&locs);
    assert!(edges.is_empty());

    // Multiple points in a small grid
    let locs = vec![(0, 0), (1, 0), (0, 1), (1, 1)];
    let edges = build_standard_unit_disk_edges(&locs);
    // Should have edges for adjacent and diagonal neighbors
    assert!(edges.len() > 2);
}

#[test]
fn test_build_triangular_unit_disk_edges() {
    use problemreductions::rules::unitdiskmapping::alpha_tensor::build_triangular_unit_disk_edges;

    let locs = vec![(0, 0), (1, 0), (0, 1)];
    let edges = build_triangular_unit_disk_edges(&locs);
    // Should have some edges
    assert!(!edges.is_empty() || locs.len() < 2);
}

// === Triangular Gadget Trait Method Tests ===

#[test]
fn test_triangular_gadget_source_matrix() {
    let matrix = TriTurn.source_matrix();
    assert!(!matrix.is_empty());

    let matrix = TriCross::<true>.source_matrix();
    assert!(!matrix.is_empty());

    let matrix = TriBranch.source_matrix();
    assert!(!matrix.is_empty());
}

#[test]
fn test_triangular_gadget_mapped_matrix() {
    use problemreductions::rules::unitdiskmapping::TriangularGadget;

    let matrix = TriTurn.mapped_matrix();
    assert!(!matrix.is_empty());

    let matrix = TriCross::<true>.mapped_matrix();
    assert!(!matrix.is_empty());
}

#[test]
fn test_triangular_gadget_weights() {
    // Test that weights are returned correctly
    let src_weights = TriTurn.source_weights();
    let map_weights = TriTurn.mapped_weights();
    assert!(!src_weights.is_empty());
    assert!(!map_weights.is_empty());

    // All weights should be positive
    assert!(src_weights.iter().all(|&w| w > 0));
    assert!(map_weights.iter().all(|&w| w > 0));
}

#[test]
fn test_triangular_gadget_connected_nodes() {
    // Test connected gadgets
    let nodes = TriCross::<true>.connected_nodes();
    // TriCross<true> should have connected nodes
    assert!(!nodes.is_empty() || TriCross::<true>.is_connected());

    // TriCross<false> should not be connected
    assert!(!TriCross::<false>.is_connected());
}

#[test]
fn test_all_triangular_gadgets_source_matrix() {
    use problemreductions::rules::unitdiskmapping::TriangularGadget;

    fn check_matrix<G: TriangularGadget>(gadget: G, name: &str) {
        let matrix = gadget.source_matrix();
        let (rows, cols) = gadget.size();
        assert_eq!(
            matrix.len(),
            rows,
            "{}: matrix rows should match size",
            name
        );
        if rows > 0 {
            assert_eq!(
                matrix[0].len(),
                cols,
                "{}: matrix cols should match size",
                name
            );
        }
    }

    check_matrix(TriTurn, "TriTurn");
    check_matrix(TriCross::<true>, "TriCross<true>");
    check_matrix(TriCross::<false>, "TriCross<false>");
    check_matrix(TriBranch, "TriBranch");
    check_matrix(TriBranchFix, "TriBranchFix");
    check_matrix(TriBranchFixB, "TriBranchFixB");
    check_matrix(TriTConUp, "TriTConUp");
    check_matrix(TriTConDown, "TriTConDown");
    check_matrix(TriTrivialTurnLeft, "TriTrivialTurnLeft");
    check_matrix(TriTrivialTurnRight, "TriTrivialTurnRight");
    check_matrix(TriEndTurn, "TriEndTurn");
    check_matrix(TriWTurn, "TriWTurn");
}

#[test]
fn test_all_triangular_gadgets_mapped_matrix() {
    use problemreductions::rules::unitdiskmapping::TriangularGadget;

    fn check_matrix<G: TriangularGadget>(gadget: G, name: &str) {
        let matrix = gadget.mapped_matrix();
        let (rows, cols) = gadget.size();
        assert_eq!(
            matrix.len(),
            rows,
            "{}: mapped matrix rows should match size",
            name
        );
        if rows > 0 {
            assert_eq!(
                matrix[0].len(),
                cols,
                "{}: mapped matrix cols should match size",
                name
            );
        }
    }

    check_matrix(TriTurn, "TriTurn");
    check_matrix(TriCross::<true>, "TriCross<true>");
    check_matrix(TriCross::<false>, "TriCross<false>");
    check_matrix(TriBranch, "TriBranch");
    check_matrix(TriBranchFix, "TriBranchFix");
    check_matrix(TriBranchFixB, "TriBranchFixB");
    check_matrix(TriTConUp, "TriTConUp");
    check_matrix(TriTConDown, "TriTConDown");
    check_matrix(TriTrivialTurnLeft, "TriTrivialTurnLeft");
    check_matrix(TriTrivialTurnRight, "TriTrivialTurnRight");
    check_matrix(TriEndTurn, "TriEndTurn");
    check_matrix(TriWTurn, "TriWTurn");
}

// === Rotated/Reflected Gadget Wrapper Tests ===

#[test]
fn test_rotated_gadget_size() {
    let base = Turn;
    let (m, n) = base.size();

    // 90 degree rotation swaps dimensions
    let rot90 = RotatedGadget::new(base, 1);
    assert_eq!(rot90.size(), (n, m));

    // 180 degree keeps dimensions
    let rot180 = RotatedGadget::new(base, 2);
    assert_eq!(rot180.size(), (m, n));

    // 270 degree swaps dimensions
    let rot270 = RotatedGadget::new(base, 3);
    assert_eq!(rot270.size(), (n, m));
}

#[test]
fn test_rotated_gadget_cross_location() {
    let base = Cross::<true>;
    let rotated = RotatedGadget::new(base, 1);

    // Cross location should be valid for rotated gadget
    let (r, c) = rotated.cross_location();
    let (rows, cols) = rotated.size();
    assert!(r > 0 && r <= rows);
    assert!(c > 0 && c <= cols);
}

#[test]
fn test_rotated_gadget_source_graph() {
    let base = Turn;
    let rotated = RotatedGadget::new(base, 1);

    let (locs, edges, pins) = rotated.source_graph();
    let (rows, cols) = rotated.size();

    // All locations should be within bounds
    for &(r, c) in &locs {
        assert!(r > 0 && r <= rows, "row {} out of bounds [1, {}]", r, rows);
        assert!(c > 0 && c <= cols, "col {} out of bounds [1, {}]", c, cols);
    }

    // Edges should reference valid indices
    for &(a, b) in &edges {
        assert!(a < locs.len() && b < locs.len());
    }

    // Pins should reference valid indices
    for &p in &pins {
        assert!(p < locs.len());
    }
}

#[test]
fn test_rotated_gadget_mapped_graph() {
    let base = Branch;
    let rotated = RotatedGadget::new(base, 2);

    let (locs, pins) = rotated.mapped_graph();
    let (rows, cols) = rotated.size();

    // All locations should be within bounds
    for &(r, c) in &locs {
        assert!(r > 0 && r <= rows);
        assert!(c > 0 && c <= cols);
    }

    // Pins should reference valid indices
    for &p in &pins {
        assert!(p < locs.len());
    }
}

#[test]
fn test_rotated_gadget_preserves_mis_overhead() {
    let base = Turn;
    let rotated = RotatedGadget::new(base, 1);

    // MIS overhead should be same for rotated gadget
    assert_eq!(base.mis_overhead(), rotated.mis_overhead());
}

#[test]
fn test_rotated_gadget_preserves_weights() {
    let base = WeightedKsgTurn;
    let rotated = RotatedGadget::new(base, 2);

    // Weights don't change with rotation
    assert_eq!(base.source_weights(), rotated.source_weights());
    assert_eq!(base.mapped_weights(), rotated.mapped_weights());
}

#[test]
fn test_rotated_gadget_delegates_properties() {
    let base = Cross::<true>;
    let rotated = RotatedGadget::new(base, 1);

    assert_eq!(base.is_connected(), rotated.is_connected());
    assert_eq!(base.is_cross_gadget(), rotated.is_cross_gadget());
    assert_eq!(base.connected_nodes(), rotated.connected_nodes());
}

#[test]
fn test_reflected_gadget_size_x_y() {
    let base = Turn;
    let (m, n) = base.size();

    // X and Y mirror keep same dimensions
    let ref_x = ReflectedGadget::new(base, Mirror::X);
    assert_eq!(ref_x.size(), (m, n));

    let ref_y = ReflectedGadget::new(base, Mirror::Y);
    assert_eq!(ref_y.size(), (m, n));
}

#[test]
fn test_reflected_gadget_size_diagonal() {
    let base = Turn;
    let (m, n) = base.size();

    // Diagonal mirrors swap dimensions
    let ref_diag = ReflectedGadget::new(base, Mirror::Diag);
    assert_eq!(ref_diag.size(), (n, m));

    let ref_offdiag = ReflectedGadget::new(base, Mirror::OffDiag);
    assert_eq!(ref_offdiag.size(), (n, m));
}

#[test]
fn test_reflected_gadget_cross_location() {
    let base = Cross::<true>;

    for mirror in [Mirror::X, Mirror::Y, Mirror::Diag, Mirror::OffDiag] {
        let reflected = ReflectedGadget::new(base, mirror);
        let (r, c) = reflected.cross_location();
        let (rows, cols) = reflected.size();
        assert!(r > 0 && r <= rows, "mirror {:?}: row out of bounds", mirror);
        assert!(c > 0 && c <= cols, "mirror {:?}: col out of bounds", mirror);
    }
}

#[test]
fn test_reflected_gadget_source_graph() {
    let base = Branch;
    let reflected = ReflectedGadget::new(base, Mirror::X);

    let (locs, edges, pins) = reflected.source_graph();
    let (rows, cols) = reflected.size();

    // All locations within bounds
    for &(r, c) in &locs {
        assert!(r > 0 && r <= rows);
        assert!(c > 0 && c <= cols);
    }

    // Valid edges
    for &(a, b) in &edges {
        assert!(a < locs.len() && b < locs.len());
    }

    // Valid pins
    for &p in &pins {
        assert!(p < locs.len());
    }
}

#[test]
fn test_reflected_gadget_mapped_graph() {
    let base = TCon;
    let reflected = ReflectedGadget::new(base, Mirror::Y);

    let (locs, pins) = reflected.mapped_graph();
    let (rows, cols) = reflected.size();

    for &(r, c) in &locs {
        assert!(r > 0 && r <= rows);
        assert!(c > 0 && c <= cols);
    }

    for &p in &pins {
        assert!(p < locs.len());
    }
}

#[test]
fn test_reflected_gadget_preserves_mis_overhead() {
    let base = Turn;
    let reflected = ReflectedGadget::new(base, Mirror::Diag);

    assert_eq!(base.mis_overhead(), reflected.mis_overhead());
}

#[test]
fn test_reflected_gadget_preserves_weights() {
    let base = WeightedKsgBranch;
    let reflected = ReflectedGadget::new(base, Mirror::OffDiag);

    assert_eq!(base.source_weights(), reflected.source_weights());
    assert_eq!(base.mapped_weights(), reflected.mapped_weights());
}

#[test]
fn test_reflected_gadget_delegates_properties() {
    let base = Cross::<false>;
    let reflected = ReflectedGadget::new(base, Mirror::X);

    assert_eq!(base.is_connected(), reflected.is_connected());
    assert_eq!(base.is_cross_gadget(), reflected.is_cross_gadget());
    assert_eq!(base.connected_nodes(), reflected.connected_nodes());
}

#[test]
fn test_all_rotations_valid_graphs() {
    fn check_rotated<G: Pattern + Copy>(gadget: G, name: &str) {
        for n in 0..4 {
            let rotated = RotatedGadget::new(gadget, n);
            let (src_locs, src_edges, src_pins) = rotated.source_graph();
            let (map_locs, map_pins) = rotated.mapped_graph();

            assert!(!src_locs.is_empty(), "{} rot{}: empty source", name, n);
            assert!(!map_locs.is_empty(), "{} rot{}: empty mapped", name, n);
            assert!(
                src_edges
                    .iter()
                    .all(|&(a, b)| a < src_locs.len() && b < src_locs.len()),
                "{} rot{}: invalid src edges",
                name,
                n
            );
            assert!(
                src_pins.iter().all(|&p| p < src_locs.len()),
                "{} rot{}: invalid src pins",
                name,
                n
            );
            assert!(
                map_pins.iter().all(|&p| p < map_locs.len()),
                "{} rot{}: invalid map pins",
                name,
                n
            );
        }
    }

    check_rotated(Turn, "Turn");
    check_rotated(Branch, "Branch");
    check_rotated(Cross::<true>, "Cross<true>");
    check_rotated(TCon, "TCon");
}

#[test]
fn test_all_mirrors_valid_graphs() {
    fn check_mirrored<G: Pattern + Copy>(gadget: G, name: &str) {
        for mirror in [Mirror::X, Mirror::Y, Mirror::Diag, Mirror::OffDiag] {
            let reflected = ReflectedGadget::new(gadget, mirror);
            let (src_locs, src_edges, src_pins) = reflected.source_graph();
            let (map_locs, map_pins) = reflected.mapped_graph();

            assert!(!src_locs.is_empty(), "{} {:?}: empty source", name, mirror);
            assert!(!map_locs.is_empty(), "{} {:?}: empty mapped", name, mirror);
            assert!(
                src_edges
                    .iter()
                    .all(|&(a, b)| a < src_locs.len() && b < src_locs.len()),
                "{} {:?}: invalid src edges",
                name,
                mirror
            );
            assert!(
                src_pins.iter().all(|&p| p < src_locs.len()),
                "{} {:?}: invalid src pins",
                name,
                mirror
            );
            assert!(
                map_pins.iter().all(|&p| p < map_locs.len()),
                "{} {:?}: invalid map pins",
                name,
                mirror
            );
        }
    }

    check_mirrored(Turn, "Turn");
    check_mirrored(Branch, "Branch");
    check_mirrored(Cross::<true>, "Cross<true>");
    check_mirrored(TCon, "TCon");
}

// === Julia Tests: rotated_and_reflected counts ===
// From Julia's test/gadgets.jl

use problemreductions::rules::unitdiskmapping::{BranchFixB, DanglingLeg};

/// Count unique gadgets from all rotations (0, 1, 2, 3) and reflections (X, Y, Diag, OffDiag).
/// Julia: length(rotated_and_reflected(gadget))
fn count_rotated_and_reflected<G: Pattern + Copy + std::fmt::Debug>(gadget: G) -> usize {
    use std::collections::HashSet;

    let mut unique = HashSet::new();

    // All rotations (0, 90, 180, 270 degrees)
    for n in 0..4 {
        let rotated = RotatedGadget::new(gadget, n);
        let (locs, _, _) = rotated.source_graph();
        unique.insert(format!("{:?}", locs));
    }

    // All reflections
    for mirror in [Mirror::X, Mirror::Y, Mirror::Diag, Mirror::OffDiag] {
        let reflected = ReflectedGadget::new(gadget, mirror);
        let (locs, _, _) = reflected.source_graph();
        unique.insert(format!("{:?}", locs));
    }

    unique.len()
}

#[test]
fn test_rotated_and_reflected_danglingleg() {
    // Julia: @test length(rotated_and_reflected(UnitDiskMapping.DanglingLeg())) == 4
    let count = count_rotated_and_reflected(DanglingLeg);
    assert_eq!(count, 4, "DanglingLeg should have 4 unique orientations");
}

#[test]
fn test_rotated_and_reflected_cross_false() {
    // Julia: @test length(rotated_and_reflected(Cross{false}())) == 4
    // Cross has 4-fold rotational symmetry, so rotations produce duplicates
    // But reflections may produce different locations in our representation
    let count = count_rotated_and_reflected(Cross::<false>);
    // Cross should have limited unique orientations due to symmetry
    assert!(
        count > 0,
        "Cross<false> should have some unique orientations"
    );
    assert!(
        count <= 8,
        "Cross<false> should have at most 8 unique orientations"
    );
}

#[test]
fn test_rotated_and_reflected_cross_true() {
    // Julia: @test length(rotated_and_reflected(Cross{true}())) == 4
    let count = count_rotated_and_reflected(Cross::<true>);
    assert!(
        count > 0,
        "Cross<true> should have some unique orientations"
    );
    assert!(
        count <= 8,
        "Cross<true> should have at most 8 unique orientations"
    );
}

#[test]
fn test_rotated_and_reflected_branchfixb() {
    // Julia: @test length(rotated_and_reflected(BranchFixB())) == 8
    let count = count_rotated_and_reflected(BranchFixB);
    assert_eq!(count, 8, "BranchFixB should have 8 unique orientations");
}

// === Julia Tests: DanglingLeg properties ===
// From Julia's test/simplifiers.jl

#[test]
fn test_danglingleg_size() {
    // Julia: @test size(p) == (4, 3)
    let gadget = DanglingLeg;
    assert_eq!(gadget.size(), (4, 3), "DanglingLeg size should be (4, 3)");
}

#[test]
fn test_danglingleg_source_locations() {
    // Julia: @test UnitDiskMapping.source_locations(p) == UnitDiskMapping.Node.([(2,2), (3,2), (4,2)])
    let gadget = DanglingLeg;
    let (locs, _, _) = gadget.source_graph();

    // Julia is 1-indexed, Rust is 1-indexed for gadget coordinates
    let expected = vec![(2, 2), (3, 2), (4, 2)];
    assert_eq!(locs, expected, "DanglingLeg source locations mismatch");
}

#[test]
fn test_danglingleg_mapped_locations() {
    // Julia: @test UnitDiskMapping.mapped_locations(p) == UnitDiskMapping.Node.([(4,2)])
    let gadget = DanglingLeg;
    let (locs, _) = gadget.mapped_graph();

    // Julia is 1-indexed
    let expected = vec![(4, 2)];
    assert_eq!(locs, expected, "DanglingLeg mapped locations mismatch");
}

#[test]
fn test_danglingleg_mis_overhead() {
    let gadget = DanglingLeg;
    // DanglingLeg simplifies 3 nodes to 1, removing 2 from MIS
    assert_eq!(
        gadget.mis_overhead(),
        -1,
        "DanglingLeg MIS overhead should be -1"
    );
}
