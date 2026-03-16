use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

/// Tucker matrix (3×4) — the classic C1P obstruction.
fn tucker_matrix() -> Vec<Vec<bool>> {
    vec![
        vec![true, true, false, true],
        vec![true, false, true, true],
        vec![false, true, true, false],
    ]
}

#[test]
fn test_consecutive_ones_submatrix_basic() {
    let problem = ConsecutiveOnesSubmatrix::new(tucker_matrix(), 3);
    assert_eq!(problem.num_rows(), 3);
    assert_eq!(problem.num_cols(), 4);
    assert_eq!(problem.bound_k(), 3);
    assert_eq!(problem.dims(), vec![2; 4]);
    assert_eq!(
        <ConsecutiveOnesSubmatrix as Problem>::NAME,
        "ConsecutiveOnesSubmatrix"
    );
    assert_eq!(<ConsecutiveOnesSubmatrix as Problem>::variant(), vec![]);
}

#[test]
fn test_consecutive_ones_submatrix_evaluate_satisfying() {
    let problem = ConsecutiveOnesSubmatrix::new(tucker_matrix(), 3);
    // Select columns {0, 1, 3} → config [1, 1, 0, 1]
    // Permutation [1, 0, 3]:
    //   r1: 1, 1, 1 → consecutive
    //   r2: 0, 1, 1 → consecutive
    //   r3: 1, 0, 0 → consecutive
    assert!(problem.evaluate(&[1, 1, 0, 1]));
}

#[test]
fn test_consecutive_ones_submatrix_evaluate_unsatisfying() {
    let problem = ConsecutiveOnesSubmatrix::new(tucker_matrix(), 4);
    // Full Tucker matrix does NOT have C1P
    assert!(!problem.evaluate(&[1, 1, 1, 1]));
}

#[test]
fn test_consecutive_ones_submatrix_evaluate_wrong_count() {
    let problem = ConsecutiveOnesSubmatrix::new(tucker_matrix(), 3);
    // Selecting 2 columns instead of 3 → false
    assert!(!problem.evaluate(&[1, 1, 0, 0]));
    // Selecting 4 columns instead of 3 → false
    assert!(!problem.evaluate(&[1, 1, 1, 1]));
}

#[test]
fn test_consecutive_ones_submatrix_evaluate_wrong_config_length() {
    let problem = ConsecutiveOnesSubmatrix::new(tucker_matrix(), 3);
    assert!(!problem.evaluate(&[1, 0]));
    assert!(!problem.evaluate(&[1, 0, 0, 0, 0]));
}

#[test]
fn test_consecutive_ones_submatrix_evaluate_invalid_variable_value() {
    let problem = ConsecutiveOnesSubmatrix::new(tucker_matrix(), 3);
    assert!(!problem.evaluate(&[2, 0, 0, 1]));
}

#[test]
fn test_consecutive_ones_submatrix_brute_force() {
    let problem = ConsecutiveOnesSubmatrix::new(tucker_matrix(), 3);
    let solver = BruteForce::new();
    let solution = solver
        .find_satisfying(&problem)
        .expect("should find a solution");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_consecutive_ones_submatrix_brute_force_all() {
    let problem = ConsecutiveOnesSubmatrix::new(tucker_matrix(), 3);
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_consecutive_ones_submatrix_unsatisfiable() {
    // Tucker matrix with K=4: no permutation of all 4 columns gives C1P
    let problem = ConsecutiveOnesSubmatrix::new(tucker_matrix(), 4);
    let solver = BruteForce::new();
    assert!(solver.find_satisfying(&problem).is_none());
}

#[test]
fn test_consecutive_ones_submatrix_trivial_c1p() {
    // Identity-like matrix: already has C1P for K = n
    let matrix = vec![
        vec![true, true, false],
        vec![false, true, true],
        vec![true, false, false],
    ];
    let problem = ConsecutiveOnesSubmatrix::new(matrix, 3);
    let solver = BruteForce::new();
    let solution = solver
        .find_satisfying(&problem)
        .expect("full matrix has C1P");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_consecutive_ones_submatrix_single_column() {
    // Any single column trivially has C1P
    let matrix = vec![vec![true, false, true], vec![false, true, false]];
    let problem = ConsecutiveOnesSubmatrix::new(matrix, 1);
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert_eq!(solutions.len(), 3); // each column works individually
}

#[test]
fn test_consecutive_ones_submatrix_empty_rows() {
    // Rows with all zeros or all ones are always satisfied
    let matrix = vec![
        vec![false, false, false],
        vec![true, true, true],
        vec![true, false, true],
    ];
    let problem = ConsecutiveOnesSubmatrix::new(matrix, 2);
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_consecutive_ones_submatrix_serialization() {
    let problem = ConsecutiveOnesSubmatrix::new(tucker_matrix(), 3);
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "matrix": [
                [true, true, false, true],
                [true, false, true, true],
                [false, true, true, false],
            ],
            "bound_k": 3,
        })
    );
    let restored: ConsecutiveOnesSubmatrix = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_rows(), 3);
    assert_eq!(restored.num_cols(), 4);
    assert_eq!(restored.bound_k(), 3);
}

#[test]
fn test_consecutive_ones_submatrix_paper_example() {
    // Tucker matrix with K=3: same instance as paper
    let problem = ConsecutiveOnesSubmatrix::new(tucker_matrix(), 3);
    // Verify that selecting cols {0,1,3} is satisfying
    assert!(problem.evaluate(&[1, 1, 0, 1]));

    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    // All solutions must be valid
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
    // Exactly 2 of the C(4,3)=4 subsets have C1P: {0,1,3} and {0,2,3}
    // ({0,1,2} and {1,2,3} fail due to Tucker obstructions in submatrix)
    assert_eq!(solutions.len(), 2);
}

#[test]
fn test_consecutive_ones_submatrix_k_zero() {
    // K=0: empty selection always satisfies (vacuously true)
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = ConsecutiveOnesSubmatrix::new(matrix, 0);
    assert!(problem.evaluate(&[0, 0])); // select nothing
    assert!(!problem.evaluate(&[1, 0])); // selected 1, need 0
}

#[test]
#[should_panic(expected = "bound_k")]
fn test_consecutive_ones_submatrix_k_too_large() {
    let matrix = vec![vec![true, false]];
    ConsecutiveOnesSubmatrix::new(matrix, 3);
}

#[test]
#[should_panic(expected = "same length")]
fn test_consecutive_ones_submatrix_inconsistent_rows() {
    let matrix = vec![vec![true, false], vec![true]];
    ConsecutiveOnesSubmatrix::new(matrix, 1);
}
