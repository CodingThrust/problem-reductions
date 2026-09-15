//! Closest Vector Problem (CVP).
//!
//! Given a lattice basis `B` and a target vector `t`, find integer
//! coefficients `x` minimizing the squared distance `||Bx - t||_2^2`.

use crate::registry::{ConstructionError, CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::traits::{EvaluationError, Problem};
use crate::types::Min;
use num_rational::BigRational;
use num_traits::Zero;
use serde::{Deserialize, Serialize};

macro_rules! cvp_create_spec {
    ($name:ident, $target:ty) => {
        #[derive(Debug, Deserialize, crate::CreateSpec)]
        struct $name {
            /// Basis matrix as semicolon-separated column vectors.
            #[create(codec = "semicolon-separated")]
            basis: Vec<Vec<$target>>,
            /// Target vector.
            #[create(name = "target_vec", codec = "comma-separated")]
            target: Vec<$target>,
        }

        impl TryFrom<$name> for ClosestVectorProblem<$target> {
            type Error = ConstructionError;

            fn try_from(spec: $name) -> Result<Self, Self::Error> {
                ClosestVectorProblem::<$target>::new(spec.basis, spec.target)
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
        dimensions: &[VariantDimension::new("coefficient", "i64", &["i64", "f64"])],
        category: crate::registry::ProblemCategory::Algebraic,
        module_path: module_path!(),
        description: "Find the closest point in a lattice to a target vector",
        fields: ClosestVectorProblemI64CreateSpec::FIELDS,
    }
}

/// Euclidean Closest Vector Problem with integer or floating-point coordinates.
#[derive(Debug, Clone, Serialize)]
pub struct ClosestVectorProblem<T = i64> {
    /// Basis matrix stored as column vectors.
    basis: Vec<Vec<T>>,
    /// Target vector in the ambient space.
    target: Vec<T>,
}

impl<T> ClosestVectorProblem<T> {
    /// Number of basis vectors.
    pub fn num_basis_vectors(&self) -> usize {
        self.basis.len()
    }
    /// Dimension of the ambient space.
    pub fn ambient_dimension(&self) -> usize {
        self.target.len()
    }
    /// Basis columns in the variant's numeric domain.
    pub fn basis(&self) -> &[Vec<T>] {
        &self.basis
    }
    /// Target coordinates.
    pub fn target(&self) -> &[T] {
        &self.target
    }

    fn validate_dimensions(&self) -> Result<(), ConstructionError> {
        for (index, column) in self.basis.iter().enumerate() {
            if column.len() != self.ambient_dimension() {
                return Err(ConstructionError::Conversion(format!(
                    "basis vector {index} has length {}, expected {}",
                    column.len(),
                    self.ambient_dimension()
                )));
            }
        }
        if self.num_basis_vectors() > self.ambient_dimension() {
            return Err(ConstructionError::Conversion(
                "more basis vectors than ambient dimensions".into(),
            ));
        }
        Ok(())
    }

    fn validate_solution(&self, solution: &[i64]) -> Result<(), EvaluationError> {
        if solution.len() != self.num_basis_vectors() {
            return Err(EvaluationError::InvalidConfiguration(format!(
                "expected {} closest-vector coefficients, got {}",
                self.num_basis_vectors(),
                solution.len()
            )));
        }
        Ok(())
    }
}

macro_rules! cvp_numeric_impl {
    ($numeric:ty, $name:literal, $rational:expr) => {
        impl ClosestVectorProblem<$numeric> {
            /// Construct a CVP instance with full-column-rank basis.
            pub fn new(
                basis: Vec<Vec<$numeric>>,
                target: Vec<$numeric>,
            ) -> Result<Self, ConstructionError> {
                use crate::types::WeightElement;
                let instance = Self { basis, target };
                instance.validate_dimensions()?;
                for value in instance.basis.iter().flatten().chain(&instance.target) {
                    value.validate_element("CVP coordinate")?;
                }
                let matrix = (0..instance.ambient_dimension())
                    .map(|row| {
                        instance
                            .basis
                            .iter()
                            .map(|column| ($rational)(column[row]))
                            .collect()
                    })
                    .collect();
                if independent_rows(matrix, instance.num_basis_vectors()).is_none() {
                    return Err(ConstructionError::Conversion(
                        "closest-vector basis columns must be linearly independent".into(),
                    ));
                }
                Ok(instance)
            }
        }

        impl<'de> Deserialize<'de> for ClosestVectorProblem<$numeric> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                #[derive(Deserialize)]
                struct Raw {
                    basis: Vec<Vec<$numeric>>,
                    target: Vec<$numeric>,
                }
                let raw = Raw::deserialize(deserializer)?;
                Self::new(raw.basis, raw.target).map_err(serde::de::Error::custom)
            }
        }

        impl Problem for ClosestVectorProblem<$numeric> {
            const NAME: &'static str = "ClosestVectorProblem";
            type Solution = Vec<i64>;
            type Value = Min<$numeric>;
            crate::problem_parameters![
                ("ambient_dimension", ambient_dimension),
                ("num_basis_vectors", num_basis_vectors),
            ];
            fn evaluate(&self, solution: &Self::Solution) -> Result<Self::Value, EvaluationError> {
                Ok(Min(Some(self.squared_distance(solution)?)))
            }
            fn variant() -> Vec<(&'static str, &'static str)> {
                vec![("coefficient", $name)]
            }
        }
    };
}

cvp_numeric_impl!(i64, "i64", |value: i64| BigRational::from_integer(
    value.into()
));
cvp_numeric_impl!(f64, "f64", |value: f64| BigRational::from_float(value)
    .expect("validated finite coordinate"));

impl ClosestVectorProblem<i64> {
    pub(crate) fn independent_rows(&self) -> Vec<usize> {
        let matrix = (0..self.ambient_dimension())
            .map(|row| {
                self.basis
                    .iter()
                    .map(|column| BigRational::from_integer(column[row].into()))
                    .collect()
            })
            .collect();
        independent_rows(matrix, self.num_basis_vectors()).expect("validated independent columns")
    }

    /// Squared distance using checked integer arithmetic.
    pub fn squared_distance(&self, solution: &[i64]) -> Result<i64, EvaluationError> {
        self.validate_solution(solution)?;
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
        Ok(squared)
    }
}

impl ClosestVectorProblem<f64> {
    /// Squared distance in row/column order with ordinary floating-point rounding.
    pub fn squared_distance(&self, solution: &[i64]) -> Result<f64, EvaluationError> {
        self.validate_solution(solution)?;
        let finite = |value: f64| {
            value.is_finite().then_some(value).ok_or_else(|| {
                EvaluationError::NonFiniteResult("computing CVP squared distance".into())
            })
        };
        let mut squared = 0.0;
        for (row, &target) in self.target.iter().enumerate() {
            let mut coordinate = 0.0;
            for (&coefficient, column) in solution.iter().zip(&self.basis) {
                coordinate = finite(coordinate + finite(coefficient as f64 * column[row])?)?;
            }
            let difference = finite(coordinate - target)?;
            squared = finite(squared + finite(difference * difference)?)?;
        }
        Ok(squared)
    }
}

fn independent_rows(mut matrix: Vec<Vec<BigRational>>, n: usize) -> Option<Vec<usize>> {
    let ambient_dimension = matrix.len();
    let mut indices = (0..ambient_dimension).collect::<Vec<_>>();
    for column in 0..n {
        let pivot_row = (column..ambient_dimension).find(|&row| !matrix[row][column].is_zero())?;
        matrix.swap(column, pivot_row);
        indices.swap(column, pivot_row);
        let (pivots, remaining) = matrix.split_at_mut(column + 1);
        let pivot = &pivots[column];
        for row in remaining {
            let ratio = &row[column] / &pivot[column];
            for (entry, value) in row.iter_mut().zip(pivot).skip(column + 1) {
                *entry -= &ratio * value;
            }
        }
    }
    indices.truncate(n);
    Some(indices)
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
            ClosestVectorProblem::<i64>::new(vec![vec![2, 0], vec![1, 2]], vec![3_i64, 2])
                .expect("canonical closest-vector instance must be valid"),
        ),
        optimal_config: serde_json::json!(vec![1, 1]),
        optimal_value: serde_json::json!(0),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/closest_vector_problem.rs"]
mod tests;

crate::decision_problem_meta!(ClosestVectorProblem<i64>, "DecisionClosestVectorProblem");
crate::decision_problem_meta!(ClosestVectorProblem<f64>, "DecisionClosestVectorProblem");

inventory::submit! {
    crate::registry::ProblemSchemaEntry {
        name: "DecisionClosestVectorProblem", display_name: "Decision ClosestVectorProblem", aliases: &[],
        dimensions: &[VariantDimension::new("coefficient", "i64", &["i64", "f64"])], category: crate::registry::ProblemCategory::Algebraic, module_path: module_path!(),
        description: "Does a feasible solution meet the objective bound?",
        fields: &[
        crate::registry::FieldInfo { name: "basis", type_name: "Vec<Vec<i64>>", description: "Basis matrix as semicolon-separated column vectors." },
        crate::registry::FieldInfo { name: "target_vec", type_name: "Vec<i64>", description: "Target vector." },
        crate::registry::FieldInfo { name: "bound", type_name: "i64", description: "Decision objective bound" },
    ],
    }
}
crate::declare_variants! {
    default crate::models::decision::Decision<ClosestVectorProblem<i64>> => "2^(num_basis_vectors * log(num_basis_vectors))" create crate::models::decision::DecisionCreateSpec<ClosestVectorProblem<i64>>,
    crate::models::decision::Decision<ClosestVectorProblem<f64>> => "2^(num_basis_vectors * log(num_basis_vectors))" create crate::models::decision::DecisionCreateSpec<ClosestVectorProblem<f64>>,
}
crate::register_decision_variant!(@edges ClosestVectorProblem<i64>, "DecisionClosestVectorProblem");
crate::register_decision_variant!(@edges ClosestVectorProblem<f64>, "DecisionClosestVectorProblem");

#[cfg(feature = "example-db")]
pub(crate) fn decision_canonical_rule_example_specs(
) -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![
        crate::example_db::specs::RuleExampleSpec {
            id: "decision_closest_vector_problem_to_closest_vector_problem",
            build: || {
                let source = crate::models::decision::Decision::new(
                    ClosestVectorProblem::<i64>::new(vec![vec![2, 0], vec![1, 2]], vec![3_i64, 2])
                        .expect("canonical closest-vector instance must be valid"),
                    0,
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
        },
        crate::example_db::specs::RuleExampleSpec {
            id: "decision_closest_vector_problem_float_to_closest_vector_problem",
            build: || {
                let source = crate::models::decision::Decision::new(
                    ClosestVectorProblem::<f64>::new(
                        vec![vec![1.0, 0.0], vec![0.5, 0.8]],
                        vec![1.6, 0.9],
                    )
                    .expect("canonical oblique-grid instance must be valid"),
                    0.03,
                );
                let witness = serde_json::json!(vec![1, 1]);
                crate::example_db::specs::rule_example_with_witness::<_, ClosestVectorProblem<f64>>(
                    source,
                    crate::export::SolutionPair {
                        source_config: witness.clone(),
                        target_config: witness,
                    },
                )
            },
        },
    ]
}
