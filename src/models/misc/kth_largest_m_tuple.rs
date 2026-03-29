//! Kth Largest m-Tuple problem implementation.
//!
//! Given m sets of positive integers and a bound B, count how many
//! distinct m-tuples (one element per set) have total size at least B.
//! Garey & Johnson MP10.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Sum;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "KthLargestMTuple",
        display_name: "Kth Largest m-Tuple",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Count m-tuples whose total size meets a bound",
        fields: &[
            FieldInfo { name: "sets", type_name: "Vec<Vec<u64>>", description: "m sets, each containing positive integer sizes" },
            FieldInfo { name: "bound", type_name: "u64", description: "Lower bound B on tuple sum" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "KthLargestMTuple",
        fields: &["num_sets", "total_tuples"],
    }
}

/// The Kth Largest m-Tuple problem.
///
/// Given sets `X_1, ..., X_m` of positive integers and a bound `B`, count how
/// many distinct m-tuples `(x_1, ..., x_m)` in `X_1 x ... x X_m` satisfy
/// `sum(x_i) >= B`.
///
/// # Representation
///
/// Variable `i` selects an element from set `X_i`, ranging over `{0, ..., |X_i|-1}`.
/// `evaluate` returns `Sum(1)` if the tuple sum >= B, else `Sum(0)`.
/// The aggregate over all configurations gives the total count of qualifying tuples.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::KthLargestMTuple;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = KthLargestMTuple::new(
///     vec![vec![2, 5, 8], vec![3, 6], vec![1, 4, 7]],
///     12,
/// );
/// let solver = BruteForce::new();
/// let value = solver.solve(&problem);
/// // 14 of the 18 tuples have sum >= 12
/// assert_eq!(value, problemreductions::types::Sum(14));
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct KthLargestMTuple {
    sets: Vec<Vec<u64>>,
    bound: u64,
}

impl KthLargestMTuple {
    fn validate(sets: &[Vec<u64>], bound: u64) -> Result<(), String> {
        if sets.is_empty() {
            return Err("KthLargestMTuple requires at least one set".to_string());
        }
        if sets.iter().any(|s| s.is_empty()) {
            return Err("Every set must be non-empty".to_string());
        }
        if sets.iter().any(|s| s.contains(&0)) {
            return Err("All sizes must be positive (> 0)".to_string());
        }
        if bound == 0 {
            return Err("Bound B must be positive".to_string());
        }
        Ok(())
    }

    /// Try to create a new KthLargestMTuple instance.
    pub fn try_new(sets: Vec<Vec<u64>>, bound: u64) -> Result<Self, String> {
        Self::validate(&sets, bound)?;
        Ok(Self { sets, bound })
    }

    /// Create a new KthLargestMTuple instance.
    ///
    /// # Panics
    ///
    /// Panics if the inputs are invalid.
    pub fn new(sets: Vec<Vec<u64>>, bound: u64) -> Self {
        Self::try_new(sets, bound).unwrap_or_else(|msg| panic!("{msg}"))
    }

    /// Returns the sets.
    pub fn sets(&self) -> &[Vec<u64>] {
        &self.sets
    }

    /// Returns the bound B.
    pub fn bound(&self) -> u64 {
        self.bound
    }

    /// Returns the number of sets (m).
    pub fn num_sets(&self) -> usize {
        self.sets.len()
    }

    /// Returns the total number of m-tuples (product of set sizes).
    pub fn total_tuples(&self) -> usize {
        self.sets.iter().map(|s| s.len()).product()
    }
}

#[derive(Deserialize)]
struct KthLargestMTupleDef {
    sets: Vec<Vec<u64>>,
    bound: u64,
}

impl<'de> Deserialize<'de> for KthLargestMTuple {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = KthLargestMTupleDef::deserialize(deserializer)?;
        Self::try_new(data.sets, data.bound).map_err(D::Error::custom)
    }
}

impl Problem for KthLargestMTuple {
    const NAME: &'static str = "KthLargestMTuple";
    type Value = Sum<u64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        self.sets.iter().map(|s| s.len()).collect()
    }

    fn evaluate(&self, config: &[usize]) -> Sum<u64> {
        if config.len() != self.num_sets() {
            return Sum(0);
        }
        for (i, &choice) in config.iter().enumerate() {
            if choice >= self.sets[i].len() {
                return Sum(0);
            }
        }
        let total: u64 = config
            .iter()
            .enumerate()
            .map(|(i, &choice)| self.sets[i][choice])
            .sum();
        if total >= self.bound {
            Sum(1)
        } else {
            Sum(0)
        }
    }
}

// Best known: brute-force enumeration of all tuples, O(total_tuples * num_sets).
// No sub-exponential exact algorithm is known for the general case.
crate::declare_variants! {
    default KthLargestMTuple => "total_tuples * num_sets",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // m=3, X_1={2,5,8}, X_2={3,6}, X_3={1,4,7}, B=12.
    // 14 of 18 tuples have sum >= 12. The config [2,1,2] picks (8,6,7) with sum=21 >= 12.
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "kth_largest_m_tuple",
        instance: Box::new(KthLargestMTuple::new(
            vec![vec![2, 5, 8], vec![3, 6], vec![1, 4, 7]],
            12,
        )),
        optimal_config: vec![2, 1, 2],
        optimal_value: serde_json::json!(1),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/kth_largest_m_tuple.rs"]
mod tests;
