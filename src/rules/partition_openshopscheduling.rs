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
        let Some(orders) = self.target.decode_orders(target_solution) else {
            return source_config;
        };
        if num_elements == 0 {
            return source_config;
        }

        let special_job = num_elements;
        let half_sum = self.target.processing_times()[special_job][0];

        // Find the middle machine and compute start times
        let makespan_orders = &orders;
        let n = self.target.num_jobs();
        let m = self.target.num_machines();

        // Simulate to get start times
        let mut machine_avail = vec![0usize; m];
        let mut job_avail = vec![0usize; n];
        let mut start_times = vec![vec![0usize; m]; n];

        // Schedule by processing the orders
        let mut cursor = vec![0usize; m];
        let total_ops = n * m;
        for _ in 0..total_ops {
            let mut best: Option<(usize, usize, usize)> = None; // (start, machine, job)
            for (mi, order) in makespan_orders.iter().enumerate() {
                if cursor[mi] < order.len() {
                    let job = order[cursor[mi]];
                    let start = machine_avail[mi].max(job_avail[job]);
                    if best.is_none_or(|(bs, _, _)| start < bs) {
                        best = Some((start, mi, job));
                    }
                }
            }
            let (start, mi, job) = best.expect("schedule incomplete");
            start_times[job][mi] = start;
            let end = start + self.target.processing_times()[job][mi];
            machine_avail[mi] = end;
            job_avail[job] = end;
            cursor[mi] += 1;
        }

        // Find the middle machine where the special job starts at half_sum
        let middle_machine = (0..m)
            .find(|&machine| start_times[special_job][machine] == half_sum)
            .unwrap_or_else(|| {
                let mut machines: Vec<usize> = (0..m).collect();
                machines.sort_by_key(|&machine| (start_times[special_job][machine], machine));
                machines[m / 2]
            });
        let pivot = start_times[special_job][middle_machine];

        for (job, slot) in source_config.iter_mut().enumerate() {
            let completion = start_times[job][middle_machine]
                + self.target.processing_times()[job][middle_machine];
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
        let half_sum = self.total_sum() as usize / 2;
        let mut processing_times: Vec<Vec<usize>> = self
            .sizes()
            .iter()
            .map(|&size| vec![size as usize; 3])
            .collect();
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
                    target_config: vec![0, 1, 2, 3, 0, 1, 2, 3, 2, 3, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_openshopscheduling.rs"]
mod tests;
