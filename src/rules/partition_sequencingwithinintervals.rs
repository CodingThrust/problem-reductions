//! Reduction from Partition to SequencingWithinIntervals.
//!
//! Given a Partition instance with sizes A = {a_1, ..., a_n} and total sum
//! S = sum(a_i), construct a SequencingWithinIntervals instance:
//!
//! - For each element a_i, create task t_i: release = 0, deadline = S + 1,
//!   length = a_i.
//! - Create an enforcer task t̄: release = floor(S/2), deadline = ceil((S+1)/2),
//!   length = 1.
//!
//! The enforcer is pinned at time S/2 (for even S), splitting the timeline into
//! two blocks of size S/2 each. A feasible schedule exists iff the original
//! sizes can be partitioned into two subsets of equal sum.
//!
//! Solution extraction: tasks starting before S/2 are assigned to subset 0,
//! tasks starting after the enforcer are assigned to subset 1.

use crate::models::misc::{Partition, SequencingWithinIntervals};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing Partition to SequencingWithinIntervals.
#[derive(Debug, Clone)]
pub struct ReductionPartitionToSWI {
    target: SequencingWithinIntervals,
    /// Number of elements in the original Partition (excludes the enforcer task).
    num_elements: usize,
    /// floor(S/2) — the boundary between the two blocks.
    half: u64,
}

impl ReductionResult for ReductionPartitionToSWI {
    type Source = Partition;
    type Target = SequencingWithinIntervals;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a Partition assignment from a SequencingWithinIntervals solution.
    ///
    /// The target config encodes start-time offsets from release times.
    /// For regular tasks (release = 0), the offset is the start time itself.
    /// Tasks starting before `half` belong to subset 0; tasks starting at or
    /// after `half + 1` belong to subset 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        (0..self.num_elements)
            .map(|i| {
                let start = target_solution[i] as u64; // release = 0, so offset = start
                if start > self.half {
                    1
                } else {
                    0
                }
            })
            .collect()
    }
}

#[reduction(overhead = {
    num_tasks = "num_elements + 1",
})]
impl ReduceTo<SequencingWithinIntervals> for Partition {
    type Result = ReductionPartitionToSWI;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_elements();
        let s = self.total_sum();
        let half = s / 2;

        // Regular tasks: one per element, release=0, deadline=S+1, length=a_i
        let mut release_times = vec![0u64; n];
        let mut deadlines = vec![s + 1; n];
        let mut lengths: Vec<u64> = self.sizes().to_vec();

        // Enforcer task: release=floor(S/2), deadline=ceil((S+1)/2), length=1
        let enforcer_release = half;
        let enforcer_deadline = (s + 1).div_ceil(2); // ceil((S+1)/2)
        release_times.push(enforcer_release);
        deadlines.push(enforcer_deadline);
        lengths.push(1);

        ReductionPartitionToSWI {
            target: SequencingWithinIntervals::new(release_times, deadlines, lengths),
            num_elements: n,
            half,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_sequencingwithinintervals",
        build: || {
            // sizes [1, 2, 3, 4], sum=10, half=5
            // partition: {2,3} in subset 0 (before enforcer), {1,4} in subset 1 (after enforcer)
            // Schedule: tasks 1,2 (lengths 2,3) fill [0,5), enforcer at [5,6), tasks 0,3 (lengths 1,4) fill [6,11)
            // Target config = start time offsets: task0=6, task1=0, task2=2, task3=7, enforcer=0
            crate::example_db::specs::rule_example_with_witness::<_, SequencingWithinIntervals>(
                Partition::new(vec![1, 2, 3, 4]),
                SolutionPair {
                    source_config: vec![1, 0, 0, 1],
                    target_config: vec![6, 0, 2, 7, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_sequencingwithinintervals.rs"]
mod tests;
