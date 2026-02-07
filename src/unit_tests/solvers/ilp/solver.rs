use super::*;
use crate::models::optimization::{LinearConstraint, VarBounds};
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_ilp_solver_basic_maximize() {
    // Maximize x0 + 2*x1 subject to x0 + x1 <= 1, binary vars
    let ilp = ILP::binary(
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
    let result = ilp.solution_size(&sol);
    assert!(result.is_valid, "ILP solution should be valid");

    // Optimal: x1=1, x0=0 => objective = 2
    assert!((result.size - 2.0).abs() < 1e-9);
}

#[test]
fn test_ilp_solver_basic_minimize() {
    // Minimize x0 + x1 subject to x0 + x1 >= 1, binary vars
    let ilp = ILP::binary(
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
    let result = ilp.solution_size(&sol);
    assert!(result.is_valid, "ILP solution should be valid");

    // Optimal: one variable = 1, other = 0 => objective = 1
    assert!((result.size - 1.0).abs() < 1e-9);
}

#[test]
fn test_ilp_solver_matches_brute_force() {
    // Maximize x0 + x1 + x2 subject to:
    //   x0 + x1 <= 1
    //   x1 + x2 <= 1
    let ilp = ILP::binary(
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

    let bf_solutions = bf.find_best(&ilp);
    let ilp_solution = ilp_solver.solve(&ilp).unwrap();

    // Both should find optimal value (2)
    let bf_size = ilp.solution_size(&bf_solutions[0]).size;
    let ilp_size = ilp.solution_size(&ilp_solution).size;
    assert!(
        (bf_size - ilp_size).abs() < 1e-9,
        "ILP should find optimal solution"
    );
}

#[test]
fn test_ilp_empty_problem() {
    let ilp = ILP::empty();
    let solver = ILPSolver::new();
    let solution = solver.solve(&ilp);
    assert_eq!(solution, Some(vec![]));
}

#[test]
fn test_ilp_equality_constraint() {
    // Minimize x0 subject to x0 + x1 == 1, binary vars
    let ilp = ILP::binary(
        2,
        vec![LinearConstraint::eq(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0)],
        ObjectiveSense::Minimize,
    );

    let solver = ILPSolver::new();
    let solution = solver.solve(&ilp).unwrap();

    let result = ilp.solution_size(&solution);
    assert!(result.is_valid);
    // Optimal: x0=0, x1=1 => objective = 0
    assert!((result.size - 0.0).abs() < 1e-9);
}

#[test]
fn test_ilp_non_binary_bounds() {
    // Variables with larger ranges
    // x0 in [0, 3], x1 in [0, 2]
    // Maximize x0 + x1 subject to x0 + x1 <= 4
    let ilp = ILP::new(
        2,
        vec![VarBounds::bounded(0, 3), VarBounds::bounded(0, 2)],
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 4.0)],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );

    let solver = ILPSolver::new();
    let solution = solver.solve(&ilp).unwrap();

    let result = ilp.solution_size(&solution);
    assert!(result.is_valid);
    // Optimal: x0=3, x1=2 => objective = 5 (3 + 2 = 5 <= 4 is false!)
    // Wait, 3+2=5 > 4, so constraint is violated. Let's check actual optimal:
    // x0=2, x1=2 => 4 <= 4 valid, obj=4
    // x0=3, x1=1 => 4 <= 4 valid, obj=4
    assert!((result.size - 4.0).abs() < 1e-9);
}

#[test]
fn test_ilp_negative_lower_bounds() {
    // Variables with negative lower bounds
    // x0 in [-2, 2], x1 in [-1, 1]
    // Maximize x0 + x1 (no constraints)
    let ilp = ILP::new(
        2,
        vec![VarBounds::bounded(-2, 2), VarBounds::bounded(-1, 1)],
        vec![],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );

    let solver = ILPSolver::new();
    let solution = solver.solve(&ilp).unwrap();

    let result = ilp.solution_size(&solution);
    assert!(result.is_valid);
    // Optimal: x0=2, x1=1 => objective = 3
    assert!((result.size - 3.0).abs() < 1e-9);
}

#[test]
fn test_ilp_config_to_values_roundtrip() {
    // Ensure the config encoding/decoding works correctly
    let ilp = ILP::new(
        2,
        vec![VarBounds::bounded(-2, 2), VarBounds::bounded(1, 3)],
        vec![],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );

    let solver = ILPSolver::new();
    let solution = solver.solve(&ilp).unwrap();

    // The solution should be valid
    let result = ilp.solution_size(&solution);
    assert!(result.is_valid);
    // Optimal: x0=2, x1=3 => objective = 5
    assert!((result.size - 5.0).abs() < 1e-9);
}

#[test]
fn test_ilp_multiple_constraints() {
    // Maximize 2*x0 + 3*x1 + x2 subject to:
    //   x0 + x1 + x2 <= 2
    //   x0 + x1 >= 1
    // Binary vars
    let ilp = ILP::binary(
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

    let result = ilp.solution_size(&solution);
    assert!(result.is_valid);

    // Check against brute force
    let bf = BruteForce::new();
    let bf_solutions = bf.find_best(&ilp);
    let bf_size = ilp.solution_size(&bf_solutions[0]).size;

    assert!(
        (bf_size - result.size).abs() < 1e-9,
        "ILP should match brute force"
    );
}

#[test]
fn test_ilp_unconstrained() {
    // Maximize x0 + x1, no constraints, binary vars
    let ilp = ILP::binary(
        2,
        vec![],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );

    let solver = ILPSolver::new();
    let solution = solver.solve(&ilp).unwrap();

    let result = ilp.solution_size(&solution);
    assert!(result.is_valid);
    // Optimal: both = 1
    assert!((result.size - 2.0).abs() < 1e-9);
}

#[test]
fn test_ilp_with_time_limit() {
    let solver = ILPSolver::with_time_limit(10.0);
    assert_eq!(solver.time_limit, Some(10.0));

    // Should still work for simple problems
    let ilp = ILP::binary(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );

    let solution = solver.solve(&ilp);
    assert!(solution.is_some());
}
