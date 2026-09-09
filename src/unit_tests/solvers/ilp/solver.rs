use super::*;
use crate::models::algebraic::{IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::traits::Problem;

fn binary_ilp(
    num_vars: usize,
    constraints: Vec<LinearConstraint>,
    objective: Vec<(usize, i64)>,
    sense: ObjectiveSense,
) -> ILP<bool> {
    ILP::new(num_vars, constraints, objective, sense).unwrap()
}

#[test]
fn test_ilp_solver_basic_maximize() {
    let ilp = binary_ilp(
        2,
        vec![LinearConstraint::le(vec![(0, 1), (1, 1)], 1)],
        vec![(0, 1), (1, 2)],
        ObjectiveSense::Maximize,
    );
    let solution = ILPSolver::new().solve(&ilp).unwrap();
    assert_eq!(solution, vec![0, 1]);
    assert_eq!(ilp.evaluate_objective(&solution).unwrap(), 2);
}

#[test]
fn test_ilp_solver_basic_minimize() {
    let ilp = binary_ilp(
        2,
        vec![LinearConstraint::ge(vec![(0, 1), (1, 1)], 1)],
        vec![(0, 1), (1, 1)],
        ObjectiveSense::Minimize,
    );
    let solution = ILPSolver::new().solve(&ilp).unwrap();
    assert_eq!(ilp.evaluate_objective(&solution).unwrap(), 1);
}

#[test]
fn test_ilp_solver_matches_brute_force() {
    let ilp = binary_ilp(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1), (1, 1)], 1),
            LinearConstraint::le(vec![(1, 1), (2, 1)], 1),
        ],
        vec![(0, 1), (1, 1), (2, 1)],
        ObjectiveSense::Maximize,
    );
    let solution = ILPSolver::new().solve(&ilp).unwrap();
    assert_eq!(ilp.evaluate_objective(&solution).unwrap(), 2);
}

#[test]
fn test_ilp_empty_problem() {
    assert_eq!(ILPSolver::new().solve(&ILP::<bool>::empty()), Ok(vec![]));
}

#[test]
fn test_ilp_empty_problem_with_infeasible_constraint_returns_infeasible() {
    let ilp = binary_ilp(
        0,
        vec![LinearConstraint::le(vec![], -1)],
        vec![],
        ObjectiveSense::Minimize,
    );
    assert_eq!(ILPSolver::new().solve(&ilp), Err(ILPSolveError::Infeasible));
}

#[test]
fn test_ilp_solver_disambiguates_unbounded_model() {
    let ilp = ILP::<i64>::with_variables(
        vec![IntegerVariable::free()],
        vec![LinearConstraint::ge(vec![(0, 1)], 0)],
        vec![(0, 1)],
        ObjectiveSense::Maximize,
    )
    .unwrap();

    assert_eq!(ILPSolver::new().solve(&ilp), Err(ILPSolveError::Unbounded));
}

#[test]
fn test_ilp_solver_disambiguates_infeasible_model() {
    let ilp = ILP::<i64>::with_variables(
        vec![IntegerVariable::free()],
        vec![
            LinearConstraint::ge(vec![(0, 1)], 1),
            LinearConstraint::le(vec![(0, 1)], 0),
        ],
        vec![(0, 1)],
        ObjectiveSense::Maximize,
    )
    .unwrap();

    assert_eq!(ILPSolver::new().solve(&ilp), Err(ILPSolveError::Infeasible));
}

#[test]
fn test_ilp_solver_rejects_inexact_integer_transport() {
    let value = crate::types::MAX_EXACT_F64_INTEGER + 1;
    let ilp = ILP::<i64>::with_variables(
        vec![IntegerVariable::new(Some(value), Some(value)).unwrap()],
        vec![],
        vec![],
        ObjectiveSense::Minimize,
    )
    .unwrap();

    assert!(matches!(
        ILPSolver::new().solve(&ilp),
        Err(ILPSolveError::InexactTransport(_))
    ));
}

#[test]
fn test_ilp_rejects_solution_that_is_infeasible_after_rounding() {
    let ilp = binary_ilp(
        1,
        vec![LinearConstraint::le(vec![(0, 1)], 0)],
        vec![(0, 1)],
        ObjectiveSense::Maximize,
    );
    let solution = ILPSolver::new().solve(&ilp).unwrap();
    assert_eq!(solution, vec![0]);
    assert!(ilp.is_feasible(&solution).unwrap());
}

#[test]
fn test_ilp_equality_constraint() {
    let ilp = binary_ilp(
        2,
        vec![LinearConstraint::eq(vec![(0, 1), (1, 1)], 1)],
        vec![(0, 1)],
        ObjectiveSense::Minimize,
    );
    assert_eq!(ILPSolver::new().solve(&ilp).unwrap(), vec![0, 1]);
}

fn bounded_integer_ilp(upper_bounds: &[i64]) -> ILP<i64> {
    let variables = upper_bounds
        .iter()
        .map(|&upper| IntegerVariable::new(Some(0), Some(upper)).unwrap())
        .collect();
    ILP::with_variables(
        variables,
        vec![],
        (0..upper_bounds.len()).map(|index| (index, 1)).collect(),
        ObjectiveSense::Maximize,
    )
    .unwrap()
}

