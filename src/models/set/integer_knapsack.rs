//! Integer Knapsack problem implementation.
//!
//! The Integer Knapsack problem generalizes the 0-1 Knapsack by allowing
//! each item to be selected with a non-negative integer multiplicity.

use crate::registry::ConstructionError;
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
        category: crate::registry::ProblemCategory::Set,
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
/// let problem = IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15).unwrap();
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem).unwrap();
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
    pub fn new(
        sizes: Vec<i64>,
        values: Vec<i64>,
        capacity: i64,
    ) -> Result<Self, ConstructionError> {
        Self::try_from(RawIntegerKnapsack {
            sizes,
            values,
            capacity,
        })
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
            .map(|&s| {
                let dimension = i128::from(self.capacity) / i128::from(s) + 1;
                usize::try_from(dimension)
                    .expect("validated integer-knapsack dimension must fit usize")
            })
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> Result<Max<i64>, crate::traits::EvaluationError> {
        Ok({
            if config.len() != self.num_items() {
                return Ok(Max(None));
            }
            let dims = self.dims();
            if config.iter().zip(dims.iter()).any(|(&c, &d)| c >= d) {
                return Ok(Max(None));
            }
            let total_size = config
                .iter()
                .enumerate()
                .try_fold(0_i64, |total, (i, &c)| {
                    let count = i64::try_from(c).map_err(|_| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "converting knapsack item count to i64".into(),
                        )
                    })?;
                    let contribution = count.checked_mul(self.sizes[i]).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "multiplying knapsack item count by size".into(),
                        )
                    })?;
                    total.checked_add(contribution).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "summing knapsack item sizes".into(),
                        )
                    })
                })?;
            if total_size > self.capacity {
                return Ok(Max(None));
            }
            let total_value = config
                .iter()
                .enumerate()
                .try_fold(0_i64, |total, (i, &c)| {
                    let count = i64::try_from(c).map_err(|_| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "converting knapsack item count to i64".into(),
                        )
                    })?;
                    let contribution = count.checked_mul(self.values[i]).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "multiplying knapsack item count by value".into(),
                        )
                    })?;
                    total.checked_add(contribution).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "summing knapsack item values".into(),
                        )
                    })
                })?;
            Max(Some(total_value))
        })
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
    type Error = ConstructionError;

    fn try_from(raw: RawIntegerKnapsack) -> Result<Self, Self::Error> {
        if raw.sizes.len() != raw.values.len() {
            return Err(ConstructionError::Conversion(format!(
                "sizes and values must have the same length, got {} and {}",
                raw.sizes.len(),
                raw.values.len()
            )));
        }
        if let Some(&s) = raw.sizes.iter().find(|&&s| s <= 0) {
            return Err(ConstructionError::Conversion(format!(
                "expected positive sizes, got {s}"
            )));
        }
        if let Some(&v) = raw.values.iter().find(|&&v| v <= 0) {
            return Err(ConstructionError::Conversion(format!(
                "expected positive values, got {v}"
            )));
        }
        if raw.capacity < 0 {
            return Err(ConstructionError::Conversion(format!(
                "expected nonnegative capacity, got {}",
                raw.capacity
            )));
        }
        for &size in &raw.sizes {
            let dimension = i128::from(raw.capacity) / i128::from(size) + 1;
            usize::try_from(dimension).map_err(|_| {
                ConstructionError::IntegerOverflow(format!(
                    "knapsack dimension for capacity {} and item size {size} does not fit usize",
                    raw.capacity
                ))
            })?;
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
        instance: Box::new(
            IntegerKnapsack::new(vec![3, 4, 5, 2, 7], vec![4, 5, 7, 3, 9], 15).unwrap(),
        ),
        optimal_config: vec![0, 0, 1, 5, 0],
        optimal_value: serde_json::json!(22),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/integer_knapsack.rs"]
mod tests;
