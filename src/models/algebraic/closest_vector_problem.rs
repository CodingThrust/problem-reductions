//! Closest Vector Problem (CVP).
//!
//! Given an integer lattice basis `B` and a target vector `t`, find integer
//! coefficients `x` minimizing the squared distance `||Bx - t||_2^2`.

use crate::registry::{ConstructionError, CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::traits::{EvaluationError, Problem};
use crate::types::Min;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;
use serde::{Deserialize, Serialize};

/// Target coordinate domains supported by [`ClosestVectorProblem`].
pub trait ClosestVectorTarget: Clone + std::fmt::Debug + 'static {
    /// Registered value of the `target` variant dimension.
    const NAME: &'static str;

    /// Validate one stored target coordinate.
    fn validate(&self, index: usize) -> Result<(), ConstructionError>;

    /// Represent a stored coordinate exactly for distance evaluation and solving.
    fn to_rational(&self) -> BigRational;
}

impl ClosestVectorTarget for i64 {
    const NAME: &'static str = "i64";

    fn validate(&self, _index: usize) -> Result<(), ConstructionError> {
        Ok(())
    }

    fn to_rational(&self) -> BigRational {
        BigRational::from_integer((*self).into())
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

    fn to_rational(&self) -> BigRational {
        BigRational::from_float(*self).expect("CVP target coordinate must be finite")
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
    pub fn target(&self) -> &[T] {
        &self.target
    }

    pub(crate) fn independent_rows(&self) -> Vec<usize> {
        independent_rows(&self.basis, self.ambient_dimension())
            .expect("CVP basis columns must be independent")
    }

    /// Exact squared distance from the lattice point to the stored target.
    pub fn squared_distance(&self, solution: &[i64]) -> Result<BigRational, EvaluationError> {
        if solution.len() != self.num_basis_vectors() {
            return Err(EvaluationError::InvalidConfiguration(format!(
                "expected {} closest-vector coefficients, got {}",
                self.num_basis_vectors(),
                solution.len()
            )));
        }
        Ok(self
            .target
            .iter()
            .enumerate()
            .map(|(row, target)| {
                let coordinate: BigInt = solution
                    .iter()
                    .zip(&self.basis)
                    .map(|(&coefficient, column)| BigInt::from(coefficient) * column[row])
                    .sum();
                let difference = BigRational::from_integer(coordinate) - target.to_rational();
                &difference * &difference
            })
            .sum())
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
    type Value = Min<BigRational>;

    crate::problem_parameters![
        ("ambient_dimension", ambient_dimension),
        ("num_basis_vectors", num_basis_vectors),
    ];

    fn evaluate(&self, solution: &Self::Solution) -> Result<Self::Value, EvaluationError> {
        Ok(Min(Some(self.squared_distance(solution)?)))
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
        optimal_value: serde_json::json!(BigRational::zero()),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/closest_vector_problem.rs"]
mod tests;

crate::decision_problem_meta!(ClosestVectorProblem<i64>, "DecisionClosestVectorProblem");

inventory::submit! {
    crate::registry::ProblemSchemaEntry {
        name: "DecisionClosestVectorProblem", display_name: "Decision ClosestVectorProblem", aliases: &[],
        dimensions: &[VariantDimension::new("target", "i64", &["i64"])], category: crate::registry::ProblemCategory::Algebraic, module_path: module_path!(),
        description: "Does a feasible solution meet the objective bound?",
        fields: &[
        crate::registry::FieldInfo { name: "basis", type_name: "Vec<Vec<i64>>", description: "Integer basis matrix as semicolon-separated column vectors." },
        crate::registry::FieldInfo { name: "target_vec", type_name: "Vec<i64>", description: "Target vector." },
        crate::registry::FieldInfo { name: "bound", type_name: "BigRational", description: "Decision objective bound" },
    ],
    }
}
crate::declare_variants! {
    default crate::models::decision::Decision<ClosestVectorProblem<i64>> => "2^(num_basis_vectors * log(num_basis_vectors))" create crate::models::decision::DecisionCreateSpec<ClosestVectorProblem<i64>>,
}
crate::register_decision_variant!(@edges ClosestVectorProblem<i64>, "DecisionClosestVectorProblem");

#[cfg(feature = "example-db")]
pub(crate) fn decision_canonical_rule_example_specs(
) -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "decision_closest_vector_problem_to_closest_vector_problem",
        build: || {
            let source = crate::models::decision::Decision::new(
                ClosestVectorProblem::new(vec![vec![2, 0], vec![1, 2]], vec![3_i64, 2])
                    .expect("canonical closest-vector instance must be valid"),
                BigRational::zero(),
            );
            let witness = serde_json::json!(vec![1, 1]);
            crate::example_db::specs::rule_example_with_witness::<_, ClosestVectorProblem<i64>>(
                source,
                crate::export::SolutionPair {
                    source_config: witness.clone(),
                    target_config: witness,
                },
            )
        },
    }]
}
