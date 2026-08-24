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

    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.to_vec())
    }
}

#[reduction(
    size = exact { num_items = "num_elements" },
)]
impl ReduceTo<Knapsack> for Partition {
    type Result = ReductionPartitionToKnapsack;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let weights = self.sizes().to_vec();
        let values = weights.clone();
        let capacity = self.total_sum() / 2;

        Ok(ReductionPartitionToKnapsack {
            target: Knapsack::new(weights, values, capacity),
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_knapsack",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Knapsack>(
                Partition::new(vec![3, 1, 1, 2, 2, 1]).unwrap(),
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
