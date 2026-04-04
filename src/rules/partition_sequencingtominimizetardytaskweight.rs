//! Reduction from Partition to Sequencing to Minimize Tardy Task Weight.

use crate::models::misc::{Partition, SequencingToMinimizeTardyTaskWeight};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing Partition to SequencingToMinimizeTardyTaskWeight.
#[derive(Debug, Clone)]
pub struct ReductionPartitionToSequencingToMinimizeTardyTaskWeight {
    target: SequencingToMinimizeTardyTaskWeight,
}

impl ReductionPartitionToSequencingToMinimizeTardyTaskWeight {
    fn decode_schedule(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.target.num_tasks();
        assert_eq!(
            target_solution.len(),
            n,
            "target solution length must equal target num_tasks"
        );

        // The target model uses direct permutation encoding (dims = [n; n]).
        // Each position is a task index; the solver returns a valid permutation.
        target_solution.to_vec()
    }
}

impl ReductionResult for ReductionPartitionToSequencingToMinimizeTardyTaskWeight {
    type Source = Partition;
    type Target = SequencingToMinimizeTardyTaskWeight;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let schedule = self.decode_schedule(target_solution);
        let mut source_config = vec![1; self.target.num_tasks()];
        let mut completion_time = 0u64;

        for task in schedule {
            completion_time = completion_time
                .checked_add(self.target.lengths()[task])
                .expect("completion time overflowed u64");
            if completion_time <= self.target.deadlines()[task] {
                source_config[task] = 0;
            }
        }

        source_config
    }
}

#[reduction(overhead = {
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
