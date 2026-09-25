//! Reduction from Partition to Knapsack.

use crate::models::misc::{Knapsack, Partition};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing Partition to Knapsack.
#[derive(Debug, Clone)]
pub struct ReductionPartitionToKnapsack {
    target: Knapsack,
    source_sum: i64,
}

impl ReductionResult for ReductionPartitionToKnapsack {
    type Source = Partition;
    type Target = Knapsack;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| crate::rules::AggregateReductionResult::extract_value(self, value).0,
            "target witness does not certify a YES answer for the source",
        )?;

        Ok(target_solution.to_vec())
    }
}

#[crate::aggregate_reduction]
impl crate::rules::AggregateReductionResult for ReductionPartitionToKnapsack {
    type Source = Partition;
    type Target = Knapsack;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, value: crate::types::Max<i64>) -> crate::types::Or {
        crate::types::Or(self.source_sum % 2 == 0 && value.0 == Some(self.source_sum / 2))
    }
}

#[reduction(
    transform = exact { num_items = "num_elements" },
    unavailable = {
        capacity = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<Knapsack> for Partition {
    type Result = ReductionPartitionToKnapsack;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let weights = self.sizes().to_vec();
        let values = weights.clone();
        let capacity = self.total_sum() / 2;

        Ok(ReductionPartitionToKnapsack {
            source_sum: self.total_sum(),
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
                    source_config: serde_json::json!(vec![true, false, false, true, false, false]),
                    target_config: serde_json::json!(vec![true, false, false, true, false, false]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_knapsack.rs"]
mod tests;
