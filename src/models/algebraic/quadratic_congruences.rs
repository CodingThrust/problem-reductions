//! Quadratic Congruences problem implementation.
//!
//! Given non-negative integers a, b, c with b > 0 and a < b, determine whether
//! there exists a positive integer x with 1 ≤ x < c such that x² ≡ a (mod b).

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "QuadraticCongruences",
        display_name: "Quadratic Congruences",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Decide whether x² ≡ a (mod b) has a solution for x in {1, ..., c-1}",
        fields: &[
            FieldInfo { name: "a", type_name: "u64", description: "a" },
            FieldInfo { name: "b", type_name: "u64", description: "b" },
            FieldInfo { name: "c", type_name: "u64", description: "c" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "QuadraticCongruences",
        fields: &["c"],
    }
}

/// Quadratic Congruences problem.
///
/// Given non-negative integers a, b, c with b > 0 and a < b, determine whether
/// there exists a positive integer x with 1 ≤ x < c such that x² ≡ a (mod b).
///
/// The search space is x ∈ {1, …, c−1}.  The configuration variable `config[0]`
/// encodes x as `x = config[0] + 1`.
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::QuadraticCongruences;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // a=4, b=15, c=10: x=2 → 4 mod 15 = 4 ✓
/// let problem = QuadraticCongruences::new(4, 15, 10);
/// let solver = BruteForce::new();
/// let witness = solver.find_witness(&problem);
/// assert!(witness.is_some());
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct QuadraticCongruences {
    /// Quadratic residue target.
    a: u64,
    /// Modulus.
    b: u64,
    /// Search-space bound; x ranges over {1, ..., c-1}.
    c: u64,
}

impl QuadraticCongruences {
    fn validate_inputs(a: u64, b: u64, c: u64) -> Result<(), String> {
        if b == 0 {
            return Err("Modulus b must be positive".to_string());
        }
        if c == 0 {
            return Err("Bound c must be positive".to_string());
        }
        if a >= b {
            return Err(format!("Residue a ({a}) must be less than modulus b ({b})"));
        }
        Ok(())
    }

    /// Create a new QuadraticCongruences instance, returning an error instead of
    /// panicking when the inputs are invalid.
    pub fn try_new(a: u64, b: u64, c: u64) -> Result<Self, String> {
        Self::validate_inputs(a, b, c)?;
        Ok(Self { a, b, c })
    }

    /// Create a new QuadraticCongruences instance.
    ///
    /// # Panics
    ///
    /// Panics if `b == 0`, `c == 0`, or `a >= b`.
    pub fn new(a: u64, b: u64, c: u64) -> Self {
        Self::try_new(a, b, c).unwrap_or_else(|msg| panic!("{msg}"))
    }

    /// Get the quadratic residue target a.
    pub fn a(&self) -> u64 {
        self.a
    }

    /// Get the modulus b.
    pub fn b(&self) -> u64 {
        self.b
    }

    /// Get the search-space bound c.
    pub fn c(&self) -> u64 {
        self.c
    }
}

#[derive(Deserialize)]
struct QuadraticCongruencesData {
    a: u64,
    b: u64,
    c: u64,
}

impl<'de> Deserialize<'de> for QuadraticCongruences {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = QuadraticCongruencesData::deserialize(deserializer)?;
        Self::try_new(data.a, data.b, data.c).map_err(D::Error::custom)
    }
}

impl Problem for QuadraticCongruences {
    const NAME: &'static str = "QuadraticCongruences";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        if self.c <= 1 {
            // No x in {1, ..., c-1} exists.
            return vec![];
        }
        // config[0] ∈ {0, ..., c-2} maps to x = config[0] + 1 ∈ {1, ..., c-1}.
        vec![self.c as usize - 1]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        if self.c <= 1 {
            return Or(false);
        }
        if config.len() != 1 {
            return Or(false);
        }
        let x = (config[0] as u64) + 1; // 1-indexed
        let satisfies = ((x as u128) * (x as u128)) % (self.b as u128) == (self.a as u128);
        Or(satisfies)
    }
}

crate::declare_variants! {
    default QuadraticCongruences => "c",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "quadratic_congruences",
        instance: Box::new(QuadraticCongruences::new(4, 15, 10)),
        // x=2 (config[0]=1): 2²=4 ≡ 4 (mod 15) ✓
        optimal_config: vec![1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/quadratic_congruences.rs"]
mod tests;
