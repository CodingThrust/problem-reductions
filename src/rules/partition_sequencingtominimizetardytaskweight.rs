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
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let mut seen = vec![false; self.target.num_tasks()];
            for &task in target_solution {
                if std::mem::replace(&mut seen[task], true) {
                    return Err(crate::rules::ExtractionError::invalid(format!(
                        "target schedule contains task {task} more than once"
                    )));
                }
            }

            let mut source_config = vec![1; self.target.num_tasks()];
            let mut completion_time = 0u64;

            for &task in target_solution {
                completion_time = completion_time
                    .checked_add(self.target.lengths()[task])
                    .ok_or_else(|| {
                        crate::rules::ExtractionError::invalid(
                            "target schedule completion time overflows u64",
                        )
                    })?;
                if completion_time <= self.target.deadlines()[task] {
                    source_config[task] = 0;
                }
            }

            source_config
        })
    }
}

#[reduction(
    exact = {
        num_tasks = "num_elements",
    })]
impl ReduceTo<SequencingToMinimizeTardyTaskWeight> for Partition {
    type Result = ReductionPartitionToSequencingToMinimizeTardyTaskWeight;

    fn reduce_to(&self) -> Self::Result {
        let common_deadline = self.total_sum() / 2;
        let lengths = self.sizes().to_vec();
        let weights = self.sizes().to_vec();
        let deadlines = vec![common_deadline; self.num_elements()];

        ReductionPartitionToSequencingToMinimizeTardyTaskWeight {
            target: SequencingToMinimizeTardyTaskWeight::new(lengths, weights, deadlines),
        }
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
                Partition::new(vec![3, 1, 1, 2, 2, 1]),
                SolutionPair {
                    source_config: vec![1, 0, 0, 1, 0, 0],
                    target_config: vec![1, 2, 4, 5, 0, 3],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_sequencingtominimizetardytaskweight.rs"]
mod tests;
