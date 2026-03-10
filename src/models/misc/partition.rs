//! Partition problem implementation.
//!
//! The Partition problem asks whether a given multiset of positive integers
//! can be partitioned into two subsets with equal sum.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "Partition",
        module_path: module_path!(),
        description: "Partition a multiset of positive integers into two subsets with equal sum",
        fields: &[
            FieldInfo { name: "sizes", type_name: "Vec<u64>", description: "Element sizes s(a)" },
        ],
    }
}

/// The Partition problem.
///
/// Given a finite set `A` of `n` positive integers with sizes `s(a)`,
/// determine whether there exists a subset `A' ⊆ A` such that
/// `∑_{a ∈ A'} s(a) = ∑_{a ∈ A \ A'} s(a)`.
///
/// # Representation
///
/// Each element has a binary variable: `x_i = 0` if element `i` is in
/// subset `S0`, `x_i = 1` if in subset `S1`.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::Partition;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = Partition::new(vec![3, 1, 1, 2, 2, 1]);
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partition {
    sizes: Vec<u64>,
}

impl Partition {
    /// Create a new Partition instance.
    ///
    /// # Panics
    /// Panics if any size is 0.
    pub fn new(sizes: Vec<u64>) -> Self {
        assert!(sizes.iter().all(|&s| s > 0), "all sizes must be positive");
        Self { sizes }
    }

    /// Returns the element sizes.
    pub fn sizes(&self) -> &[u64] {
        &self.sizes
    }

    /// Returns the number of elements.
    pub fn num_elements(&self) -> usize {
        self.sizes.len()
    }

    /// Returns the total sum of all element sizes.
    pub fn total_sum(&self) -> u64 {
        self.sizes.iter().sum()
    }
}

impl Problem for Partition {
    const NAME: &'static str = "Partition";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_elements()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.num_elements() {
            return false;
        }
        if config.iter().any(|&v| v >= 2) {
            return false;
        }
        let sum_0: u64 = config
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 0)
            .map(|(i, _)| self.sizes[i])
            .sum();
        let sum_1: u64 = config
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| self.sizes[i])
            .sum();
        sum_0 == sum_1
    }
}

impl SatisfactionProblem for Partition {}

crate::declare_variants! {
    Partition => "2^(num_elements / 2)",
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/partition.rs"]
mod tests;
