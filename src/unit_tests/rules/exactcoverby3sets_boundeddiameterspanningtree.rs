use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::topology::Graph;

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
    let reduction = ReduceTo::<BoundedDiameterSpanningTree<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "ExactCoverBy3Sets -> BoundedDiameterSpanningTree closed loop",
    );
}

#[test]
fn test_exactcoverby3sets_to_boundeddiameterspanningtree_structure() {
    let source = yes_instance_simple();
    let reduction = ReduceTo::<BoundedDiameterSpanningTree<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
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
    let expected_weight_bound = i64::try_from(4 * q + m + 2).unwrap();
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
    let reduction = ReduceTo::<BoundedDiameterSpanningTree<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");

    // The full feasible tree selects every edge except the set-clique edge.
    let mut target_config = vec![true; reduction.target_problem().num_edges()];
    *target_config.last_mut().unwrap() = false;
    assert_eq!(
        reduction.extract_solution(&target_config).unwrap(),
        vec![true, true]
    );

    // Root indicators alone are not a spanning-tree certificate.
    let mut invalid = vec![false; target_config.len()];
    invalid[2] = true;
    invalid[3] = true;
    assert!(reduction.extract_solution(&invalid).is_err());
    assert!(reduction.extract_solution(&vec![]).is_err());
    assert!(reduction
        .extract_solution(&vec![true; target_config.len()])
        .is_err());
}

#[test]
fn test_exactcoverby3sets_to_boundeddiameterspanningtree_no_instance() {
    let source = no_instance_simple();
    let reduction = ReduceTo::<BoundedDiameterSpanningTree<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    // The target should be infeasible: no spanning tree satisfies both weight
    // bound B = 4q + m + 2 = 12 and diameter bound D = 4. For an Or-valued
    // problem with no satisfying configuration, BruteForce::solve
    // returns None (witnesses are configs that evaluate to Or(true), and none
    // exist here). Equivalently, the brute-force aggregate evaluates to
    // Or(false).
    assert!(BruteForce::new().solve(target).unwrap().is_none());
    assert!(reduction.extract_solution(&vec![]).is_err());
}

#[test]
fn test_exactcoverby3sets_to_boundeddiameterspanningtree_universe_boundaries() {
    for universe in [3, usize::MAX - usize::MAX % 3] {
        let source = ExactCoverBy3Sets::new(universe, vec![]);
        let reduction =
            ReduceTo::<BoundedDiameterSpanningTree<SimpleGraph, i64>>::reduce_to(&source).unwrap();
        assert_eq!(reduction.target_problem().num_vertices(), 2);
        assert_eq!(reduction.target_problem().num_edges(), 0);
        assert!(BruteForce::new()
            .solve(reduction.target_problem())
            .unwrap()
            .is_none());
        assert!(reduction.extract_solution(&vec![]).is_err());
    }
    let source = ExactCoverBy3Sets::new(0, vec![]);
    let reduction =
        ReduceTo::<BoundedDiameterSpanningTree<SimpleGraph, i64>>::reduce_to(&source).unwrap();
    let witness = BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
        .unwrap();
    assert_eq!(witness, vec![true, true]);
    assert_eq!(
        reduction.extract_solution(&witness).unwrap(),
        Vec::<bool>::new()
    );
}

#[test]
fn test_exactcoverby3sets_to_boundeddiameterspanningtree_duplicate_sets() {
    let source = ExactCoverBy3Sets::new(3, vec![[0, 1, 2], [0, 1, 2]]);
    let reduction =
        ReduceTo::<BoundedDiameterSpanningTree<SimpleGraph, i64>>::reduce_to(&source).unwrap();
    let witnesses = BruteForce::new()
        .find_all_witnesses(reduction.target_problem())
        .unwrap();
    assert!(!witnesses.is_empty());
    for witness in witnesses {
        let extracted = reduction.extract_solution(&witness).unwrap();
        assert!(source.is_valid_solution(&extracted).unwrap());
        assert_eq!(extracted.iter().filter(|&&x| x).count(), 1);
    }
}

#[test]
fn test_exactcoverby3sets_to_boundeddiameterspanningtree_dimension_arithmetic() {
    type R = ReductionX3CToBoundedDiameterSpanningTree;
    assert_eq!(R::dimensions(0, 0).unwrap(), (3, 2, 2));
    assert_eq!(R::dimensions(6, 4).unwrap(), (13, 24, 14));
    assert!(R::dimensions(usize::MAX, 0).is_err());
    assert!(R::dimensions(0, usize::MAX / 2).is_err());
    assert!(R::dimensions(usize::MAX - 3, 0).is_err());
}
