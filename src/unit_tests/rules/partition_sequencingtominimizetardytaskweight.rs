#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use crate::models::decision::Decision;
use crate::models::misc::{Partition, SequencingToMinimizeTardyTaskWeight};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::traits::ReductionResult;
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
use crate::solvers::SolveOutcome;
use crate::traits::EvaluationError::InvalidConfiguration;
use crate::traits::Problem;
use crate::types::OptimizationValue;

#[test]
fn test_partition_to_sequencing_to_minimize_tardy_task_weight_closed_loop() {
    let source = Partition::new(vec![3, 1, 1, 2, 2, 1]).unwrap();
    let reduction = ReduceTo::<Decision<SequencingToMinimizeTardyTaskWeight>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> SequencingToMinimizeTardyTaskWeight closed loop",
    );
}

#[test]
fn test_partition_to_sequencing_to_minimize_tardy_task_weight_structure() {
    let source = Partition::new(vec![3, 1, 1, 2, 2, 1]).unwrap();
    let reduction = ReduceTo::<Decision<SequencingToMinimizeTardyTaskWeight>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.inner().lengths(), &[3, 1, 1, 2, 2, 1]);
    assert_eq!(target.inner().weights(), &[3, 1, 1, 2, 2, 1]);
    assert_eq!(target.inner().deadlines(), &[5, 5, 5, 5, 5, 5]);
    assert_eq!(target.inner().num_tasks(), source.num_elements());
}

#[test]
fn test_partition_to_sequencing_to_minimize_tardy_task_weight_extract_solution() {
    let source = Partition::new(vec![3, 1, 1, 2, 2, 1]).unwrap();
    let reduction = ReduceTo::<Decision<SequencingToMinimizeTardyTaskWeight>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_eq!(
        reduction
            .recover_result(
                &source,
                SolveOutcome::optimal(reduction.target_problem(), vec![1, 2, 4, 5, 0, 3].clone())
                    .unwrap()
            )
            .unwrap()
            .into_solution()
            .expect("qualifying target result must recover a source solution"),
        vec![true, false, false, true, false, false]
    );
}

#[test]
fn test_partition_to_sequencing_to_minimize_tardy_task_weight_odd_total_is_unsatisfying() {
    let source = Partition::new(vec![2, 4, 5]).unwrap();
    let reduction = ReduceTo::<Decision<SequencingToMinimizeTardyTaskWeight>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    assert!(BruteForce::new().solve(target).unwrap().is_none());
    let best = BruteForce::new()
        .solve(target.inner())
        .unwrap()
        .expect("target should always have an optimal schedule");

    assert_eq!(
        target.inner().evaluate(&best).unwrap(),
        crate::types::Min(Some(6))
    );
    assert!(!target.evaluate(&best).unwrap().0);
    assert!(!ReductionResult::target_problem(&reduction)
        .evaluate(&best)
        .unwrap()
        .is_valid());
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
        "DecisionSequencingToMinimizeTardyTaskWeight"
    );
    assert_eq!(
        example.target.instance["inner"]["lengths"],
        serde_json::json!([3, 1, 1, 2, 2, 1])
    );
    assert_eq!(
        example.target.instance["inner"]["weights"],
        serde_json::json!([3, 1, 1, 2, 2, 1])
    );
    assert_eq!(
        example.target.instance["inner"]["deadlines"],
        serde_json::json!([5, 5, 5, 5, 5, 5])
    );
    assert_eq!(example.solutions.len(), 1);
    assert_eq!(
        example.solutions[0].source_config,
        serde_json::json!([true, false, false, true, false, false])
    );
    assert_eq!(
        example.solutions[0].target_config,
        serde_json::json!([1, 2, 4, 5, 0, 3])
    );

    let source: Partition = serde_json::from_value(example.source.instance.clone())
        .expect("source example deserializes");
    let target: Decision<SequencingToMinimizeTardyTaskWeight> =
        serde_json::from_value(example.target.instance.clone())
            .expect("target example deserializes");

    let source_config: Vec<bool> =
        serde_json::from_value(example.solutions[0].source_config.clone()).unwrap();
    let target_config: Vec<usize> =
        serde_json::from_value(example.solutions[0].target_config.clone()).unwrap();
    assert!(source.evaluate(&source_config).unwrap().is_valid());
    assert!(target.evaluate(&target_config).unwrap().is_valid());
}

