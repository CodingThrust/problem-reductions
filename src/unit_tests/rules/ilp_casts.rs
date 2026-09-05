use super::*;
use crate::models::algebraic::{IntegerVariable, ObjectiveSense};
use crate::rules::ReductionGraph;
use crate::solvers::ILPSolver;
use crate::types::MAX_EXACT_F64_INTEGER;

#[test]
fn test_ilp_i64_coefficients_to_f64_closed_loop() {
    let source = ILP::<bool>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1), (1, 2)], 2)],
        vec![(0, 3), (1, -1)],
        ObjectiveSense::Maximize,
    )
    .unwrap();
    let reduction = ReduceTo::<ILP<bool, f64>>::reduce_to(&source).unwrap();

    assert_eq!(
        reduction.target_problem().constraints()[0].terms(),
        &[(0, 1.0), (1, 2.0)]
    );
    assert_eq!(
        reduction.target_problem().objective(),
        &[(0, 3.0), (1, -1.0)]
    );
    let target_solution = ILPSolver::new().solve(reduction.target_problem()).unwrap();
    assert_eq!(
        reduction.extract_solution(&target_solution).unwrap(),
        vec![1, 0]
    );
}

#[test]
fn test_ilp_i64_coefficients_to_f64_rejects_inexact_value() {
    let source = ILP::<bool>::new(
        1,
        vec![],
        vec![(0, MAX_EXACT_F64_INTEGER + 1)],
        ObjectiveSense::Minimize,
    )
    .unwrap();

    assert!(matches!(
        ReduceTo::<ILP<bool, f64>>::reduce_to(&source),
        Err(ReductionError::InexactFloatConversion { .. })
    ));
}

#[test]
fn test_ilp_cast_rechecks_source_feasibility() {
    let rhs = 1_000_000_000_000_i64;
    let source = ILP::<i64>::with_variables(
        vec![IntegerVariable::nonnegative()],
        vec![LinearConstraint::le(vec![(0, 1)], rhs)],
        vec![],
        ObjectiveSense::Minimize,
    )
    .unwrap();
    let reduction = ReduceTo::<ILP<i64, f64>>::reduce_to(&source).unwrap();
    let target_solution = vec![rhs + 1];

    assert!(reduction
        .target_problem()
        .is_feasible(&target_solution)
        .unwrap());
    assert!(reduction.extract_solution(&target_solution).is_err());
}

#[test]
fn test_ilp_coefficient_variants_are_connected() {
    let graph = ReductionGraph::new();
    assert!(graph.has_direct_reduction::<ILP<bool>, ILP<bool, f64>>());
    assert!(graph.has_direct_reduction::<ILP<i64>, ILP<i64, f64>>());
}
