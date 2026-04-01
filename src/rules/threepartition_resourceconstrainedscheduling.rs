//! Reduction from ThreePartition to ResourceConstrainedScheduling.
//!
//! Given a 3-Partition instance with 3m elements and target sum B (where each
//! element a_i satisfies B/4 < a_i < B/2), construct a ResourceConstrainedScheduling
//! instance with:
//! - 3m unit-length tasks (one per element)
//! - 3 processors (at most 3 tasks per time slot)
//! - 1 resource with bound B
//! - Resource requirement for task i = s(a_i)
//! - Deadline D = m (number of triples)
//!
//! A valid 3-partition exists iff the tasks can be feasibly scheduled:
//! the B/4 < a_i < B/2 constraint forces exactly 3 tasks per slot, and
//! the resource bound forces each slot's triple to sum to exactly B.
//!
//! Solution extraction is the identity: config[i] = time slot for task i
//! directly gives the group assignment for element i.
//!
//! Reference: Garey & Johnson, *Computers and Intractability*, Appendix A5.2.

use crate::models::misc::{ResourceConstrainedScheduling, ThreePartition};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing ThreePartition to ResourceConstrainedScheduling.
#[derive(Debug, Clone)]
pub struct ReductionThreePartitionToRCS {
    target: ResourceConstrainedScheduling,
}

impl ReductionResult for ReductionThreePartitionToRCS {
    type Source = ThreePartition;
    type Target = ResourceConstrainedScheduling;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: identity mapping.
    /// ThreePartition config (group index 0..m-1) maps directly to time slot assignment.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = {
    num_tasks = "num_elements",
})]
impl ReduceTo<ResourceConstrainedScheduling> for ThreePartition {
    type Result = ReductionThreePartitionToRCS;

    fn reduce_to(&self) -> Self::Result {
        let m = self.num_groups();
        let bound = self.bound();

        // Each element becomes a task with resource requirement = element size
        let resource_requirements: Vec<Vec<u64>> = self.sizes().iter().map(|&s| vec![s]).collect();

        ReductionThreePartitionToRCS {
            target: ResourceConstrainedScheduling::new(
                3,           // 3 processors
                vec![bound], // 1 resource with bound B
                resource_requirements,
                m as u64, // deadline = m time slots
            ),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "threepartition_to_resourceconstrainedscheduling",
        build: || {
            // sizes [4, 5, 6, 4, 6, 5], B=15, m=2
            // partition: {4,5,6} and {4,6,5} — both sum to 15
            // config: elements 0,1,2 in group 0; elements 3,4,5 in group 1
            crate::example_db::specs::rule_example_with_witness::<_, ResourceConstrainedScheduling>(
                ThreePartition::new(vec![4, 5, 6, 4, 6, 5], 15),
                SolutionPair {
                    source_config: vec![0, 0, 0, 1, 1, 1],
                    target_config: vec![0, 0, 0, 1, 1, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/threepartition_resourceconstrainedscheduling.rs"]
mod tests;