#[test]
fn test_partition_to_tardy_weight_all_small_configurations() {
    for n in 1u32..=4 {
        for mut encoded in 0..3usize.pow(n) {
            let sizes: Vec<i64> = (0..n)
                .map(|_| {
                    let size = (encoded % 3 + 1) as i64;
                    encoded /= 3;
                    size
                })
                .collect();
            let source = Partition::new(sizes).unwrap();
            let reduction =
                ReduceTo::<Decision<SequencingToMinimizeTardyTaskWeight>>::reduce_to(&source)
                    .unwrap();
            let target = crate::rules::ReductionResult::target_problem(&reduction);
            let source_feasible = (0..1usize << n).any(|mask| {
                let bits = (0..n).map(|i| mask & (1 << i) != 0).collect();
                source.evaluate(&bits).unwrap().0
            });
            let mut optimum = i64::MAX;
            for mut code in 0..(n as usize).pow(n) {
                let schedule = (0..n)
                    .map(|_| {
                        let task = code % n as usize;
                        code /= n as usize;
                        task
                    })
                    .collect();
                let value = target.inner().evaluate(&schedule).unwrap();
                if let Some(weight) = value.0 {
                    optimum = optimum.min(weight);
                }
                let certified = crate::types::Or(OptimizationValue::meets_bound(
                    &(value),
                    crate::rules::ReductionResult::target_problem(&reduction).bound(),
                ))
                .0;
                if certified {
                    let bits = reduction
                        .recover_result(
                            &source,
                            SolveOutcome::optimal(reduction.target_problem(), schedule.clone())
                                .unwrap(),
                        )
                        .unwrap()
                        .into_solution()
                        .expect("qualifying target result must recover a source solution");
                    assert!(source.evaluate(&bits).unwrap().0);
                }
            }
            assert_eq!(
                crate::types::Or(OptimizationValue::meets_bound(
                    &(crate::types::Min(Some(optimum))),
                    crate::rules::ReductionResult::target_problem(&reduction).bound()
                ))
                .0,
                source_feasible
            );
            assert!(matches!(
                ReductionResult::target_problem(&reduction)
                    .inner()
                    .evaluate(&vec![]),
                Err(InvalidConfiguration(_))
            ));
            assert!(matches!(
                ReductionResult::target_problem(&reduction)
                    .inner()
                    .evaluate(&vec![n as usize; n as usize]),
                Err(InvalidConfiguration(_))
            ));
            assert!(
                !crate::types::Or(OptimizationValue::meets_bound(
                    &(crate::types::Min(None)),
                    crate::rules::ReductionResult::target_problem(&reduction).bound()
                ))
                .0
            );
        }
    }
}

#[test]
fn test_partition_to_tardy_weight_full_i64_domain() {
    let half = i64::MAX / 2;
    for (sizes, schedule, expected, balanced) in [
        (vec![i64::MAX], vec![0], i64::MAX, false),
        (vec![i64::MAX - 1, 1], vec![1, 0], i64::MAX - 1, false),
        (vec![half, half], vec![0, 1], half, true),
        (vec![half - 1, half - 1, 1, 1], vec![0, 2, 1, 3], half, true),
    ] {
        let source = Partition::new(sizes).unwrap();
        let reduction =
            ReduceTo::<Decision<SequencingToMinimizeTardyTaskWeight>>::reduce_to(&source).unwrap();
        let value = reduction
            .target_problem()
            .inner()
            .evaluate(&schedule)
            .unwrap();
        assert_eq!(value, crate::types::Min(Some(expected)));
        assert_eq!(
            crate::types::Or(OptimizationValue::meets_bound(
                &(value),
                crate::rules::ReductionResult::target_problem(&reduction).bound()
            ))
            .0,
            balanced
        );
        if balanced {
            let bits = reduction
                .recover_result(
                    &source,
                    SolveOutcome::optimal(reduction.target_problem(), schedule.clone()).unwrap(),
                )
                .unwrap()
                .into_solution()
                .expect("qualifying target result must recover a source solution");
            assert!(source.evaluate(&bits).unwrap().0);
        }
    }
}
