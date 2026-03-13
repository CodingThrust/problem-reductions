//! Subset Sum problem implementation.
//!
//! Given a set of integers and a target value, the problem asks whether any
//! subset sums to exactly the target. One of Karp's original 21 NP-complete
//! problems (1972).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SubsetSum",
        module_path: module_path!(),
        description: "Find a subset of integers that sums to exactly a target value",
        fields: &[
            FieldInfo { name: "sizes", type_name: "Vec<i64>", description: "Integer sizes s(a) for each element" },
            FieldInfo { name: "target", type_name: "i64", description: "Target sum B" },
        ],
    }
}

/// The Subset Sum problem.
///
/// Given a set of `n` integers and a target `B`, determine whether there exists
/// a subset whose elements sum to exactly `B`.
///
/// # Representation
///
/// Each element has a binary variable: `x_i = 1` if element `i` is selected,
/// `0` otherwise. The problem is satisfiable iff `∑_{i: x_i=1} sizes[i] == target`.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::SubsetSum;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = SubsetSum::new(vec![3, 7, 1, 8, 2, 4], 11);
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsetSum {
    sizes: Vec<i64>,
    target: i64,
}

impl SubsetSum {
    /// Create a new SubsetSum instance.
    pub fn new(sizes: Vec<i64>, target: i64) -> Self {
        Self { sizes, target }
    }

    /// Returns the element sizes.
    pub fn sizes(&self) -> &[i64] {
        &self.sizes
    }

    /// Returns the target sum.
    pub fn target(&self) -> i64 {
        self.target
    }

    /// Returns the number of elements.
    pub fn num_elements(&self) -> usize {
        self.sizes.len()
    }
}

impl Problem for SubsetSum {
    const NAME: &'static str = "SubsetSum";
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
        let total: i64 = config
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
    SubsetSum => "2^(num_elements / 2)",
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/subset_sum.rs"]
mod tests;
