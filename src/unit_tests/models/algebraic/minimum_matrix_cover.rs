use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_minimum_matrix_cover_creation() {
    let matrix = vec![
        vec![0, 3, 1, 0],
        vec![3, 0, 0, 2],
        vec![1, 0, 0, 4],
        vec![0, 2, 4, 0],
    ];
    let problem = MinimumMatrixCover::new(matrix.clone());
    assert_eq!(problem.num_rows(), 4);
    assert_eq!(problem.matrix(), &matrix);
    assert_eq!(problem.dims(), vec![2; 4]);
    assert_eq!(problem.num_variables(), 4);
}

#[test]
fn test_minimum_matrix_cover_evaluate_all_minus() {
    // All -1: f = (-1,-1,-1,-1)
    // value = Σ a_ij * 1 * 1 = sum of all matrix entries
    let matrix = vec![
        vec![0, 3, 1, 0],
        vec![3, 0, 0, 2],
        vec![1, 0, 0, 4],
        vec![0, 2, 4, 0],
    ];
    let problem = MinimumMatrixCover::new(matrix);
    let value = problem.evaluate(&[0, 0, 0, 0]);
    // Sum of all entries = 0+3+1+0 + 3+0+0+2 + 1+0+0+4 + 0+2+4+0 = 20
    assert_eq!(value, Min(Some(20)));
}

#[test]
fn test_minimum_matrix_cover_evaluate_mixed() {
    let matrix = vec![
        vec![0, 3, 1, 0],
        vec![3, 0, 0, 2],
        vec![1, 0, 0, 4],
        vec![0, 2, 4, 0],
    ];
    let problem = MinimumMatrixCover::new(matrix);

    // Config [0,1,1,0] → f=(-1,+1,+1,-1)
    // Compute: Σ a_ij * f(i) * f(j)
    // For each (i,j):
    // (0,1): 3 * (-1)(+1) = -3
    // (0,2): 1 * (-1)(+1) = -1
    // (1,0): 3 * (+1)(-1) = -3
    // (1,3): 2 * (+1)(-1) = -2
    // (2,0): 1 * (+1)(-1) = -1
    // (2,3): 4 * (+1)(-1) = -4
    // (3,1): 2 * (-1)(+1) = -2
    // (3,2): 4 * (-1)(+1) = -4
    // All other terms are 0 (zero matrix entries or diagonal zeros)
    // Total = -3 + -1 + -3 + -2 + -1 + -4 + -2 + -4 = -20
    let value = problem.evaluate(&[0, 1, 1, 0]);
    assert_eq!(value, Min(Some(-20)));
}

#[test]
fn test_minimum_matrix_cover_evaluate_invalid() {
    let problem = MinimumMatrixCover::new(vec![vec![0, 1], vec![1, 0]]);

    // Wrong length
    assert_eq!(problem.evaluate(&[0]), Min(None));
    // Out-of-range value
    assert_eq!(problem.evaluate(&[0, 2]), Min(None));
}

#[test]
fn test_minimum_matrix_cover_solver() {
    let matrix = vec![
        vec![0, 3, 1, 0],
        vec![3, 0, 0, 2],
        vec![1, 0, 0, 4],
        vec![0, 2, 4, 0],
    ];
    let problem = MinimumMatrixCover::new(matrix);
    let solver = BruteForce::new();

    let value = solver.solve(&problem);
    assert_eq!(value, Min(Some(-20)));

    let witness = solver.find_witness(&problem);
    assert!(witness.is_some());
    let w = witness.unwrap();
    assert_eq!(problem.evaluate(&w), Min(Some(-20)));
}

#[test]
fn test_minimum_matrix_cover_serialization() {
    let matrix = vec![vec![0, 1], vec![1, 0]];
    let problem = MinimumMatrixCover::new(matrix);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinimumMatrixCover = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_rows(), 2);
    assert_eq!(deserialized.matrix(), problem.matrix());
}

#[test]
fn test_minimum_matrix_cover_1x1() {
    // 1×1 matrix: only one variable, f(1) = ±1
    // value = a_11 * f(1)^2 = a_11 regardless of sign
    let problem = MinimumMatrixCover::new(vec![vec![5]]);
    assert_eq!(problem.evaluate(&[0]), Min(Some(5)));
    assert_eq!(problem.evaluate(&[1]), Min(Some(5)));

    let solver = BruteForce::new();
    assert_eq!(solver.solve(&problem), Min(Some(5)));
}

#[test]
fn test_minimum_matrix_cover_paper_example() {
    // Same as canonical example: 4×4 symmetric matrix
    let matrix = vec![
        vec![0, 3, 1, 0],
        vec![3, 0, 0, 2],
        vec![1, 0, 0, 4],
        vec![0, 2, 4, 0],
    ];
    let problem = MinimumMatrixCover::new(matrix);
    let solver = BruteForce::new();

    // Verify the claimed optimal from the issue
    let value = problem.evaluate(&[0, 1, 1, 0]);
    assert_eq!(value, Min(Some(-20)));

    // Verify it is truly optimal
    let optimal_value = solver.solve(&problem);
    assert_eq!(optimal_value, Min(Some(-20)));

    // Verify the witness is one of the optimal solutions
    let all_witnesses = solver.find_all_witnesses(&problem);
    assert!(all_witnesses.contains(&vec![0, 1, 1, 0]));
}

#[cfg(feature = "example-db")]
#[test]
fn test_minimum_matrix_cover_canonical_example_spec() {
    use super::canonical_model_example_specs;
    let specs = canonical_model_example_specs();
    assert_eq!(specs.len(), 1);
    let spec = &specs[0];
    assert_eq!(spec.id, "minimum_matrix_cover");
    assert_eq!(spec.optimal_value, serde_json::json!(-20));
    assert_eq!(spec.optimal_config, vec![0, 1, 1, 0]);
}