#[test]
fn test_ilp_non_binary_bounds() {
    let ilp = ILP::<i64>::with_variables(
        vec![
            IntegerVariable::new(Some(0), Some(3)).unwrap(),
            IntegerVariable::new(Some(0), Some(2)).unwrap(),
        ],
        vec![LinearConstraint::le(vec![(0, 1), (1, 1)], 4)],
        vec![(0, 1), (1, 1)],
        ObjectiveSense::Maximize,
    )
    .unwrap();
    let solution = ILPSolver::new().solve(&ilp).unwrap();
    assert_eq!(ilp.evaluate_objective(&solution).unwrap(), 4);
}

#[test]
fn test_ilp_integer_upper_bounds() {
    let ilp = bounded_integer_ilp(&[4, 2]);
    assert_eq!(ILPSolver::new().solve(&ilp).unwrap(), vec![4, 2]);
}

#[test]
fn test_ilp_config_to_values_roundtrip() {
    let ilp = bounded_integer_ilp(&[5, 3]);
    let solution = ILPSolver::new().solve(&ilp).unwrap();
    assert_eq!(solution, vec![5, 3]);
    assert!(ilp.is_feasible(&solution).unwrap());
}

#[test]
fn test_ilp_multiple_constraints() {
    let ilp = binary_ilp(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1), (1, 1), (2, 1)], 2),
            LinearConstraint::ge(vec![(0, 1), (1, 1)], 1),
        ],
        vec![(0, 2), (1, 3), (2, 1)],
        ObjectiveSense::Maximize,
    );
    let solution = ILPSolver::new().solve(&ilp).unwrap();
    assert_eq!(ilp.evaluate_objective(&solution).unwrap(), 5);
}

#[test]
fn test_ilp_unconstrained() {
    let ilp = binary_ilp(2, vec![], vec![(0, 1), (1, 1)], ObjectiveSense::Maximize);
    assert_eq!(ILPSolver::new().solve(&ilp).unwrap(), vec![1, 1]);
}

#[test]
fn test_ilp_with_time_limit() {
    let solver = ILPSolver::with_time_limit(10.0);
    assert_eq!(solver.time_limit, Some(10.0));
    let ilp = binary_ilp(1, vec![], vec![(0, 1)], ObjectiveSense::Maximize);
    assert!(solver.solve(&ilp).is_ok());
}

#[test]
fn test_registered_ilp_pipeline_success() {
    use crate::models::graph::MaximumIndependentSet;
    use crate::registry::load_dyn;
    use crate::solvers::{solve, SolveOutcome, SolverExecution, SolverRequest};
    use crate::topology::SimpleGraph;
    use std::collections::BTreeMap;

    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1_i64; 3]);
    let variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i64".to_string()),
    ]);
    let loaded = load_dyn(
        "MaximumIndependentSet",
        &variant,
        serde_json::to_value(&problem).unwrap(),
    )
    .unwrap();
    let result = solve(&loaded, SolverRequest::Ilp).unwrap();
    assert!(matches!(result.solver, SolverExecution::Ilp { .. }));
    let SolveOutcome::Optimal { solution, .. } = result.outcome else {
        panic!("registered ILP pipeline should return an optimal witness");
    };
    let solution: Vec<bool> = serde_json::from_value(solution).unwrap();
    assert!(problem.evaluate(&solution).unwrap().is_valid());
}

// Test acceptance policy in source-objective units, separate from variable rounding.
// This allows small absolute numerical differences near zero; it is not a
// guaranteed objective-error bound derived from HiGHS feasibility tolerances.
fn objective_close(a: f64, b: f64) -> bool {
    let abs_tol = 1e-7;
    let rel_tol = 1e-7;
    (a - b).abs() <= abs_tol + rel_tol * a.abs().max(b.abs())
}

#[test]
fn test_float_qubo_objective_matches_reference_within_tolerance() {
    use crate::models::algebraic::QUBO;
    use crate::solvers::BruteForce;

    for scale in [1e-9, 1.0] {
        let matrix = vec![
            vec![1.0, -5.0, 3.0, -8.0],
            vec![0.0, -7.0, 8.0, -6.0],
            vec![0.0, 0.0, 2.0, 9.0],
            vec![0.0, 0.0, 0.0, -8.0],
        ]
        .into_iter()
        .map(|row| row.into_iter().map(|v| v * scale).collect())
        .collect();
        let source = QUBO::<f64>::from_matrix(matrix).unwrap();
        let actual = ILPSolver::new().solve(&source).unwrap();
        let reference = BruteForce::new().solve(&source).unwrap().unwrap();
        let actual_value = source.evaluate(&actual).unwrap();
        let reference_value = source.evaluate(&reference).unwrap();
        assert!(actual_value.is_valid());
        assert!(objective_close(
            actual_value.0.unwrap(),
            reference_value.0.unwrap()
        ));
    }
}

#[test]
fn test_ilp_solver_rejects_objective_overflow_after_backend_success() {
    let ilp = ILP::<i64>::with_variables(
        vec![IntegerVariable::new(Some(1025), Some(1025)).unwrap()],
        vec![],
        vec![(0, crate::types::MAX_EXACT_F64_INTEGER)],
        ObjectiveSense::Maximize,
    )
    .unwrap();
    assert!(matches!(
        ILPSolver::new().solve(&ilp),
        Err(ILPSolveError::InvalidSolution(_))
    ));
}

#[test]
fn test_invalid_public_time_limit_returns_existing_backend_error() {
    assert!(matches!(
        ILPSolver::with_time_limit(-1.0).solve(&ILP::<bool>::empty()),
        Err(ILPSolveError::BackendFailure(_))
    ));
}
