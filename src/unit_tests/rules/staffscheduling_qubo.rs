use crate::models::{algebraic::QUBO, misc::StaffScheduling, Decision};
use crate::rules::direct_qubo_tests::{direct, feasible};
use crate::rules::registry::reduction_entries;
use crate::rules::ReductionError;
use crate::Problem;

#[test]
fn test_staffscheduling_to_qubo_closed_loop() {
    // A single pattern must receive two workers, not a Boolean selection.
    feasible(
        StaffScheduling::new(1, vec![vec![true]], vec![2], 2),
        true,
        4,
    );
    feasible(
        StaffScheduling::new(1, vec![vec![true], vec![true]], vec![2], 2),
        true,
        6,
    );
    // Three pair-covering patterns cannot cover six required shifts with two workers.
    feasible(
        StaffScheduling::new(
            2,
            vec![
                vec![true, true, false],
                vec![false, true, true],
                vec![true, false, true],
            ],
            vec![2, 2, 2],
            2,
        ),
        false,
        8,
    );
    // Coverage may exceed the requirement; the worker budget remains an inequality.
    feasible(
        StaffScheduling::new(1, vec![vec![true]], vec![1], 3),
        true,
        6,
    );
}

#[test]
fn test_staffscheduling_empty_signed_and_impossible_requirements() {
    feasible(
        StaffScheduling::new(0, vec![vec![], vec![]], vec![], 0),
        true,
        0,
    );
    feasible(StaffScheduling::new(9, vec![], vec![], 2), true, 2);
    feasible(StaffScheduling::new(1, vec![], vec![1], 2), false, 3);
    feasible(
        StaffScheduling::new(1, vec![vec![true]], vec![3], 2),
        false,
        0,
    );
    feasible(
        StaffScheduling::new(0, vec![vec![false]], vec![i64::MIN], 1),
        true,
        2,
    );
    feasible(
        StaffScheduling::new(1, vec![vec![true]], vec![0], 0),
        true,
        0,
    );
}

#[test]
fn test_staffscheduling_completed_decision_recovery() {
    let source = StaffScheduling::new(1, vec![vec![true]], vec![2], 2);
    let reduction = direct::<_, Decision<QUBO<i64>>>(&source);
    let recovered = reduction
        .extract_solution_dyn(&vec![true, true, false, false])
        .unwrap();
    assert_eq!(recovered.downcast_ref::<Vec<usize>>().unwrap(), &[2]);
    let entry = reduction_entries()
        .into_iter()
        .find(|entry| {
            entry.source_name == StaffScheduling::NAME && entry.target_name == "DecisionQUBO"
        })
        .unwrap();
    let aggregate = entry.reduce_aggregate_fn.unwrap()(&source).unwrap();
    for value in [false, true] {
        assert_eq!(
            aggregate
                .extract_value_dyn(serde_json::json!(value))
                .unwrap(),
            serde_json::json!(value)
        );
    }
}

#[test]
fn test_staffscheduling_rejects_unrepresentable_source_and_target_arithmetic() {
    let entry = reduction_entries()
        .into_iter()
        .find(|entry| {
            entry.source_name == StaffScheduling::NAME && entry.target_name == "DecisionQUBO"
        })
        .expect("staffing must have a direct decision QUBO route");
    for source in [
        StaffScheduling::new(0, vec![vec![], vec![]], vec![], i64::MAX),
        StaffScheduling::new(1, vec![vec![true]], vec![1], 1_i64 << 32),
    ] {
        assert!(matches!(
            (entry.reduce_fn.unwrap())(&source),
            Err(ReductionError::IntegerOverflow { .. })
        ));
    }
}
