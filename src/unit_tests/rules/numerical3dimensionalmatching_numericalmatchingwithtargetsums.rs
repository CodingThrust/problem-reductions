use super::*;
use crate::models::misc::{Numerical3DimensionalMatching, NumericalMatchingWithTargetSums};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn yes_problem() -> Numerical3DimensionalMatching {
    Numerical3DimensionalMatching::new(vec![4, 5], vec![4, 5], vec![5, 7], 15)
}

#[test]
fn test_n3dm_to_nmts_structure() {
    let source = yes_problem();
    let reduction = ReduceTo::<NumericalMatchingWithTargetSums>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_pairs(), source.num_groups());
    assert_eq!(target.sizes_x(), &[4, 5]);
    assert_eq!(target.sizes_y(), &[5, 7]);
    assert_eq!(target.targets(), &[11, 10]);
}

#[test]
fn test_n3dm_to_nmts_closed_loop() {
    let source = yes_problem();
    let reduction = ReduceTo::<NumericalMatchingWithTargetSums>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "N3DM -> NMTS closed loop",
    );
}

#[test]
fn test_n3dm_to_nmts_extracts_target_witness_into_source_witness() {
    let source =
        Numerical3DimensionalMatching::new(vec![6, 8, 7], vec![6, 7, 8], vec![7, 7, 7], 21);
    let reduction = ReduceTo::<NumericalMatchingWithTargetSums>::reduce_to(&source)
        .expect("reduction should succeed");
    let target_solution = vec![2, 1, 0];

    assert!(
        reduction
            .target_problem()
            .evaluate(&target_solution)
            .unwrap()
            .0
    );

    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(extracted, vec![2, 0, 1, 0, 2, 1]);
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_n3dm_to_nmts_handles_repeated_targets() {
    let source = Numerical3DimensionalMatching::new(vec![4, 4], vec![4, 5], vec![7, 6], 15);
    let reduction = ReduceTo::<NumericalMatchingWithTargetSums>::reduce_to(&source)
        .expect("reduction should succeed");
    let target_solution = vec![0, 1];

    assert!(
        reduction
            .target_problem()
            .evaluate(&target_solution)
            .unwrap()
            .0
    );

    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(extracted.len(), 4);
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_n3dm_to_nmts_unsatisfiable_maps_to_unsatisfiable() {
    let source = Numerical3DimensionalMatching::new(vec![4, 6], vec![4, 6], vec![4, 6], 15);
    let reduction = ReduceTo::<NumericalMatchingWithTargetSums>::reduce_to(&source)
        .expect("reduction should succeed");

    assert!(BruteForce::new().find_witness(&source).unwrap().is_none());
    assert!(BruteForce::new()
        .find_witness(reduction.target_problem())
        .unwrap()
        .is_none());
}

#[cfg(feature = "example-db")]
#[test]
fn test_n3dm_to_nmts_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "numerical3dimensionalmatching_to_numericalmatchingwithtargetsums")
        .expect("missing canonical N3DM -> NMTS example spec");
    let example = (spec.build)();

    assert_eq!(example.source.problem, "Numerical3DimensionalMatching");
    assert_eq!(example.target.problem, "NumericalMatchingWithTargetSums");
    assert_eq!(example.solutions.len(), 1);
}
