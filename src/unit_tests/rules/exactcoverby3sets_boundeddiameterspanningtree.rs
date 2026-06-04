use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::{BruteForce, Solver};
use crate::topology::Graph;
use crate::types::Or;

/// q = 2, m = 2: X = {0..5} with C = [{0,1,2}, {3,4,5}].
/// Both subsets together form the unique exact cover.
fn yes_instance_simple() -> ExactCoverBy3Sets {
    ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5]])
}

/// q = 2, m = 2 but the two subsets overlap on element 0,
/// so no exact cover exists.
fn no_instance_simple() -> ExactCoverBy3Sets {
    ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [0, 3, 4]])
}

#[test]
fn test_exactcoverby3sets_to_boundeddiameterspanningtree_closed_loop() {
    let source = yes_instance_simple();
    let reduction = ReduceTo::<BoundedDiameterSpanningTree<SimpleGraph, i32>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "ExactCoverBy3Sets -> BoundedDiameterSpanningTree closed loop",
    );
}

#[test]
fn test_exactcoverby3sets_to_boundeddiameterspanningtree_structure() {
    let source = yes_instance_simple();
    let reduction = ReduceTo::<BoundedDiameterSpanningTree<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    let m = source.num_subsets();
    let q = source.q();
    // n = 3 + m + 3q
    assert_eq!(target.num_vertices(), 3 + m + source.universe_size());
    // Expected edge count: 2 forced + m root-to-set + 3m set-to-element + m(m-1)/2 clique.
    let expected_edges = 2 + m + 3 * m + m * (m - 1) / 2;
    assert_eq!(target.num_edges(), expected_edges);

    // Diameter bound is always 4 in the canonical construction.
    assert_eq!(target.diameter_bound(), 4);
    // Weight bound B = 4q + m + 2.
    let expected_weight_bound = (4 * q + m + 2) as i32;
    assert_eq!(*target.weight_bound(), expected_weight_bound);

    // Verify the first two edges are the forced-center path with weight 1.
    let edges = target.graph().edges();
    let weights = target.edge_weights();
    assert_eq!(edges[0], (0, 1));
    assert_eq!(weights[0], 1);
    assert_eq!(edges[1], (1, 2));
    assert_eq!(weights[1], 1);

    // Root-to-set edges follow, at indices 2..2+m, weight 2.
    for i in 0..m {
        assert_eq!(edges[2 + i], (0, 3 + i));
        assert_eq!(weights[2 + i], 2);
    }
}

#[test]
fn test_exactcoverby3sets_to_boundeddiameterspanningtree_extract_solution() {
    let source = yes_instance_simple();
    let reduction = ReduceTo::<BoundedDiameterSpanningTree<SimpleGraph, i32>>::reduce_to(&source);

    // Build a target config that selects both root-to-set edges (indices 2 and 3).
    // The remaining selections do not matter for extraction.
    let mut target_config = vec![0; reduction.target_problem().num_edges()];
    target_config[2] = 1;
    target_config[3] = 1;
    let extracted = reduction.extract_solution(&target_config);
    assert_eq!(extracted, vec![1, 1]);

    // Only s_0 selected via root edge.
    let mut target_config = vec![0; reduction.target_problem().num_edges()];
    target_config[2] = 1;
    let extracted = reduction.extract_solution(&target_config);
    assert_eq!(extracted, vec![1, 0]);
}

#[test]
fn test_exactcoverby3sets_to_boundeddiameterspanningtree_no_instance() {
    let source = no_instance_simple();
    let reduction = ReduceTo::<BoundedDiameterSpanningTree<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // The target should be infeasible: no spanning tree satisfies both weight
    // bound B = 4q + m + 2 = 12 and diameter bound D = 4. For an Or-valued
    // problem with no satisfying configuration, BruteForce::find_witness
    // returns None (witnesses are configs that evaluate to Or(true), and none
    // exist here). Equivalently, the brute-force aggregate evaluates to
    // Or(false).
    assert!(BruteForce::new().find_witness(target).is_none());
    assert_eq!(BruteForce::new().solve(target), Or(false));
}
