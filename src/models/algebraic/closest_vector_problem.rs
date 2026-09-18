//! Closest Vector Problem (CVP).
//!
//! Given an integer lattice basis `B` and a target vector `t`, find integer
//! coefficients `x` minimizing the squared distance `||Bx - t||_2^2`.

use crate::registry::{ConstructionError, CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::traits::{EvaluationError, Problem};
use crate::types::Min;
use num_bigint::BigInt;
use num_traits::Zero;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct ClosestVectorProblemCreateSpec {
    /// Integer basis matrix as semicolon-separated column vectors.
    #[create(codec = "semicolon-separated")]
    basis: Vec<Vec<i64>>,
    /// Integer target vector.
    #[create(name = "target_vec", codec = "comma-separated")]
    target: Vec<i64>,
}

impl TryFrom<ClosestVectorProblemCreateSpec> for ClosestVectorProblem {
    type Error = ConstructionError;

    fn try_from(spec: ClosestVectorProblemCreateSpec) -> Result<Self, Self::Error> {
        Self::new(spec.basis, spec.target)
    }
}

inventory::submit! {
    ProblemSchemaEntry {
        name: "ClosestVectorProblem",
        display_name: "Closest Vector Problem",
        aliases: &["CVP"],
        dimensions: &[VariantDimension::new("target", "i64", &["i64"])],
        category: crate::registry::ProblemCategory::Algebraic,
        module_path: module_path!(),
        description: "Find the closest point in an integer lattice to a target vector",
        fields: ClosestVectorProblemCreateSpec::FIELDS,
    }
}

/// Euclidean Closest Vector Problem over an integer lattice basis.
#[derive(Debug, Clone, Serialize)]
pub struct ClosestVectorProblem {
    /// Basis matrix stored as column vectors.
    basis: Vec<Vec<i64>>,
    /// Target vector in the ambient space.
    target: Vec<i64>,
}

impl ClosestVectorProblem {
    /// Construct a CVP instance with a full-column-rank integer basis.
    pub fn new(basis: Vec<Vec<i64>>, target: Vec<i64>) -> Result<Self, ConstructionError> {
        let ambient_dimension = target.len();
        for (index, column) in basis.iter().enumerate() {
            if column.len() != ambient_dimension {
                return Err(ConstructionError::Conversion(format!(
                    "basis vector {index} has length {}, expected {ambient_dimension}",
                    column.len()
                )));
            }
        }
        if basis.len() > ambient_dimension {
            return Err(ConstructionError::Conversion(format!(
                "{} basis vectors cannot be independent in ambient dimension {ambient_dimension}",
                basis.len()
            )));
        }
        if independent_rows(&basis, ambient_dimension).is_none() {
            return Err(ConstructionError::Conversion(
                "closest-vector basis columns must be linearly independent".into(),
            ));
        }
        Ok(Self { basis, target })
    }

    /// Number of basis vectors.
    pub fn num_basis_vectors(&self) -> usize {
        self.basis.len()
    }

    /// Dimension of the ambient space.
    pub fn ambient_dimension(&self) -> usize {
        self.target.len()
    }

    /// Integer basis columns.
    pub fn basis(&self) -> &[Vec<i64>] {
        &self.basis
    }

    /// Target coordinates.
    pub fn target(&self) -> &[i64] {
        &self.target
    }

    pub(crate) fn independent_rows(&self) -> Result<Vec<usize>, ConstructionError> {
        independent_rows(&self.basis, self.ambient_dimension()).ok_or_else(|| {
            ConstructionError::Conversion(
                "closest-vector basis columns must be linearly independent".into(),
            )
        })
    }
}

fn independent_rows(basis: &[Vec<i64>], ambient_dimension: usize) -> Option<Vec<usize>> {
    let num_columns = basis.len();
    if num_columns == 0 {
        return Some(Vec::new());
    }

    let mut matrix = (0..ambient_dimension)
        .map(|row| {
            basis
                .iter()
                .map(|column| BigInt::from(column[row]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // Rank is an exact predicate; elimination intermediates are not model fields.
    let mut previous_pivot = BigInt::from(1);
    let mut row_indices = (0..ambient_dimension).collect::<Vec<_>>();

    for column in 0..num_columns {
        let pivot_row = (column..ambient_dimension).find(|&row| !matrix[row][column].is_zero())?;
        matrix.swap(column, pivot_row);
        row_indices.swap(column, pivot_row);
        let pivot = matrix[column][column].clone();

        for row in (column + 1)..ambient_dimension {
            for next_column in (column + 1)..num_columns {
                matrix[row][next_column] = (&matrix[row][next_column] * &pivot
                    - &matrix[row][column] * &matrix[column][next_column])
                    / &previous_pivot;
            }
            matrix[row][column] = BigInt::zero();
        }
        previous_pivot = pivot;
    }
    row_indices.truncate(num_columns);
    Some(row_indices)
}

impl<'de> Deserialize<'de> for ClosestVectorProblem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            basis: Vec<Vec<i64>>,
            target: Vec<i64>,
        }

        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.basis, raw.target).map_err(serde::de::Error::custom)
    }
}

impl Problem for ClosestVectorProblem {
    const NAME: &'static str = "ClosestVectorProblem";
    type Solution = Vec<i64>;
    type Value = Min<i64>;

    crate::problem_parameters![
        ("ambient_dimension", ambient_dimension),
        ("num_basis_vectors", num_basis_vectors),
    ];

    fn evaluate(&self, solution: &Self::Solution) -> Result<Min<i64>, EvaluationError> {
        if solution.len() != self.num_basis_vectors() {
            return Err(EvaluationError::InvalidConfiguration(format!(
                "expected {} closest-vector coefficients, got {}",
                self.num_basis_vectors(),
                solution.len()
            )));
        }

        let overflow = || EvaluationError::IntegerOverflow("computing CVP squared distance".into());
        let mut squared = 0_i64;
        for (row, &target) in self.target.iter().enumerate() {
            let mut coordinate = 0_i64;
            for (&coefficient, column) in solution.iter().zip(&self.basis) {
                coordinate = coordinate
                    .checked_add(coefficient.checked_mul(column[row]).ok_or_else(overflow)?)
                    .ok_or_else(overflow)?;
            }
            let difference = coordinate.checked_sub(target).ok_or_else(overflow)?;
            squared = squared
                .checked_add(difference.checked_mul(difference).ok_or_else(overflow)?)
                .ok_or_else(overflow)?;
        }
        Ok(Min(Some(squared)))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("target", "i64")]
    }
}

crate::declare_variants! {
    default ClosestVectorProblem => "2^(num_basis_vectors * log(num_basis_vectors))" create ClosestVectorProblemCreateSpec,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "closest_vector_problem",
        instance: Box::new(
            ClosestVectorProblem::new(vec![vec![2, 0], vec![1, 2]], vec![3_i64, 2])
                .expect("canonical closest-vector instance must be valid"),
        ),
        optimal_config: serde_json::json!(vec![1, 1]),
        optimal_value: serde_json::json!(0),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/closest_vector_problem.rs"]
mod tests;
