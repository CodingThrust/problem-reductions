//! Feasible Basis Extension problem implementation.
//!
//! Given an m x n integer matrix A (m < n), a column vector a_bar of length m,
//! and a subset S of column indices with |S| < m, determine whether there exists
//! a feasible basis B (a set of m column indices including S) such that the
//! m x m submatrix A_B is nonsingular and A_B^{-1} a_bar >= 0.
//!
//! NP-complete (Murty, 1972).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "FeasibleBasisExtension",
        display_name: "Feasible Basis Extension",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Given matrix A, vector a_bar, and required columns S, find a feasible basis extending S",
        fields: &[
            FieldInfo { name: "matrix", type_name: "Vec<Vec<i64>>", description: "m x n integer matrix A (row-major)" },
            FieldInfo { name: "rhs", type_name: "Vec<i64>", description: "Column vector a_bar of length m" },
            FieldInfo { name: "required_columns", type_name: "Vec<usize>", description: "Subset S of column indices that must be in the basis" },
        ],
    }
}

/// The Feasible Basis Extension problem.
///
/// Given an m x n integer matrix A with m < n, a column vector a_bar of length m,
/// and a subset S of column indices with |S| < m, determine whether there exists
/// a feasible basis B of m columns (including all of S) such that the submatrix
/// A_B is nonsingular and A_B^{-1} a_bar >= 0.
///
/// # Representation
///
/// Each non-required column has a binary variable: `x_j = 1` if column j is
/// selected. A valid config must select exactly m - |S| additional columns.
/// The problem is satisfiable iff the resulting A_B is nonsingular and
/// A_B^{-1} a_bar >= 0.
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::FeasibleBasisExtension;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let matrix = vec![
///     vec![1, 0, 1, 2, -1, 0],
///     vec![0, 1, 0, 1,  1, 2],
///     vec![0, 0, 1, 1,  0, 1],
/// ];
/// let rhs = vec![7, 5, 3];
/// let required = vec![0, 1];
/// let problem = FeasibleBasisExtension::new(matrix, rhs, required);
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeasibleBasisExtension {
    matrix: Vec<Vec<i64>>,
    rhs: Vec<i64>,
    required_columns: Vec<usize>,
}

impl FeasibleBasisExtension {
    /// Create a new FeasibleBasisExtension instance.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - The matrix is empty or has inconsistent row lengths
    /// - m >= n (must have more columns than rows)
    /// - rhs length does not equal m
    /// - |S| >= m (must have room for at least one additional column)
    /// - Any required column index is out of bounds
    /// - Required columns contain duplicates
    pub fn new(matrix: Vec<Vec<i64>>, rhs: Vec<i64>, required_columns: Vec<usize>) -> Self {
        let m = matrix.len();
        assert!(m > 0, "Matrix must have at least one row");
        let n = matrix[0].len();
        for row in &matrix {
            assert_eq!(row.len(), n, "All rows must have the same length");
        }
        assert!(
            m < n,
            "Number of rows ({m}) must be less than number of columns ({n})"
        );
        assert_eq!(
            rhs.len(),
            m,
            "rhs length ({}) must equal number of rows ({m})",
            rhs.len()
        );
        assert!(
            required_columns.len() < m,
            "|S| ({}) must be less than m ({m})",
            required_columns.len()
        );
        for &col in &required_columns {
            assert!(col < n, "Required column index {col} out of bounds (n={n})");
        }
        // Check for duplicates
        let mut sorted = required_columns.clone();
        sorted.sort_unstable();
        for i in 1..sorted.len() {
            assert_ne!(
                sorted[i - 1],
                sorted[i],
                "Duplicate required column index {}",
                sorted[i]
            );
        }
        Self {
            matrix,
            rhs,
            required_columns,
        }
    }

    /// Returns the matrix A.
    pub fn matrix(&self) -> &[Vec<i64>] {
        &self.matrix
    }

    /// Returns the right-hand side vector a_bar.
    pub fn rhs(&self) -> &[i64] {
        &self.rhs
    }

    /// Returns the required column indices S.
    pub fn required_columns(&self) -> &[usize] {
        &self.required_columns
    }

    /// Returns the number of rows (m).
    pub fn num_rows(&self) -> usize {
        self.matrix.len()
    }

    /// Returns the number of columns (n).
    pub fn num_columns(&self) -> usize {
        self.matrix[0].len()
    }

    /// Returns the number of required columns (|S|).
    pub fn num_required(&self) -> usize {
        self.required_columns.len()
    }

    /// Returns the indices of non-required columns (the "free" columns).
    fn free_columns(&self) -> Vec<usize> {
        let required_set: std::collections::HashSet<usize> =
            self.required_columns.iter().copied().collect();
        (0..self.num_columns())
            .filter(|c| !required_set.contains(c))
            .collect()
    }

    /// Check if basis columns form a nonsingular system and the solution is non-negative.
    ///
    /// Uses exact rational arithmetic via integer Gaussian elimination with
    /// numerator/denominator tracking to avoid floating-point errors.
    #[allow(clippy::needless_range_loop)]
    fn check_feasible_basis(&self, basis_cols: &[usize]) -> bool {
        let m = self.num_rows();
        assert_eq!(basis_cols.len(), m);

        // Build augmented matrix [A_B | a_bar] using (numerator, denominator) pairs.
        // We'll use Bareiss algorithm for fraction-free Gaussian elimination.
        let mut aug: Vec<Vec<i64>> = Vec::with_capacity(m);
        for i in 0..m {
            let mut row = Vec::with_capacity(m + 1);
            for &col in basis_cols {
                row.push(self.matrix[i][col]);
            }
            row.push(self.rhs[i]);
            aug.push(row);
        }

        // Bareiss algorithm: fraction-free Gaussian elimination.
        // After elimination, the system is upper-triangular.
        let mut prev_pivot = 1i64;

        for k in 0..m {
            // Partial pivoting
            let mut max_row = k;
            let mut max_val = aug[k][k].abs();
            for i in (k + 1)..m {
                if aug[i][k].abs() > max_val {
                    max_val = aug[i][k].abs();
                    max_row = i;
                }
            }
            if max_val == 0 {
                return false; // singular
            }
            if max_row != k {
                aug.swap(k, max_row);
            }

            for i in (k + 1)..m {
                for j in (k + 1)..=m {
                    aug[i][j] = (aug[k][k] * aug[i][j] - aug[i][k] * aug[k][j]) / prev_pivot;
                }
                aug[i][k] = 0;
            }
            prev_pivot = aug[k][k];
        }

        // Back-substitution to solve. We solve in rational form: x_i = num_i / det.
        // The solution x = A_B^{-1} a_bar must satisfy x >= 0, which means
        // num_i / det >= 0 for all i, i.e., num_i and det have the same sign (or num_i = 0).
        // Back-substitution using rational arithmetic to check x >= 0.
        // Simple rational back-substitution:
        // x[i] = (aug[i][m] - sum_{j>i} aug[i][j] * x[j]) / aug[i][i]
        // We track x[i] as (numerator, denominator) pairs.

        let mut x_nums = vec![0i128; m];
        let mut x_dens = vec![1i128; m];

        for i in (0..m).rev() {
            // numerator of (aug[i][m] - sum_{j>i} aug[i][j] * x[j])
            let mut num = aug[i][m] as i128;
            let mut den = 1i128;

            for j in (i + 1)..m {
                // subtract aug[i][j] * (x_nums[j] / x_dens[j])
                // num/den - aug[i][j] * x_nums[j] / x_dens[j]
                // = (num * x_dens[j] - den * aug[i][j] * x_nums[j]) / (den * x_dens[j])
                let a = aug[i][j] as i128;
                num = num * x_dens[j] - den * a * x_nums[j];
                den *= x_dens[j];
                // Simplify to avoid overflow
                let g = gcd_i128(num.abs(), den.abs());
                if g > 1 {
                    num /= g;
                    den /= g;
                }
            }
            // x[i] = (num/den) / aug[i][i] = num / (den * aug[i][i])
            let diag = aug[i][i] as i128;
            x_nums[i] = num;
            x_dens[i] = den * diag;
            // Normalize sign: make denominator positive
            if x_dens[i] < 0 {
                x_nums[i] = -x_nums[i];
                x_dens[i] = -x_dens[i];
            }
            let g = gcd_i128(x_nums[i].abs(), x_dens[i].abs());
            if g > 1 {
                x_nums[i] /= g;
                x_dens[i] /= g;
            }
        }

        // Check x >= 0: each x_nums[i] / x_dens[i] >= 0
        // Since x_dens[i] > 0 (normalized), we need x_nums[i] >= 0
        x_nums.iter().take(m).all(|&num| num >= 0)
    }
}

/// Compute GCD of two i128 values.
fn gcd_i128(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

impl Problem for FeasibleBasisExtension {
    const NAME: &'static str = "FeasibleBasisExtension";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_columns() - self.num_required()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        let free_cols = self.free_columns();
        let num_free = free_cols.len();

        if config.len() != num_free {
            return crate::types::Or(false);
        }
        if config.iter().any(|&v| v >= 2) {
            return crate::types::Or(false);
        }

        let m = self.num_rows();
        let s = self.num_required();
        let needed = m - s;

        // Count selected free columns
        let selected_free: Vec<usize> = config
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| free_cols[i])
            .collect();

        if selected_free.len() != needed {
            return crate::types::Or(false);
        }

        // Form basis: required columns + selected free columns
        let mut basis_cols: Vec<usize> = self.required_columns.clone();
        basis_cols.extend_from_slice(&selected_free);

        crate::types::Or(self.check_feasible_basis(&basis_cols))
    }
}

crate::declare_variants! {
    default FeasibleBasisExtension => "2^num_columns * num_rows^3",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "feasible_basis_extension",
        // 3x6 matrix, rhs=[7,5,3], required={0,1}, select col 2 -> B={0,1,2}, x=(4,5,3)>=0
        instance: Box::new(FeasibleBasisExtension::new(
            vec![
                vec![1, 0, 1, 2, -1, 0],
                vec![0, 1, 0, 1, 1, 2],
                vec![0, 0, 1, 1, 0, 1],
            ],
            vec![7, 5, 3],
            vec![0, 1],
        )),
        optimal_config: vec![1, 0, 0, 0], // select col 2 (first free column)
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/feasible_basis_extension.rs"]
mod tests;
