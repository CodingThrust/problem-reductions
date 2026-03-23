use super::*;
use crate::models::algebraic::LinearConstraint;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_ilp_solver_basic_maximize() {
    // Maximize x0 + 2*x1 subject to x0 + x1 <= 1, binary vars
    let ilp = ILP::<bool>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    );

    let solver = ILPSolver::new();
    let solution = solver.solve(&ilp);

    assert!(solution.is_some());
    let sol = solution.unwrap();

    // Solution should be valid
    let result = ilp.evaluate(&sol);
    assert!(result.is_valid(), "ILP solution should be valid");

    // Optimal: x1=1, x0=0 => objective = 2
    assert!((result.unwrap() - 2.0).abs() < 1e-9);
}

#[test]
fn test_ilp_solver_basic_minimize() {
    // Minimize x0 + x1 subject to x0 + x1 >= 1, binary vars
    let ilp = ILP::<bool>::new(
        2,
        vec![LinearConstraint::ge(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Minimize,
    );

    let solver = ILPSolver::new();
    let solution = solver.solve(&ilp);

    assert!(solution.is_some());
    let sol = solution.unwrap();

    // Solution should be valid
    let result = ilp.evaluate(&sol);
    assert!(result.is_valid(), "ILP solution should be valid");

    // Optimal: one variable = 1, other = 0 => objective = 1
    assert!((result.unwrap() - 1.0).abs() < 1e-9);
}

#[test]
fn test_ilp_solver_matches_brute_force() {
    // Maximize x0 + x1 + x2 subject to:
    //   x0 + x1 <= 1
    //   x1 + x2 <= 1
    let ilp = ILP::<bool>::new(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0),
            LinearConstraint::le(vec![(1, 1.0), (2, 1.0)], 1.0),
        ],
        vec![(0, 1.0), (1, 1.0), (2, 1.0)],
        ObjectiveSense::Maximize,
    );

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_all_witnesses(&ilp);
    let ilp_solution = ilp_solver.solve(&ilp).unwrap();

    // Both should find optimal value (2)
    let bf_size = ilp.evaluate(&bf_solutions[0]).unwrap();
    let ilp_size = ilp.evaluate(&ilp_solution).unwrap();
    assert!(
        (bf_size - ilp_size).abs() < 1e-9,
        "ILP should find optimal solution"
    );
}

#[test]
fn test_ilp_empty_problem() {
    let ilp = ILP::<bool>::empty();
    let solver = ILPSolver::new();
    let solution = solver.solve(&ilp);
    assert_eq!(solution, Some(vec![]));
}

#[test]
fn test_ilp_equality_constraint() {
    // Minimize x0 subject to x0 + x1 == 1, binary vars
    let ilp = ILP::<bool>::new(
        2,
        vec![LinearConstraint::eq(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0)],
        ObjectiveSense::Minimize,
    );

    let solver = ILPSolver::new();
    let solution = solver.solve(&ilp).unwrap();

    let result = ilp.evaluate(&solution);
    assert!(result.is_valid());
    // Optimal: x0=0, x1=1 => objective = 0
    assert!((result.unwrap() - 0.0).abs() < 1e-9);
}

#[test]
fn test_ilp_non_binary_bounds() {
    // Variables with larger ranges
    // x0 in [0, 3], x1 in [0, 2]
    // Maximize x0 + x1 subject to x0 + x1 <= 4
    // Use ILP::<i32> with explicit upper-bound constraints
    let ilp = ILP::<i32>::new(
        2,
        vec![
            LinearConstraint::le(vec![(0, 1.0)], 3.0),
            LinearConstraint::le(vec![(1, 1.0)], 2.0),
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 4.0),
        ],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );

    let solver = ILPSolver::new();
    let solution = solver.solve(&ilp).unwrap();

    let result = ilp.evaluate(&solution);
    assert!(result.is_valid());
    // Optimal: x0=2, x1=2 => 4 <= 4 valid, obj=4
    // or x0=3, x1=1 => 4 <= 4 valid, obj=4
    assert!((result.unwrap() - 4.0).abs() < 1e-9);
}

#[test]
fn test_ilp_integer_upper_bounds() {
    // Variables with upper bounds (non-negative integers)
    // x0 in [0, 4], x1 in [0, 2]
    // Maximize x0 + x1 (with explicit upper-bound constraints)
    let ilp = ILP::<i32>::new(
        2,
        vec![
            LinearConstraint::le(vec![(0, 1.0)], 4.0),
            LinearConstraint::le(vec![(1, 1.0)], 2.0),
        ],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );

    let solver = ILPSolver::new();
    let solution = solver.solve(&ilp).unwrap();

    let result = ilp.evaluate(&solution);
    assert!(result.is_valid());
    // Optimal: x0=4, x1=2 => objective = 6
    assert!((result.unwrap() - 6.0).abs() < 1e-9);
}

#[test]
fn test_ilp_config_to_values_roundtrip() {
    // Ensure the config encoding/decoding works correctly
    // x0 in [0, 5], x1 in [0, 3], maximize x0 + x1
    let ilp = ILP::<i32>::new(
        2,
        vec![
            LinearConstraint::le(vec![(0, 1.0)], 5.0),
            LinearConstraint::le(vec![(1, 1.0)], 3.0),
        ],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );

    let solver = ILPSolver::new();
    let solution = solver.solve(&ilp).unwrap();

    // The solution should be valid
    let result = ilp.evaluate(&solution);
    assert!(result.is_valid());
    // Optimal: x0=5, x1=3 => objective = 8
    assert!((result.unwrap() - 8.0).abs() < 1e-9);
}

