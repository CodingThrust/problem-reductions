use super::*;
use crate::models::algebraic::ClosestVectorProblem;
use crate::solvers::{solver_capabilities, ExactProblemKey};
use crate::traits::Problem;
use std::collections::BTreeMap;

#[test]
fn test_cvp_solver_handles_integer_and_real_targets() {
    let integer = ClosestVectorProblem::new(vec![vec![1]], vec![12_i64]).unwrap();
    assert_eq!(solve(&integer).unwrap(), vec![12]);

    let real = ClosestVectorProblem::new(vec![vec![1]], vec![0.6]).unwrap();
    assert_eq!(solve(&real).unwrap(), vec![1]);
}

#[test]
fn test_cvp_solver_handles_nonorthogonal_rectangular_and_negative_coefficients() {
    let problem =
        ClosestVectorProblem::new(vec![vec![2, 0, 1], vec![1, 2, 0]], vec![-3_i64, -2, -1])
            .unwrap();
    assert_eq!(solve(&problem).unwrap(), vec![-1, -1]);
}

#[test]
fn test_cvp_solver_keeps_zero_on_tie_and_handles_empty_basis() {
    let tied = ClosestVectorProblem::new(vec![vec![1]], vec![0.5]).unwrap();
    assert_eq!(solve(&tied).unwrap(), vec![0]);

    let empty = ClosestVectorProblem::new(Vec::new(), vec![1_i64, 2]).unwrap();
    assert!(solve(&empty).unwrap().is_empty());
}

#[test]
fn test_cvp_solver_reports_inexact_integer_conversion() {
    let problem = ClosestVectorProblem::new(
        vec![vec![crate::types::MAX_EXACT_F64_INTEGER + 1]],
        vec![0_i64],
    )
    .unwrap();
    assert!(matches!(
        solve(&problem),
        Err(crate::solvers::SolveError::InexactFloatConversion(_))
    ));
}

#[test]
fn test_cvp_solver_is_registered_without_brute_force() {
    let key = ExactProblemKey::new(
        ClosestVectorProblem::<i64>::NAME,
        BTreeMap::from([("target".to_string(), "i64".to_string())]),
    );
    let capabilities = solver_capabilities(&key).unwrap();
    assert_eq!(
        capabilities.customized.unwrap().implementation,
        "cvp-sphere-enumeration"
    );
    assert!(!capabilities.brute_force);
}
