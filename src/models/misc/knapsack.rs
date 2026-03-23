//! Knapsack problem implementation.
//!
//! The 0-1 Knapsack problem asks for a subset of items that maximizes
//! total value while respecting a weight capacity constraint.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Max;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "Knapsack",
        display_name: "Knapsack",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Select items to maximize total value subject to weight capacity constraint",
        fields: &[
            FieldInfo { name: "weights", type_name: "Vec<i64>", description: "Nonnegative item weights w_i" },
            FieldInfo { name: "values", type_name: "Vec<i64>", description: "Nonnegative item values v_i" },
            FieldInfo { name: "capacity", type_name: "i64", description: "Nonnegative knapsack capacity C" },
        ],
    }
}

/// The 0-1 Knapsack problem.
///
/// Given `n` items, each with nonnegative weight `w_i` and nonnegative value `v_i`,
/// and a nonnegative capacity `C`,
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
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knapsack {
    #[serde(deserialize_with = "nonnegative_i64_vec::deserialize")]
    weights: Vec<i64>,
    #[serde(deserialize_with = "nonnegative_i64_vec::deserialize")]
    values: Vec<i64>,
    #[serde(deserialize_with = "nonnegative_i64::deserialize")]
    capacity: i64,
}

impl Knapsack {
    /// Create a new Knapsack instance.
    ///
    /// # Panics
    /// Panics if `weights` and `values` have different lengths, or if any
    /// weight, value, or the capacity is negative.
    pub fn new(weights: Vec<i64>, values: Vec<i64>, capacity: i64) -> Self {
        assert_eq!(
            weights.len(),
            values.len(),
            "weights and values must have the same length"
        );
        assert!(
            weights.iter().all(|&weight| weight >= 0),
            "Knapsack weights must be nonnegative"
        );
        assert!(
            values.iter().all(|&value| value >= 0),
            "Knapsack values must be nonnegative"
        );
        assert!(capacity >= 0, "Knapsack capacity must be nonnegative");
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

    /// Returns the number of binary slack bits used by the QUBO encoding.
    ///
    /// For positive capacity this is `floor(log2(C)) + 1`; for zero capacity we
    /// keep one slack bit so the encoding shape remains uniform.
    pub fn num_slack_bits(&self) -> usize {
        if self.capacity == 0 {
            1
        } else {
            (u64::BITS - (self.capacity as u64).leading_zeros()) as usize
        }
    }
}

impl Problem for Knapsack {
    const NAME: &'static str = "Knapsack";
    type Value = Max<i64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_items()]
    }

    fn evaluate(&self, config: &[usize]) -> Max<i64> {
        if config.len() != self.num_items() {
            return Max(None);
        }
        if config.iter().any(|&v| v >= 2) {
            return Max(None);
        }
        let total_weight: i64 = config
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| self.weights[i])
            .sum();
        if total_weight > self.capacity {
            return Max(None);
        }
        let total_value: i64 = config
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| self.values[i])
            .sum();
        Max(Some(total_value))
    }
}

crate::declare_variants! {
    default Knapsack => "2^(num_items / 2)",
}

mod nonnegative_i64 {
    use serde::de::Error;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i64::deserialize(deserializer)?;
        if value < 0 {
            return Err(D::Error::custom(format!(
                "expected nonnegative integer, got {value}"
            )));
        }
        Ok(value)
    }
}

mod nonnegative_i64_vec {
    use serde::de::Error;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values = Vec::<i64>::deserialize(deserializer)?;
        if let Some(value) = values.iter().copied().find(|value| *value < 0) {
            return Err(D::Error::custom(format!(
                "expected nonnegative integers, got {value}"
            )));
        }
        Ok(values)
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 4 items: weights [2,3,4,5], values [3,4,5,7], capacity 7
    // Optimal: items 0,3 → weight=7, value=10
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "knapsack",
        instance: Box::new(Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7)),
        optimal_config: vec![1, 0, 0, 1],
        optimal_value: serde_json::json!(10),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/knapsack.rs"]
mod tests;
