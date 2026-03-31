//! Reduction from ThreePartition to FlowShopScheduling.
//!
//! Given a 3-Partition instance with 3m elements of sizes s(a_i) and bound B,
//! construct a 3-machine flow-shop scheduling instance:
//!
//! - 3m "element jobs": job i has task_lengths = [s(a_i), s(a_i), s(a_i)]
//! - (m-1) "separator jobs": task_lengths = [0, L, 0] where L = m*B + 1
//! - Deadline D = makespan of a canonical schedule (computed via compute_makespan)
//!
//! A valid 3-partition exists iff the flow-shop schedule meets deadline D.
//! The large separator tasks on machine 2 force exactly 3 element jobs
//! (summing to B) between consecutive separators.
//!
//! Solution extraction: decode Lehmer code to job order, count separators
//! to determine which group each element job belongs to.

use crate::models::misc::{FlowShopScheduling, ThreePartition};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing ThreePartition to FlowShopScheduling.
#[derive(Debug, Clone)]
pub struct ReductionThreePartitionToFSS {
    target: FlowShopScheduling,
    /// Number of elements (3m) in the source problem.
    num_elements: usize,
}

impl ReductionResult for ReductionThreePartitionToFSS {
    type Source = ThreePartition;
    type Target = FlowShopScheduling;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract source solution from target solution.
    ///
    /// The target config is a Lehmer code encoding a job permutation.
    /// Decode to job order, then walk through counting separators
    /// to assign each element job to a group.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.target.num_jobs();
        let job_order =
            crate::models::misc::decode_lehmer(target_solution, n).expect("valid Lehmer code");

        let mut config = vec![0usize; self.num_elements];
        let mut current_group = 0;

        for &job in &job_order {
            if job < self.num_elements {
                // Element job: assign to current group
                config[job] = current_group;
            } else {
                // Separator job: advance to next group
                current_group += 1;
            }
        }

        config
    }
}

#[reduction(overhead = {
    num_jobs = "num_elements + num_groups - 1",
})]
impl ReduceTo<FlowShopScheduling> for ThreePartition {
    type Result = ReductionThreePartitionToFSS;

    fn reduce_to(&self) -> Self::Result {
        let num_elements = self.num_elements();
        let num_groups = self.num_groups();
        let bound = self.bound();

        // L = m * B + 1 — large enough to force grouping
        let big_l = (num_groups as u64) * bound + 1;

        // Build task_lengths: element jobs first, then separator jobs
        let mut task_lengths = Vec::with_capacity(num_elements + num_groups - 1);

        // Element jobs: identical task length on all 3 machines
        for &size in self.sizes() {
            task_lengths.push(vec![size, size, size]);
        }

        // Separator jobs: [0, L, 0]
        for _ in 0..num_groups.saturating_sub(1) {
            task_lengths.push(vec![0, big_l, 0]);
        }

        // Compute deadline from canonical schedule.
        // Canonical order: group1 elements, sep1, group2 elements, sep2, ...
        // We use a valid partition ordering to compute the achievable makespan.
        let canonical_order: Vec<usize> = {
            let mut order = Vec::with_capacity(num_elements + num_groups - 1);
            for g in 0..num_groups {
                // Add 3 element jobs per group (in natural order)
                for i in 0..3 {
                    order.push(g * 3 + i);
                }
                // Add separator after each group except the last
                if g < num_groups - 1 {
                    order.push(num_elements + g);
                }
            }
            order
        };

        let target_no_deadline = FlowShopScheduling::new(3, task_lengths.clone(), u64::MAX);
        let deadline = target_no_deadline.compute_makespan(&canonical_order);

        let target = FlowShopScheduling::new(3, task_lengths, deadline);

        ReductionThreePartitionToFSS {
            target,
            num_elements,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "threepartition_to_flowshopscheduling",
        build: || {
            // ThreePartition: sizes [4, 5, 6, 4, 6, 5], bound=15, m=2
            // Valid partition: {4,5,6} and {4,6,5}
            let source = ThreePartition::new(vec![4, 5, 6, 4, 6, 5], 15);
            let reduction = ReduceTo::<FlowShopScheduling>::reduce_to(&source);
            let target = reduction.target_problem();

            // Canonical order: elements [0,1,2], separator [6], elements [3,4,5]
            // Lehmer encode: job order [0,1,2,6,3,4,5]
            // For Lehmer encoding of [0,1,2,6,3,4,5]:
            //   available=[0,1,2,3,4,5,6], pick 0 -> index 0; available=[1,2,3,4,5,6]
            //   available=[1,2,3,4,5,6], pick 1 -> index 0; available=[2,3,4,5,6]
            //   available=[2,3,4,5,6], pick 2 -> index 0; available=[3,4,5,6]
            //   available=[3,4,5,6], pick 6 -> index 3; available=[3,4,5]
            //   available=[3,4,5], pick 3 -> index 0; available=[4,5]
            //   available=[4,5], pick 4 -> index 0; available=[5]
            //   available=[5], pick 5 -> index 0;
            let target_config = vec![0, 0, 0, 3, 0, 0, 0];

            // Source config: element 0,1,2 -> group 0; element 3,4,5 -> group 1
            let source_config = vec![0, 0, 0, 1, 1, 1];

            crate::example_db::specs::assemble_rule_example(
                &source,
                target,
                vec![SolutionPair {
                    source_config,
                    target_config,
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/threepartition_flowshopscheduling.rs"]
mod tests;