#[test]
fn test_ilp_multiple_constraints() {
    // Maximize 2*x0 + 3*x1 + x2 subject to:
    //   x0 + x1 + x2 <= 2
    //   x0 + x1 >= 1
    // Binary vars
    let ilp = ILP::<bool>::new(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0), (2, 1.0)], 2.0),
            LinearConstraint::ge(vec![(0, 1.0), (1, 1.0)], 1.0),
        ],
        vec![(0, 2.0), (1, 3.0), (2, 1.0)],
        ObjectiveSense::Maximize,
    );

    let solver = ILPSolver::new();
    let solution = solver.solve(&ilp).unwrap();

    let result = ilp.evaluate(&solution);
    assert!(result.is_valid());

    // Check against brute force
    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_witnesses(&ilp);
    let bf_size = ilp.evaluate(&bf_solutions[0]).unwrap();

    assert!(
        (bf_size - result.unwrap()).abs() < 1e-9,
        "ILP should match brute force"
    );
}

#[test]
fn test_ilp_unconstrained() {
    // Maximize x0 + x1, no constraints, binary vars
    let ilp = ILP::<bool>::new(
        2,
        vec![],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );

    let solver = ILPSolver::new();
    let solution = solver.solve(&ilp).unwrap();

    let result = ilp.evaluate(&solution);
    assert!(result.is_valid());
    // Optimal: both = 1
    assert!((result.unwrap() - 2.0).abs() < 1e-9);
}

#[test]
fn test_ilp_with_time_limit() {
    let solver = ILPSolver::with_time_limit(10.0);
    assert_eq!(solver.time_limit, Some(10.0));

    // Should still work for simple problems
    let ilp = ILP::<bool>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );

    let solution = solver.solve(&ilp);
    assert!(solution.is_some());
}

#[test]
fn test_ilp_solve_via_reduction_success() {
    use crate::models::graph::MaximumIndependentSet;
    use crate::topology::SimpleGraph;
    use std::collections::BTreeMap;

    let solver = ILPSolver::new();
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    let variant = BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);
    let result = solver.try_solve_via_reduction("MaximumIndependentSet", &variant, &problem);
    assert!(result.is_ok());
    let sol = result.unwrap();
    let eval = problem.evaluate(&sol);
    assert!(eval.is_valid());
}

#[test]
fn test_ilp_solve_via_reduction_no_path() {
    use std::collections::BTreeMap;

    // Use a problem name that doesn't exist in the graph
    let solver = ILPSolver::new();
    let ilp = ILP::<bool>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0)],
        ObjectiveSense::Maximize,
    );
    // solve_via_reduction on an ILP itself should succeed directly
    let result = solver.try_solve_via_reduction(
        "ILP",
        &BTreeMap::from([("type".to_string(), "bool".to_string())]),
        &ilp,
    );
    assert!(result.is_ok());
}

#[test]
fn test_ilp_solve_dyn_bool() {
    let solver = ILPSolver::new();
    let ilp = ILP::<bool>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    );
    let result = solver.solve_dyn(&ilp as &dyn std::any::Any);
    assert!(result.is_some());
}

#[test]
fn test_ilp_solve_dyn_i32() {
    let solver = ILPSolver::new();
    let ilp = ILP::<i32>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0)], 3.0)],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );
    let result = solver.solve_dyn(&ilp as &dyn std::any::Any);
    assert!(result.is_some());
}

#[test]
fn test_ilp_solve_dyn_unknown_type_returns_none() {
    let solver = ILPSolver::new();
    let not_ilp: i32 = 42;
    let result = solver.solve_dyn(&not_ilp as &dyn std::any::Any);
    assert!(result.is_none());
}

#[test]
fn test_ilp_supports_direct_dyn() {
    let solver = ILPSolver::new();
    let ilp_bool = ILP::<bool>::empty();
    let ilp_i32 = ILP::<i32>::new(1, vec![], vec![], ObjectiveSense::Maximize);
    let not_ilp: i32 = 42;

    assert!(solver.supports_direct_dyn(&ilp_bool as &dyn std::any::Any));
    assert!(solver.supports_direct_dyn(&ilp_i32 as &dyn std::any::Any));
    assert!(!solver.supports_direct_dyn(&not_ilp as &dyn std::any::Any));
}

#[test]
fn test_solve_via_reduction_error_display() {
    use crate::solvers::ilp::SolveViaReductionError;

    let err = SolveViaReductionError::WitnessPathRequired {
        name: "Foo".to_string(),
    };
    assert!(err.to_string().contains("witness-capable"));
    assert!(err.to_string().contains("Foo"));

    let err = SolveViaReductionError::NoReductionPath {
        name: "Bar".to_string(),
    };
    assert!(err.to_string().contains("No reduction path"));
    assert!(err.to_string().contains("Bar"));

    let err = SolveViaReductionError::NoSolution {
        name: "Baz".to_string(),
    };
    assert!(err.to_string().contains("no solution"));
    assert!(err.to_string().contains("Baz"));

    // std::error::Error is implemented
    let _: &dyn std::error::Error = &err;
}

#[test]
fn test_solve_via_reduction_returns_none_for_no_path() {
    let solver = ILPSolver::new();
    let not_ilp: i32 = 42;
    let result = solver.solve_via_reduction(
        "NonexistentProblem",
        &std::collections::BTreeMap::new(),
        &not_ilp as &dyn std::any::Any,
    );
    assert!(result.is_none());
}
