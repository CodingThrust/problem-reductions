use super::*;
use crate::solvers::BruteForce;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

#[test]
fn test_bmf_creation() {
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = BMF::new(matrix, 2);
    assert_eq!(problem.rows(), 2);
    assert_eq!(problem.cols(), 2);
    assert_eq!(problem.rank(), 2);
    assert_eq!(problem.num_variables(), 8); // 2*2 + 2*2
}

#[test]
fn test_extract_factors() {
    let matrix = vec![vec![true]];
    let problem = BMF::new(matrix, 1);
    // Config: [b00, c00] = [1, 1]
    let (b, c) = problem.extract_factors(&[1, 1]);
    assert_eq!(b, vec![vec![true]]);
    assert_eq!(c, vec![vec![true]]);
}

#[test]
fn test_extract_factors_larger() {
    // 2x2 matrix with rank 1
    let matrix = vec![vec![true, true], vec![true, true]];
    let problem = BMF::new(matrix, 1);
    // B: 2x1, C: 1x2
    // Config: [b00, b10, c00, c01] = [1, 1, 1, 1]
    let (b, c) = problem.extract_factors(&[1, 1, 1, 1]);
    assert_eq!(b, vec![vec![true], vec![true]]);
    assert_eq!(c, vec![vec![true, true]]);
}

#[test]
fn test_boolean_product() {
    // B = [[1], [1]], C = [[1, 1]]
    // B ⊙ C = [[1,1], [1,1]]
    let b = vec![vec![true], vec![true]];
    let c = vec![vec![true, true]];
    let product = BMF::boolean_product(&b, &c);
    assert_eq!(product, vec![vec![true, true], vec![true, true]]);
}

#[test]
fn test_boolean_product_rank2() {
    // B = [[1,0], [0,1]], C = [[1,0], [0,1]]
    // B ⊙ C = [[1,0], [0,1]] (identity)
    let b = vec![vec![true, false], vec![false, true]];
    let c = vec![vec![true, false], vec![false, true]];
    let product = BMF::boolean_product(&b, &c);
    assert_eq!(product, vec![vec![true, false], vec![false, true]]);
}

#[test]
fn test_hamming_distance() {
    // Target: [[1,0], [0,1]]
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = BMF::new(matrix, 2);

    // B = [[1,0], [0,1]], C = [[1,0], [0,1]] -> exact match
    // Config: [1,0,0,1, 1,0,0,1]
    let config = vec![1, 0, 0, 1, 1, 0, 0, 1];
    assert_eq!(problem.hamming_distance(&config), 0);

    // All zeros -> product is all zeros, distance = 2
    let config = vec![0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(problem.hamming_distance(&config), 2);
}

#[test]
fn test_evaluate() {
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = BMF::new(matrix, 2);

    // Exact factorization -> distance 0
    let config = vec![1, 0, 0, 1, 1, 0, 0, 1];
    assert_eq!(Problem::evaluate(&problem, &config), SolutionSize::Valid(0));

    // Non-exact -> distance 2
    let config = vec![0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(Problem::evaluate(&problem, &config), SolutionSize::Valid(2));
}

#[test]
fn test_brute_force_ones() {
    // All ones matrix can be factored with rank 1
    let matrix = vec![vec![true, true], vec![true, true]];
    let problem = BMF::new(matrix, 1);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    for sol in &solutions {
        // Exact factorization has distance 0
        assert_eq!(Problem::evaluate(&problem, sol), SolutionSize::Valid(0));
    }
}

#[test]
fn test_brute_force_identity() {
    // Identity matrix needs rank 2
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = BMF::new(matrix, 2);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Should find exact factorization
    for sol in &solutions {
        assert!(problem.is_exact(sol));
    }
}

#[test]
fn test_brute_force_insufficient_rank() {
    // Identity matrix with rank 1 cannot be exact
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = BMF::new(matrix, 1);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Best approximation has distance > 0
    let best_distance = problem.hamming_distance(&solutions[0]);
    // With rank 1, best we can do is distance 1 (all ones or all zeros except one)
    assert!(best_distance >= 1);
}

#[test]
fn test_boolean_matrix_product_function() {
    let b = vec![vec![true], vec![true]];
    let c = vec![vec![true, true]];
    let product = boolean_matrix_product(&b, &c);
    assert_eq!(product, vec![vec![true, true], vec![true, true]]);
}

#[test]
fn test_matrix_hamming_distance_function() {
    let a = vec![vec![true, false], vec![false, true]];
    let b = vec![vec![true, true], vec![true, true]];
    assert_eq!(matrix_hamming_distance(&a, &b), 2);

    let c = vec![vec![true, false], vec![false, true]];
    assert_eq!(matrix_hamming_distance(&a, &c), 0);
}

#[test]
fn test_direction() {
    let matrix = vec![vec![true]];
    let problem = BMF::new(matrix, 1);
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_empty_matrix() {
    let matrix: Vec<Vec<bool>> = vec![];
    let problem = BMF::new(matrix, 1);
    assert_eq!(problem.num_variables(), 0);
    // Empty matrix has distance 0
    assert_eq!(Problem::evaluate(&problem, &[]), SolutionSize::Valid(0));
}

#[test]
fn test_is_exact() {
    let matrix = vec![vec![true]];
    let problem = BMF::new(matrix, 1);
    assert!(problem.is_exact(&[1, 1]));
    assert!(!problem.is_exact(&[0, 0]));
}

#[test]
fn test_bmf_problem() {
    use crate::traits::{OptimizationProblem, Problem};
    use crate::types::Direction;

    // 2x2 identity matrix with rank 2
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = BMF::new(matrix, 2);

    // dims: B(2*2) + C(2*2) = 8 binary variables
    assert_eq!(problem.dims(), vec![2; 8]);

    // Exact factorization: B = I, C = I
    // Config: [1,0,0,1, 1,0,0,1]
    assert_eq!(Problem::evaluate(&problem, &[1, 0, 0, 1, 1, 0, 0, 1]), SolutionSize::Valid(0));

    // All zeros -> product is all zeros, distance = 2
    assert_eq!(Problem::evaluate(&problem, &[0, 0, 0, 0, 0, 0, 0, 0]), SolutionSize::Valid(2));

    // Direction is minimize
    assert_eq!(problem.direction(), Direction::Minimize);

    // Test with 1x1 matrix
    let matrix = vec![vec![true]];
    let problem = BMF::new(matrix, 1);
    assert_eq!(problem.dims(), vec![2; 2]); // B(1*1) + C(1*1)
    assert_eq!(Problem::evaluate(&problem, &[1, 1]), SolutionSize::Valid(0)); // Exact
    assert_eq!(Problem::evaluate(&problem, &[0, 0]), SolutionSize::Valid(1)); // Distance 1
}
