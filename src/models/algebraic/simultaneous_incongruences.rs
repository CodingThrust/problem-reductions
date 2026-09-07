//! Simultaneous Incongruences problem implementation.
//!
//! Given a list of pairs (aᵢ, bᵢ) with bᵢ > 0 and 1 ≤ aᵢ ≤ bᵢ, determine whether
//! there exists a non-negative integer x such that x ≢ aᵢ (mod bᵢ) for all i.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
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
        category: crate::registry::ProblemCategory::Algebraic,
        module_path: module_path!(),
        description: "Decide whether there exists x with x ≢ aᵢ (mod bᵢ) for all i",
        fields: &[
            FieldInfo {
                name: "pairs",
                type_name: "Vec<(i64, i64)>",
                description: "Pairs (aᵢ, bᵢ) with bᵢ > 0 and 1 ≤ aᵢ ≤ bᵢ",
            },
        ],
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
/// use problemreductions::{Problem, BruteForce};
///
/// // pairs: [(2,2),(1,3),(2,5),(3,7)] — lcm=210, x=5 is a solution
/// let problem = SimultaneousIncongruences::new(vec![(2,2),(1,3),(2,5),(3,7)]).unwrap();
/// let solver = BruteForce::new();
/// let witness = solver.solve(&problem).unwrap();
/// assert!(witness.is_some());
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct SimultaneousIncongruences {
    /// Incongruence pairs (aᵢ, bᵢ).
    pairs: Vec<(i64, i64)>,
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

impl SimultaneousIncongruences {
    fn validate_inputs(pairs: &[(i64, i64)]) -> Result<(), crate::registry::ConstructionError> {
        for (i, &(a, b)) in pairs.iter().enumerate() {
            if b <= 0 {
                return Err(format!("Modulus b at index {i} must be positive (got b={b})").into());
            }
            if a <= 0 {
                return Err(format!("Residue a at index {i} must be at least 1 (got a=0)").into());
            }
            if a > b {
                return Err(format!(
                    "Residue a ({a}) must not exceed modulus b ({b}) at index {i}"
                )
                .into());
            }
        }
        pairs.iter().try_fold(1i64, |lcm, &(_, modulus)| {
            (lcm / gcd(lcm, modulus))
                .checked_mul(modulus)
                .ok_or_else(|| "Least common multiple of moduli exceeds i64 range".to_string())
        })?;
        Ok(())
    }

    /// Create a new `SimultaneousIncongruences` instance, returning an error
    /// if any pair is invalid.
    pub fn new(pairs: Vec<(i64, i64)>) -> Result<Self, crate::registry::ConstructionError> {
        Self::validate_inputs(&pairs)?;
        Ok(Self { pairs })
    }

    /// Get the number of incongruence pairs.
    pub fn num_pairs(&self) -> usize {
        self.pairs.len()
    }

    /// Get the incongruence pairs.
    pub fn pairs(&self) -> &[(i64, i64)] {
        &self.pairs
    }

    /// Compute the LCM of all moduli.
    pub fn lcm_moduli(&self) -> i64 {
        self.pairs.iter().fold(1i64, |lcm, &(_, modulus)| {
            (lcm / gcd(lcm, modulus)) * modulus
        })
    }
}

#[derive(Deserialize)]
struct SimultaneousIncongruencesData {
    pairs: Vec<(i64, i64)>,
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
    type Solution = i64;
    type Value = Or;

    crate::problem_parameters![("num_pairs", num_pairs),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(&self, solution: &Self::Solution) -> Result<Or, crate::traits::EvaluationError> {
        Ok({
            // x is a solution iff x % bᵢ ≠ aᵢ % bᵢ for every pair.
            Or(self.pairs.iter().all(|&(a, b)| solution % b != a % b))
        })
    }
}

impl crate::solvers::BruteForceProblem for SimultaneousIncongruences {
    fn dimensions(&self) -> Vec<usize> {
        let lcm = usize::try_from(self.lcm_moduli()).expect("validated positive LCM fits usize");
        vec![lcm]
    }
}

crate::declare_variants! {
    default SimultaneousIncongruences => "num_pairs",
}

crate::register_brute_force! {
    SimultaneousIncongruences decode |_, indices: Vec<usize>| i64::try_from(indices[0]).expect("enumerated incongruence value fits i64"),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "simultaneous_incongruences",
        instance: Box::new(
            SimultaneousIncongruences::new(vec![(2, 2), (1, 3), (2, 5), (3, 7)]).unwrap(),
        ),
        // x=5: 5%2=1≠0(=2%2), 5%3=2≠1, 5%5=0≠2, 5%7=5≠3 ✓
        optimal_config: serde_json::json!(5),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/simultaneous_incongruences.rs"]
mod tests;
