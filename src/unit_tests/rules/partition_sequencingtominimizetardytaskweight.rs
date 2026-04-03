#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use crate::models::misc::{Partition, SequencingToMinimizeTardyTaskWeight};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::rules::traits::ReductionResult;
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_partition_to_sequencing_to_minimize_tardy_task_weight_closed_loop() {
    let source = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    let reduction = ReduceTo::<SequencingToMinimizeTardyTaskWeight>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "Partition -> SequencingToMinimizeTardyTaskWeight closed loop",
    );
}

#[test]
fn test_partition_to_sequencing_to_minimize_tardy_task_weight_structure() {
    let source = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    let reduction = ReduceTo::<SequencingToMinimizeTardyTaskWeight>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.lengths(), &[3, 1, 1, 2, 2, 1]);
    assert_eq!(target.weights(), &[3, 1, 1, 2, 2, 1]);
    assert_eq!(target.deadlines(), &[5, 5, 5, 5, 5, 5]);
    assert_eq!(target.num_tasks(), source.num_elements());
}

#[test]
fn test_partition_to_sequencing_to_minimize_tardy_task_weight_extract_solution() {
    let source = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    let reduction = ReduceTo::<SequencingToMinimizeTardyTaskWeight>::reduce_to(&source);

    assert_eq!(
        reduction.extract_solution(&[1, 1, 2, 2, 0, 0]),
        vec![1, 0, 0, 1, 0, 0]
    );
}

#[test]
fn test_partition_to_sequencing_to_minimize_tardy_task_weight_odd_total_is_unsatisfying() {
    let source = Partition::new(vec![2, 4, 5]);
    let reduction = ReduceTo::<SequencingToMinimizeTardyTaskWeight>::reduce_to(&source);
    let target = reduction.target_problem();
    let best = BruteForce::new()
        .find_witness(target)
        .expect("target should always have an optimal schedule");

    assert_eq!(target.evaluate(&best), Min(Some(6)));
    assert!(!source.evaluate(&reduction.extract_solution(&best)));
}

#[cfg(feature = "example-db")]
#[test]
fn test_partition_to_sequencing_to_minimize_tardy_task_weight_canonical_example_spec() {
    let example = (canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "partition_to_sequencing_to_minimize_tardy_task_weight")
        .expect("missing canonical Partition -> SequencingToMinimizeTardyTaskWeight example spec")
        .build)();

    assert_eq!(example.source.problem, "Partition");
    assert_eq!(
        example.target.problem,
        "SequencingToMinimizeTardyTaskWeight"
    );
    assert_eq!(
        example.target.instance["lengths"],
        serde_json::json!([3, 1, 1, 2, 2, 1])
    );
    assert_eq!(
        example.target.instance["weights"],
        serde_json::json!([3, 1, 1, 2, 2, 1])
    );
    assert_eq!(
        example.target.instance["deadlines"],
        serde_json::json!([5, 5, 5, 5, 5, 5])
    );
    assert_eq!(example.solutions.len(), 1);
    assert_eq!(example.solutions[0].source_config, vec![1, 0, 0, 1, 0, 0]);
    assert_eq!(example.solutions[0].target_config, vec![1, 1, 2, 2, 0, 0]);

    let source: Partition = serde_json::from_value(example.source.instance.clone())
        .expect("source example deserializes");
    let target: SequencingToMinimizeTardyTaskWeight =
        serde_json::from_value(example.target.instance.clone())
            .expect("target example deserializes");

    assert!(source
        .evaluate(&example.solutions[0].source_config)
        .is_valid());
    assert!(target
        .evaluate(&example.solutions[0].target_config)
        .is_valid());
}
