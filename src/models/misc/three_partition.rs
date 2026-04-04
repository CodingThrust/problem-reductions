//! 3-Partition problem implementation.
//!
//! Given 3m positive integers that each lie strictly between B/4 and B/2,
//! determine whether they can be partitioned into m triples that all sum to B.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ThreePartition",
        display_name: "3-Partition",
        aliases: &["3Partition", "3-Partition"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Partition 3m bounded positive integers into m triples whose sums all equal B",
        fields: &[
            FieldInfo { name: "sizes", type_name: "Vec<u64>", description: "Positive integer sizes s(a) for each element a in A" },
            FieldInfo { name: "bound", type_name: "u64", description: "Target sum B for each triple" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "ThreePartition",
        fields: &["num_elements", "num_groups"],
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ThreePartition {
    sizes: Vec<u64>,
    bound: u64,
}

impl ThreePartition {
    fn validate_inputs(sizes: &[u64], bound: u64) -> Result<(), String> {
        if sizes.is_empty() {
            return Err("ThreePartition requires at least one element".to_string());
        }
        if !sizes.len().is_multiple_of(3) {
            return Err(
                "ThreePartition requires the number of elements to be a multiple of 3".to_string(),
            );
        }
        if bound == 0 {
            return Err("ThreePartition requires a positive bound".to_string());
        }
        if sizes.contains(&0) {
            return Err("All sizes must be positive (> 0)".to_string());
        }

        let bound128 = u128::from(bound);
        for &size in sizes {
            let size = u128::from(size);
            if !(4 * size > bound128 && 2 * size < bound128) {
                return Err("Every size must lie strictly between B/4 and B/2".to_string());
            }
        }

        let total_sum: u128 = sizes.iter().map(|&size| u128::from(size)).sum();
        let expected_sum = u128::from(bound) * (sizes.len() as u128 / 3);
        if total_sum != expected_sum {
            return Err("Total sum of sizes must equal m * bound".to_string());
        }
        if total_sum > u128::from(u64::MAX) {
            return Err("Total sum exceeds u64 range".to_string());
        }

        Ok(())
    }

    pub fn try_new(sizes: Vec<u64>, bound: u64) -> Result<Self, String> {
        Self::validate_inputs(&sizes, bound)?;
        Ok(Self { sizes, bound })
    }

    /// Create a new 3-Partition instance.
    ///
    /// # Panics
    ///
    /// Panics if the input violates the classical 3-Partition invariants.
    pub fn new(sizes: Vec<u64>, bound: u64) -> Self {
        Self::try_new(sizes, bound).unwrap_or_else(|message| panic!("{message}"))
    }

    pub fn sizes(&self) -> &[u64] {
        &self.sizes
    }

    pub fn bound(&self) -> u64 {
        self.bound
    }

    pub fn num_elements(&self) -> usize {
        self.sizes.len()
    }

    pub fn num_groups(&self) -> usize {
        self.sizes.len() / 3
    }

    pub fn total_sum(&self) -> u64 {
        self.sizes
            .iter()
            .copied()
            .reduce(|acc, value| {
                acc.checked_add(value)
                    .expect("validated sum must fit in u64")
            })
            .unwrap_or(0)
    }

    fn group_counts_and_sums(&self, config: &[usize]) -> Option<(Vec<usize>, Vec<u128>)> {
        if config.len() != self.num_elements() {
            return None;
        }

        let mut counts = vec![0usize; self.num_groups()];
        let mut sums = vec![0u128; self.num_groups()];

        for (index, &group) in config.iter().enumerate() {
            if group >= self.num_groups() {
                return None;
            }
            counts[group] += 1;
            sums[group] += u128::from(self.sizes[index]);
        }

        Some((counts, sums))
    }
}

#[derive(Deserialize)]
struct ThreePartitionData {
    sizes: Vec<u64>,
    bound: u64,
}

impl<'de> Deserialize<'de> for ThreePartition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = ThreePartitionData::deserialize(deserializer)?;
        Self::try_new(data.sizes, data.bound).map_err(D::Error::custom)
    }
}

impl Problem for ThreePartition {
    const NAME: &'static str = "ThreePartition";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_groups(); self.num_elements()]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        Or({
            let Some((counts, sums)) = self.group_counts_and_sums(config) else {
                return Or(false);
            };

            let target = u128::from(self.bound);
            counts.into_iter().all(|count| count == 3) && sums.into_iter().all(|sum| sum == target)
        })
    }
}

crate::declare_variants! {
    default ThreePartition => "3^num_elements",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "three_partition",
        instance: Box::new(ThreePartition::new(vec![4, 5, 6, 4, 6, 5], 15)),
        optimal_config: vec![0, 0, 0, 1, 1, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/three_partition.rs"]
mod tests;
