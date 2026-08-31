use super::*;
use crate::rules::{ReduceTo, ReductionError, ReductionGraph, ReductionResult};
use crate::types::MAX_EXACT_F64_INTEGER;

#[test]
fn test_closestvectorproblem_i64_to_f64_closed_loop() {
    let source = ClosestVectorProblem::new(vec![vec![2, 0], vec![1, 2]], vec![3_i64, 2]).unwrap();
    let reduction = ReduceTo::<ClosestVectorProblem<f64>>::reduce_to(&source).unwrap();

    assert_eq!(reduction.target_problem().basis(), source.basis());
    assert_eq!(reduction.target_problem().target(), &[3.0, 2.0]);
    assert_eq!(reduction.extract_solution(&vec![1, 1]).unwrap(), vec![1, 1]);
}

#[test]
fn test_closestvectorproblem_i64_to_f64_rejects_inexact_target() {
    let source = ClosestVectorProblem::new(vec![vec![1]], vec![MAX_EXACT_F64_INTEGER + 1]).unwrap();

    assert!(matches!(
        ReduceTo::<ClosestVectorProblem<f64>>::reduce_to(&source),
        Err(ReductionError::InexactFloatConversion { .. })
    ));
}

#[test]
fn test_closestvectorproblem_numeric_variants_are_connected() {
    assert!(ReductionGraph::new()
        .has_direct_reduction::<ClosestVectorProblem<i64>, ClosestVectorProblem<f64>>());
}
