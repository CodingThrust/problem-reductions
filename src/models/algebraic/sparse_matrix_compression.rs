//! Sparse Matrix Compression problem implementation.
//!
//! Given an `m x n` binary matrix `A` and a positive integer `K`, determine
//! whether the rows can be overlaid into a storage vector of length `n + K`
//! by assigning each row a shift in `{1, ..., K}` without collisions.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SparseMatrixCompression",
        display_name: "Sparse Matrix Compression",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Overlay binary-matrix rows into a short storage vector by shifting each row without collisions",
        fields: &[
            FieldInfo { name: "matrix", type_name: "Vec<Vec<bool>>", description: "m x n binary matrix A" },
            FieldInfo { name: "bound_k", type_name: "usize", description: "Maximum shift range K" },
        ],
    }
}

/// Sparse Matrix Compression.
///
/// A configuration assigns one zero-based shift value to each row. The
/// implementation reconstructs the implied storage vector internally instead of
/// enumerating storage-vector entries directly, so brute-force search runs over
/// `bound_k ^ num_rows` shift assignments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparseMatrixCompression {
    matrix: Vec<Vec<bool>>,
    bound_k: usize,
}

impl SparseMatrixCompression {
    /// Create a new SparseMatrixCompression instance.
    ///
    /// # Panics
    ///
    /// Panics if `bound_k == 0` or if the matrix rows are ragged.
    pub fn new(matrix: Vec<Vec<bool>>, bound_k: usize) -> Self {
        assert!(bound_k > 0, "bound_k must be positive");

        let num_cols = matrix.first().map_or(0, Vec::len);
        for row in &matrix {
            assert_eq!(row.len(), num_cols, "All rows must have the same length");
        }

        Self { matrix, bound_k }
    }

    /// Return the binary matrix.
    pub fn matrix(&self) -> &[Vec<bool>] {
        &self.matrix
    }

    /// Return the shift bound `K`.
    pub fn bound_k(&self) -> usize {
        self.bound_k
    }

    /// Return the number of rows `m`.
    pub fn num_rows(&self) -> usize {
        self.matrix.len()
    }

    /// Return the number of columns `n`.
    pub fn num_cols(&self) -> usize {
        self.matrix.first().map_or(0, Vec::len)
    }

    /// Return the storage-vector length `n + K`.
    pub fn storage_len(&self) -> usize {
        self.num_cols() + self.bound_k
    }

    /// Decode a zero-based config into the one-based shifts used in the
    /// mathematical definition.
    pub fn decode_shifts(&self, config: &[usize]) -> Option<Vec<usize>> {
        if config.len() != self.num_rows() || config.iter().any(|&shift| shift >= self.bound_k) {
            return None;
        }

        Some(config.iter().map(|&shift| shift + 1).collect())
    }

    /// Construct the implied storage vector for a shift assignment.
    ///
    /// Returns `None` if the shifts are malformed or if the overlay is invalid.
    /// Row labels are stored as `1..=m`; `0` denotes an unused storage slot.
    pub fn storage_vector(&self, config: &[usize]) -> Option<Vec<usize>> {
        let shifts = self.decode_shifts(config)?;
        let mut storage = vec![0; self.storage_len()];

        for (row_idx, row) in self.matrix.iter().enumerate() {
            let row_label = row_idx + 1;
            let shift_offset = shifts[row_idx] - 1;

            for (col_idx, &entry) in row.iter().enumerate() {
                if !entry {
                    continue;
                }

                let slot_idx = shift_offset + col_idx;
                let slot = &mut storage[slot_idx];
                if *slot != 0 && *slot != row_label {
                    return None;
                }
                *slot = row_label;
            }
        }

        for (row_idx, row) in self.matrix.iter().enumerate() {
            let row_label = row_idx + 1;
            let shift_offset = shifts[row_idx] - 1;

            for (col_idx, &entry) in row.iter().enumerate() {
                let matches_row = storage[shift_offset + col_idx] == row_label;
                if entry != matches_row {
                    return None;
                }
            }
        }

        Some(storage)
    }
}

impl Problem for SparseMatrixCompression {
    const NAME: &'static str = "SparseMatrixCompression";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![self.bound_k; self.num_rows()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.storage_vector(config).is_some()
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for SparseMatrixCompression {}

crate::declare_variants! {
    default sat SparseMatrixCompression => "(bound_k ^ num_rows) * num_rows * num_cols",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "sparse_matrix_compression",
        instance: Box::new(SparseMatrixCompression::new(
            vec![
                vec![true, false, false, true],
                vec![false, true, false, false],
                vec![false, false, true, false],
                vec![true, false, false, false],
            ],
            2,
        )),
        optimal_config: vec![1, 1, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/sparse_matrix_compression.rs"]
mod tests;
