//! Consecutive Block Minimization (CBM) problem implementation.
//!
//! Given an m x n binary matrix A and a positive integer K,
//! determine whether there exists a permutation of the columns of A
//! such that the resulting matrix has at most K maximal blocks of
//! consecutive 1-entries (summed over all rows).
//!
//! A "block" is a maximal contiguous run of 1-entries in a row.
//! This is problem SR17 in Garey & Johnson.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ConsecutiveBlockMinimization",
        display_name: "Consecutive Block Minimization",
        aliases: &["CBM"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Permute columns of a binary matrix to have at most K consecutive blocks of 1s",
        fields: &[
            FieldInfo { name: "matrix", type_name: "Vec<Vec<bool>>", description: "Binary matrix A (m x n)" },
            FieldInfo { name: "bound", type_name: "i64", description: "Upper bound K on total consecutive blocks" },
        ],
    }
}

/// Consecutive Block Minimization (CBM) problem.
///
/// Given an m x n binary matrix A and a positive integer K,
/// determine whether there exists a permutation of the columns of A
/// such that the resulting matrix has at most K maximal blocks of
/// consecutive 1-entries (summed over all rows).
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::ConsecutiveBlockMinimization;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 2x3 binary matrix
/// let problem = ConsecutiveBlockMinimization::new(
///     vec![
///         vec![true, false, true],
///         vec![false, true, true],
///     ],
///     2,
/// );
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_satisfying(&problem);
///
/// // Verify solutions satisfy the block bound
/// for sol in solutions {
///     assert!(problem.evaluate(&sol));
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "ConsecutiveBlockMinimizationDef")]
pub struct ConsecutiveBlockMinimization {
    /// The binary matrix A (m x n).
    matrix: Vec<Vec<bool>>,
    /// Number of rows (m).
    num_rows: usize,
    /// Number of columns (n).
    num_cols: usize,
    /// Upper bound K on total consecutive blocks.
    bound: i64,
}

impl ConsecutiveBlockMinimization {
    /// Create a new ConsecutiveBlockMinimization problem.
    ///
    /// # Arguments
    /// * `matrix` - The m x n binary matrix
    /// * `bound` - Upper bound on total consecutive blocks
    ///
    /// # Panics
    /// Panics if rows have inconsistent lengths.
    pub fn new(matrix: Vec<Vec<bool>>, bound: i64) -> Self {
        Self::try_new(matrix, bound).unwrap_or_else(|err| panic!("{err}"))
    }

    /// Create a new ConsecutiveBlockMinimization problem, returning an error
    /// instead of panicking when the matrix is ragged.
    pub fn try_new(matrix: Vec<Vec<bool>>, bound: i64) -> Result<Self, String> {
        let (num_rows, num_cols) = validate_matrix_dimensions(&matrix)?;
        Ok(Self {
            matrix,
            num_rows,
            num_cols,
            bound,
        })
    }

    /// Get the binary matrix.
    pub fn matrix(&self) -> &[Vec<bool>] {
        &self.matrix
    }

    /// Get the number of rows.
    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    /// Get the number of columns.
    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    /// Get the upper bound K.
    pub fn bound(&self) -> i64 {
        self.bound
    }

    /// Count the total number of maximal consecutive blocks of 1s
    /// when columns are permuted according to `config`.
    ///
    /// `config[position] = column_index` defines the column permutation.
    /// Returns `Some(total_blocks)` if the config is a valid permutation,
    /// or `None` if it is not (wrong length, duplicate columns, or out-of-range).
    pub fn count_consecutive_blocks(&self, config: &[usize]) -> Option<usize> {
        if config.len() != self.num_cols {
            return None;
        }

        // Validate permutation: all values distinct and in 0..num_cols.
        let mut seen = vec![false; self.num_cols];
        for &col in config {
            if col >= self.num_cols || seen[col] {
                return None;
            }
            seen[col] = true;
        }

        let mut total_blocks = 0;
        for row in &self.matrix {
            let mut in_block = false;
            for &pos in config {
                if row[pos] {
                    if !in_block {
                        total_blocks += 1;
                        in_block = true;
                    }
                } else {
                    in_block = false;
                }
            }
        }

        Some(total_blocks)
    }
}

impl Problem for ConsecutiveBlockMinimization {
    const NAME: &'static str = "ConsecutiveBlockMinimization";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![self.num_cols; self.num_cols]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        match self.count_consecutive_blocks(config) {
            Some(total) => (total as i64) <= self.bound,
            None => false,
        }
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn num_variables(&self) -> usize {
        self.num_cols
    }
}

impl SatisfactionProblem for ConsecutiveBlockMinimization {}

crate::declare_variants! {
    default sat ConsecutiveBlockMinimization => "factorial(num_cols) * num_rows * num_cols",
}

#[derive(Debug, Clone, Deserialize)]
struct ConsecutiveBlockMinimizationDef {
    matrix: Vec<Vec<bool>>,
    num_rows: usize,
    num_cols: usize,
    bound: i64,
}

impl TryFrom<ConsecutiveBlockMinimizationDef> for ConsecutiveBlockMinimization {
    type Error = String;

    fn try_from(value: ConsecutiveBlockMinimizationDef) -> Result<Self, Self::Error> {
        let problem = Self::try_new(value.matrix, value.bound)?;
        if value.num_rows != problem.num_rows {
            return Err(format!(
                "num_rows must match matrix row count ({})",
                problem.num_rows
            ));
        }
        if value.num_cols != problem.num_cols {
            return Err(format!(
                "num_cols must match matrix column count ({})",
                problem.num_cols
            ));
        }
        Ok(problem)
    }
}

fn validate_matrix_dimensions(matrix: &[Vec<bool>]) -> Result<(usize, usize), String> {
    let num_rows = matrix.len();
    let num_cols = matrix.first().map_or(0, Vec::len);

    if matrix.iter().any(|row| row.len() != num_cols) {
        return Err("all matrix rows must have the same length".to_string());
    }

    Ok((num_rows, num_cols))
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // Adjacency matrix of path graph P_6, K=6 (one block per row).
    // Issue #420 Instance 2.
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "consecutive_block_minimization",
        instance: Box::new(ConsecutiveBlockMinimization::new(
            vec![
                vec![false, true, false, false, false, false],
                vec![true, false, true, false, false, false],
                vec![false, true, false, true, false, false],
                vec![false, false, true, false, true, false],
                vec![false, false, false, true, false, true],
                vec![false, false, false, false, true, false],
            ],
            6,
        )),
        optimal_config: vec![0, 2, 4, 1, 3, 5],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/consecutive_block_minimization.rs"]
mod tests;
