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
#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "QuboInputData<W>")]
#[serde(bound(deserialize = "W: WeightElement + Deserialize<'de>"))]
pub struct QUBO<W = i64> {
    /// Number of variables.
    num_vars: usize,
    /// Nonzero upper-triangular coefficients `(i, j, Q[i][j])` with `i <= j`,
    /// sorted by `(i, j)`; absent entries are zero.
    entries: Vec<(usize, usize, W)>,
    /// Zero coefficient returned by [`QUBO::get`] for absent entries.
    zero: W,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct QuboData<W> {
    num_vars: usize,
    entries: Vec<(usize, usize, W)>,
}

// Keep parse-error format guidance separate from constructor validation errors.
#[derive(Deserialize)]
#[serde(transparent)]
struct QuboInputData<W> {
    #[serde(
        deserialize_with = "deserialize_qubo_data",
        bound(deserialize = "W: Deserialize<'de>")
    )]
    data: QuboData<W>,
}

fn deserialize_qubo_data<'de, W: Deserialize<'de>, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<QuboData<W>, D::Error> {
    QuboData::deserialize(deserializer).map_err(|error| {
        serde::de::Error::custom(format!(
            "{error}; expected QUBO format: num_vars and sparse entries [row, col, value] with row <= col"
        ))
    })
}

impl<W: WeightElement> TryFrom<QuboInputData<W>> for QUBO<W> {
    type Error = ConstructionError;

    fn try_from(input: QuboInputData<W>) -> Result<Self, Self::Error> {
        Self::try_from(input.data)
    }
}

impl<W: WeightElement + Serialize> Serialize for QUBO<W> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        QuboData {
            num_vars: self.num_vars,
            entries: self
                .entries
                .iter()
                .map(|(i, j, value)| (*i, *j, value))
                .collect(),
        }
        .serialize(serializer)
    }
}

impl<W: WeightElement> TryFrom<QuboData<W>> for QUBO<W> {
    type Error = ConstructionError;

    fn try_from(data: QuboData<W>) -> Result<Self, Self::Error> {
        Self::from_entries(data.num_vars, data.entries)
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
        for (row, values) in matrix.iter().enumerate() {
            for (column, value) in values.iter().enumerate() {
                value.validate_element(&format!("QUBO coefficient at ({row}, {column})"))?;
            }
        }
        let entries = matrix
            .into_iter()
            .enumerate()
            .flat_map(|(row, values)| {
                values
                    .into_iter()
                    .enumerate()
                    .skip(row)
                    .map(move |(column, value)| (row, column, value))
            })
            .collect();
        Self::from_entries(num_vars, entries)
    }

    /// Create a QUBO from sparse coefficients `(i, j, Q[i][j])`.
    ///
    /// Indices must be in `0..num_vars` with `i <= j`, and each pair may
    /// appear at most once. Zero coefficients are dropped.
    pub fn from_entries(
        num_vars: usize,
        mut entries: Vec<(usize, usize, W)>,
    ) -> Result<Self, ConstructionError> {
        for (row, column, value) in &entries {
            if *row >= num_vars || *column >= num_vars {
                return Err(ConstructionError::Conversion(format!(
                    "QUBO index ({row}, {column}) is outside 0..{num_vars}"
                )));
            }
            if row > column {
                return Err(ConstructionError::Conversion(format!(
                    "QUBO index ({row}, {column}) is below the diagonal; use ({column}, {row}) instead"
                )));
            }
            value.validate_element(&format!("QUBO coefficient at ({row}, {column})"))?;
        }
        entries.sort_by_key(|&(row, column, _)| (row, column));
        if let Some(pair) = entries
            .windows(2)
            .find(|pair| (pair[0].0, pair[0].1) == (pair[1].0, pair[1].1))
        {
            return Err(ConstructionError::Conversion(format!(
                "duplicate QUBO index ({}, {})",
                pair[0].0, pair[0].1
            )));
        }
        Ok(Self {
            num_vars,
            entries: entries
                .into_iter()
                .filter(|(_, _, value)| !value.to_sum().is_zero())
                .collect(),
            zero: W::default(),
        })
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
        let mut matrix = vec![vec![W::default(); num_vars]; num_vars];

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

impl<W: Clone> QUBO<W> {
    /// Get the number of variables.
    pub fn num_vars(&self) -> usize {
        self.num_vars
    }

    /// Number of nonzero off-diagonal coefficients.
    pub fn num_quadratic_terms(&self) -> usize {
        self.entries.iter().filter(|(i, j, _)| i != j).count()
    }

    /// Nonzero upper-triangular coefficients `(i, j, Q[i][j])`, sorted by `(i, j)`.
    pub fn entries(&self) -> &[(usize, usize, W)] {
        &self.entries
    }

    /// Dense upper-triangular copy of Q. Allocates `num_vars^2` elements.
    pub fn matrix(&self) -> Vec<Vec<W>> {
        let mut matrix = vec![vec![self.zero.clone(); self.num_vars]; self.num_vars];
        for (i, j, value) in &self.entries {
            matrix[*i][*j] = value.clone();
        }
        matrix
    }

    /// Get a specific matrix element `Q[i][j]`; entries below the diagonal are zero.
    pub fn get(&self, i: usize, j: usize) -> Option<&W> {
        if i >= self.num_vars || j >= self.num_vars {
            return None;
        }
        Some(
            self.entries
                .binary_search_by_key(&(i, j), |&(row, column, _)| (row, column))
                .map_or(&self.zero, |index| &self.entries[index].2),
        )
    }
}

impl<W> Problem for QUBO<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "QUBO";
    type Solution = Vec<bool>;
    type Value = Min<W::Sum>;

    crate::problem_parameters![
        ("num_vars", num_vars),
        ("num_quadratic_terms", num_quadratic_terms),
    ];

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
        for (i, j, coefficient) in &self.entries {
            if solution[*i] && solution[*j] {
                value = W::checked_add_to_sum(
                    value,
                    coefficient.to_sum(),
                    "summing selected QUBO coefficients",
                )?;
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

crate::decision_problem_meta!(QUBO<i64>, "DecisionQUBO");
crate::register_decision_variant!(
    QUBO<i64>, "DecisionQUBO", "2^num_vars", &[],
    "Does a feasible solution have objective value <= the bound?",
    category: crate::registry::ProblemCategory::Algebraic,
    dims: [VariantDimension::new("weight", "i64", &["i64"])],
    fields: [
        crate::registry::FieldInfo { name: "matrix", type_name: "Vec<Vec<W>>", description: "Q matrix; the number of variables is its row count." },
        crate::registry::FieldInfo { name: "bound", type_name: "i64", description: "Accept objective values <= this bound" },
    ],
    decode: |_, indices: Vec<usize>| crate::config::config_to_bits(&indices)
);

#[cfg(feature = "example-db")]
pub(crate) fn decision_canonical_rule_example_specs(
) -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "decision_qubo_to_qubo",
        build: || {
            let source = crate::models::decision::Decision::new(
                QUBO::from_matrix(vec![vec![-1, 2, 0], vec![0, -1, 2], vec![0, 0, -1]]).unwrap(),
                -2,
            );
            let witness = serde_json::json!(vec![true, false, true]);
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                source,
                crate::export::SolutionPair {
                    source_config: witness.clone(),
                    target_config: witness,
                },
            )
        },
    }]
}
