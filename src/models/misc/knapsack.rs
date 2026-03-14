//! Knapsack problem implementation.
//!
//! The 0-1 Knapsack problem asks for a subset of items that maximizes
//! total value while respecting a weight capacity constraint.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "Knapsack",
        display_name: "",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Select items to maximize total value subject to weight capacity constraint",
        fields: &[
            FieldInfo { name: "weights", type_name: "Vec<i64>", description: "Item weights w_i" },
            FieldInfo { name: "values", type_name: "Vec<i64>", description: "Item values v_i" },
            FieldInfo { name: "capacity", type_name: "i64", description: "Knapsack capacity C" },
        ],
    }
}

/// The 0-1 Knapsack problem.
///
/// Given `n` items, each with weight `w_i` and value `v_i`, and a capacity `C`,
/// find a subset `S ⊆ {0, ..., n-1}` such that `∑_{i∈S} w_i ≤ C`,
/// maximizing `∑_{i∈S} v_i`.
///
/// # Representation
///
/// Each item has a binary variable: `x_i = 1` if item `i` is selected, `0` otherwise.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::Knapsack;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
/// let solver = BruteForce::new();
/// let solution = solver.find_best(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knapsack {
    weights: Vec<i64>,
    values: Vec<i64>,
    capacity: i64,
}

impl Knapsack {
    /// Create a new Knapsack instance.
    ///
    /// # Panics
    /// Panics if `weights` and `values` have different lengths.
    pub fn new(weights: Vec<i64>, values: Vec<i64>, capacity: i64) -> Self {
        assert_eq!(
            weights.len(),
            values.len(),
            "weights and values must have the same length"
        );
        Self {
            weights,
            values,
            capacity,
        }
    }

    /// Returns the item weights.
    pub fn weights(&self) -> &[i64] {
        &self.weights
    }

    /// Returns the item values.
    pub fn values(&self) -> &[i64] {
        &self.values
    }

    /// Returns the knapsack capacity.
    pub fn capacity(&self) -> i64 {
        self.capacity
    }

    /// Returns the number of items.
    pub fn num_items(&self) -> usize {
        self.weights.len()
    }
}

impl Problem for Knapsack {
    const NAME: &'static str = "Knapsack";
    type Metric = SolutionSize<i64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_items()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<i64> {
        if config.len() != self.num_items() {
            return SolutionSize::Invalid;
        }
        if config.iter().any(|&v| v >= 2) {
            return SolutionSize::Invalid;
        }
        let total_weight: i64 = config
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| self.weights[i])
            .sum();
        if total_weight > self.capacity {
            return SolutionSize::Invalid;
        }
        let total_value: i64 = config
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| self.values[i])
            .sum();
        SolutionSize::Valid(total_value)
    }
}

impl OptimizationProblem for Knapsack {
    type Value = i64;

    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

crate::declare_variants! {
    default opt Knapsack => "2^(num_items / 2)",
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/knapsack.rs"]
mod tests;
