//! Reduction from Partition to MultiprocessorScheduling.
//!
//! Given a Partition instance with sizes A = {a_1, ..., a_n}, construct a
//! MultiprocessorScheduling instance with:
//! - Tasks: one per element, with length equal to the element's size
//! - m = 2 processors
//! - Deadline D = floor(total_sum / 2)
//!
//! A valid partition (two subsets of equal sum) exists iff the tasks can be
//! scheduled on 2 processors with makespan at most D.
//!
//! Solution extraction is the identity: the binary subset assignment in Partition
//! directly corresponds to the processor assignment in MultiprocessorScheduling.

use crate::models::misc::{MultiprocessorScheduling, Partition};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing Partition to MultiprocessorScheduling.
#[derive(Debug, Clone)]
pub struct ReductionPartitionToMPS {
    target: MultiprocessorScheduling,
}

impl ReductionResult for ReductionPartitionToMPS {
    type Source = Partition;
    type Target = MultiprocessorScheduling;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: identity mapping.
    /// Partition config (0/1 for subset) maps directly to processor assignment (0/1).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = {
    num_tasks = "num_elements",
})]
impl ReduceTo<MultiprocessorScheduling> for Partition {
    type Result = ReductionPartitionToMPS;

    fn reduce_to(&self) -> Self::Result {
        let lengths: Vec<u64> = self.sizes().to_vec();
        let deadline = self.total_sum() / 2;

        ReductionPartitionToMPS {
            target: MultiprocessorScheduling::new(lengths, 2, deadline),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_multiprocessorscheduling",
        build: || {
            // sizes [1, 2, 3, 4], sum=10, target=5
            // partition: {1,4} on proc 0 and {2,3} on proc 1
            crate::example_db::specs::rule_example_with_witness::<_, MultiprocessorScheduling>(
                Partition::new(vec![1, 2, 3, 4]),
                SolutionPair {
                    source_config: vec![0, 1, 1, 0],
                    target_config: vec![0, 1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_multiprocessorscheduling.rs"]
mod tests;
