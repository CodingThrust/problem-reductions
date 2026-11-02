use super::*;
use crate::rules::ReductionError;
use crate::rules::test_helpers::{
    assert_parameter_predictions, assert_satisfaction_round_trip_from_satisfaction_target,
};
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

fn reduce(
    source: &ThreeDimensionalMatching,
) -> ReductionThreeDimensionalMatchingToExactCoverBy3Sets {
    ReduceTo::<ExactCoverBy3Sets>::reduce_to(source).expect("reduction should succeed")
}

/// Every selection vector of length `n`, in lexicographic bit order.
fn all_configs(n: usize) -> impl Iterator<Item = Vec<bool>> {
    (0..1usize << n).map(move |bits| (0..n).map(|i| bits >> i & 1 == 1).collect())
}

/// The source and target agree on every selection vector, and every target
/// exact cover extracts to itself.
fn assert_identical_feasibility(source: &ThreeDimensionalMatching) {
    let reduction = reduce(source);
    let target = reduction.target_problem();
    assert_parameter_predictions(source, &reduction);
    for config in all_configs(source.num_triples()) {
        let source_value = source.evaluate(&config).unwrap();
        assert_eq!(
            source_value,
            target.evaluate(&config).unwrap(),
            "triples {:?}, config {config:?}",
            source.triples()
        );
        match reduction.extract_solution(&config) {
            Ok(extracted) => {
                assert!(source_value.0);
                assert_eq!(extracted, config);
            }
            Err(_) => assert!(!source_value.0),
        }
    }
}

#[test]
fn test_threedimensionalmatching_to_exactcoverby3sets_closed_loop() {
    // The issue's example: the diagonal is the unique perfect matching.
    let source = ThreeDimensionalMatching::new(
        3,
        vec![(0, 0, 0), (1, 1, 1), (2, 2, 2), (0, 1, 2), (1, 2, 0)],
    );
    let reduction = reduce(&source);
    let target = reduction.target_problem();

    assert_eq!(target.universe_size(), 9);
    assert_eq!(
        target.subsets(),
        &[[0, 3, 6], [1, 4, 7], [2, 5, 8], [0, 4, 8], [1, 5, 6]]
    );
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "ThreeDimensionalMatching -> ExactCoverBy3Sets closed loop",
    );

    let covers = BruteForce::new().find_all_witnesses(target).unwrap();
    assert_eq!(covers, vec![vec![true, true, true, false, false]]);
    assert_eq!(
        reduction.extract_solution(&covers[0]).unwrap(),
        vec![true, true, true, false, false]
    );
    // The two cross triples are disjoint but cannot be completed.
    let partial = vec![false, false, false, true, true];
    assert_eq!(target.evaluate(&partial).unwrap(), Or(false));
    assert!(reduction.extract_solution(&partial).is_err());
    assert_identical_feasibility(&source);
}

#[test]
fn test_threedimensionalmatching_to_exactcoverby3sets_every_family_q2() {
    // All 8 triples over {0,1}^3; each of the 256 families is checked on every
    // selection vector, so witness sets agree exactly.
    let all_triples: Vec<(usize, usize, usize)> = (0..8)
        .map(|t| (t & 1, (t >> 1) & 1, (t >> 2) & 1))
        .collect();
    let mut feasible_families = 0;
    for family in 0..1usize << all_triples.len() {
        let triples: Vec<_> = (0..all_triples.len())
            .filter(|&i| family >> i & 1 == 1)
            .map(|i| all_triples[i])
            .collect();
        let source = ThreeDimensionalMatching::new(2, triples);
        assert_identical_feasibility(&source);

        let source_witnesses = BruteForce::new().find_all_witnesses(&source).unwrap();
        let reduction = reduce(&source);
        let target_witnesses = BruteForce::new()
            .find_all_witnesses(reduction.target_problem())
            .unwrap();
        assert_eq!(source_witnesses, target_witnesses);
        feasible_families += usize::from(!source_witnesses.is_empty());
    }
    // The perfect matchings are the 4 disjoint pairs {(0,a,b), (1,1-a,1-b)},
    // which partition the 8 triples. A family is infeasible iff it misses at
    // least one triple of every pair: 3^4 = 81 families.
    assert_eq!(feasible_families, 256 - 81);
}

#[test]
fn test_threedimensionalmatching_to_exactcoverby3sets_empty() {
    let source = ThreeDimensionalMatching::new(0, vec![]);
    let reduction = reduce(&source);
    let target = reduction.target_problem();
    assert_eq!(target.universe_size(), 0);
    assert_eq!(target.num_subsets(), 0);
    assert_eq!(target.evaluate(&vec![]).unwrap(), Or(true));
    assert_eq!(
        reduction.extract_solution(&vec![]).unwrap(),
        Vec::<bool>::new()
    );
    assert_identical_feasibility(&source);
}

