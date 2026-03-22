//! Reduction from Partition to Knapsack.

use crate::models::misc::{Knapsack, Partition};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing Partition to Knapsack.
#[derive(Debug, Clone)]
pub struct ReductionPartitionToKnapsack {
    target: Knapsack,
}

impl ReductionResult for ReductionPartitionToKnapsack {
    type Source = Partition;
    type Target = Knapsack;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

fn partition_size_to_i64(value: u64) -> i64 {
    i64::try_from(value)
        .expect("Partition -> Knapsack requires all sizes and total_sum / 2 to fit in i64")
}

#[reduction(overhead = {
    num_items = "num_elements",
})]
impl ReduceTo<Knapsack> for Partition {
    type Result = ReductionPartitionToKnapsack;

    fn reduce_to(&self) -> Self::Result {
        let weights: Vec<i64> = self
            .sizes()
            .iter()
            .copied()
            .map(partition_size_to_i64)
            .collect();
        let values = weights.clone();
        let capacity = partition_size_to_i64(self.total_sum() / 2);

        ReductionPartitionToKnapsack {
            target: Knapsack::new(weights, values, capacity),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_knapsack",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Knapsack>(
                Partition::new(vec![3, 1, 1, 2, 2, 1]),
                SolutionPair {
                    source_config: vec![1, 0, 0, 1, 0, 0],
                    target_config: vec![1, 0, 0, 1, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_knapsack.rs"]
mod tests;
