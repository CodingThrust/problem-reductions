//! Subset Sum problem implementation.
//!
//! The Subset Sum problem asks whether there exists a subset of a given
//! set of positive integers that sums to exactly a target value B.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SubsetSum",
        module_path: module_path!(),
        description: "Decide if a subset of positive integers sums to a target value",
        fields: &[
            FieldInfo { name: "sizes", type_name: "Vec<u64>", description: "Positive integer size s(a) for each element a in A" },
            FieldInfo { name: "target", type_name: "u64", description: "Target sum B" },
        ],
    }
}

/// The Subset Sum problem.
///
/// Given a finite set A with sizes `s(a) ∈ Z⁺` for each element and a
/// positive integer B, determine whether there exists a subset `A' ⊆ A`
/// such that `∑_{a ∈ A'} s(a) = B`.
///
/// # Representation
///
/// Each element has a binary variable: `x_i = 1` if element `a_i` is in
/// the selected subset, `0` otherwise.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::SubsetSum;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // {3, 7, 1, 8, 2, 4} with target 11
/// let problem = SubsetSum::new(vec![3, 7, 1, 8, 2, 4], 11);
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsetSum {
    /// Positive integer sizes for each element.
    sizes: Vec<u64>,
    /// Target sum B.
    target: u64,
}

impl SubsetSum {
    /// Create a new Subset Sum instance.
    pub fn new(sizes: Vec<u64>, target: u64) -> Self {
        Self { sizes, target }
    }

    /// Get the element sizes.
    pub fn sizes(&self) -> &[u64] {
        &self.sizes
    }

    /// Get the target sum.
    pub fn target(&self) -> u64 {
        self.target
    }

    /// Get the number of elements (items).
    pub fn num_items(&self) -> usize {
        self.sizes.len()
    }

    /// Check if a configuration is a valid solution (subset sums to target).
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.evaluate(config)
    }
}

impl Problem for SubsetSum {
    const NAME: &'static str = "SubsetSum";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_items()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.num_items() {
            return false;
        }
        if config.iter().any(|&v| v >= 2) {
            return false;
        }
        let total: u64 = config
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| self.sizes[i])
            .sum();
        total == self.target
    }
}

impl SatisfactionProblem for SubsetSum {}

crate::declare_variants! {
    SubsetSum => "2^(num_items / 2)",
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/subset_sum.rs"]
mod tests;
