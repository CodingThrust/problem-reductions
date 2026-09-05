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

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let num_elements = self.target.num_jobs() - 1;
            let mut source_config = vec![false; num_elements];
            let m = self.target.num_machines();
            let start_times = target_solution
                .chunks_exact(m)
                .map(|times| {
                    times
                        .iter()
                        .map(|&time| {
                            i64::try_from(time).map_err(|_| {
                                crate::rules::ExtractionError::invalid(
                                    "target schedule time does not fit i64",
                                )
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?;
            if num_elements == 0 {
                return Ok(source_config);
            }

            let special_job = num_elements;
            let half_sum = self.target.processing_times()[special_job][0];

            // Find the middle machine where the special job starts at half_sum
            let middle_machine = (0..m)
                .find(|&machine| start_times[special_job][machine] == half_sum)
                .ok_or_else(|| {
                    crate::rules::ExtractionError::invalid(
                        "target schedule has no machine at the partition boundary",
                    )
                })?;
            let pivot = start_times[special_job][middle_machine];

            for (job, slot) in source_config.iter_mut().enumerate() {
                let completion = start_times[job][middle_machine]
                    .checked_add(self.target.processing_times()[job][middle_machine])
                    .ok_or_else(|| {
                        crate::rules::ExtractionError::invalid("target schedule time overflows i64")
                    })?;
                if completion <= pivot {
                    *slot = true;
                }
            }

            source_config
        })
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
impl ReduceTo<OpenShopScheduling> for Partition {
    type Result = ReductionPartitionToOpenShopScheduling;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let half_sum = self.total_sum() / 2;
        let mut processing_times: Vec<Vec<i64>> =
            self.sizes().iter().map(|&size| vec![size; 3]).collect();
        processing_times.push(vec![half_sum; 3]);

        Ok(ReductionPartitionToOpenShopScheduling {
            target: OpenShopScheduling::try_new(3, processing_times)
                .map_err(<Self as ReduceTo<OpenShopScheduling>>::target_construction)?,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_open_shop_scheduling",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, OpenShopScheduling>(
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