#[test]
fn test_threedimensionalmatching_to_exactcoverby3sets_duplicate_triples() {
    let source = ThreeDimensionalMatching::new(2, vec![(0, 1, 0), (0, 1, 0), (1, 0, 1)]);
    let reduction = reduce(&source);
    assert_eq!(
        reduction.target_problem().subsets(),
        &[[0, 3, 4], [0, 3, 4], [1, 2, 5]]
    );
    // Selecting both copies reuses every element.
    assert_eq!(
        reduction
            .target_problem()
            .evaluate(&vec![true, true, false])
            .unwrap(),
        Or(false)
    );
    let mut witnesses = BruteForce::new()
        .find_all_witnesses(reduction.target_problem())
        .unwrap();
    witnesses.sort();
    assert_eq!(
        witnesses,
        vec![vec![false, true, true], vec![true, false, true]]
    );
    assert_identical_feasibility(&source);
}

#[test]
fn test_threedimensionalmatching_to_exactcoverby3sets_unused_coordinate_is_infeasible() {
    // X coordinate 2 never occurs, so no perfect matching and no exact cover.
    let source = ThreeDimensionalMatching::new(
        3,
        vec![(0, 0, 0), (1, 1, 1), (2, 1, 2), (2, 0, 2), (1, 0, 1)],
    );
    let reduction = reduce(&source);
    assert_eq!(BruteForce::new().solve(&source).unwrap(), None);
    assert_eq!(
        BruteForce::new().solve(reduction.target_problem()).unwrap(),
        None
    );
    assert_identical_feasibility(&source);
}

#[test]
fn test_threedimensionalmatching_to_exactcoverby3sets_tags_equal_coordinates() {
    // Equal numeric coordinates in different domains become distinct elements.
    let source = ThreeDimensionalMatching::new(4, vec![(3, 3, 3), (0, 2, 1)]);
    let reduction = reduce(&source);
    assert_eq!(reduction.target_problem().universe_size(), 12);
    assert_eq!(
        reduction.target_problem().subsets(),
        &[[3, 7, 11], [0, 6, 9]]
    );
    assert_identical_feasibility(&source);
}

#[test]
fn test_threedimensionalmatching_to_exactcoverby3sets_extraction_rejects_non_covers() {
    let source = ThreeDimensionalMatching::new(2, vec![(0, 0, 0), (1, 1, 1), (0, 1, 1)]);
    let reduction = reduce(&source);
    assert_eq!(
        reduction
            .extract_solution(&vec![true, true, false])
            .unwrap(),
        vec![true, true, false]
    );
    // Too few subsets.
    assert!(
        reduction
            .extract_solution(&vec![true, false, false])
            .is_err()
    );
    // Right count but overlapping.
    assert!(
        reduction
            .extract_solution(&vec![true, false, true])
            .is_err()
    );
    // Too many subsets.
    assert!(reduction.extract_solution(&vec![true, true, true]).is_err());
    // Wrong length is malformed rather than infeasible.
    assert!(reduction.extract_solution(&vec![true, true]).is_err());
    assert!(
        reduction
            .extract_solution(&vec![true, true, false, false])
            .is_err()
    );
}

#[test]
fn test_threedimensionalmatching_to_exactcoverby3sets_aggregate_identity() {
    for triples in [
        vec![(0, 0, 0), (1, 1, 1)],
        vec![(0, 0, 0), (0, 1, 1), (1, 0, 1)],
    ] {
        let source = ThreeDimensionalMatching::new(2, triples);
        let reduction = reduce(&source);
        let source_value = Or(BruteForce::new().solve(&source).unwrap().is_some());
        let target_value = Or(BruteForce::new()
            .solve(ReductionResult::target_problem(&reduction))
            .unwrap()
            .is_some());
        assert_eq!(
            crate::rules::AggregateReductionResult::extract_value(&reduction, target_value),
            source_value
        );
    }
}

#[test]
fn test_threedimensionalmatching_to_exactcoverby3sets_universe_overflow() {
    // The largest q whose tagged universe 3q fits in usize still reduces.
    let largest = usize::MAX / 3;
    let reduction = reduce(&ThreeDimensionalMatching::new(largest, vec![]));
    assert_eq!(reduction.target_problem().universe_size(), 3 * largest);
    // One more is rejected with a typed overflow error rather than a panic.
    let source = ThreeDimensionalMatching::new(largest + 1, vec![]);
    let error = ReduceTo::<ExactCoverBy3Sets>::reduce_to(&source).unwrap_err();
    assert!(matches!(error, ReductionError::IntegerOverflow { .. }));
}
