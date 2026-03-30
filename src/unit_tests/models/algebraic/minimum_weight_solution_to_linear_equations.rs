use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

/// A = [[1,2,3,1],[2,1,1,3]], b = [5,4]
fn example_instance() -> MinimumWeightSolutionToLinearEquations {
    let matrix = vec![vec![1, 2, 3, 1], vec![2, 1, 1, 3]];
    let rhs = vec![5, 4];
    MinimumWeightSolutionToLinearEquations::new(matrix, rhs)
}

#[test]
fn test_minimum_weight_solution_creation() {
    let problem = example_instance();
    assert_eq!(problem.num_equations(), 2);
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(problem.dims(), vec![2; 4]);
    assert_eq!(
        <MinimumWeightSolutionToLinearEquations as Problem>::NAME,
        "MinimumWeightSolutionToLinearEquations"
    );
    assert_eq!(
        <MinimumWeightSolutionToLinearEquations as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_minimum_weight_solution_evaluate_consistent() {
    let problem = example_instance();
    // Select columns 0,1: submatrix [[1,2],[2,1]], b=[5,4] → y=(1,2). Consistent.
    let config = vec![1, 1, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(Some(2)));
}

#[test]
fn test_minimum_weight_solution_evaluate_inconsistent() {
    let problem = example_instance();
    // Select only column 0: [1;2]y=[5;4] → y=5, but 2*5=10 ≠ 4. Inconsistent.
    let config = vec![1, 0, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimum_weight_solution_evaluate_all_selected() {
    let problem = example_instance();
    // All 4 columns selected — system has solution, so feasible with value 4.
    let config = vec![1, 1, 1, 1];
    assert_eq!(problem.evaluate(&config), Min(Some(4)));
}

#[test]
fn test_minimum_weight_solution_evaluate_none_selected() {
    let problem = example_instance();
    // No columns selected, b ≠ 0 → infeasible.
    let config = vec![0, 0, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimum_weight_solution_evaluate_wrong_length() {
    let problem = example_instance();
    assert_eq!(problem.evaluate(&[1, 0]), Min(None));
    assert_eq!(problem.evaluate(&[1; 5]), Min(None));
}

#[test]
fn test_minimum_weight_solution_evaluate_invalid_variable() {
    let problem = example_instance();
    let config = vec![2, 0, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimum_weight_solution_brute_force() {
    let problem = example_instance();
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).expect("should find optimal");
    let val = problem.evaluate(&witness);
    assert_eq!(val, Min(Some(2)));
}

#[test]
fn test_minimum_weight_solution_zero_rhs() {
    // A = [[1,1],[2,2]], b = [0,0] — trivially consistent with 0 columns.
    let matrix = vec![vec![1, 1], vec![2, 2]];
    let rhs = vec![0, 0];
    let problem = MinimumWeightSolutionToLinearEquations::new(matrix, rhs);
    let config = vec![0, 0];
    assert_eq!(problem.evaluate(&config), Min(Some(0)));
}

#[test]
fn test_minimum_weight_solution_serialization() {
    let problem = example_instance();
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "matrix": [[1, 2, 3, 1], [2, 1, 1, 3]],
            "rhs": [5, 4],
        })
    );
    let restored: MinimumWeightSolutionToLinearEquations = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_equations(), 2);
    assert_eq!(restored.num_variables(), 4);
}

#[test]
fn test_minimum_weight_solution_complexity_metadata() {
    use crate::registry::VariantEntry;

    let entry = inventory::iter::<VariantEntry>()
        .find(|entry| entry.name == "MinimumWeightSolutionToLinearEquations")
        .expect("MinimumWeightSolutionToLinearEquations variant entry should exist");

    assert_eq!(entry.complexity, "2^num_variables");
}

#[test]
#[should_panic(expected = "at least one row")]
fn test_minimum_weight_solution_empty_matrix() {
    MinimumWeightSolutionToLinearEquations::new(vec![], vec![]);
}

#[test]
#[should_panic(expected = "same length")]
fn test_minimum_weight_solution_inconsistent_rows() {
    let matrix = vec![vec![1, 2], vec![3]];
    MinimumWeightSolutionToLinearEquations::new(matrix, vec![1, 2]);
}

#[test]
#[should_panic(expected = "RHS length")]
fn test_minimum_weight_solution_rhs_mismatch() {
    let matrix = vec![vec![1, 2], vec![3, 4]];
    MinimumWeightSolutionToLinearEquations::new(matrix, vec![1]);
}
