//! Reduction from Partition to Sequencing to Minimize Tardy Task Weight.

use crate::models::misc::{Partition, SequencingToMinimizeTardyTaskWeight};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing Partition to SequencingToMinimizeTardyTaskWeight.
#[derive(Debug, Clone)]
pub struct ReductionPartitionToSequencingToMinimizeTardyTaskWeight {
    target: SequencingToMinimizeTardyTaskWeight,
}

impl ReductionResult for ReductionPartitionToSequencingToMinimizeTardyTaskWeight {
    type Source = Partition;
    type Target = SequencingToMinimizeTardyTaskWeight;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if !crate::rules::AggregateReductionResult::extract_value(self, value).0 {
            return Err(crate::rules::ExtractionError::invalid(
                "target schedule does not certify a balanced partition",
            ));
        }

        Ok({
            let mut source_config = vec![true; self.target.num_tasks()];
            let mut completion_time = 0i64;

            for &task in target_solution {
                completion_time = completion_time
                    .checked_add(self.target.lengths()[task])
                    .ok_or_else(|| {
                        crate::rules::ExtractionError::invalid(
                            "target schedule completion time overflows i64",
                        )
                    })?;
                if completion_time <= self.target.deadlines()[task] {
                    source_config[task] = false;
                }
            }

            source_config
        })
    }
}

impl crate::rules::AggregateReductionResult
    for ReductionPartitionToSequencingToMinimizeTardyTaskWeight
{
    type Source = Partition;
    type Target = SequencingToMinimizeTardyTaskWeight;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, value: crate::types::Min<i64>) -> crate::types::Or {
        // The source is nonempty, so the common deadline always exists.
        crate::types::Or(value.0 == Some(self.target.deadlines()[0]))
    }
}

#[reduction(
    aggregate = custom,
    transform = exact {
        num_tasks = "num_elements",
    })]
impl ReduceTo<SequencingToMinimizeTardyTaskWeight> for Partition {
    type Result = ReductionPartitionToSequencingToMinimizeTardyTaskWeight;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let common_deadline = self.total_sum() / 2;
        let lengths = self.sizes().to_vec();
        let weights = self.sizes().to_vec();
        let deadlines = vec![common_deadline; self.num_elements()];

        Ok(ReductionPartitionToSequencingToMinimizeTardyTaskWeight {
            target: SequencingToMinimizeTardyTaskWeight::new(lengths, weights, deadlines),
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_sequencing_to_minimize_tardy_task_weight",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<
                _,
                SequencingToMinimizeTardyTaskWeight,
            >(
                Partition::new(vec![3, 1, 1, 2, 2, 1]).unwrap(),
                SolutionPair {
                    source_config: serde_json::json!(vec![true, false, false, true, false, false]),
                    target_config: serde_json::json!(vec![1, 2, 4, 5, 0, 3]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_sequencingtominimizetardytaskweight.rs"]
mod tests;
