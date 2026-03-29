//! Cosine Product Integration problem implementation.
//!
//! Given integer frequencies `a_1, ..., a_n`, determine whether a sign
//! assignment `ε ∈ {-1, +1}^n` exists with `∑ εᵢ aᵢ = 0`.
//!
//! This is equivalent to asking whether
//! `∫₀²π ∏ᵢ cos(aᵢ θ) dθ ≠ 0` (Garey & Johnson A7 AN14).
//! The integral is nonzero exactly when such a balanced sign assignment
//! exists, so the G&J question "does the integral equal zero?" is the
//! complement of this satisfaction problem.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "CosineProductIntegration",
        display_name: "Cosine Product Integration",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Decide whether a balanced sign assignment exists for a sequence of integer frequencies",
        fields: &[
            FieldInfo {
                name: "coefficients",
                type_name: "Vec<i64>",
                description: "Integer cosine frequencies",
            },
        ],
    }
}

/// The Cosine Product Integration problem.
///
/// Given integer coefficients `a_1, ..., a_n`, determine whether there
/// exists a sign assignment `ε ∈ {-1, +1}^n` with `∑ εᵢ aᵢ = 0`.
///
/// # Representation
///
/// Each variable chooses a sign: `0` means `+aᵢ`, `1` means `−aᵢ`.
/// A configuration is satisfying when the resulting signed sum is zero.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::CosineProductIntegration;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // coefficients [2, 3, 5]: sign assignment (+2, +3, -5) = 0
/// let problem = CosineProductIntegration::new(vec![2, 3, 5]);
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosineProductIntegration {
    coefficients: Vec<i64>,
}

impl CosineProductIntegration {
    /// Create a new CosineProductIntegration instance.
    ///
    /// # Panics
    ///
    /// Panics if `coefficients` is empty.
    pub fn new(coefficients: Vec<i64>) -> Self {
        assert!(
            !coefficients.is_empty(),
            "CosineProductIntegration requires at least one coefficient"
        );
        Self { coefficients }
    }

    /// Returns the cosine coefficients.
    pub fn coefficients(&self) -> &[i64] {
        &self.coefficients
    }

    /// Returns the number of coefficients.
    pub fn num_coefficients(&self) -> usize {
        self.coefficients.len()
    }
}

impl Problem for CosineProductIntegration {
    const NAME: &'static str = "CosineProductIntegration";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_coefficients()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or({
            if config.len() != self.num_coefficients() {
                return crate::types::Or(false);
            }
            if config.iter().any(|&v| v >= 2) {
                return crate::types::Or(false);
            }
            let signed_sum: i128 = self
                .coefficients
                .iter()
                .zip(config.iter())
                .map(|(&a, &bit)| {
                    let val = a as i128;
                    if bit == 0 {
                        val
                    } else {
                        -val
                    }
                })
                .sum();
            signed_sum == 0
        })
    }
}

crate::declare_variants! {
    default CosineProductIntegration => "2^(num_coefficients / 2)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "cosine_product_integration",
        instance: Box::new(CosineProductIntegration::new(vec![2, 3, 5])),
        optimal_config: vec![0, 0, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/cosine_product_integration.rs"]
mod tests;
