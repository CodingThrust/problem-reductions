use super::*;

#[test]
fn test_triturn_weighted() {
    let weighted = TriTurn.weighted();
    assert_eq!(weighted.source_weights, vec![2, 2, 2, 2]);
    assert_eq!(weighted.mapped_weights, vec![2, 2, 2, 2]);
}

#[test]
fn test_tribranch_weighted() {
    let weighted = TriBranch.weighted();
    // Julia: sw = [2,2,3,2,2,2,2,2,2], mw = [2,2,2,3,2,2,2,2,2]
    assert_eq!(weighted.source_weights, vec![2, 2, 3, 2, 2, 2, 2, 2, 2]);
    assert_eq!(weighted.mapped_weights, vec![2, 2, 2, 3, 2, 2, 2, 2, 2]);
}

#[test]
fn test_tricross_true_weighted() {
    let weighted = TriCross::<true>.weighted();
    // Julia: sw = [2,2,2,2,2,2,2,2,2,2], mw = [3,2,3,3,2,2,2,2,2,2,2]
    assert_eq!(weighted.source_weights, vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2]);
    assert_eq!(
        weighted.mapped_weights,
        vec![3, 2, 3, 3, 2, 2, 2, 2, 2, 2, 2]
    );
}

#[test]
fn test_tricross_false_weighted() {
    let weighted = TriCross::<false>.weighted();
    // Julia: sw = [2,2,2,2,2,2,2,2,2,2,2,2], mw = [3,3,2,4,2,2,2,4,3,2,2,2,2,2,2,2]
    assert_eq!(
        weighted.source_weights,
        vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]
    );
    assert_eq!(
        weighted.mapped_weights,
        vec![3, 3, 2, 4, 2, 2, 2, 4, 3, 2, 2, 2, 2, 2, 2, 2]
    );
}

#[test]
fn test_all_weighted_gadgets_have_correct_lengths() {
    use super::super::triangular::TriangularGadget;

    fn check<G: TriangularGadget + Weightable + Clone>(g: G, name: &str) {
        let weighted = g.clone().weighted();
        let (src_locs, _, _) = g.source_graph();
        let (map_locs, _) = g.mapped_graph();
        assert_eq!(
            weighted.source_weights.len(),
            src_locs.len(),
            "{}: source weights length mismatch",
            name
        );
        assert_eq!(
            weighted.mapped_weights.len(),
            map_locs.len(),
            "{}: mapped weights length mismatch",
            name
        );
    }

    check(TriTurn, "TriTurn");
    check(TriBranch, "TriBranch");
    check(TriCross::<true>, "TriCross<true>");
    check(TriCross::<false>, "TriCross<false>");
    check(TriTConLeft, "TriTConLeft");
    check(TriTConDown, "TriTConDown");
    check(TriTConUp, "TriTConUp");
    check(TriTrivialTurnLeft, "TriTrivialTurnLeft");
    check(TriTrivialTurnRight, "TriTrivialTurnRight");
    check(TriEndTurn, "TriEndTurn");
    check(TriWTurn, "TriWTurn");
    check(TriBranchFix, "TriBranchFix");
    check(TriBranchFixB, "TriBranchFixB");
}

#[test]
fn test_triangular_weighted_ruleset_has_13_gadgets() {
    let ruleset = super::triangular_weighted_ruleset();
    assert_eq!(ruleset.len(), 13);
}

#[test]
fn test_trace_centers_basic() {
    use crate::rules::unitdiskmapping::triangular::map_weighted;

    let edges = vec![(0, 1), (1, 2)];
    let result = map_weighted(3, &edges);

    let centers = super::trace_centers(&result);
    assert_eq!(centers.len(), 3);

    // Centers should be valid grid positions
    for (row, col) in &centers {
        assert!(*row > 0);
        assert!(*col > 0);
    }
}

#[test]
fn test_map_weights_basic() {
    use crate::rules::unitdiskmapping::triangular::map_weighted;
    let edges = vec![(0, 1), (1, 2)];
    let result = map_weighted(3, &edges);

    let source_weights = vec![0.5, 0.3, 0.7];
    let grid_weights = super::map_weights(&result, &source_weights);

    // Should have same length as grid nodes
    assert_eq!(grid_weights.len(), result.positions.len());

    // All weights should be positive
    assert!(grid_weights.iter().all(|&w| w > 0.0));
}

#[test]
#[should_panic(expected = "all weights must be in range")]
fn test_map_weights_rejects_invalid() {
    use crate::rules::unitdiskmapping::triangular::map_weighted;

    let edges = vec![(0, 1)];
    let result = map_weighted(2, &edges);

    let source_weights = vec![1.5, 0.3]; // Invalid: > 1
    super::map_weights(&result, &source_weights);
}
