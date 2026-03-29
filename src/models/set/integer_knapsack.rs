//! Integer Knapsack problem implementation.
//!
//! The Integer Knapsack problem generalizes the 0-1 Knapsack by allowing
//! each item to be selected with a non-negative integer multiplicity.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Max;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "IntegerKnapsack",
        display_name: "Integer Knapsack",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Select items with integer multiplicities to maximize total value subject to capacity constraint",
        fields: &[
            FieldInfo { name: "sizes", type_name: "Vec<i64>", description: "Positive item sizes s(u)" },
            FieldInfo { name: "values", type_name: "Vec<i64>", description: "Positive item values v(u)" },
            FieldInfo { name: "capacity", type_name: "i64", description: "Nonnegative knapsack capacity B" },
        ],
    }
}

/// The Integer Knapsack problem.
///
/// Given `n` items, each with positive size `s_i` and positive value `v_i`,
/// and a nonnegative capacity `B`,
/// find non-negative integer multiplicities `c_0, ..., c_{n-1}` such that
/// `sum c_i * s_i <= B`, maximizing `sum c_i * v_i`.
///
/// # Representation
///
/// Variable `i` has domain `{0, ..., floor(B / s_i)}` representing the
/// multiplicity of item `i`.
///
/// # Example
///
/// ```
/// use problemreductions::models::set::IntegerKnapsack;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15);
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize)]
#[serde(into = "RawIntegerKnapsack")]
pub struct IntegerKnapsack {
    sizes: Vec<i64>,
    values: Vec<i64>,
    capacity: i64,
}

impl IntegerKnapsack {
    /// Create a new IntegerKnapsack instance.
    ///
    /// # Panics
    /// Panics if `sizes` and `values` have different lengths, or if any
    /// size or value is not positive, or if capacity is negative.
    pub fn new(sizes: Vec<i64>, values: Vec<i64>, capacity: i64) -> Self {
        assert_eq!(
            sizes.len(),
            values.len(),
            "sizes and values must have the same length"
        );
        assert!(
            sizes.iter().all(|&s| s > 0),
            "IntegerKnapsack sizes must be positive"
        );
        assert!(
            values.iter().all(|&v| v > 0),
            "IntegerKnapsack values must be positive"
        );
        assert!(
            capacity >= 0,
            "IntegerKnapsack capacity must be nonnegative"
        );
        Self {
            sizes,
            values,
            capacity,
        }
    }

    /// Returns the item sizes.
    pub fn sizes(&self) -> &[i64] {
        &self.sizes
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
        self.sizes.len()
    }
}

impl Problem for IntegerKnapsack {
    const NAME: &'static str = "IntegerKnapsack";
    type Value = Max<i64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        self.sizes
            .iter()
            .map(|&s| (self.capacity / s + 1) as usize)
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> Max<i64> {
        if config.len() != self.num_items() {
            return Max(None);
        }
        let dims = self.dims();
        if config.iter().zip(dims.iter()).any(|(&c, &d)| c >= d) {
            return Max(None);
        }
        let total_size: i64 = config
            .iter()
            .enumerate()
            .map(|(i, &c)| c as i64 * self.sizes[i])
            .sum();
        if total_size > self.capacity {
            return Max(None);
        }
        let total_value: i64 = config
            .iter()
            .enumerate()
            .map(|(i, &c)| c as i64 * self.values[i])
            .sum();
        Max(Some(total_value))
    }
}

crate::declare_variants! {
    default IntegerKnapsack => "(capacity + 1)^num_items",
}

/// Raw representation for serde deserialization with full validation.
#[derive(Deserialize, Serialize)]
struct RawIntegerKnapsack {
    sizes: Vec<i64>,
    values: Vec<i64>,
    capacity: i64,
}

impl From<IntegerKnapsack> for RawIntegerKnapsack {
    fn from(ik: IntegerKnapsack) -> Self {
        RawIntegerKnapsack {
            sizes: ik.sizes,
            values: ik.values,
            capacity: ik.capacity,
        }
    }
}

impl TryFrom<RawIntegerKnapsack> for IntegerKnapsack {
    type Error = String;

    fn try_from(raw: RawIntegerKnapsack) -> Result<Self, String> {
        if raw.sizes.len() != raw.values.len() {
            return Err(format!(
                "sizes and values must have the same length, got {} and {}",
                raw.sizes.len(),
                raw.values.len()
            ));
        }
        if let Some(&s) = raw.sizes.iter().find(|&&s| s <= 0) {
            return Err(format!("expected positive sizes, got {s}"));
        }
        if let Some(&v) = raw.values.iter().find(|&&v| v <= 0) {
            return Err(format!("expected positive values, got {v}"));
        }
        if raw.capacity < 0 {
            return Err(format!(
                "expected nonnegative capacity, got {}",
                raw.capacity
            ));
        }
        Ok(IntegerKnapsack {
            sizes: raw.sizes,
            values: raw.values,
            capacity: raw.capacity,
        })
    }
}

impl<'de> Deserialize<'de> for IntegerKnapsack {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = RawIntegerKnapsack::deserialize(deserializer)?;
        IntegerKnapsack::try_from(raw).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 5 items: sizes [3,4,5,2,7], values [4,5,7,3,9], capacity 15
    // Optimal: c=(0,0,1,5,0) → total_size=5+10=15, total_value=7+15=22
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "integer-knapsack",
        instance: Box::new(IntegerKnapsack::new(
            vec![3, 4, 5, 2, 7],
            vec![4, 5, 7, 3, 9],
            15,
        )),
        optimal_config: vec![0, 0, 1, 5, 0],
        optimal_value: serde_json::json!(22),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/integer_knapsack.rs"]
mod tests;
