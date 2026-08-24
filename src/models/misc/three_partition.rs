//! 3-Partition problem implementation.
//!
//! Given 3m positive integers that each lie strictly between B/4 and B/2,
//! determine whether they can be partitioned into m triples that all sum to B.

use crate::registry::{CreateSpec, ProblemSchemaEntry, ProblemSizeFieldEntry};
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
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Partition 3m bounded positive integers into m triples whose sums all equal B",
        fields: ThreePartitionCreateSpec::FIELDS,
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
    sizes: Vec<i64>,
    bound: i64,
}

type GroupCountsAndSums = (Vec<usize>, Vec<i64>);

impl ThreePartition {
    fn validate_inputs(
        sizes: &[i64],
        bound: i64,
    ) -> Result<(), crate::registry::ConstructionError> {
        if sizes.is_empty() {
            return Err("ThreePartition requires at least one element"
                .to_string()
                .into());
        }
        if !sizes.len().is_multiple_of(3) {
            return Err(
                "ThreePartition requires the number of elements to be a multiple of 3".into(),
            );
        }
        if bound <= 0 {
            return Err("ThreePartition requires a positive bound"
                .to_string()
                .into());
        }
        if sizes.iter().any(|&size| size <= 0) {
            return Err("All sizes must be positive (> 0)".to_string().into());
        }

        for &size in sizes {
            let four_times_size = i128::from(size) * 4;
            let two_times_size = i128::from(size) * 2;
            let bound = i128::from(bound);
            if !(four_times_size > bound && two_times_size < bound) {
                return Err("Every size must lie strictly between B/4 and B/2"
                    .to_string()
                    .into());
            }
        }

        let total_sum = sizes
            .iter()
            .try_fold(0_i64, |total, &size| total.checked_add(size))
            .ok_or("total size sum exceeds i64 range")?;
        let group_count =
            i64::try_from(sizes.len() / 3).map_err(|_| "group count exceeds i64 range")?;
        let expected_sum = bound
            .checked_mul(group_count)
            .ok_or("group count times bound exceeds i64 range")?;
        if total_sum != expected_sum {
            return Err("Total sum of sizes must equal m * bound".to_string().into());
        }
        Ok(())
    }

    pub fn try_new(
        sizes: Vec<i64>,
        bound: i64,
    ) -> Result<Self, crate::registry::ConstructionError> {
        Self::validate_inputs(&sizes, bound)?;
        Ok(Self { sizes, bound })
    }

    /// Create a new 3-Partition instance.
    ///
    /// # Panics
    ///
    /// Panics if the input violates the classical 3-Partition invariants.
    pub fn new(sizes: Vec<i64>, bound: i64) -> Self {
        Self::try_new(sizes, bound).unwrap_or_else(|message| panic!("{message}"))
    }

    pub fn sizes(&self) -> &[i64] {
        &self.sizes
    }

    pub fn bound(&self) -> i64 {
        self.bound
    }

    pub fn num_elements(&self) -> usize {
        self.sizes.len()
    }

    pub fn num_groups(&self) -> usize {
        self.sizes.len() / 3
    }

    pub fn total_sum(&self) -> i64 {
        self.sizes
            .iter()
            .copied()
            .reduce(|acc, value| {
                acc.checked_add(value)
                    .expect("validated sum must fit in i64")
            })
            .unwrap_or(0)
    }

    fn group_counts_and_sums(
        &self,
        config: &[usize],
    ) -> Result<Option<GroupCountsAndSums>, crate::traits::EvaluationError> {
        if config.len() != self.num_elements() {
            return Ok(None);
        }

        let mut counts = vec![0usize; self.num_groups()];
        let mut sums = vec![0_i64; self.num_groups()];

        for (index, &group) in config.iter().enumerate() {
            if group >= self.num_groups() {
                return Ok(None);
            }
            counts[group] += 1;
            sums[group] = sums[group].checked_add(self.sizes[index]).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing three-partition group".into(),
                )
            })?;
        }

        Ok(Some((counts, sums)))
    }
}

#[derive(Deserialize, crate::CreateSpec)]
struct ThreePartitionCreateSpec {
    /// Positive integer sizes for the elements to partition.
    #[create(codec = "comma-separated")]
    sizes: Vec<i64>,
    /// Target sum for each triple.
    bound: i64,
}

impl TryFrom<ThreePartitionCreateSpec> for ThreePartition {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: ThreePartitionCreateSpec) -> Result<Self, Self::Error> {
        Self::try_new(spec.sizes, spec.bound)
    }
}

impl<'de> Deserialize<'de> for ThreePartition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let spec = ThreePartitionCreateSpec::deserialize(deserializer)?;
        Self::try_from(spec).map_err(D::Error::custom)
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

    fn evaluate(&self, config: &[usize]) -> Result<Or, crate::traits::EvaluationError> {
        Ok({
            Or({
                let Some((counts, sums)) = self.group_counts_and_sums(config)? else {
                    return Ok(Or(false));
                };

                counts.into_iter().all(|count| count == 3)
                    && sums.into_iter().all(|sum| sum == self.bound)
            })
        })
    }
}

crate::declare_variants! {
    default ThreePartition => "3^num_elements" create ThreePartitionCreateSpec,
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
