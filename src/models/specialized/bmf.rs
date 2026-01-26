//! Boolean Matrix Factorization (BMF) problem implementation.
//!
//! Given a boolean matrix A, find matrices B and C such that
//! the boolean product B ⊙ C approximates A.
//! The boolean product `(B ⊙ C)[i,j] = OR_k (B[i,k] AND C[k,j])`.

use crate::graph_types::SimpleGraph;
use crate::traits::Problem;
use crate::types::{EnergyMode, ProblemSize, SolutionSize};
use serde::{Deserialize, Serialize};

/// The Boolean Matrix Factorization problem.
///
/// Given an m×n boolean matrix A and rank k, find:
/// - B: m×k boolean matrix
/// - C: k×n boolean matrix
///
/// Such that the Hamming distance between A and B⊙C is minimized.
///
/// # Example
///
/// ```
/// use problemreductions::models::specialized::BMF;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 2x2 identity matrix
/// let a = vec![
///     vec![true, false],
///     vec![false, true],
/// ];
/// let problem = BMF::new(a, 1);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_best(&problem);
///
/// // Check the error
/// for sol in &solutions {
///     let error = problem.solution_size(sol).size;
///     println!("Hamming error: {}", error);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BMF {
    /// The target matrix A (m×n).
    matrix: Vec<Vec<bool>>,
    /// Number of rows (m).
    m: usize,
    /// Number of columns (n).
    n: usize,
    /// Factorization rank.
    k: usize,
}

impl BMF {
    /// Create a new BMF problem.
    ///
    /// # Arguments
    /// * `matrix` - The target m×n boolean matrix
    /// * `k` - The factorization rank
    pub fn new(matrix: Vec<Vec<bool>>, k: usize) -> Self {
        let m = matrix.len();
        let n = if m > 0 { matrix[0].len() } else { 0 };

        // Validate matrix dimensions
        for row in &matrix {
            assert_eq!(row.len(), n, "All rows must have the same length");
        }

        Self { matrix, m, n, k }
    }

    /// Get the number of rows.
    pub fn rows(&self) -> usize {
        self.m
    }

    /// Get the number of columns.
    pub fn cols(&self) -> usize {
        self.n
    }

    /// Get the factorization rank.
    pub fn rank(&self) -> usize {
        self.k
    }

    /// Get the target matrix.
    pub fn matrix(&self) -> &[Vec<bool>] {
        &self.matrix
    }

    /// Extract matrices B and C from a configuration.
    ///
    /// Config layout: first m*k bits are B (row-major), next k*n bits are C (row-major).
    pub fn extract_factors(&self, config: &[usize]) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
        let b_size = self.m * self.k;

        // Extract B (m×k)
        let b: Vec<Vec<bool>> = (0..self.m)
            .map(|i| {
                (0..self.k)
                    .map(|j| config.get(i * self.k + j).copied().unwrap_or(0) == 1)
                    .collect()
            })
            .collect();

        // Extract C (k×n)
        let c: Vec<Vec<bool>> = (0..self.k)
            .map(|i| {
                (0..self.n)
                    .map(|j| config.get(b_size + i * self.n + j).copied().unwrap_or(0) == 1)
                    .collect()
            })
            .collect();

        (b, c)
    }

    /// Compute the boolean product B ⊙ C.
    ///
    /// `(B ⊙ C)[i,j] = OR_k (B[i,k] AND C[k,j])`
    pub fn boolean_product(b: &[Vec<bool>], c: &[Vec<bool>]) -> Vec<Vec<bool>> {
        let m = b.len();
        let n = if !c.is_empty() { c[0].len() } else { 0 };
        let k = if !b.is_empty() { b[0].len() } else { 0 };

        (0..m)
            .map(|i| {
                (0..n)
                    .map(|j| (0..k).any(|kk| b[i][kk] && c[kk][j]))
                    .collect()
            })
            .collect()
    }

    /// Compute the Hamming distance between the target and the product.
    pub fn hamming_distance(&self, config: &[usize]) -> usize {
        let (b, c) = self.extract_factors(config);
        let product = Self::boolean_product(&b, &c);

        self.matrix
            .iter()
            .zip(product.iter())
            .map(|(a_row, p_row)| {
                a_row
                    .iter()
                    .zip(p_row.iter())
                    .filter(|(a, p)| a != p)
                    .count()
            })
            .sum()
    }

    /// Check if the factorization is exact (Hamming distance = 0).
    pub fn is_exact(&self, config: &[usize]) -> bool {
        self.hamming_distance(config) == 0
    }
}

impl Problem for BMF {
    const NAME: &'static str = "BMF";
    type GraphType = SimpleGraph;
    type Weight = i32;
    type Size = i32;

    fn num_variables(&self) -> usize {
        // B: m×k + C: k×n
        self.m * self.k + self.k * self.n
    }

    fn num_flavors(&self) -> usize {
        2 // Binary
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("rows", self.m),
            ("cols", self.n),
            ("rank", self.k),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::SmallerSizeIsBetter // Minimize Hamming distance
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let distance = self.hamming_distance(config) as i32;
        let is_valid = distance == 0; // Valid if exact factorization
        SolutionSize::new(distance, is_valid)
    }
}

/// Compute the boolean matrix product.
pub fn boolean_matrix_product(b: &[Vec<bool>], c: &[Vec<bool>]) -> Vec<Vec<bool>> {
    BMF::boolean_product(b, c)
}

/// Compute the Hamming distance between two boolean matrices.
pub fn matrix_hamming_distance(a: &[Vec<bool>], b: &[Vec<bool>]) -> usize {
    a.iter()
        .zip(b.iter())
        .map(|(a_row, b_row)| {
            a_row
                .iter()
                .zip(b_row.iter())
                .filter(|(x, y)| x != y)
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
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
        assert_eq!(
            product,
            vec![vec![true, true], vec![true, true]]
        );
    }

    #[test]
    fn test_boolean_product_rank2() {
        // B = [[1,0], [0,1]], C = [[1,0], [0,1]]
        // B ⊙ C = [[1,0], [0,1]] (identity)
        let b = vec![vec![true, false], vec![false, true]];
        let c = vec![vec![true, false], vec![false, true]];
        let product = BMF::boolean_product(&b, &c);
        assert_eq!(
            product,
            vec![vec![true, false], vec![false, true]]
        );
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
}
