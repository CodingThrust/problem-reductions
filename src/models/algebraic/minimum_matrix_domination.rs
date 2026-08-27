//! Minimum Matrix Domination problem implementation.
//!
//! Given an n×n binary matrix M, find a minimum subset C of 1-entries such that
//! every 1-entry not in C shares a row or column with some entry in C.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumMatrixDomination",
        display_name: "Minimum Matrix Domination",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Algebraic,
        module_path: module_path!(),
        description: "Find minimum subset of 1-entries in a binary matrix that dominates all other 1-entries by shared row or column",
        fields: &[
            FieldInfo { name: "matrix", type_name: "Vec<Vec<bool>>", description: "n×n binary matrix M" },
        ],
    }
}

/// Minimum Matrix Domination.
///
/// Given an n×n binary matrix M, find a minimum-cardinality subset C of
/// 1-entries such that every 1-entry not in C shares a row or column with
/// some entry in C.
///
/// # Representation
///
/// Each 1-entry in the matrix is a binary variable: `x_k = 1` if the k-th
/// 1-entry is selected into C. The 1-entries are enumerated in row-major
/// order.
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::MinimumMatrixDomination;
/// use problemreductions::{Problem, BruteForce};
///
/// // 3×3 identity matrix: 3 ones on the diagonal, no shared rows/cols
/// let matrix = vec![
///     vec![true, false, false],
///     vec![false, true, false],
///     vec![false, false, true],
/// ];
/// let problem = MinimumMatrixDomination::new(matrix);
/// let solver = BruteForce::new();
/// let witness = solver.solve(&problem).unwrap();
/// // All 3 diagonal entries must be selected (no domination possible)
/// assert_eq!(witness, Some(vec![true, true, true]));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumMatrixDomination {
    /// The binary matrix.
    matrix: Vec<Vec<bool>>,
    /// Positions of 1-entries in row-major order: (row, col).
    ones: Vec<(usize, usize)>,
}

impl MinimumMatrixDomination {
    /// Create a new MinimumMatrixDomination instance.
    ///
    /// # Panics
    ///
    /// Panics if the matrix rows have inconsistent lengths.
    pub fn new(matrix: Vec<Vec<bool>>) -> Self {
        let num_cols = matrix.first().map_or(0, Vec::len);
        for row in &matrix {
            assert_eq!(row.len(), num_cols, "All rows must have the same length");
        }
        let ones: Vec<(usize, usize)> = matrix
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &v)| v)
                    .map(move |(j, _)| (i, j))
            })
            .collect();
        Self { matrix, ones }
    }

    /// Returns a reference to the binary matrix.
    pub fn matrix(&self) -> &[Vec<bool>] {
        &self.matrix
    }

    /// Returns the positions of 1-entries in row-major order.
    pub fn ones(&self) -> &[(usize, usize)] {
        &self.ones
    }

    /// Returns the number of rows in the matrix.
    pub fn num_rows(&self) -> usize {
        self.matrix.len()
    }

    /// Returns the number of columns in the matrix.
    pub fn num_cols(&self) -> usize {
        self.matrix.first().map_or(0, Vec::len)
    }

    /// Returns the number of 1-entries in the matrix.
    pub fn num_ones(&self) -> usize {
        self.ones.len()
    }
}

impl Problem for MinimumMatrixDomination {
    const NAME: &'static str = "MinimumMatrixDomination";
    type Solution = Vec<bool>;
    type Value = Min<i64>;

    crate::problem_size![
        ("num_cols", num_cols),
        ("num_ones", num_ones),
        ("num_rows", num_rows),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        Ok({
            if config.len() != self.num_ones() {
                return Err(crate::traits::EvaluationError::InvalidConfiguration(
                    "selected-entry vector length does not match the matrix".into(),
                ));
            }
            // Collect the set of selected 1-entry indices
            let selected: Vec<usize> = config
                .iter()
                .enumerate()
                .filter(|(_, &v)| v)
                .map(|(i, _)| i)
                .collect();

            // Build sets of rows and columns covered by selected entries
            let mut covered_rows = std::collections::HashSet::new();
            let mut covered_cols = std::collections::HashSet::new();
            for &idx in &selected {
                let (r, c) = self.ones[idx];
                covered_rows.insert(r);
                covered_cols.insert(c);
            }

            // Check domination: every unselected 1-entry must share a row or
            // column with some selected entry
            for (k, &(r, c)) in self.ones.iter().enumerate() {
                if config[k] {
                    continue; // selected entries don't need domination
                }
                if !covered_rows.contains(&r) && !covered_cols.contains(&c) {
                    return Ok(Min(None)); // not dominated
                }
            }

            Min(Some(i64::try_from(selected.len()).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting matrix-domination cardinality to i64".into(),
                )
            })?))
        })
    }
}

impl crate::solvers::BruteForceProblem for MinimumMatrixDomination {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.num_ones()]
    }
}

crate::declare_variants! {
    default MinimumMatrixDomination => "2^num_ones",
}

crate::register_brute_force! {
    MinimumMatrixDomination decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // P6 adjacency matrix (6×6, 10 ones)
    // 1-entries: (0,1),(1,0),(1,2),(2,1),(2,3),(3,2),(3,4),(4,3),(4,5),(5,4)
    // Optimal: select indices 0,1,7,6 -> C = {(0,1),(1,0),(4,3),(3,4)}, value = 4
    let matrix = vec![
        vec![false, true, false, false, false, false],
        vec![true, false, true, false, false, false],
        vec![false, true, false, true, false, false],
        vec![false, false, true, false, true, false],
        vec![false, false, false, true, false, true],
        vec![false, false, false, false, true, false],
    ];
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_matrix_domination",
        instance: Box::new(MinimumMatrixDomination::new(matrix)),
        optimal_config: serde_json::json!(vec![
            true, true, false, false, false, false, true, true, false, false
        ]),
        optimal_value: serde_json::json!(4),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/minimum_matrix_domination.rs"]
mod tests;
