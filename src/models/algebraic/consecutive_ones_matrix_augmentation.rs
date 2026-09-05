//! Consecutive Ones Matrix Augmentation problem implementation.
//!
//! Given an m x n binary matrix A and a nonnegative integer K, determine
//! whether there exists a permutation of the columns and at most K zero-to-one
//! augmentations such that every row has consecutive 1s.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ConsecutiveOnesMatrixAugmentation",
        display_name: "Consecutive Ones Matrix Augmentation",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Algebraic,
        module_path: module_path!(),
        description: "Augment a binary matrix with at most K zero-to-one flips so some column permutation has the consecutive ones property",
        fields: ConsecutiveOnesMatrixAugmentationCreateSpec::FIELDS,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsecutiveOnesMatrixAugmentation {
    matrix: Vec<Vec<bool>>,
    bound: i64,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct ConsecutiveOnesMatrixAugmentationCreateSpec {
    /// m x n binary matrix A.
    matrix: Vec<Vec<bool>>,
    /// Upper bound K on zero-to-one augmentations.
    bound: i64,
}
impl TryFrom<ConsecutiveOnesMatrixAugmentationCreateSpec> for ConsecutiveOnesMatrixAugmentation {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: ConsecutiveOnesMatrixAugmentationCreateSpec) -> Result<Self, Self::Error> {
        Self::try_new(spec.matrix, spec.bound)
    }
}

impl ConsecutiveOnesMatrixAugmentation {
    pub fn new(matrix: Vec<Vec<bool>>, bound: i64) -> Self {
        Self::try_new(matrix, bound).unwrap_or_else(|err| panic!("{err}"))
    }

    pub fn try_new(
        matrix: Vec<Vec<bool>>,
        bound: i64,
    ) -> Result<Self, crate::registry::ConstructionError> {
        let num_cols = matrix.first().map_or(0, Vec::len);
        if matrix.iter().any(|row| row.len() != num_cols) {
            return Err("all matrix rows must have the same length"
                .to_string()
                .into());
        }
        if bound < 0 {
            return Err("bound must be nonnegative".to_string().into());
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

    fn total_augmentation_cost(
        &self,
        config: &[usize],
    ) -> Result<Option<usize>, crate::traits::EvaluationError> {
        if !self.validate_permutation(config) {
            return Ok(None);
        }

        let mut total = 0usize;
        for row in &self.matrix {
            total = total
                .checked_add(Self::row_augmentation_cost(row, config))
                .ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "summing consecutive-ones matrix augmentation costs".to_string(),
                    )
                })?;
            if total > self.bound as usize {
                return Ok(Some(total));
            }
        }

        Ok(Some(total))
    }
}

impl Problem for ConsecutiveOnesMatrixAugmentation {
    const NAME: &'static str = "ConsecutiveOnesMatrixAugmentation";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_parameters![("num_cols", num_cols), ("num_rows", num_rows),];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        if config.len() != self.num_cols() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "column ordering length does not match the matrix".into(),
            ));
        }
        if config.iter().any(|&column| column >= self.num_cols()) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "column ordering contains an out-of-range column".into(),
            ));
        }
        Ok({
            crate::types::Or({
                self.total_augmentation_cost(config)?
                    .is_some_and(|cost| cost <= self.bound as usize)
            })
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl crate::solvers::BruteForceProblem for ConsecutiveOnesMatrixAugmentation {
    fn dimensions(&self) -> Vec<usize> {
        vec![self.num_cols(); self.num_cols()]
    }
}

crate::declare_variants! {
    default ConsecutiveOnesMatrixAugmentation => "factorial(num_cols) * num_rows * num_cols" create ConsecutiveOnesMatrixAugmentationCreateSpec,
}

crate::register_brute_force! {
    ConsecutiveOnesMatrixAugmentation,
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
        optimal_config: serde_json::json!(vec![0, 1, 4, 2, 3]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/consecutive_ones_matrix_augmentation.rs"]
mod tests;
