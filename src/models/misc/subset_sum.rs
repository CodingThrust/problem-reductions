//! Subset Sum problem implementation.
//!
//! Given a set of positive integers and a target value, the problem asks whether
//! any subset sums to exactly the target. One of Karp's original 21 NP-complete
//! problems (1972).
//!
//! The type parameter `T` controls the integer precision:
//! - `SubsetSum<i64>` (default) — handles instances with up to ~18 decimal digits
//! - `SubsetSum<i128>` — handles instances with up to ~38 decimal digits

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use num_traits::Zero;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::AddAssign;

inventory::submit! {
    ProblemSchemaEntry {
        name: "SubsetSum",
        module_path: module_path!(),
        description: "Find a subset of positive integers that sums to exactly a target value",
        fields: &[
            FieldInfo { name: "sizes", type_name: "Vec<i64>", description: "Positive integer sizes s(a) for each element" },
            FieldInfo { name: "target", type_name: "i64", description: "Target sum B" },
        ],
    }
}

/// Trait alias for SubsetSum element types.
pub trait SubsetSumElement:
    Clone + Debug + PartialEq + PartialOrd + Zero + AddAssign + Serialize + DeserializeOwned + 'static
{
}

impl<T> SubsetSumElement for T where
    T: Clone
        + Debug
        + PartialEq
        + PartialOrd
        + Zero
        + AddAssign
        + Serialize
        + DeserializeOwned
        + 'static
{
}

/// The Subset Sum problem.
///
/// Given a set of `n` positive integers and a target `B`, determine whether
/// there exists a subset whose elements sum to exactly `B`.
///
/// # Type Parameter
///
/// `T` controls integer precision. Use `i64` (default) for most instances,
/// or `i128` for reductions that produce larger integers.
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
/// let problem = SubsetSum::new(vec![3i64, 7, 1, 8, 2, 4], 11i64);
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(
    serialize = "T: Serialize",
    deserialize = "T: serde::de::DeserializeOwned"
))]
pub struct SubsetSum<T = i64> {
    sizes: Vec<T>,
    target: T,
}

impl<T: SubsetSumElement> SubsetSum<T> {
    /// Create a new SubsetSum instance.
    ///
    /// # Panics
    ///
    /// Panics if any size is not positive (must be > 0).
    pub fn new(sizes: Vec<T>, target: T) -> Self {
        assert!(
            sizes.iter().all(|s| *s > T::zero()),
            "All sizes must be positive (> 0)"
        );
        Self { sizes, target }
    }

    /// Create a new SubsetSum instance without validating sizes.
    ///
    /// This is intended for reductions that produce SubsetSum instances
    /// where positivity is guaranteed by construction.
    pub(crate) fn new_unchecked(sizes: Vec<T>, target: T) -> Self {
        Self { sizes, target }
    }

    /// Returns the element sizes.
    pub fn sizes(&self) -> &[T] {
        &self.sizes
    }

    /// Returns the target sum.
    pub fn target(&self) -> &T {
        &self.target
    }

    /// Returns the number of elements.
    pub fn num_elements(&self) -> usize {
        self.sizes.len()
    }
}

impl<T: SubsetSumElement> Problem for SubsetSum<T> {
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
        let mut total = T::zero();
        for (i, &x) in config.iter().enumerate() {
            if x == 1 {
                total += self.sizes[i].clone();
            }
        }
        total == self.target
    }
}

impl<T: SubsetSumElement> SatisfactionProblem for SubsetSum<T> {}

crate::declare_variants! {
    SubsetSum<i64> => "2^(num_elements / 2)",
    SubsetSum<i128> => "2^(num_elements / 2)",
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/subset_sum.rs"]
mod tests;
