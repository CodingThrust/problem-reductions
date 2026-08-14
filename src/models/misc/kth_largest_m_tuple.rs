//! Kth Largest m-Tuple problem implementation.
//!
//! Given m sets of positive integers and thresholds K and B, determine whether
//! at least K distinct m-tuples (one element per set) have total size at least B.
//! Garey & Johnson MP10.

use crate::registry::{CreateSpec, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "KthLargestMTuple",
        display_name: "Kth Largest m-Tuple",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Count m-tuples whose total size meets a bound and compare against a threshold K",
        fields: KthLargestMTupleCreateSpec::FIELDS,
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
/// Given sets `X_1, ..., X_m` of positive integers, a threshold `K`, and a
/// bound `B`, determine whether at least `K` distinct m-tuples
/// `(x_1, ..., x_m)` in `X_1 x ... x X_m` satisfy `sum(x_i) >= B`.
///
/// # Representation
///
/// The empty configuration triggers enumeration of the Cartesian product.
/// `evaluate` returns `Or(true)` as soon as `K` qualifying tuples have been
/// found and `Or(false)` if the complete product contains fewer than `K`.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::KthLargestMTuple;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = KthLargestMTuple::new(
///     vec![vec![2, 5, 8], vec![3, 6], vec![1, 4, 7]],
///     14,
///     12,
/// );
/// let solver = BruteForce::new();
/// let answer = solver.solve(&problem);
/// // 14 of the 18 tuples have sum >= 12, so count >= K.
/// assert_eq!(answer, problemreductions::types::Or(true));
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct KthLargestMTuple {
    sets: Vec<Vec<u64>>,
    k: u64,
    bound: u64,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct KthLargestMTupleCreateSpec {
    /// m sets, each containing positive integer sizes.
    subsets: Vec<Vec<u64>>,
    /// Threshold K (answer YES iff count >= K).
    k: u64,
    /// Lower bound B on tuple sum.
    bound: u64,
}

impl TryFrom<KthLargestMTupleCreateSpec> for KthLargestMTuple {
    type Error = String;

    fn try_from(spec: KthLargestMTupleCreateSpec) -> Result<Self, Self::Error> {
        Self::try_new(spec.subsets, spec.k, spec.bound)
    }
}

impl KthLargestMTuple {
    fn validate(sets: &[Vec<u64>], k: u64, bound: u64) -> Result<(), String> {
        if sets.is_empty() {
            return Err("KthLargestMTuple requires at least one set".to_string());
        }
        if sets.iter().any(|s| s.is_empty()) {
            return Err("Every set must be non-empty".to_string());
        }
        if sets.iter().any(|s| s.contains(&0)) {
            return Err("All sizes must be positive (> 0)".to_string());
        }
        if k == 0 {
            return Err("Threshold K must be positive".to_string());
        }
        if bound == 0 {
            return Err("Bound B must be positive".to_string());
        }
        Ok(())
    }

    /// Try to create a new KthLargestMTuple instance.
    pub fn try_new(sets: Vec<Vec<u64>>, k: u64, bound: u64) -> Result<Self, String> {
        Self::validate(&sets, k, bound)?;
        Ok(Self { sets, k, bound })
    }

    /// Create a new KthLargestMTuple instance.
    ///
    /// # Panics
    ///
    /// Panics if the inputs are invalid.
    pub fn new(sets: Vec<Vec<u64>>, k: u64, bound: u64) -> Self {
        Self::try_new(sets, k, bound).unwrap_or_else(|msg| panic!("{msg}"))
    }

    /// Returns the sets.
    pub fn sets(&self) -> &[Vec<u64>] {
        &self.sets
    }

    /// Returns the threshold K.
    pub fn k(&self) -> u64 {
        self.k
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
        self.sets
            .iter()
            .try_fold(1usize, |total, set| total.checked_mul(set.len()))
            .expect("KthLargestMTuple total tuple count exceeds usize")
    }

    fn has_at_least_k_qualifying_tuples(&self) -> bool {
        let mut choices = vec![0; self.sets.len()];
        let mut qualifying = 0;

        loop {
            let mut remaining_bound = self.bound;
            for (set, &choice) in self.sets.iter().zip(&choices) {
                remaining_bound = remaining_bound.saturating_sub(set[choice]);
            }
            if remaining_bound == 0 {
                qualifying += 1;
                if qualifying == self.k {
                    return true;
                }
            }

            let mut advanced = false;
            for set_index in (0..choices.len()).rev() {
                choices[set_index] += 1;
                if choices[set_index] == self.sets[set_index].len() {
                    choices[set_index] = 0;
                } else {
                    advanced = true;
                    break;
                }
            }
            if !advanced {
                return false;
            }
        }
    }
}

#[derive(Deserialize)]
struct KthLargestMTupleDef {
    sets: Vec<Vec<u64>>,
    k: u64,
    bound: u64,
}

impl<'de> Deserialize<'de> for KthLargestMTuple {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = KthLargestMTupleDef::deserialize(deserializer)?;
        Self::try_new(data.sets, data.k, data.bound).map_err(D::Error::custom)
    }
}

impl Problem for KthLargestMTuple {
    const NAME: &'static str = "KthLargestMTuple";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        Or(config.is_empty() && self.has_at_least_k_qualifying_tuples())
    }
}

// Best known: brute-force enumeration of all tuples, O(total_tuples * num_sets).
// No sub-exponential exact algorithm is known for the general case.
crate::declare_variants! {
    default KthLargestMTuple => "total_tuples * num_sets" create KthLargestMTupleCreateSpec,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // m=3, X_1={2,5,8}, X_2={3,6}, X_3={1,4,7}, B=12, K=14.
    // 14 of 18 tuples have sum >= 12, so the answer is YES at K=14.
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "kth_largest_m_tuple",
        instance: Box::new(KthLargestMTuple::new(
            vec![vec![2, 5, 8], vec![3, 6], vec![1, 4, 7]],
            14,
            12,
        )),
        optimal_config: vec![],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/kth_largest_m_tuple.rs"]
mod tests;
