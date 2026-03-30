//! Minimum Weight Solution to Linear Equations problem implementation.
//!
//! Given an n×m integer matrix A and integer vector b, find a rational vector y
//! with Ay = b that minimizes the number of non-zero entries (Hamming weight).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumWeightSolutionToLinearEquations",
        display_name: "Minimum Weight Solution to Linear Equations",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a rational solution to Ay=b minimizing the number of non-zero entries",
        fields: &[
            FieldInfo { name: "matrix", type_name: "Vec<Vec<i64>>", description: "n×m integer matrix A" },
            FieldInfo { name: "rhs", type_name: "Vec<i64>", description: "right-hand side vector b of length n" },
        ],
    }
}

/// Minimum Weight Solution to Linear Equations.
///
/// Given an n×m integer matrix A and an integer vector b, find a rational
/// vector y with Ay = b that minimizes ||y||_0 (the number of non-zero
/// entries, i.e., the Hamming weight of y).
///
/// # Representation
///
/// Each of the m columns is a binary variable: `x_j = 1` means column j is
/// selected (i.e., y_j may be non-zero). The evaluator checks whether the
/// restricted system (using only selected columns) is consistent over the
/// rationals, and returns the count of selected columns if so.
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::MinimumWeightSolutionToLinearEquations;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let matrix = vec![
///     vec![1, 2, 3, 1],
///     vec![2, 1, 1, 3],
/// ];
/// let rhs = vec![5, 4];
/// let problem = MinimumWeightSolutionToLinearEquations::new(matrix, rhs);
/// let solver = BruteForce::new();
/// let witness = solver.find_witness(&problem);
/// assert!(witness.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumWeightSolutionToLinearEquations {
    /// The n×m integer matrix A.
    matrix: Vec<Vec<i64>>,
    /// The right-hand side vector b of length n.
    rhs: Vec<i64>,
}

impl MinimumWeightSolutionToLinearEquations {
    /// Create a new MinimumWeightSolutionToLinearEquations instance.
    ///
    /// # Panics
    ///
    /// Panics if the matrix is empty, rows have inconsistent lengths,
    /// rhs length does not match the number of rows, or there are no columns.
    pub fn new(matrix: Vec<Vec<i64>>, rhs: Vec<i64>) -> Self {
        assert!(!matrix.is_empty(), "Matrix must have at least one row");
        let num_cols = matrix[0].len();
        assert!(num_cols > 0, "Matrix must have at least one column");
        for row in &matrix {
            assert_eq!(row.len(), num_cols, "All rows must have the same length");
        }
        assert_eq!(
            rhs.len(),
            matrix.len(),
            "RHS length must equal number of rows"
        );
        Self { matrix, rhs }
    }

    /// Returns a reference to the matrix A.
    pub fn matrix(&self) -> &[Vec<i64>] {
        &self.matrix
    }

    /// Returns a reference to the right-hand side vector b.
    pub fn rhs(&self) -> &[i64] {
        &self.rhs
    }

    /// Returns the number of equations (rows of A).
    pub fn num_equations(&self) -> usize {
        self.matrix.len()
    }

    /// Returns the number of variables (columns of A).
    pub fn num_variables(&self) -> usize {
        self.matrix[0].len()
    }

    /// Check whether the system restricted to the given column indices is
    /// consistent over the rationals. Uses integer Gaussian elimination on
    /// the augmented matrix [A'|b] with i128 arithmetic.
    fn is_consistent(&self, columns: &[usize]) -> bool {
        let n = self.num_equations();
        let k = columns.len();

        // Build augmented matrix [A'|b] as i128 to avoid overflow.
        // Each row has k coefficient columns + 1 rhs column.
        let mut aug: Vec<Vec<i128>> = (0..n)
            .map(|i| {
                let mut row = Vec::with_capacity(k + 1);
                for &j in columns {
                    row.push(self.matrix[i][j] as i128);
                }
                row.push(self.rhs[i] as i128);
                row
            })
            .collect();

        let mut pivot_row = 0;
        for col in 0..k {
            // Find a non-zero entry in column `col` at or below `pivot_row`.
            let Some(swap_row) = (pivot_row..n).find(|&r| aug[r][col] != 0) else {
                continue;
            };
            aug.swap(pivot_row, swap_row);

            let pivot_val = aug[pivot_row][col];
            // Eliminate all other rows.
            for r in 0..n {
                if r == pivot_row {
                    continue;
                }
                let factor = aug[r][col];
                if factor == 0 {
                    continue;
                }
                // row[r] = pivot_val * row[r] - factor * row[pivot_row]
                for c in 0..k + 1 {
                    aug[r][c] = pivot_val * aug[r][c] - factor * aug[pivot_row][c];
                }
            }
            pivot_row += 1;
        }

        // Check for inconsistency: any row with all-zero coefficients but
        // non-zero rhs means the system is inconsistent.
        for row in &aug[pivot_row..n] {
            if row[k] != 0 {
                return false;
            }
        }
        true
    }
}

impl Problem for MinimumWeightSolutionToLinearEquations {
    const NAME: &'static str = "MinimumWeightSolutionToLinearEquations";
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_variables()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        if config.len() != self.num_variables() {
            return Min(None);
        }
        if config.iter().any(|&v| v >= 2) {
            return Min(None);
        }

        let columns: Vec<usize> = config
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(j, _)| j)
            .collect();

        if columns.is_empty() {
            // No columns selected — consistent iff b = 0.
            if self.rhs.iter().all(|&v| v == 0) {
                return Min(Some(0));
            } else {
                return Min(None);
            }
        }

        if self.is_consistent(&columns) {
            Min(Some(columns.len()))
        } else {
            Min(None)
        }
    }
}

crate::declare_variants! {
    default MinimumWeightSolutionToLinearEquations => "2^num_variables",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // A = [[1,2,3,1],[2,1,1,3]], b = [5,4], m=4, n=2
    // Config [1,1,0,0]: select columns 0,1. Submatrix [[1,2],[2,1]].
    // Solve [1,2;2,1]y=[5,4] → y=(1,2). Consistent. Min(2).
    let matrix = vec![vec![1, 2, 3, 1], vec![2, 1, 1, 3]];
    let rhs = vec![5, 4];
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_weight_solution_to_linear_equations",
        instance: Box::new(MinimumWeightSolutionToLinearEquations::new(matrix, rhs)),
        optimal_config: vec![1, 1, 0, 0],
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/minimum_weight_solution_to_linear_equations.rs"]
mod tests;
