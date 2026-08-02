use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::traits::Problem;
use std::collections::HashSet;

#[test]
fn test_threedimensionalmatching_to_exactcoverby3sets_closed_loop() {
    let source = ThreeDimensionalMatching::new(
        3,
        vec![(0, 0, 0), (1, 1, 1), (2, 2, 2), (0, 1, 2), (1, 2, 0)],
    );
    let reduction = ReduceTo::<ExactCoverBy3Sets>::reduce_to(&source);
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "ThreeDimensionalMatching -> ExactCoverBy3Sets",
    );
}

#[test]
fn test_all_q2_triple_families_preserve_exact_witnesses() {
    let triples: Vec<_> = (0..2)
        .flat_map(|w| (0..2).flat_map(move |x| (0..2).map(move |y| (w, x, y))))
        .collect();
    let solver = BruteForce::new();

    for family_mask in 0..(1usize << triples.len()) {
        let family: Vec<_> = triples
            .iter()
            .enumerate()
            .filter_map(|(index, &triple)| ((family_mask >> index) & 1 == 1).then_some(triple))
            .collect();
        let source = ThreeDimensionalMatching::new(2, family);
        let reduction = ReduceTo::<ExactCoverBy3Sets>::reduce_to(&source);
        let source_witnesses: HashSet<_> = solver.find_all_witnesses(&source).into_iter().collect();
        let extracted_witnesses: HashSet<_> = solver
            .find_all_witnesses(reduction.target_problem())
            .into_iter()
            .map(|witness| reduction.extract_solution(&witness))
            .collect();

        assert_eq!(
            extracted_witnesses, source_witnesses,
            "witness mismatch for family mask {family_mask:#010b}"
        );
    }
}

#[test]
fn test_target_tagging_and_overhead() {
    let source = ThreeDimensionalMatching::new(3, vec![(0, 2, 1), (2, 0, 2)]);
    let reduction = ReduceTo::<ExactCoverBy3Sets>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.universe_size(), 9);
    assert_eq!(target.subsets(), &[[0, 5, 7], [2, 3, 8]]);
    assert_eq!(target.num_subsets(), 2);
    assert_eq!(target.num_sets(), 2);

    let entries: Vec<_> = inventory::iter::<crate::rules::ReductionEntry>()
        .filter(|entry| {
            entry.source_name == "ThreeDimensionalMatching"
                && entry.target_name == "ExactCoverBy3Sets"
        })
        .collect();
    assert_eq!(entries.len(), 1);
    let overhead = (entries[0].overhead_eval_fn)(&source as &dyn std::any::Any);
    assert_eq!(overhead.get("universe_size"), Some(9));
    assert_eq!(overhead.get("num_subsets"), Some(2));
    assert_eq!(overhead.get("num_sets"), Some(2));
}

#[test]
fn test_infeasible_instance_has_no_target_witness() {
    let source = ThreeDimensionalMatching::new(3, vec![(0, 0, 0), (0, 1, 1), (1, 2, 2)]);
    let reduction = ReduceTo::<ExactCoverBy3Sets>::reduce_to(&source);

    assert!(BruteForce::new().find_witness(&source).is_none());
    assert!(BruteForce::new()
        .find_witness(reduction.target_problem())
        .is_none());
}

#[test]
fn test_empty_universe_preserves_empty_witness() {
    let source = ThreeDimensionalMatching::new(0, vec![]);
    let reduction = ReduceTo::<ExactCoverBy3Sets>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.universe_size(), 0);
    assert!(target.subsets().is_empty());
    let target_witness = BruteForce::new().find_witness(target).unwrap();
    assert!(target_witness.is_empty());
    assert!(
        source
            .evaluate(&reduction.extract_solution(&target_witness))
            .0
    );
}

#[test]
fn test_duplicate_triples_preserve_indices() {
    let source = ThreeDimensionalMatching::new(2, vec![(0, 0, 0), (0, 0, 0), (1, 1, 1)]);
    let reduction = ReduceTo::<ExactCoverBy3Sets>::reduce_to(&source);

    assert_eq!(
        reduction.target_problem().subsets(),
        &[[0, 2, 4], [0, 2, 4], [1, 3, 5]]
    );
    let witnesses: HashSet<_> = BruteForce::new()
        .find_all_witnesses(reduction.target_problem())
        .into_iter()
        .collect();
    assert_eq!(witnesses, HashSet::from([vec![0, 1, 1], vec![1, 0, 1]]));
    for witness in witnesses {
        assert!(source.evaluate(&reduction.extract_solution(&witness)).0);
    }
}

#[test]
fn test_unused_coordinate_makes_both_instances_infeasible() {
    let source = ThreeDimensionalMatching::new(2, vec![(0, 0, 0), (1, 0, 1)]);
    let reduction = ReduceTo::<ExactCoverBy3Sets>::reduce_to(&source);

    assert!(BruteForce::new().find_witness(&source).is_none());
    assert!(BruteForce::new()
        .find_witness(reduction.target_problem())
        .is_none());
}

#[test]
fn test_equal_numeric_coordinates_are_distinct_across_domains() {
    let source = ThreeDimensionalMatching::new(2, vec![(0, 0, 0), (1, 1, 1)]);
    let reduction = ReduceTo::<ExactCoverBy3Sets>::reduce_to(&source);

    assert_eq!(
        reduction.target_problem().subsets(),
        &[[0, 2, 4], [1, 3, 5]]
    );
    assert!(reduction.target_problem().evaluate(&[1, 1]).0);
    assert!(source.evaluate(&reduction.extract_solution(&[1, 1])).0);
}

#[test]
fn test_solution_extraction_is_identity() {
    let source = ThreeDimensionalMatching::new(2, vec![(0, 0, 0), (1, 1, 1)]);
    let reduction = ReduceTo::<ExactCoverBy3Sets>::reduce_to(&source);

    assert_eq!(reduction.extract_solution(&[1, 0]), vec![1, 0]);
}
