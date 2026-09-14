//! Reduction from Partition to Open Shop Scheduling.

use crate::models::decision::Decision;
use crate::models::misc::{OpenShopScheduling, Partition};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;
use crate::solvers::SolveOutcome;

#[derive(Debug, Clone)]
pub struct ReductionPartitionToOpenShopScheduling {
    target: Decision<OpenShopScheduling>,
}

impl ReductionResult for ReductionPartitionToOpenShopScheduling {
    type Source = Partition;
    type Target = Decision<OpenShopScheduling>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        match target {
            SolveOutcome::Infeasible => Ok(SolveOutcome::Infeasible),
            SolveOutcome::Optimal { solution, .. } => {
                let solution = self.map_solution(&solution)?;
                Ok(SolveOutcome::optimal(source, solution)?)
            }
            SolveOutcome::Feasible { solution, .. } => {
                let solution = self.map_solution(&solution)?;
                Ok(SolveOutcome::feasible(source, solution)?)
            }
        }
    }
}

impl ReductionPartitionToOpenShopScheduling {
    fn map_solution(
        &self,
        target_solution: &<<Self as ReductionResult>::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<
        <<Self as ReductionResult>::Source as crate::traits::Problem>::Solution,
    > {
        let target = self.target.inner();
        let num_elements = target.num_jobs() - 1;
        let mut source_config = vec![false; num_elements];
        let m = target.num_machines();
        let special_job = num_elements;
        let half_sum = target.processing_times()[special_job][0];

        // Find the middle machine where the special job starts at half_sum.
        let middle_machine: usize = (0..m)
            .filter(|&machine| target_solution[special_job * m + machine] as i64 == half_sum)
            .sum();
        let pivot = target_solution[special_job * m + middle_machine] as i64;

        for (job, slot) in source_config.iter_mut().enumerate() {
            let completion = target_solution[job * m + middle_machine] as i64
                + target.processing_times()[job][middle_machine];
            *slot = completion <= pivot;
        }

        Ok(source_config)
    }
}

#[reduction(
    transform = exact {
        num_jobs = "num_elements + 1",
        num_machines = "3",
    },
    unavailable = {
        schedule_horizon = "depends on the numeric partition sizes, which are not represented by source size parameters",
    }
)]
impl ReduceTo<Decision<OpenShopScheduling>> for Partition {
    type Result = ReductionPartitionToOpenShopScheduling;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let half_sum = self.total_sum() / 2;
        let mut processing_times: Vec<Vec<i64>> =
            self.sizes().iter().map(|&size| vec![size; 3]).collect();
        processing_times.push(vec![half_sum; 3]);

        let target = OpenShopScheduling::try_new(3, processing_times)
            .map_err(<Self as ReduceTo<Decision<OpenShopScheduling>>>::target_construction)?;
        // The validated nonnegative schedule horizon includes these three terms.
        let feasible_makespan = 3 * half_sum;
        Ok(ReductionPartitionToOpenShopScheduling {
            target: Decision::new(target, feasible_makespan),
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_open_shop_scheduling",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<OpenShopScheduling>>(
                Partition::new(vec![1, 2, 3]).unwrap(),
                SolutionPair {
                    source_config: serde_json::json!(vec![true, true, false]),
                    target_config: serde_json::json!(vec![0, 5, 6, 1, 3, 7, 6, 0, 3, 3, 6, 0]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_openshopscheduling.rs"]
mod tests;
