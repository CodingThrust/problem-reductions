use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;

#[test]
fn test_minimumdominatingset_to_minimumsetcovering_closed_loop() {
    let source = MinimumDominatingSet::new(SimpleGraph::path(5), vec![3, 1, 4, 1, 3]);
    let reduction = ReduceTo::<MinimumSetCovering<i32>>::reduce_to(&source);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MinimumDominatingSet -> MinimumSetCovering weighted path",
    );

    let target_witnesses = BruteForce::new().find_all_witnesses(reduction.target_problem());
    assert_eq!(target_witnesses, vec![vec![0, 1, 0, 1, 0]]);
    assert_eq!(
        reduction.extract_solution(&target_witnesses[0]),
        vec![0, 1, 0, 1, 0]
    );
}

#[test]
fn test_exact_target_structure() {
    let source = MinimumDominatingSet::new(SimpleGraph::path(5), vec![3, 1, 4, 1, 3]);
    let reduction = ReduceTo::<MinimumSetCovering<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.universe_size(), 5);
    assert_eq!(target.num_sets(), 5);
    assert_eq!(
        target.sets(),
        &[
            vec![0, 1],
            vec![0, 1, 2],
            vec![1, 2, 3],
            vec![2, 3, 4],
            vec![3, 4],
        ]
    );
    assert_eq!(target.weights_ref(), &[3, 1, 4, 1, 3]);
}

#[test]
fn test_signed_weight_optimality_and_extraction() {
    let source = MinimumDominatingSet::new(SimpleGraph::path(3), vec![-5, 10, -7]);
    let reduction = ReduceTo::<MinimumSetCovering<i32>>::reduce_to(&source);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MinimumDominatingSet -> MinimumSetCovering signed weights",
    );

    let target_witnesses = BruteForce::new().find_all_witnesses(reduction.target_problem());
    assert_eq!(target_witnesses, vec![vec![1, 0, 1]]);
    assert_eq!(
        reduction.extract_solution(&target_witnesses[0]),
        vec![1, 0, 1]
    );
}

#[test]
fn test_empty_and_isolated_graphs() {
    let empty = MinimumDominatingSet::new(SimpleGraph::empty(0), vec![]);
    let empty_reduction = ReduceTo::<MinimumSetCovering<i32>>::reduce_to(&empty);
    assert_eq!(empty_reduction.target_problem().universe_size(), 0);
    assert!(empty_reduction.target_problem().sets().is_empty());
    assert_optimization_round_trip_from_optimization_target(
        &empty,
        &empty_reduction,
        "empty MinimumDominatingSet",
    );

    let isolated = MinimumDominatingSet::new(SimpleGraph::empty(3), vec![3, 2, 1]);
    let isolated_reduction = ReduceTo::<MinimumSetCovering<i32>>::reduce_to(&isolated);
    assert_eq!(
        isolated_reduction.target_problem().sets(),
        &[vec![0], vec![1], vec![2]]
    );
    assert_eq!(
        BruteForce::new().find_all_witnesses(isolated_reduction.target_problem()),
        vec![vec![1, 1, 1]]
    );
    assert_optimization_round_trip_from_optimization_target(
        &isolated,
        &isolated_reduction,
        "isolated MinimumDominatingSet",
    );
}

#[test]
fn test_self_loops_and_repeated_edges_are_deduplicated() {
    let source = MinimumDominatingSet::new(
        SimpleGraph::new(4, vec![(0, 0), (0, 1), (0, 1), (1, 0), (2, 2)]),
        vec![-4, 2, -1, 7],
    );
    let reduction = ReduceTo::<MinimumSetCovering<i32>>::reduce_to(&source);

    assert_eq!(
        reduction.target_problem().sets(),
        &[vec![0, 1], vec![0, 1], vec![2], vec![3]]
    );
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MinimumDominatingSet with loops and repeated edges",
    );
}
