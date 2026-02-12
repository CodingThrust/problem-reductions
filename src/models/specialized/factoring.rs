//! Integer Factoring problem implementation.
//!
//! The Factoring problem represents integer factorization as a computational problem.
//! Given a number N, find two factors (a, b) such that a * b = N.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::{EnergyMode, ProblemSize, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "Factoring",
        category: "specialized",
        description: "Factor a composite integer into two factors",
        fields: &[
            FieldInfo { name: "m", type_name: "usize", description: "Bits for first factor" },
            FieldInfo { name: "n", type_name: "usize", description: "Bits for second factor" },
            FieldInfo { name: "target", type_name: "u64", description: "Number to factor" },
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
/// use problemreductions::models::specialized::Factoring;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Factor 6 with 2-bit factors (allowing factors 0-3)
/// let problem = Factoring::new(2, 2, 6);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_best(&problem);
///
/// // Should find: 2*3=6 or 3*2=6
/// for sol in &solutions {
///     let (a, b) = problem.read_factors(sol);
///     assert_eq!(a * b, 6);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Factoring {
    /// Number of bits for the first factor.
    m: usize,
    /// Number of bits for the second factor.
    n: usize,
    /// The number to factor.
    target: u64,
}

impl Factoring {
    /// Create a new Factoring problem.
    ///
    /// # Arguments
    /// * `m` - Number of bits for the first factor
    /// * `n` - Number of bits for the second factor
    /// * `target` - The number to factor
    pub fn new(m: usize, n: usize, target: u64) -> Self {
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

    /// Get the target number to factor.
    pub fn target(&self) -> u64 {
        self.target
    }

    /// Read the two factors from a configuration.
    ///
    /// The first `m` bits represent the first factor,
    /// the next `n` bits represent the second factor.
    pub fn read_factors(&self, config: &[usize]) -> (u64, u64) {
        let a = bits_to_int(&config[..self.m]);
        let b = bits_to_int(&config[self.m..self.m + self.n]);
        (a, b)
    }

    /// Check if the configuration is a valid factorization.
    pub fn is_valid_factorization(&self, config: &[usize]) -> bool {
        let (a, b) = self.read_factors(config);
        a * b == self.target
    }
}

/// Convert a bit vector (little-endian) to an integer.
fn bits_to_int(bits: &[usize]) -> u64 {
    bits.iter().enumerate().map(|(i, &b)| (b as u64) << i).sum()
}

/// Convert an integer to a bit vector (little-endian).
#[allow(dead_code)]
fn int_to_bits(n: u64, num_bits: usize) -> Vec<usize> {
    (0..num_bits).map(|i| ((n >> i) & 1) as usize).collect()
}

impl Problem for Factoring {
    const NAME: &'static str = "Factoring";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }

    type Size = i32;

    fn num_variables(&self) -> usize {
        self.m + self.n
    }

    fn num_flavors(&self) -> usize {
        2 // Binary
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_bits_first", self.m),
            ("num_bits_second", self.n),
            ("target", self.target as usize),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::SmallerSizeIsBetter // Minimize distance from target
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let (a, b) = self.read_factors(config);
        let product = a * b;

        // Distance from target (0 means exact match)
        let distance = if product > self.target {
            (product - self.target) as i32
        } else {
            (self.target - product) as i32
        };

        let is_valid = product == self.target;
        SolutionSize::new(distance, is_valid)
    }
}

/// Check if the given factors correctly factorize the target.
pub fn is_factoring(target: u64, a: u64, b: u64) -> bool {
    a * b == target
}

// === ProblemV2 / OptimizationProblemV2 implementations ===

impl crate::traits::ProblemV2 for Factoring {
    const NAME: &'static str = "Factoring";
    type Metric = i32;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.m + self.n]
    }

    fn evaluate(&self, config: &[usize]) -> i32 {
        let (a, b) = self.read_factors(config);
        let product = a * b;
        // Distance from target (0 means exact match)
        if product > self.target {
            (product - self.target) as i32
        } else {
            (self.target - product) as i32
        }
    }
}

impl crate::traits::OptimizationProblemV2 for Factoring {
    fn direction(&self) -> crate::types::Direction {
        crate::types::Direction::Minimize
    }
}

#[cfg(test)]
#[path = "../../unit_tests/models/specialized/factoring.rs"]
mod tests;
