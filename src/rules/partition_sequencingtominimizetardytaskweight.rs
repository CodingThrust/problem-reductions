//! Reduction from Partition to Sequencing to Minimize Tardy Task Weight.

use crate::models::decision::Decision;
use crate::models::misc::{Partition, SequencingToMinimizeTardyTaskWeight};
use crate::reduction;
use crate::rules::traits::{recover_preserving_status, ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;

/// Result of reducing Partition to SequencingToMinimizeTardyTaskWeight.
#[derive(Debug, Clone)]
pub struct ReductionPartitionToSequencingToMinimizeTardyTaskWeight {
    target: Decision<SequencingToMinimizeTardyTaskWeight>,
}

impl ReductionResult for ReductionPartitionToSequencingToMinimizeTardyTaskWeight {
    type Source = Partition;
    type Target = Decision<SequencingToMinimizeTardyTaskWeight>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        recover_preserving_status(source, target, |solution| self.map_solution(solution))
    }
}

impl ReductionPartitionToSequencingToMinimizeTardyTaskWeight {
    fn map_solution(
        &self,
        target_solution: &<<Self as ReductionResult>::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<
        <<Self as ReductionResult>::Source as crate::traits::Problem>::Solution,
    > {
        Ok({
            let mut source_config = vec![true; self.target.inner().num_tasks()];
            let mut completion_time = 0i64;

            for &task in target_solution {
                completion_time += self.target.inner().lengths()[task];
                if completion_time <= self.target.inner().deadlines()[task] {
                    source_config[task] = false;
                }
            }

            source_config
        })
    }
}

#[reduction(
    transform = exact {
        num_tasks = "num_elements",
    })]
impl ReduceTo<Decision<SequencingToMinimizeTardyTaskWeight>> for Partition {
    type Result = ReductionPartitionToSequencingToMinimizeTardyTaskWeight;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let common_deadline = self.total_sum() / 2;
        let lengths = self.sizes().to_vec();
        let weights = self.sizes().to_vec();
        let deadlines = vec![common_deadline; self.num_elements()];

        Ok(ReductionPartitionToSequencingToMinimizeTardyTaskWeight {
            target: Decision::new(
                SequencingToMinimizeTardyTaskWeight::new(lengths, weights, deadlines),
                common_deadline,
            ),
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
                Decision<SequencingToMinimizeTardyTaskWeight>,
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
