//! Partition problem implementation.
//!
//! Given a finite set of positive integers, determine whether it can be
//! partitioned into two subsets of equal sum. One of Karp's original 21
//! NP-complete problems (1972), Garey & Johnson SP12.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "Partition",
        display_name: "Partition",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether a multiset of positive integers can be partitioned into two subsets of equal sum",
        fields: &[
            FieldInfo { name: "sizes", type_name: "Vec<u64>", description: "Positive integer size for each element" },
        ],
    }
}

/// The Partition problem.
///
/// Given a finite set `A` with `n` positive integer sizes, determine whether
/// there exists a subset `A' ⊆ A` such that `∑_{a ∈ A'} s(a) = ∑_{a ∈ A\A'} s(a)`.
///
/// # Representation
///
/// Each element has a binary variable: `x_i = 1` if element `i` is in the
/// second subset, `0` if in the first. The problem is satisfiable iff
/// `∑_{i: x_i=1} sizes[i] = total_sum / 2`.
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
    ///
    /// Panics if `sizes` is empty or any size is zero.
    pub fn new(sizes: Vec<u64>) -> Self {
        assert!(!sizes.is_empty(), "Partition requires at least one element");
        assert!(
            sizes.iter().all(|&s| s > 0),
            "All sizes must be positive (> 0)"
        );
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

    /// Returns the total sum of all sizes.
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
        let selected_sum: u64 = config
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| self.sizes[i])
            .sum();
        selected_sum * 2 == self.total_sum()
    }
}

impl SatisfactionProblem for Partition {}

crate::declare_variants! {
    default sat Partition => "2^(num_elements / 2)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "partition",
        build: || {
            let problem = Partition::new(vec![3, 1, 1, 2, 2, 1]);
            crate::example_db::specs::satisfaction_example(
                problem,
                vec![vec![1, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0]],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/partition.rs"]
mod tests;
