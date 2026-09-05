//! Minimum Weight Decoding problem implementation.
//!
//! Given an n x m binary matrix H (parity-check matrix) and a binary syndrome
//! vector s of length n, find a binary vector x of length m minimizing the
//! Hamming weight |x| subject to Hx ≡ s (mod 2).

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumWeightDecoding",
        display_name: "Minimum Weight Decoding",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Algebraic,
        module_path: module_path!(),
        description: "Find minimum Hamming weight binary vector x such that Hx ≡ s (mod 2)",
        fields: MinimumWeightDecodingCreateSpec::FIELDS,
    }
}

/// Minimum Weight Decoding.
///
/// Given an n×m binary matrix H and a binary syndrome vector s, find a binary
/// vector x of length m that minimizes the Hamming weight |x| (number of 1s)
/// subject to Hx ≡ s (mod 2).
///
/// # Representation
///
/// Each of the m columns corresponds to a binary variable x_j ∈ {0, 1}.
/// The evaluator checks whether the GF(2) linear system Hx = s is satisfied,
/// and returns the Hamming weight of x if feasible.
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::MinimumWeightDecoding;
/// use problemreductions::{Problem, BruteForce};
///
/// let matrix = vec![
///     vec![true, false, true, true],
///     vec![false, true, true, false],
///     vec![true, true, false, true],
/// ];
/// let target = vec![true, true, false];
/// let problem = MinimumWeightDecoding::new(matrix, target);
/// let solver = BruteForce::new();
/// let witness = solver.solve(&problem).unwrap();
/// assert!(witness.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumWeightDecoding {
    /// The n×m binary parity-check matrix H.
    matrix: Vec<Vec<bool>>,
    /// The binary syndrome vector s of length n.
    target: Vec<bool>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct MinimumWeightDecodingCreateSpec {
    /// Binary parity-check matrix as JSON.
    #[create(codec = "json")]
    matrix: Vec<Vec<bool>>,
    /// Binary syndrome vector.
    #[create(name = "rhs", codec = "comma-separated")]
    target: Vec<bool>,
}

impl TryFrom<MinimumWeightDecodingCreateSpec> for MinimumWeightDecoding {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: MinimumWeightDecodingCreateSpec) -> Result<Self, Self::Error> {
        let first = spec
            .matrix
            .first()
            .ok_or("matrix must have at least one row")?;
        if first.is_empty() {
            return Err("matrix must have at least one column".into());
        }
        if spec.matrix.iter().any(|row| row.len() != first.len()) {
            return Err("all matrix rows must have the same length".into());
        }
        if spec.target.len() != spec.matrix.len() {
            return Err("rhs length must equal number of rows".into());
        }
        Ok(Self {
            matrix: spec.matrix,
            target: spec.target,
        })
    }
}

impl MinimumWeightDecoding {
    /// Create a new MinimumWeightDecoding instance.
    ///
    /// # Panics
    ///
    /// Panics if the matrix is empty, rows have inconsistent lengths,
    /// target length does not match the number of rows, or there are no columns.
    pub fn new(matrix: Vec<Vec<bool>>, target: Vec<bool>) -> Self {
        assert!(!matrix.is_empty(), "Matrix must have at least one row");
        let num_cols = matrix[0].len();
        assert!(num_cols > 0, "Matrix must have at least one column");
        for row in &matrix {
            assert_eq!(row.len(), num_cols, "All rows must have the same length");
        }
        assert_eq!(
            target.len(),
            matrix.len(),
            "Target length must equal number of rows"
        );
        Self { matrix, target }
    }

    /// Returns a reference to the parity-check matrix H.
    pub fn matrix(&self) -> &[Vec<bool>] {
        &self.matrix
    }

    /// Returns a reference to the syndrome vector s.
    pub fn target(&self) -> &[bool] {
        &self.target
    }

    /// Returns the number of rows of H.
    pub fn num_rows(&self) -> usize {
        self.matrix.len()
    }

    /// Returns the number of columns of H.
    pub fn num_cols(&self) -> usize {
        self.matrix[0].len()
    }
}

impl Problem for MinimumWeightDecoding {
    const NAME: &'static str = "MinimumWeightDecoding";
    type Solution = Vec<bool>;
    type Value = Min<i64>;

    crate::problem_parameters![("num_cols", num_cols), ("num_rows", num_rows),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        Ok({
            if config.len() != self.num_cols() {
                return Err(crate::traits::EvaluationError::InvalidConfiguration(
                    "decoded-word length does not match the matrix columns".into(),
                ));
            }
            // Check Hx ≡ s (mod 2) for each row
            for (i, row) in self.matrix.iter().enumerate() {
                let dot: usize = row
                    .iter()
                    .zip(config.iter())
                    .filter(|(&h, &x)| h && x)
                    .count();
                let syndrome_bit = dot % 2 == 1;
                if syndrome_bit != self.target[i] {
                    return Ok(Min(None));
                }
            }

            // Feasible: return Hamming weight
            let weight: usize = config.iter().filter(|&&v| v).count();
            Min(Some(i64::try_from(weight).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting Hamming weight to i64".into(),
                )
            })?))
        })
    }
}

impl crate::solvers::BruteForceProblem for MinimumWeightDecoding {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.num_cols()]
    }
}

crate::declare_variants! {
    default MinimumWeightDecoding => "2^(0.0494 * num_cols)" create MinimumWeightDecodingCreateSpec,
}

crate::register_brute_force! {
    MinimumWeightDecoding decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // H (3×4): [[1,0,1,1],[0,1,1,0],[1,1,0,1]], s = [1,1,0]
    // Config [0,0,1,0] → weight 1, Hx = [1,1,0] ≡ s → Min(1)
    let matrix = vec![
        vec![true, false, true, true],
        vec![false, true, true, false],
        vec![true, true, false, true],
    ];
    let target = vec![true, true, false];
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_weight_decoding",
        instance: Box::new(MinimumWeightDecoding::new(matrix, target)),
        optimal_config: serde_json::json!(vec![false, false, true, false]),
        optimal_value: serde_json::json!(1),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/minimum_weight_decoding.rs"]
mod tests;
