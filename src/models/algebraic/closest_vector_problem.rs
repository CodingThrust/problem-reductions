//! Closest Vector Problem (CVP).
//!
//! Given an integer lattice basis `B` and a target vector `t`, find integer
//! coefficients `x` minimizing `||Bx - t||_2`.

use crate::registry::{ConstructionError, CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::traits::{EvaluationError, Problem};
use crate::types::Min;
use serde::{Deserialize, Serialize};

/// Target coordinate domains supported by [`ClosestVectorProblem`].
pub trait ClosestVectorTarget: Clone + std::fmt::Debug + 'static {
    /// Registered value of the `target` variant dimension.
    const NAME: &'static str;

    /// Validate one stored target coordinate.
    fn validate(&self, index: usize) -> Result<(), ConstructionError>;

    /// Convert one coordinate for numerical evaluation and solving.
    fn to_f64(&self) -> Result<f64, EvaluationError>;
}

impl ClosestVectorTarget for i64 {
    const NAME: &'static str = "i64";

    fn validate(&self, _index: usize) -> Result<(), ConstructionError> {
        Ok(())
    }

    fn to_f64(&self) -> Result<f64, EvaluationError> {
        crate::types::i64_to_exact_f64(*self)
            .map_err(|error| EvaluationError::InexactFloatConversion(error.to_string()))
    }
}

impl ClosestVectorTarget for f64 {
    const NAME: &'static str = "f64";

    fn validate(&self, index: usize) -> Result<(), ConstructionError> {
        if self.is_finite() {
            Ok(())
        } else {
            Err(ConstructionError::NonFiniteFloat(format!(
                "target coordinate at index {index} must be finite"
            )))
        }
    }

    fn to_f64(&self) -> Result<f64, EvaluationError> {
        Ok(*self)
    }
}

macro_rules! cvp_create_spec {
    ($name:ident, $target:ty) => {
        #[derive(Debug, Deserialize, crate::CreateSpec)]
        struct $name {
            /// Integer basis matrix as semicolon-separated column vectors.
            #[create(codec = "semicolon-separated")]
            basis: Vec<Vec<i64>>,
            /// Target vector.
            #[create(name = "target_vec", codec = "comma-separated")]
            target: Vec<$target>,
        }

        impl TryFrom<$name> for ClosestVectorProblem<$target> {
            type Error = ConstructionError;

            fn try_from(spec: $name) -> Result<Self, Self::Error> {
                ClosestVectorProblem::new(spec.basis, spec.target)
            }
        }
    };
}

cvp_create_spec!(ClosestVectorProblemI64CreateSpec, i64);
cvp_create_spec!(ClosestVectorProblemF64CreateSpec, f64);

inventory::submit! {
    ProblemSchemaEntry {
        name: "ClosestVectorProblem",
        display_name: "Closest Vector Problem",
        aliases: &["CVP"],
        dimensions: &[VariantDimension::new("target", "i64", &["i64", "f64"])],
        category: crate::registry::ProblemCategory::Algebraic,
        module_path: module_path!(),
        description: "Find the closest point in an integer lattice to a target vector",
        fields: ClosestVectorProblemI64CreateSpec::FIELDS,
    }
}

/// Euclidean Closest Vector Problem over an integer lattice basis.
#[derive(Debug, Clone, Serialize)]
pub struct ClosestVectorProblem<T = i64> {
    /// Basis matrix stored as column vectors.
    basis: Vec<Vec<i64>>,
    /// Target vector in the ambient space.
    target: Vec<T>,
}

