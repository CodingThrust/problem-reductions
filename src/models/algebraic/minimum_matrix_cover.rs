//! Minimum Matrix Cover problem implementation.
//!
//! Given an n×n nonnegative integer matrix A, find a sign assignment
//! f: {1,...,n} → {-1,+1} minimizing Σ a_ij · f(i) · f(j).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumMatrixCover",
        display_name: "Minimum Matrix Cover",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Algebraic,
        module_path: module_path!(),
        description: "Find sign assignment minimizing quadratic form over nonnegative integer matrix",
        fields: &[
            FieldInfo { name: "matrix", type_name: "Vec<Vec<i64>>", description: "n×n nonnegative integer matrix" },
        ],
    }
}

/// Minimum Matrix Cover.
///
/// Given an n×n nonnegative integer matrix A, find a function
/// f: {1,...,n} → {-1,+1} that minimizes the quadratic form:
///
/// Σ_{i,j} a_ij · f(i) · f(j)
///
/// Each binary variable x_i ∈ {0,1} maps to a sign: f(i) = 2·x_i - 1
/// (0 → -1, 1 → +1).
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::MinimumMatrixCover;
/// use problemreductions::{Problem, BruteForce};
///
/// let problem = MinimumMatrixCover::new(vec![
///     vec![0, 3, 1, 0],
///     vec![3, 0, 0, 2],
///     vec![1, 0, 0, 4],
///     vec![0, 2, 4, 0],
/// ]);
///
/// let solver = BruteForce::new();
/// let witness = solver.solve(&problem).unwrap();
/// assert!(witness.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumMatrixCover {
    /// The n×n nonnegative integer matrix.
    matrix: Vec<Vec<i64>>,
}

impl MinimumMatrixCover {
    /// Create a new MinimumMatrixCover instance.
    ///
    /// # Panics
    ///
    /// Panics if the matrix is not square or has inconsistent row lengths.
    pub fn new(matrix: Vec<Vec<i64>>) -> Self {
        let n = matrix.len();
        for (i, row) in matrix.iter().enumerate() {
            assert_eq!(
                row.len(),
                n,
                "Matrix must be square: row {i} has {} columns, expected {n}",
                row.len()
            );
        }
        Self { matrix }
    }

    /// Returns the number of rows (= columns) of the matrix.
    pub fn num_rows(&self) -> usize {
        self.matrix.len()
    }

    /// Returns a reference to the matrix.
    pub fn matrix(&self) -> &[Vec<i64>] {
        &self.matrix
    }
}

impl Problem for MinimumMatrixCover {
    const NAME: &'static str = "MinimumMatrixCover";
    type Solution = Vec<bool>;
    type Value = Min<i64>;

    crate::problem_size![("num_rows", num_rows),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        Ok({
            let n = self.num_rows();
            if config.len() != n {
                return Err(crate::traits::EvaluationError::InvalidConfiguration(
                    "row-sign assignment length does not match the matrix".into(),
                ));
            }
            // Map config to signs: 0 → -1, 1 → +1
            let signs: Vec<i64> = config
                .iter()
                .map(|&value| if value { 1 } else { -1 })
                .collect();

            // Compute Σ_{i,j} a_ij * f(i) * f(j)
            let mut value: i64 = 0;
            for i in 0..n {
                for j in 0..n {
                    let term = self.matrix[i][j]
                        .checked_mul(signs[i])
                        .and_then(|term| term.checked_mul(signs[j]))
                        .ok_or_else(|| {
                            crate::traits::EvaluationError::IntegerOverflow(
                                "multiplying matrix-cover objective term".into(),
                            )
                        })?;
                    value = value.checked_add(term).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "summing matrix-cover objective".into(),
                        )
                    })?;
                }
            }

            Min(Some(value))
        })
    }
}

impl crate::solvers::BruteForceProblem for MinimumMatrixCover {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.num_rows()]
    }
}

crate::declare_variants! {
    default MinimumMatrixCover => "2^num_rows",
}

crate::register_brute_force! {
    MinimumMatrixCover decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 4×4 symmetric matrix with zero diagonal
    // Config [0,1,1,0] → f=(-1,+1,+1,-1) → value = -20
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_matrix_cover",
        instance: Box::new(MinimumMatrixCover::new(vec![
            vec![0, 3, 1, 0],
            vec![3, 0, 0, 2],
            vec![1, 0, 0, 4],
            vec![0, 2, 4, 0],
        ])),
        optimal_config: serde_json::json!(vec![false, true, true, false]),
        optimal_value: serde_json::json!(-20),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/minimum_matrix_cover.rs"]
mod tests;
