//! QUBO (Quadratic Unconstrained Binary Optimization) problem implementation.
//!
//! QUBO minimizes a quadratic function over binary variables.

use crate::registry::{ConstructionError, CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::traits::Problem;
use crate::types::{Min, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use sprs::CsMat;
use std::collections::BTreeMap;

inventory::submit! {
    ProblemSchemaEntry {
        name: "QUBO",
        display_name: "QUBO",
        aliases: &[],
        dimensions: &[VariantDimension::new("weight", "i64", &["i64", "f64"])],
        category: crate::registry::ProblemCategory::Algebraic,
        module_path: module_path!(),
        description: "Minimize quadratic unconstrained binary objective",
        fields: QuboCreateSpec::<i64>::FIELDS,
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
/// `QUBO<i64>` is the default exact-integer variant. `QUBO<f64>` stores
/// finite floating-point coefficients. An explicit variant reduction converts
/// exactly representable integer coefficients to `f64`.
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
///     vec![1, 1],
///     vec![0, -2],
/// ]).unwrap();
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem).unwrap();
///
/// // Optimal is x = [0, 1] with value -2
/// assert!(solutions.contains(&vec![false, true]));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    try_from = "QuboData<W>",
    bound(deserialize = "W: WeightElement + Deserialize<'de>")
)]
pub struct QUBO<W = i64> {
    matrix: CsMat<W>,
}

#[derive(Deserialize)]
struct QuboData<W> {
    matrix: CsMat<W>,
}

impl<W: WeightElement> TryFrom<QuboData<W>> for QUBO<W> {
    type Error = ConstructionError;
    fn try_from(data: QuboData<W>) -> Result<Self, Self::Error> {
        Self::from_sparse(data.matrix)
    }
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct QuboCreateSpec<W> {
    /// Q matrix; the number of variables is its row count.
    #[create(codec = "semicolon-separated")]
    matrix: Vec<Vec<W>>,
}

impl<W: WeightElement> TryFrom<QuboCreateSpec<W>> for QUBO<W> {
    type Error = ConstructionError;

    fn try_from(spec: QuboCreateSpec<W>) -> Result<Self, Self::Error> {
        Self::from_matrix(spec.matrix)
    }
}

impl<W: WeightElement> QUBO<W> {
    /// Create a QUBO problem from a full matrix.
    ///
    /// The matrix should be square. Only the upper triangular part
    /// (including diagonal) is used.
    pub fn from_matrix(matrix: Vec<Vec<W>>) -> Result<Self, ConstructionError> {
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
        Self::from_rows(
            matrix
                .into_iter()
                .map(|row| row.into_iter().enumerate())
                .collect(),
        )
    }

    /// Create a QUBO from a square sparse matrix. Only its upper triangle is evaluated.
    pub fn from_sparse(matrix: CsMat<W>) -> Result<Self, ConstructionError> {
        if matrix.rows() != matrix.cols() {
            return Err(ConstructionError::Conversion(
                "QUBO matrix must be square".into(),
            ));
        }
        let matrix = matrix.into_csr();
        for (row, values) in matrix.outer_iterator().enumerate() {
            for (column, value) in values.iter() {
                value
                    .validate_element("QUBO coefficient")
                    .map_err(|error| match error {
                        ConstructionError::NonFiniteFloat(message) => {
                            ConstructionError::NonFiniteFloat(format!(
                                "{message} at ({row}, {column})"
                            ))
                        }
                        error => error,
                    })?;
            }
        }
        Ok(Self { matrix })
    }

    // Rows collect assignments and checked additions before compression. No library
    // duplicate summation may replace the rule's numeric operations.
    pub(crate) fn from_rows(
        rows: Vec<impl IntoIterator<Item = (usize, W)>>,
    ) -> Result<Self, ConstructionError> {
        let n = rows.len();
        let mut offsets = Vec::with_capacity(n + 1);
        let mut indices = Vec::new();
        let mut values = Vec::new();
        offsets.push(0);
        for row in rows {
            for (column, value) in row {
                if !value.to_sum().is_zero() {
                    indices.push(column);
                    values.push(value);
                }
            }
            offsets.push(values.len());
        }
        let matrix = CsMat::try_new((n, n), offsets, indices, values)
            .map_err(|(_, _, _, error)| ConstructionError::Conversion(error.to_string()))?;
        Self::from_sparse(matrix)
    }

    /// Create a QUBO from linear and quadratic terms.
    ///
    /// # Arguments
    /// * `linear` - Linear coefficients (diagonal of Q)
    /// * `quadratic` - Quadratic coefficients as ((i, j), value) for i < j
    pub fn new(
        linear: Vec<W>,
        quadratic: Vec<((usize, usize), W)>,
    ) -> Result<Self, ConstructionError> {
        let num_vars = linear.len();
        let mut matrix = vec![BTreeMap::new(); num_vars];

        // Set diagonal (linear terms)
        for (i, val) in linear.into_iter().enumerate() {
            matrix[i].insert(i, val);
        }

        // Set off-diagonal (quadratic terms)
        for ((i, j), val) in quadratic {
            if i >= num_vars || j >= num_vars {
                return Err(ConstructionError::Conversion(format!(
                    "QUBO quadratic index ({i}, {j}) is outside 0..{num_vars}"
                )));
            }
            if i < j {
                matrix[i].insert(j, val);
            } else {
                matrix[j].insert(i, val);
            }
        }

        Self::from_rows(matrix)
    }
}

impl<W> QUBO<W> {
    /// Get the number of variables.
    pub fn num_vars(&self) -> usize {
        self.matrix.rows()
    }

    /// Get the Q matrix.
    pub fn matrix(&self) -> &CsMat<W> {
        &self.matrix
    }

    /// Get a coefficient, returning zero for an unstored entry and None outside the matrix.
    pub fn get(&self, i: usize, j: usize) -> Option<W>
    where
        W: Clone + Zero,
    {
        (i < self.num_vars() && j < self.num_vars())
            .then(|| self.matrix.get(i, j).cloned().unwrap_or_else(W::zero))
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
        if solution.len() != self.num_vars() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                format!(
                    "solution has {} variables, expected {}",
                    solution.len(),
                    self.matrix.rows()
                ),
            ));
        }
        let mut value = W::Sum::zero();

        for (i, row) in self.matrix.outer_iterator().enumerate() {
            if !solution[i] {
                continue;
            }
            for (j, coefficient) in row.iter() {
                if j >= i && solution[j] {
                    value = W::checked_add_to_sum(
                        value,
                        coefficient.to_sum(),
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
    fn num_variables(&self) -> Result<usize, crate::solvers::SolveError> {
        Ok(self.num_vars())
    }

    fn dimension(&self, _variable: usize) -> Result<usize, crate::solvers::SolveError> {
        Ok(2usize)
    }
}

crate::declare_variants! {
    default QUBO<i64> => "2^num_vars" create QuboCreateSpec<i64>,
    QUBO<f64> => "2^num_vars" create QuboCreateSpec<f64>,
}

crate::register_brute_force! {
    QUBO<i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    QUBO<f64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "qubo",
        instance: Box::new(
            QUBO::from_matrix(vec![vec![-1, 2, 0], vec![0, -1, 2], vec![0, 0, -1]]).unwrap(),
        ),
        optimal_config: serde_json::json!(vec![true, false, true]),
        optimal_value: serde_json::json!(-2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/qubo.rs"]
mod tests;