impl<T: ClosestVectorTarget> ClosestVectorProblem<T> {
    /// Construct a CVP instance with a full-column-rank integer basis.
    pub fn new(basis: Vec<Vec<i64>>, target: Vec<T>) -> Result<Self, ConstructionError> {
        let ambient_dimension = target.len();
        for (index, coordinate) in target.iter().enumerate() {
            coordinate.validate(index)?;
        }
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
        if independent_rows(&basis, ambient_dimension)?.is_none() {
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
    pub fn target(&self) -> &[T] {
        &self.target
    }

    pub(crate) fn independent_rows(&self) -> Result<Vec<usize>, ConstructionError> {
        independent_rows(&self.basis, self.ambient_dimension())?.ok_or_else(|| {
            ConstructionError::Conversion(
                "closest-vector basis columns must be linearly independent".into(),
            )
        })
    }
}

fn independent_rows(
    basis: &[Vec<i64>],
    ambient_dimension: usize,
) -> Result<Option<Vec<usize>>, ConstructionError> {
    let num_columns = basis.len();
    if num_columns == 0 {
        return Ok(Some(Vec::new()));
    }

    let mut matrix = (0..ambient_dimension)
        .map(|row| basis.iter().map(|column| column[row]).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut previous_pivot = 1_i64;
    let mut row_indices = (0..ambient_dimension).collect::<Vec<_>>();

    for column in 0..num_columns {
        let Some(pivot_row) = (column..ambient_dimension).find(|&row| matrix[row][column] != 0)
        else {
            return Ok(None);
        };
        matrix.swap(column, pivot_row);
        row_indices.swap(column, pivot_row);
        let pivot = matrix[column][column];

        for row in (column + 1)..ambient_dimension {
            for next_column in (column + 1)..num_columns {
                let left = matrix[row][next_column]
                    .checked_mul(pivot)
                    .ok_or_else(rank_overflow)?;
                let right = matrix[row][column]
                    .checked_mul(matrix[column][next_column])
                    .ok_or_else(rank_overflow)?;
                let numerator = left.checked_sub(right).ok_or_else(rank_overflow)?;
                matrix[row][next_column] = numerator
                    .checked_div(previous_pivot)
                    .ok_or_else(rank_overflow)?;
            }
            matrix[row][column] = 0;
        }
        previous_pivot = pivot;
    }
    row_indices.truncate(num_columns);
    Ok(Some(row_indices))
}

fn rank_overflow() -> ConstructionError {
    ConstructionError::IntegerOverflow("checking closest-vector basis rank".into())
}

impl<'de, T> Deserialize<'de> for ClosestVectorProblem<T>
where
    T: ClosestVectorTarget + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw<T> {
            basis: Vec<Vec<i64>>,
            target: Vec<T>,
        }

        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.basis, raw.target).map_err(serde::de::Error::custom)
    }
}

impl<T> Problem for ClosestVectorProblem<T>
where
    T: ClosestVectorTarget + Serialize + for<'de> Deserialize<'de>,
{
    const NAME: &'static str = "ClosestVectorProblem";
    type Solution = Vec<i64>;
    type Value = Min<f64>;

    crate::problem_parameters![
        ("ambient_dimension", ambient_dimension),
        ("num_basis_vectors", num_basis_vectors),
    ];

    fn evaluate(&self, solution: &Self::Solution) -> Result<Min<f64>, EvaluationError> {
        if solution.len() != self.num_basis_vectors() {
            return Err(EvaluationError::InvalidConfiguration(format!(
                "expected {} closest-vector coefficients, got {}",
                self.num_basis_vectors(),
                solution.len()
            )));
        }

        let mut displacement = self
            .target
            .iter()
            .map(ClosestVectorTarget::to_f64)
            .collect::<Result<Vec<_>, _>>()?;
        for value in &mut displacement {
            *value = -*value;
        }

        for (&coefficient, column) in solution.iter().zip(&self.basis) {
            let coefficient = crate::types::i64_to_exact_f64(coefficient)
                .map_err(|error| EvaluationError::InexactFloatConversion(error.to_string()))?;
            for (value, &basis_entry) in displacement.iter_mut().zip(column) {
                let basis_entry = crate::types::i64_to_exact_f64(basis_entry)
                    .map_err(|error| EvaluationError::InexactFloatConversion(error.to_string()))?;
                let next = *value + coefficient * basis_entry;
                if !next.is_finite() {
                    return Err(EvaluationError::NonFiniteResult(
                        "computing closest-vector displacement".into(),
                    ));
                }
                *value = next;
            }
        }

        let squared_norm = displacement.into_iter().try_fold(0.0, |total, value| {
            let next = total + value * value;
            if next.is_finite() {
                Ok(next)
            } else {
                Err(EvaluationError::NonFiniteResult(
                    "computing closest-vector norm".into(),
                ))
            }
        })?;
        Ok(Min(Some(squared_norm.sqrt())))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("target", T::NAME)]
    }
}

crate::declare_variants! {
    default ClosestVectorProblem<i64> => "2^(num_basis_vectors * log(num_basis_vectors))" create ClosestVectorProblemI64CreateSpec,
    ClosestVectorProblem<f64> => "2^(num_basis_vectors * log(num_basis_vectors))" create ClosestVectorProblemF64CreateSpec,
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
        optimal_value: serde_json::json!(0.0),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/closest_vector_problem.rs"]
mod tests;
