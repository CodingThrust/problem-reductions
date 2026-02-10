//! Boolean Matrix Factorization (BMF) problem implementation.
//!
//! Given a boolean matrix A, find matrices B and C such that
//! the boolean product B ⊙ C approximates A.
//! The boolean product `(B ⊙ C)[i,j] = OR_k (B[i,k] AND C[k,j])`.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::{EnergyMode, ProblemSize, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "BMF",
        category: "specialized",
        description: "Boolean matrix factorization",
        fields: &[
            FieldInfo { name: "matrix", type_name: "Vec<Vec<bool>>", description: "Target boolean matrix A" },
            FieldInfo { name: "m", type_name: "usize", description: "Number of rows" },
            FieldInfo { name: "n", type_name: "usize", description: "Number of columns" },
            FieldInfo { name: "k", type_name: "usize", description: "Factorization rank" },
        ],
    }
}

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

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }

    type Size = i32;

    fn num_variables(&self) -> usize {
        // B: m×k + C: k×n
        self.m * self.k + self.k * self.n
    }

    fn num_flavors(&self) -> usize {
        2 // Binary
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![("rows", self.m), ("cols", self.n), ("rank", self.k)])
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
#[path = "../../unit_tests/models/specialized/bmf.rs"]
mod tests;
