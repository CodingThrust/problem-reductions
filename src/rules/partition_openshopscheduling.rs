//! Reduction from Partition to Open Shop Scheduling.

use crate::models::misc::{OpenShopScheduling, Partition};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionPartitionToOpenShopScheduling {
    target: OpenShopScheduling,
}

impl ReductionResult for ReductionPartitionToOpenShopScheduling {
    type Source = Partition;
    type Target = OpenShopScheduling;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let num_elements = self.target.num_jobs().saturating_sub(1);
        let mut source_config = vec![0; num_elements];
        let Some(schedule) = self.target.schedule_from_config(target_solution) else {
            return source_config;
        };
        if num_elements == 0 {
            return source_config;
        }

        let special_job = num_elements;
        let half_sum = self.target.processing_times()[special_job][0];
        let middle_machine = (0..self.target.num_machines())
            .find(|&machine| schedule.start_times[special_job][machine] == half_sum)
            .unwrap_or_else(|| {
                let mut machines: Vec<usize> = (0..self.target.num_machines()).collect();
                machines
                    .sort_by_key(|&machine| (schedule.start_times[special_job][machine], machine));
                machines[self.target.num_machines() / 2]
            });
        let pivot = schedule.start_times[special_job][middle_machine];

        for (job, slot) in source_config.iter_mut().enumerate() {
            let completion = schedule.start_times[job][middle_machine]
                .checked_add(self.target.processing_times()[job][middle_machine])
                .expect("completion time overflowed u64");
            if completion <= pivot {
                *slot = 1;
            }
        }

        source_config
    }
}

#[reduction(overhead = {
    num_jobs = "num_elements + 1",
    num_machines = "3",
})]
impl ReduceTo<OpenShopScheduling> for Partition {
    type Result = ReductionPartitionToOpenShopScheduling;

    fn reduce_to(&self) -> Self::Result {
        let half_sum = self.total_sum() / 2;
        let mut processing_times: Vec<Vec<u64>> =
            self.sizes().iter().map(|&size| vec![size; 3]).collect();
        processing_times.push(vec![half_sum; 3]);

        ReductionPartitionToOpenShopScheduling {
            target: OpenShopScheduling::new(3, processing_times),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_open_shop_scheduling",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, OpenShopScheduling>(
                Partition::new(vec![1, 2, 3]),
                SolutionPair {
                    source_config: vec![0, 0, 1],
                    target_config: vec![0, 0, 0, 0, 2, 2, 0, 0, 3, 0, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_openshopscheduling.rs"]
mod tests;
