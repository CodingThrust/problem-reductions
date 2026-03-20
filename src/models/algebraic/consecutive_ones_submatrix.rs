//! Consecutive Ones Submatrix problem implementation.
//!
//! Given an m×n binary matrix A and an integer 0 ≤ K ≤ n, determine whether
//! there exists a subset of K columns whose columns can be permuted so that in
//! each row all 1's occur consecutively. The implementation treats K = 0 as the
//! vacuous empty-submatrix case. NP-complete (Booth, 1975) via
//! transformation from Hamiltonian Path.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ConsecutiveOnesSubmatrix",
        display_name: "Consecutive Ones Submatrix",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find K columns of a binary matrix that can be permuted to have the consecutive ones property",
        fields: &[
            FieldInfo { name: "matrix", type_name: "Vec<Vec<bool>>", description: "m×n binary matrix A" },
            FieldInfo { name: "bound", type_name: "i64", description: "Required number of columns K" },
        ],
    }
}

/// The Consecutive Ones Submatrix problem.
///
/// Given an m×n binary matrix A and an integer 0 ≤ K ≤ n, determine
/// whether there exists a subset of K columns that has the "consecutive ones
/// property" — i.e., the columns can be permuted so that in each row all 1's
/// occur consecutively. The implementation treats K = 0 as vacuously
/// satisfiable.
///
/// # Representation
///
/// Each column has a binary variable: `x_j = 1` if column j is selected.
/// The problem is satisfiable iff exactly K columns are selected and some
/// permutation of those columns gives each row consecutive 1's. The current
/// evaluator checks those permutations explicitly with Heap's algorithm.
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::ConsecutiveOnesSubmatrix;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Tucker matrix (3×4) — full matrix does NOT have C1P, but K=3 does.
/// let matrix = vec![
///     vec![true, true, false, true],
///     vec![true, false, true, true],
///     vec![false, true, true, false],
/// ];
/// let problem = ConsecutiveOnesSubmatrix::new(matrix, 3);
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsecutiveOnesSubmatrix {
    matrix: Vec<Vec<bool>>,
    bound: i64,
}

impl ConsecutiveOnesSubmatrix {
    /// Create a new ConsecutiveOnesSubmatrix instance.
    ///
    /// # Panics
    ///
    /// Panics if `bound > n`, or if rows have inconsistent lengths.
    pub fn new(matrix: Vec<Vec<bool>>, bound: i64) -> Self {
        let n = if matrix.is_empty() {
            0
        } else {
            matrix[0].len()
        };
        for row in &matrix {
            assert_eq!(row.len(), n, "All rows must have the same length");
        }
        assert!(
            bound <= n as i64,
            "bound ({bound}) must be <= number of columns ({n})"
        );
        Self { matrix, bound }
    }

    /// Returns the binary matrix.
    pub fn matrix(&self) -> &[Vec<bool>] {
        &self.matrix
    }

    /// Returns the bound (the required number of columns).
    pub fn bound(&self) -> i64 {
        self.bound
    }

    /// Returns the number of rows (m).
    pub fn num_rows(&self) -> usize {
        self.matrix.len()
    }

    /// Returns the number of columns (n).
    pub fn num_cols(&self) -> usize {
        if self.matrix.is_empty() {
            0
        } else {
            self.matrix[0].len()
        }
    }

    /// Check if a given column ordering has the consecutive ones property.
    ///
    /// `col_order` is a permutation of K column indices.
    fn has_c1p(&self, col_order: &[usize]) -> bool {
        for row in &self.matrix {
            let mut first_one = None;
            let mut last_one = None;
            let mut count_ones = 0;
            for (pos, &col_idx) in col_order.iter().enumerate() {
                if row[col_idx] {
                    if first_one.is_none() {
                        first_one = Some(pos);
                    }
                    last_one = Some(pos);
                    count_ones += 1;
                }
            }
            // Ones are consecutive iff (last - first + 1) == count
            if count_ones > 0 {
                let span = last_one.unwrap() - first_one.unwrap() + 1;
                if span != count_ones {
                    return false;
                }
            }
        }
        true
    }

    /// Check if any permutation of the given columns has C1P.
    fn any_permutation_has_c1p(&self, cols: &[usize]) -> bool {
        let k = cols.len();
        if k == 0 {
            return true;
        }
        let mut perm: Vec<usize> = cols.to_vec();
        // Generate all permutations using Heap's algorithm
        let mut c = vec![0usize; k];
        if self.has_c1p(&perm) {
            return true;
        }
        let mut i = 0;
        while i < k {
            if c[i] < i {
                if i % 2 == 0 {
                    perm.swap(0, i);
                } else {
                    perm.swap(c[i], i);
                }
                if self.has_c1p(&perm) {
                    return true;
                }
                c[i] += 1;
                i = 0;
            } else {
                c[i] = 0;
                i += 1;
            }
        }
        false
    }
}

impl Problem for ConsecutiveOnesSubmatrix {
    const NAME: &'static str = "ConsecutiveOnesSubmatrix";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_cols()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.num_cols() {
            return false;
        }
        if config.iter().any(|&v| v >= 2) {
            return false;
        }
        // Collect selected column indices
        let selected: Vec<usize> = config
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| i)
            .collect();
        if (selected.len() as i64) != self.bound {
            return false;
        }
        self.any_permutation_has_c1p(&selected)
    }
}

impl SatisfactionProblem for ConsecutiveOnesSubmatrix {}

crate::declare_variants! {
    default sat ConsecutiveOnesSubmatrix => "2^(num_cols) * (num_rows + num_cols)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "consecutive_ones_submatrix",
        // Tucker matrix (3×4): full matrix lacks C1P, but K=3 works
        // Select columns {0,1,3} (config [1,1,0,1])
        instance: Box::new(ConsecutiveOnesSubmatrix::new(
            vec![
                vec![true, true, false, true],
                vec![true, false, true, true],
                vec![false, true, true, false],
            ],
            3,
        )),
        optimal_config: vec![1, 1, 0, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/consecutive_ones_submatrix.rs"]
mod tests;
