//! Knapsack problem implementation.
//!
//! The 0-1 Knapsack problem asks for a subset of items that maximizes
//! total value while respecting a weight capacity constraint.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Max;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "Knapsack",
        display_name: "Knapsack",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Select items to maximize total value subject to weight capacity constraint",
        fields: KnapsackCreateSpec::FIELDS,
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
/// use problemreductions::{Problem, BruteForce};
///
/// let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
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

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct KnapsackCreateSpec {
    /// Nonnegative item weights; defaults to one per value.
    weights: Option<Vec<i64>>,
    /// Nonnegative item values.
    values: Vec<i64>,
    /// Nonnegative knapsack capacity.
    capacity: i64,
}
impl TryFrom<KnapsackCreateSpec> for Knapsack {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: KnapsackCreateSpec) -> Result<Self, Self::Error> {
        let count = spec.values.len();
        let weights = spec.weights.unwrap_or_else(|| vec![1; count]);
        if weights.len() != count {
            return Err("weights length must equal values length".to_string().into());
        }
        if weights.iter().any(|&value| value < 0)
            || spec.values.iter().any(|&value| value < 0)
            || spec.capacity < 0
        {
            return Err("weights, values, and capacity must be nonnegative"
                .to_string()
                .into());
        }
        Ok(Self::new(weights, spec.values, spec.capacity))
    }
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
            self.capacity.ilog2() as usize + 1
        }
    }
}

impl Problem for Knapsack {
    const NAME: &'static str = "Knapsack";
    type Solution = Vec<bool>;
    type Value = Max<i64>;

    crate::problem_size![("num_items", num_items),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Max<i64>, crate::traits::EvaluationError> {
        Ok({
            if config.len() != self.num_items() {
                return Err(crate::traits::EvaluationError::InvalidConfiguration(
                    "item-selection length does not match the instance".into(),
                ));
            }
            let total_weight = config
                .iter()
                .enumerate()
                .filter(|(_, &x)| x)
                .map(|(i, _)| self.weights[i])
                .try_fold(0_i64, |total, weight| {
                    total.checked_add(weight).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "summing selected knapsack weights".into(),
                        )
                    })
                })?;
            if total_weight > self.capacity {
                return Ok(Max(None));
            }
            let total_value = config
                .iter()
                .enumerate()
                .filter(|(_, &x)| x)
                .map(|(i, _)| self.values[i])
                .try_fold(0_i64, |total, value| {
                    total.checked_add(value).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "summing selected knapsack values".into(),
                        )
                    })
                })?;
            Max(Some(total_value))
        })
    }
}

impl crate::solvers::BruteForceProblem for Knapsack {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.num_items()]
    }
}

crate::declare_variants! {
    default Knapsack => "2^(num_items / 2)" create KnapsackCreateSpec,
}

crate::register_brute_force! {
    Knapsack decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
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
        optimal_config: serde_json::json!(vec![true, false, false, true]),
        optimal_value: serde_json::json!(10),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/knapsack.rs"]
mod tests;
