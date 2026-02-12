use super::*;
use crate::solvers::{BruteForce, Solver};

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
fn test_solution_size() {
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = BMF::new(matrix, 2);

    // Exact factorization
    let config = vec![1, 0, 0, 1, 1, 0, 0, 1];
    let sol = problem.solution_size(&config);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 0);

    // Non-exact
    let config = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let sol = problem.solution_size(&config);
    assert!(!sol.is_valid);
    assert_eq!(sol.size, 2);
}

#[test]
fn test_brute_force_ones() {
    // All ones matrix can be factored with rank 1
    let matrix = vec![vec![true, true], vec![true, true]];
    let problem = BMF::new(matrix, 1);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    for sol in &solutions {
        let sol_size = problem.solution_size(sol);
        assert_eq!(sol_size.size, 0);
        assert!(sol_size.is_valid);
    }
}

#[test]
fn test_brute_force_identity() {
    // Identity matrix needs rank 2
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = BMF::new(matrix, 2);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
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
    let solver = BruteForce::new().valid_only(false);

    let solutions = solver.find_best(&problem);
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
fn test_energy_mode() {
    let matrix = vec![vec![true]];
    let problem = BMF::new(matrix, 1);
    assert!(problem.energy_mode().is_minimization());
}

#[test]
fn test_problem_size() {
    let matrix = vec![vec![true, false, true], vec![false, true, false]];
    let problem = BMF::new(matrix, 2);
    let size = problem.problem_size();
    assert_eq!(size.get("rows"), Some(2));
    assert_eq!(size.get("cols"), Some(3));
    assert_eq!(size.get("rank"), Some(2));
}

#[test]
fn test_empty_matrix() {
    let matrix: Vec<Vec<bool>> = vec![];
    let problem = BMF::new(matrix, 1);
    assert_eq!(problem.num_variables(), 0);
    let sol = problem.solution_size(&[]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 0);
}

#[test]
fn test_is_exact() {
    let matrix = vec![vec![true]];
    let problem = BMF::new(matrix, 1);
    assert!(problem.is_exact(&[1, 1]));
    assert!(!problem.is_exact(&[0, 0]));
}

#[test]
fn test_bmf_problem_v2() {
    use crate::traits::{OptimizationProblemV2, ProblemV2};
    use crate::types::Direction;

    // 2x2 identity matrix with rank 2
    let matrix = vec![vec![true, false], vec![false, true]];
    let problem = BMF::new(matrix, 2);

    // dims: B(2*2) + C(2*2) = 8 binary variables
    assert_eq!(problem.dims(), vec![2; 8]);

    // Exact factorization: B = I, C = I
    // Config: [1,0,0,1, 1,0,0,1]
    assert_eq!(problem.evaluate(&[1, 0, 0, 1, 1, 0, 0, 1]), 0);

    // All zeros -> product is all zeros, distance = 2
    assert_eq!(problem.evaluate(&[0, 0, 0, 0, 0, 0, 0, 0]), 2);

    // Direction is minimize
    assert_eq!(problem.direction(), Direction::Minimize);

    // Test with 1x1 matrix
    let matrix = vec![vec![true]];
    let problem = BMF::new(matrix, 1);
    assert_eq!(problem.dims(), vec![2; 2]); // B(1*1) + C(1*1)
    assert_eq!(problem.evaluate(&[1, 1]), 0); // Exact
    assert_eq!(problem.evaluate(&[0, 0]), 1); // Distance 1
}
