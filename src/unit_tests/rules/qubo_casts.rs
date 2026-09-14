use super::*;
use crate::rules::{ReduceTo, ReductionError, ReductionGraph, ReductionResult};
use crate::solvers::SolveOutcome;
use crate::types::MAX_EXACT_F64_INTEGER;

#[test]
fn test_qubo_i64_to_f64_closed_loop() {
    let source = QUBO::from_matrix(vec![vec![1_i64, -2], vec![0, 3]]).unwrap();
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source).unwrap();

    assert_eq!(
        reduction.target_problem().matrix(),
        QUBO::from_matrix(vec![vec![1.0, -2.0], vec![0.0, 3.0]])
            .unwrap()
            .matrix()
    );
    assert_eq!(
        reduction
            .recover_result(
                &source,
                SolveOutcome::optimal(reduction.target_problem(), vec![true, false].clone())
                    .unwrap()
            )
            .unwrap()
            .into_solution()
            .expect("qualifying target result must recover a source solution"),
        vec![true, false]
    );
}

#[test]
fn test_qubo_i64_to_f64_rejects_inexact_coefficient() {
    let source = QUBO::from_matrix(vec![vec![MAX_EXACT_F64_INTEGER + 1]]).unwrap();

    assert!(matches!(
        ReduceTo::<QUBO<f64>>::reduce_to(&source),
        Err(ReductionError::InexactFloatConversion { .. })
    ));
}

#[test]
fn test_qubo_numeric_variants_are_connected() {
    assert!(ReductionGraph::new().has_direct_reduction::<QUBO<i64>, QUBO<f64>>());
}
