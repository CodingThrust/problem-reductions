//! QUBO (Quadratic Unconstrained Binary Optimization) problem implementation.
//!
//! QUBO minimizes a quadratic function over binary variables.

use crate::registry::{ConstructionError, CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::traits::Problem;
use crate::types::{Min, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "QUBO",
        display_name: "QUBO",
        aliases: &[],
        dimensions: &[VariantDimension::new("weight", "f64", &["f64"])],
        category: crate::registry::ProblemCategory::Algebraic,
        module_path: module_path!(),
        description: "Minimize quadratic unconstrained binary objective",
        fields: QuboCreateSpec::FIELDS,
    }
}

/// The QUBO (Quadratic Unconstrained Binary Optimization) problem.
///
/// Given n binary variables x_i ∈ {0, 1} and a matrix Q,
/// minimize the quadratic form:
///
/// f(x) = Σ_i Σ_j Q_ij * x_i * x_j = x^T Q x
///
/// The matrix Q is typically upper triangular, with diagonal elements
/// representing linear terms and off-diagonal elements representing
/// quadratic interactions.
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::QUBO;
/// use problemreductions::{Problem, BruteForce};
///
/// // Q matrix: minimize x0 - 2*x1 + x0*x1
/// // Q = [[1, 1], [0, -2]]
/// let problem = QUBO::from_matrix(vec![
///     vec![1.0, 1.0],
///     vec![0.0, -2.0],
/// ]).unwrap();
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem).unwrap();
///
/// // Optimal is x = [0, 1] with value -2
/// assert!(solutions.contains(&vec![false, true]));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QUBO<W = f64> {
    /// Number of variables.
    num_vars: usize,
    /// Q matrix stored as upper triangular (row-major).
    /// `Q[i][j]` for i <= j represents the coefficient of x_i * x_j
    matrix: Vec<Vec<W>>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct QuboCreateSpec {
    /// Q matrix; the number of variables is its row count.
    #[create(codec = "semicolon-separated")]
    matrix: Vec<Vec<f64>>,
}

impl TryFrom<QuboCreateSpec> for QUBO<f64> {
    type Error = ConstructionError;

    fn try_from(spec: QuboCreateSpec) -> Result<Self, Self::Error> {
        Self::from_matrix(spec.matrix)
    }
}

impl QUBO<f64> {
    /// Create a QUBO problem from a full matrix.
    ///
    /// The matrix should be square. Only the upper triangular part
    /// (including diagonal) is used.
    pub fn from_matrix(matrix: Vec<Vec<f64>>) -> Result<Self, ConstructionError> {
        let num_vars = matrix.len();
        if let Some((row, actual)) = matrix
            .iter()
            .enumerate()
            .find_map(|(row, values)| (values.len() != num_vars).then_some((row, values.len())))
        {
            return Err(ConstructionError::Conversion(format!(
                "QUBO matrix row {row} has length {actual}, expected {num_vars}"
            )));
        }
        if let Some((row, column)) = matrix.iter().enumerate().find_map(|(row, values)| {
            values
                .iter()
                .position(|value| !value.is_finite())
                .map(|column| (row, column))
        }) {
            return Err(ConstructionError::NonFiniteFloat(format!(
                "QUBO coefficient at ({row}, {column}) must be finite"
            )));
        }
        Ok(Self { num_vars, matrix })
    }

    /// Create a QUBO from linear and quadratic terms.
    ///
    /// # Arguments
    /// * `linear` - Linear coefficients (diagonal of Q)
    /// * `quadratic` - Quadratic coefficients as ((i, j), value) for i < j
    pub fn new(
        linear: Vec<f64>,
        quadratic: Vec<((usize, usize), f64)>,
    ) -> Result<Self, ConstructionError> {
        let num_vars = linear.len();
        let mut matrix = vec![vec![0.0; num_vars]; num_vars];

        // Set diagonal (linear terms)
        for (i, val) in linear.into_iter().enumerate() {
            matrix[i][i] = val;
        }

        // Set off-diagonal (quadratic terms)
        for ((i, j), val) in quadratic {
            if i >= num_vars || j >= num_vars {
                return Err(ConstructionError::Conversion(format!(
                    "QUBO quadratic index ({i}, {j}) is outside 0..{num_vars}"
                )));
            }
            if i < j {
                matrix[i][j] = val;
            } else {
                matrix[j][i] = val;
            }
        }

        Self::from_matrix(matrix)
    }
}

impl<W: Clone + Default> QUBO<W> {
    /// Get the number of variables.
    pub fn num_vars(&self) -> usize {
        self.num_vars
    }

    /// Get the Q matrix.
    pub fn matrix(&self) -> &[Vec<W>] {
        &self.matrix
    }

    /// Get a specific matrix element `Q[i][j]`.
    pub fn get(&self, i: usize, j: usize) -> Option<&W> {
        self.matrix.get(i).and_then(|row| row.get(j))
    }
}

impl<W> Problem for QUBO<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "QUBO";
    type Solution = Vec<bool>;
    type Value = Min<W::Sum>;

    crate::problem_parameters![("num_vars", num_vars),];

    fn evaluate(
        &self,
        solution: &Self::Solution,
    ) -> Result<Min<W::Sum>, crate::traits::EvaluationError> {
        if solution.len() != self.num_vars {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                format!(
                    "solution has {} variables, expected {}",
                    solution.len(),
                    self.num_vars
                ),
            ));
        }
        let mut value = W::Sum::zero();

        for i in 0..self.num_vars {
            if !solution[i] {
                continue;
            }

            for (j, &selected) in solution.iter().enumerate().skip(i) {
                if !selected {
                    continue;
                }

                if let Some(q_ij) = self.matrix.get(i).and_then(|row| row.get(j)) {
                    value = W::checked_add_to_sum(
                        value,
                        q_ij.to_sum(),
                        "summing selected QUBO coefficients",
                    )?;
                }
            }
        }

        Ok(Min(Some(value)))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }
}

impl<W> crate::solvers::BruteForceProblem for QUBO<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }
}

crate::declare_variants! {
    default QUBO<f64> => "2^num_vars" create QuboCreateSpec,
}

crate::register_brute_force! {
    QUBO<f64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "qubo",
        instance: Box::new(
            QUBO::from_matrix(vec![
                vec![-1.0, 2.0, 0.0],
                vec![0.0, -1.0, 2.0],
                vec![0.0, 0.0, -1.0],
            ])
            .unwrap(),
        ),
        optimal_config: serde_json::json!(vec![true, false, true]),
        optimal_value: serde_json::json!(-2.0),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/qubo.rs"]
mod tests;
