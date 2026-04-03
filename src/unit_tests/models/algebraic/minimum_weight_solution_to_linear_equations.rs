use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_minimum_weight_solution_to_linear_equations_creation() {
    let problem = MinimumWeightSolutionToLinearEquations::new(
        vec![vec![1, 0, 1], vec![0, 1, 1]],
        vec![1, 1],
        1,
    );

    assert_eq!(problem.coefficients(), &[vec![1, 0, 1], vec![0, 1, 1]]);
    assert_eq!(problem.rhs(), &[1, 1]);
    assert_eq!(problem.bound(), 1);
    assert_eq!(problem.num_equations(), 2);
    assert_eq!(problem.num_variables(), 3);
    assert_eq!(problem.dims(), vec![2, 2, 2]);
}

#[test]
fn test_minimum_weight_solution_to_linear_equations_evaluate() {
    let problem = MinimumWeightSolutionToLinearEquations::new(
        vec![vec![1, 0, 1], vec![0, 1, 1]],
        vec![1, 1],
        1,
    );

    assert!(problem.evaluate(&[0, 0, 1]));
    assert!(!problem.evaluate(&[1, 1, 0]));
    assert!(!problem.evaluate(&[1, 0, 0]));
    assert!(!problem.evaluate(&[0, 1, 0]));
    assert!(!problem.evaluate(&[0, 0]));
}

#[test]
fn test_minimum_weight_solution_to_linear_equations_solver_and_serialization() {
    let problem = MinimumWeightSolutionToLinearEquations::new(
        vec![vec![1, 0, 1], vec![0, 1, 1]],
        vec![1, 1],
        1,
    );

    let solver = BruteForce::new();
    assert_eq!(solver.find_witness(&problem), Some(vec![0, 0, 1]));
    assert_eq!(solver.find_all_witnesses(&problem), vec![vec![0, 0, 1]]);

    let json = serde_json::to_string(&problem).unwrap();
    let round_trip: MinimumWeightSolutionToLinearEquations = serde_json::from_str(&json).unwrap();
    assert_eq!(round_trip.coefficients(), problem.coefficients());
    assert_eq!(round_trip.rhs(), problem.rhs());
    assert_eq!(round_trip.bound(), problem.bound());
}
