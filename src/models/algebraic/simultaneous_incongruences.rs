//! Simultaneous Incongruences problem implementation.
//!
//! Given a list of pairs (aᵢ, bᵢ) with bᵢ > 0 and 1 ≤ aᵢ ≤ bᵢ, determine whether
//! there exists a non-negative integer x such that x ≢ aᵢ (mod bᵢ) for all i.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SimultaneousIncongruences",
        display_name: "Simultaneous Incongruences",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Decide whether there exists x with x ≢ aᵢ (mod bᵢ) for all i",
        fields: &[
            FieldInfo {
                name: "pairs",
                type_name: "Vec<(u64, u64)>",
                description: "Pairs (aᵢ, bᵢ) with bᵢ > 0 and 1 ≤ aᵢ ≤ bᵢ",
            },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "SimultaneousIncongruences",
        fields: &["num_pairs"],
    }
}

/// Simultaneous Incongruences problem.
///
/// Given a list of pairs (aᵢ, bᵢ) with bᵢ > 0 and 1 ≤ aᵢ ≤ bᵢ, determine whether
/// there exists a non-negative integer x such that x ≢ aᵢ (mod bᵢ) for all i simultaneously.
///
/// The search space is x ∈ {0, …, L−1} where L = lcm(b₁, …, bₙ) (one full period).
/// `config[0]` encodes x directly.
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::SimultaneousIncongruences;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // pairs: [(2,2),(1,3),(2,5),(3,7)] — lcm=210, x=5 is a solution
/// let problem = SimultaneousIncongruences::new(vec![(2,2),(1,3),(2,5),(3,7)]).unwrap();
/// let solver = BruteForce::new();
/// let witness = solver.find_witness(&problem);
/// assert!(witness.is_some());
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct SimultaneousIncongruences {
    /// Incongruence pairs (aᵢ, bᵢ).
    pairs: Vec<(u64, u64)>,
}

/// Maximum lcm value we will compute in full; if the lcm exceeds this cap we
/// return this value to keep the brute-force search space manageable.
pub(crate) const MAX_LCM: u128 = 1_000_000;

fn lcm128(a: u128, b: u128) -> u128 {
    if a == 0 || b == 0 {
        return 0;
    }
    let g = gcd128(a, b);
    // Use saturating arithmetic to avoid overflow; cap at MAX_LCM.
    (a / g).saturating_mul(b).min(MAX_LCM)
}

fn gcd128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

impl SimultaneousIncongruences {
    fn validate_inputs(pairs: &[(u64, u64)]) -> Result<(), String> {
        for (i, &(a, b)) in pairs.iter().enumerate() {
            if b == 0 {
                return Err(format!("Modulus b at index {i} must be positive (got b=0)"));
            }
            if a == 0 {
                return Err(format!(
                    "Residue a at index {i} must be at least 1 (got a=0)"
                ));
            }
            if a > b {
                return Err(format!(
                    "Residue a ({a}) must not exceed modulus b ({b}) at index {i}"
                ));
            }
        }
        Ok(())
    }

    /// Create a new `SimultaneousIncongruences` instance, returning an error
    /// if any pair is invalid.
    pub fn new(pairs: Vec<(u64, u64)>) -> Result<Self, String> {
        Self::validate_inputs(&pairs)?;
        Ok(Self { pairs })
    }

    /// Get the number of incongruence pairs.
    pub fn num_pairs(&self) -> usize {
        self.pairs.len()
    }

    /// Get the incongruence pairs.
    pub fn pairs(&self) -> &[(u64, u64)] {
        &self.pairs
    }

    /// Compute the LCM of all moduli (capped at `MAX_LCM`).
    pub fn lcm_moduli(&self) -> u64 {
        if self.pairs.is_empty() {
            return 1;
        }
        let lcm = self
            .pairs
            .iter()
            .fold(1u128, |acc, &(_, b)| lcm128(acc, b as u128));
        lcm as u64
    }
}

#[derive(Deserialize)]
struct SimultaneousIncongruencesData {
    pairs: Vec<(u64, u64)>,
}

impl<'de> Deserialize<'de> for SimultaneousIncongruences {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = SimultaneousIncongruencesData::deserialize(deserializer)?;
        Self::new(data.pairs).map_err(D::Error::custom)
    }
}

impl Problem for SimultaneousIncongruences {
    const NAME: &'static str = "SimultaneousIncongruences";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let lcm = self.lcm_moduli() as usize;
        vec![lcm]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        if config.len() != 1 {
            return Or(false);
        }
        let x = config[0] as u64;
        // x is a solution iff x % bᵢ ≠ aᵢ % bᵢ for every pair.
        Or(self.pairs.iter().all(|&(a, b)| x % b != a % b))
    }
}

crate::declare_variants! {
    default SimultaneousIncongruences => "num_pairs",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "simultaneous_incongruences",
        instance: Box::new(
            SimultaneousIncongruences::new(vec![(2, 2), (1, 3), (2, 5), (3, 7)]).unwrap(),
        ),
        // x=5: 5%2=1≠0(=2%2), 5%3=2≠1, 5%5=0≠2, 5%7=5≠3 ✓
        optimal_config: vec![5],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/simultaneous_incongruences.rs"]
mod tests;
