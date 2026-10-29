use super::direct_qubo_tests::{direct, feasible, optimum};
use crate::models::{algebraic::QUBO, misc::*};
use crate::BruteForce;

#[test]
fn test_deferred_qubo_rules_retain_ilp_routes() {
    let entries = crate::rules::registry::reduction_entries();
    for source in [
        "SequencingToMinimizeWeightedTardiness",
        "FeasibleRegisterAssignment",
        "Betweenness",
        "ShortestCommonSupersequence",
    ] {
        assert!(entries
            .iter()
            .any(|e| e.source_name == source && e.target_name == "ILP"));
        assert!(
            !entries.iter().any(
                |e| e.source_name == source && matches!(e.target_name, "QUBO" | "DecisionQUBO")
            ),
            "{source} must use its existing ILP route"
        );
    }
}

#[test]
fn test_arbitrary_length_tardiness_to_qubo_closed_loop() {
    optimum(
        MinimumTardinessSequencing::with_lengths(vec![1, 1, 2], vec![1, 2, 3], vec![]),
        1,
        11,
    );
    optimum(
        MinimumTardinessSequencing::with_lengths(vec![2, 1], vec![-1, 3], vec![(0, 1)]),
        1,
        5,
    );
    optimum(
        MinimumTardinessSequencing::with_lengths(vec![], vec![], vec![]),
        0,
        0,
    );
    let source =
        MinimumTardinessSequencing::with_lengths(vec![1, 1], vec![2, 2], vec![(0, 1), (1, 0)]);
    assert!(BruteForce::new().solve(&source).unwrap().is_none());
    let r = direct::<_, QUBO<i64>>(&source);
    let q = r.target_problem_any().downcast_ref::<QUBO<i64>>().unwrap();
    for bits in BruteForce::new().find_all_witnesses(q).unwrap() {
        assert!(r.extract_solution_dyn(&bits).is_err());
    }
}

#[test]
fn test_release_deadline_to_decision_qubo_closed_loop() {
    feasible(
        SequencingWithReleaseTimesAndDeadlines::new(vec![1, 2, 1], vec![0, 0, 2], vec![3, 3, 4]),
        true,
        7,
    );
    feasible(
        SequencingWithReleaseTimesAndDeadlines::new(vec![1; 3], vec![0; 3], vec![1; 3]),
        false,
        3,
    );
    // A zero-duration task at a positive task's start must execute first.
    feasible(
        SequencingWithReleaseTimesAndDeadlines::new(vec![1, 0], vec![0, 0], vec![1, 0]),
        true,
        2,
    );
    feasible(
        SequencingWithReleaseTimesAndDeadlines::new(vec![0], vec![2], vec![1]),
        false,
        0,
    );
    feasible(
        SequencingWithReleaseTimesAndDeadlines::new(vec![], vec![], vec![]),
        true,
        0,
    );
}

#[test]
fn test_dynamic_storage_to_decision_qubo_closed_loop() {
    feasible(
        DynamicStorageAllocation::new(vec![(0, 1, 1), (1, 2, 1), (0, 2, 1)], 2),
        true,
        6,
    );
    feasible(
        DynamicStorageAllocation::new(vec![(0, 1, 1); 3], 2),
        false,
        6,
    );
    feasible(
        DynamicStorageAllocation::new(vec![(usize::MAX - 1, usize::MAX, 1)], 1),
        true,
        1,
    );
}

#[test]
fn test_start_encodings_check_source_arithmetic_and_extreme_windows() {
    use crate::models::decision::Decision;
    use crate::rules::{ReduceTo, ReductionError};
    let elapsed = MinimumTardinessSequencing::with_lengths(vec![i64::MAX, 1], vec![0, 0], vec![]);
    assert!(matches!(
        ReduceTo::<QUBO<i64>>::reduce_to(&elapsed),
        Err(ReductionError::IntegerOverflow { .. })
    ));
    // A huge numeric duration can still have only one admissible start.
    optimum(
        MinimumTardinessSequencing::with_lengths(vec![i64::MAX], vec![i64::MAX], vec![]),
        0,
        1,
    );
    feasible(
        SequencingWithReleaseTimesAndDeadlines::new(vec![0], vec![i64::MAX], vec![i64::MAX]),
        true,
        1,
    );
    feasible(
        DynamicStorageAllocation::new(vec![(0, 1, usize::MAX)], usize::MAX),
        true,
        1,
    );
    let count = DynamicStorageAllocation::new(vec![(0, 1, 1); 2], usize::MAX);
    assert!(matches!(
        ReduceTo::<Decision<QUBO<i64>>>::reduce_to(&count),
        Err(ReductionError::IntegerOverflow { .. })
    ));
}

#[test]
fn test_conditional_encodings_preserve_completed_no_values() {
    use crate::rules::{AggregateReductionResult, ReduceTo, ReductionResult};
    use crate::types::Min;
    use crate::Problem;
    let source = MinimumTardinessSequencing::with_lengths(vec![1], vec![1], vec![(0, 0)]);
    let r = ReduceTo::<QUBO<i64>>::reduce_to(&source).unwrap();
    let target = ReductionResult::target_problem(&r);
    let optimum = BruteForce::new().solve(target).unwrap().unwrap();
    assert_eq!(
        r.extract_value(target.evaluate(&optimum).unwrap()),
        Min(None)
    );
}

#[test]
fn test_indexed_encodings_reject_unrepresentable_allocation_sizes() {
    use crate::models::decision::Decision;
    use crate::rules::{ReduceTo, ReductionError};
    let schedule = SequencingWithReleaseTimesAndDeadlines::new(vec![0], vec![0], vec![i64::MAX]);
    assert!(matches!(
        ReduceTo::<Decision<QUBO<i64>>>::reduce_to(&schedule),
        Err(ReductionError::IntegerOverflow { .. })
    ));
    let storage = DynamicStorageAllocation::new(vec![(0, 1, 1)], usize::MAX);
    assert!(matches!(
        ReduceTo::<Decision<QUBO<i64>>>::reduce_to(&storage),
        Err(ReductionError::IntegerOverflow { .. })
    ));
}

#[test]
fn test_release_encoding_rejects_overflowing_source_finish_times() {
    use crate::models::decision::Decision;
    use crate::rules::{ReduceTo, ReductionError};
    for source in [
        SequencingWithReleaseTimesAndDeadlines::new(vec![i64::MAX], vec![i64::MAX], vec![0]),
        SequencingWithReleaseTimesAndDeadlines::new(
            vec![i64::MAX, 1],
            vec![0, 0],
            vec![i64::MAX; 2],
        ),
    ] {
        assert!(matches!(
            ReduceTo::<Decision<QUBO<i64>>>::reduce_to(&source),
            Err(ReductionError::IntegerOverflow { .. })
        ));
    }
}
