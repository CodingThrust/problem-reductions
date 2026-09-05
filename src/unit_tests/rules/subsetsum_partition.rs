#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use crate::models::misc::{Partition, SubsetSum};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::traits::ReductionResult;
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
#[cfg(feature = "example-db")]
use crate::traits::Problem;

#[test]
fn test_subsetsum_to_partition_closed_loop() {
    let source = SubsetSum::new(vec![1u32, 5, 6, 8], 11u32);
    let reduction = ReduceTo::<Partition>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.sizes(), &[1, 5, 6, 8, 2]);
    assert_eq!(target.num_elements(), source.num_elements() + 1);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "SubsetSum -> Partition closed loop",
    );
}

#[test]
fn test_subsetsum_to_partition_sigma_greater_than_two_t_extraction() {
    let source = SubsetSum::new(vec![10u32, 20, 30], 10u32);
    let reduction = ReduceTo::<Partition>::reduce_to(&source).expect("reduction should succeed");

    assert_eq!(reduction.target_problem().sizes(), &[10, 20, 30, 40]);
    assert_eq!(
        reduction
            .extract_solution(&vec![true, false, false, true])
            .unwrap(),
        vec![true, false, false]
    );
    assert_eq!(
        reduction
            .extract_solution(&vec![false, true, true, false])
            .unwrap(),
        vec![true, false, false]
    );
}

#[test]
fn test_subsetsum_to_partition_sigma_equals_two_t_extraction() {
    let source = SubsetSum::new(vec![3u32, 5, 2, 6], 8u32);
    let reduction = ReduceTo::<Partition>::reduce_to(&source).expect("reduction should succeed");

    assert_eq!(reduction.target_problem().sizes(), &[3, 5, 2, 6]);
    assert_eq!(
        reduction
            .extract_solution(&vec![true, true, false, false])
            .unwrap(),
        vec![true, true, false, false]
    );
}

#[test]
fn test_subsetsum_to_partition_unsatisfiable_instance_stays_unsatisfiable() {
    let source = SubsetSum::new(vec![3u32, 7, 11], 5u32);
    let reduction = ReduceTo::<Partition>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.sizes(), &[3, 7, 11, 11]);
    assert!(BruteForce::new().solve(&source).unwrap().is_none());
    assert!(BruteForce::new().solve(target).unwrap().is_none());
}

#[cfg(feature = "example-db")]
#[test]
fn test_subsetsum_to_partition_canonical_example_spec() {
    let example = (canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "subsetsum_to_partition")
        .expect("missing canonical SubsetSum -> Partition example spec")
        .build)();

    assert_eq!(example.source.problem, "SubsetSum");
    assert_eq!(example.target.problem, "Partition");
    assert_eq!(
        example.target.instance["sizes"],
        serde_json::json!([1, 5, 6, 8, 2])
    );
    assert_eq!(example.solutions.len(), 1);
    assert_eq!(
        example.solutions[0].source_config,
        serde_json::json!([false, true, true, false])
    );
    assert_eq!(
        example.solutions[0].target_config,
        serde_json::json!([false, true, true, false, false])
    );

    let source: SubsetSum = serde_json::from_value(example.source.instance.clone())
        .expect("source example deserializes");
    let target: Partition = serde_json::from_value(example.target.instance.clone())
        .expect("target example deserializes");

    let source_config: Vec<bool> =
        serde_json::from_value(example.solutions[0].source_config.clone()).unwrap();
    let target_config: Vec<bool> =
        serde_json::from_value(example.solutions[0].target_config.clone()).unwrap();
    assert!(source.evaluate(&source_config).unwrap().is_valid());
    assert!(target.evaluate(&target_config).unwrap().is_valid());
}
