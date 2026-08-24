//! Integer Factoring problem implementation.
//!
//! The Factoring problem represents integer factorization as a computational problem.
//! Given a number N, find two factors (a, b) such that a * b = N.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Or;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "Factoring",
        display_name: "Factoring",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Factor a composite integer into two factors",
        fields: &[
            FieldInfo { name: "m", type_name: "usize", description: "Bits for first factor" },
            FieldInfo { name: "n", type_name: "usize", description: "Bits for second factor" },
            FieldInfo { name: "target", type_name: "BigUint", description: "Number to factor" },
        ],
    }
}

/// The Integer Factoring problem.
///
/// Given a number to factor, find two integers that multiply to give
/// the target number. Variables represent the bits of the two factors.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::Factoring;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Factor 6 with 2-bit factors (allowing factors 0-3)
/// let problem = Factoring::new(2, 2, 6);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem).unwrap();
///
/// // Should find: 2*3=6 or 3*2=6
/// for sol in &solutions {
///     let (a, b) = problem.read_factors(sol);
///     assert_eq!(a * b, 6u32.into());
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Factoring {
    /// Number of bits for the first factor.
    m: usize,
    /// Number of bits for the second factor.
    n: usize,
    /// The number to factor.
    #[serde(with = "super::biguint_serde::decimal_biguint")]
    target: BigUint,
}

impl Factoring {
    /// Create a new Factoring problem.
    ///
    /// # Arguments
    /// * `m` - Number of bits for the first factor
    /// * `n` - Number of bits for the second factor
    /// * `target` - The number to factor
    pub fn new<T: ToBigUint>(m: usize, n: usize, target: T) -> Self {
        let target = target
            .to_biguint()
            .expect("Factoring target must be nonnegative");
        Self { m, n, target }
    }

    /// Get the number of bits for the first factor.
    pub fn m(&self) -> usize {
        self.m
    }

    /// Get the number of bits for the second factor.
    pub fn n(&self) -> usize {
        self.n
    }

    /// Get the number of bits for the first factor (alias for `m()`).
    pub fn num_bits_first(&self) -> usize {
        self.m()
    }

    /// Get the number of bits for the second factor (alias for `n()`).
    pub fn num_bits_second(&self) -> usize {
        self.n()
    }

    /// Get the target number to factor.
    pub fn target(&self) -> &BigUint {
        &self.target
    }

    /// Number of bits needed to represent the target (`1` for zero).
    pub fn target_bits(&self) -> usize {
        usize::try_from(self.target.bits().max(1)).expect("BigUint bit length fits usize")
    }

    /// Read the two factors from a configuration.
    ///
    /// The first `m` bits represent the first factor,
    /// the next `n` bits represent the second factor.
    pub fn read_factors(&self, config: &[usize]) -> (BigUint, BigUint) {
        let a = bits_to_biguint(&config[..self.m]);
        let b = bits_to_biguint(&config[self.m..self.m + self.n]);
        (a, b)
    }

    /// Check if a configuration is a valid factorization.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.is_valid_factorization(config)
    }

    /// Check if the configuration is a valid factorization.
    pub fn is_valid_factorization(&self, config: &[usize]) -> bool {
        if config.len() != self.m + self.n || config.iter().any(|&bit| bit >= 2) {
            return false;
        }
        let (a, b) = self.read_factors(config);
        a * b == self.target
    }
}

/// Convert a bit vector (little-endian) to an integer.
fn bits_to_biguint(bits: &[usize]) -> BigUint {
    bits.iter()
        .enumerate()
        .filter(|(_, bit)| **bit == 1)
        .fold(BigUint::zero(), |value, (index, _)| {
            value + (BigUint::one() << index)
        })
}

/// Convert an integer to a bit vector (little-endian).
#[allow(dead_code)]
fn int_to_bits(n: &BigUint, num_bits: usize) -> Vec<usize> {
    (0..num_bits)
        .map(|index| usize::from(n.bit(u64::try_from(index).expect("bit index fits u64"))))
        .collect()
}

/// Check if the given factors correctly factorize the target.
#[cfg(test)]
pub(crate) fn is_factoring(target: &BigUint, a: &BigUint, b: &BigUint) -> bool {
    a * b == *target
}

impl Problem for Factoring {
    const NAME: &'static str = "Factoring";
    type Value = Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.m + self.n]
    }

    fn evaluate(&self, config: &[usize]) -> Result<Or, crate::traits::EvaluationError> {
        Ok(Or(self.is_valid_factorization(config)))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default Factoring => "exp((m + n)^(1/3) * log(m + n)^(2/3))",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "factoring",
        instance: Box::new(Factoring::new(2, 3, 15)),
        optimal_config: vec![1, 1, 1, 0, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/factoring.rs"]
mod tests;
