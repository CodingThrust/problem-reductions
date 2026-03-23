//! Consecutive Ones Matrix Augmentation problem implementation.
//!
//! Given an m x n binary matrix A and a nonnegative integer K, determine
//! whether there exists a permutation of the columns and at most K zero-to-one
//! augmentations such that every row has consecutive 1s.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ConsecutiveOnesMatrixAugmentation",
        display_name: "Consecutive Ones Matrix Augmentation",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Augment a binary matrix with at most K zero-to-one flips so some column permutation has the consecutive ones property",
        fields: &[
            FieldInfo { name: "matrix", type_name: "Vec<Vec<bool>>", description: "m x n binary matrix A" },
            FieldInfo { name: "bound", type_name: "i64", description: "Upper bound K on zero-to-one augmentations" },
        ],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsecutiveOnesMatrixAugmentation {
    matrix: Vec<Vec<bool>>,
    bound: i64,
}

impl ConsecutiveOnesMatrixAugmentation {
    pub fn new(matrix: Vec<Vec<bool>>, bound: i64) -> Self {
        Self::try_new(matrix, bound).unwrap_or_else(|err| panic!("{err}"))
    }

    pub fn try_new(matrix: Vec<Vec<bool>>, bound: i64) -> Result<Self, String> {
        let num_cols = matrix.first().map_or(0, Vec::len);
        if matrix.iter().any(|row| row.len() != num_cols) {
            return Err("all matrix rows must have the same length".to_string());
        }
        if bound < 0 {
            return Err("bound must be nonnegative".to_string());
        }
        Ok(Self { matrix, bound })
    }

    pub fn matrix(&self) -> &[Vec<bool>] {
        &self.matrix
    }

    pub fn bound(&self) -> i64 {
        self.bound
    }

    pub fn num_rows(&self) -> usize {
        self.matrix.len()
    }

    pub fn num_cols(&self) -> usize {
        self.matrix.first().map_or(0, Vec::len)
    }

    fn validate_permutation(&self, config: &[usize]) -> bool {
        if config.len() != self.num_cols() {
            return false;
        }

        let mut seen = vec![false; self.num_cols()];
        for &col in config {
            if col >= self.num_cols() || seen[col] {
                return false;
            }
            seen[col] = true;
        }
        true
    }

    fn row_augmentation_cost(row: &[bool], config: &[usize]) -> usize {
        let mut first_one = None;
        let mut last_one = None;
        let mut one_count = 0usize;

        for (position, &col) in config.iter().enumerate() {
            if row[col] {
                first_one.get_or_insert(position);
                last_one = Some(position);
                one_count += 1;
            }
        }

        match (first_one, last_one) {
            (Some(first), Some(last)) => last - first + 1 - one_count,
            _ => 0,
        }
    }

    fn total_augmentation_cost(&self, config: &[usize]) -> Option<usize> {
        if !self.validate_permutation(config) {
            return None;
        }

        let mut total = 0usize;
        for row in &self.matrix {
            total += Self::row_augmentation_cost(row, config);
            if total > self.bound as usize {
                return Some(total);
            }
        }

        Some(total)
    }
}

impl Problem for ConsecutiveOnesMatrixAugmentation {
    const NAME: &'static str = "ConsecutiveOnesMatrixAugmentation";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![self.num_cols(); self.num_cols()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.total_augmentation_cost(config)
            .is_some_and(|cost| cost <= self.bound as usize)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn num_variables(&self) -> usize {
        self.num_cols()
    }
}

impl SatisfactionProblem for ConsecutiveOnesMatrixAugmentation {}

crate::declare_variants! {
    default sat ConsecutiveOnesMatrixAugmentation => "factorial(num_cols) * num_rows * num_cols",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "consecutive_ones_matrix_augmentation",
        instance: Box::new(ConsecutiveOnesMatrixAugmentation::new(
            vec![
                vec![true, false, false, true, true],
                vec![true, true, false, false, false],
                vec![false, true, true, false, true],
                vec![false, false, true, true, false],
            ],
            2,
        )),
        optimal_config: vec![0, 1, 4, 2, 3],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/consecutive_ones_matrix_augmentation.rs"]
mod tests;
