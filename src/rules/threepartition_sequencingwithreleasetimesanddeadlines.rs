//! Reduction from ThreePartition to SequencingWithReleaseTimesAndDeadlines.
//!
//! Given a 3-Partition instance with 3m elements of sizes s(a_i) and bound B,
//! construct a single-machine scheduling instance with:
//! - 3m element tasks: length = s(a_i), release = 0, deadline = m*B + (m-1)
//! - (m-1) filler tasks: length = 1, release = (j+1)*B + j, deadline = (j+1)*B + j + 1
//!
//! The filler tasks partition the timeline into m slots of width B each. Since
//! B/4 < s(a_i) < B/2, exactly 3 element tasks must fit in each slot, yielding
//! a valid 3-partition iff the schedule is feasible.
//!
//! Reference: Garey & Johnson, *Computers and Intractability*, Section 4.2.

use crate::models::misc::{SequencingWithReleaseTimesAndDeadlines, ThreePartition};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Number of element tasks (= source.num_elements() = 3m).
fn num_element_tasks(source: &ThreePartition) -> usize {
    source.num_elements()
}

/// Number of filler tasks (= m - 1).
fn num_filler_tasks(source: &ThreePartition) -> usize {
    source.num_groups() - 1
}

/// Result of reducing ThreePartition to SequencingWithReleaseTimesAndDeadlines.
#[derive(Debug, Clone)]
pub struct ReductionThreePartitionToSRTD {
    target: SequencingWithReleaseTimesAndDeadlines,
    /// Number of element tasks (3m) — first 3m tasks in the target are element tasks.
    num_element_tasks: usize,
    /// The bound B from the source.
    bound: u64,
}

impl ReductionResult for ReductionThreePartitionToSRTD {
    type Source = ThreePartition;
    type Target = SequencingWithReleaseTimesAndDeadlines;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a ThreePartition config from a target schedule config.
    ///
    /// Decode the Lehmer code to a task permutation, simulate the schedule to
    /// find each task's start time, then assign each element task to its slot
    /// based on start_time / (B + 1).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.target.num_tasks();
        // Decode Lehmer code to permutation
        let schedule = crate::models::misc::decode_lehmer(target_solution, n)
            .expect("target_solution must be a valid Lehmer code");

        // Simulate the schedule to find start times
        let mut current_time: u64 = 0;
        let mut slot_assignment = vec![0usize; self.num_element_tasks];
        let slot_width = self.bound + 1; // B + 1 (slot width including the filler gap)

        for &task in &schedule {
            let start = current_time.max(self.target.release_times()[task]);
            let finish = start + self.target.lengths()[task];
            current_time = finish;

            // Only element tasks (indices 0..3m) contribute to the partition
            if task < self.num_element_tasks {
                let slot = (start / slot_width) as usize;
                slot_assignment[task] = slot;
            }
        }

        slot_assignment
    }
}

#[reduction(overhead = {
    num_tasks = "num_elements + num_groups - 1",
})]
impl ReduceTo<SequencingWithReleaseTimesAndDeadlines> for ThreePartition {
    type Result = ReductionThreePartitionToSRTD;

    fn reduce_to(&self) -> Self::Result {
        let n_elem = num_element_tasks(self);
        let n_fill = num_filler_tasks(self);
        let m = self.num_groups();
        let b = self.bound();
        let total_tasks = n_elem + n_fill;

        // Time horizon: m*B + (m-1) = m*(B+1) - 1
        let horizon = (m as u64) * (b + 1) - 1;

        let mut lengths = Vec::with_capacity(total_tasks);
        let mut release_times = Vec::with_capacity(total_tasks);
        let mut deadlines = Vec::with_capacity(total_tasks);

        // Element tasks (indices 0..3m)
        for &size in self.sizes() {
            lengths.push(size);
            release_times.push(0);
            deadlines.push(horizon);
        }

        // Filler tasks (indices 3m..4m-1)
        for j in 0..n_fill {
            // Filler j separates slot j from slot j+1
            // Release = (j+1)*B + j, Deadline = (j+1)*B + j + 1
            let release = ((j + 1) as u64) * b + (j as u64);
            lengths.push(1);
            release_times.push(release);
            deadlines.push(release + 1);
        }

        ReductionThreePartitionToSRTD {
            target: SequencingWithReleaseTimesAndDeadlines::new(lengths, release_times, deadlines),
            num_element_tasks: n_elem,
            bound: b,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "threepartition_to_sequencingwithreleasetimesanddeadlines",
        build: || {
            // ThreePartition: sizes=[4,5,6,4,6,5], bound=15, m=2
            // Groups: {4,5,6}=15, {4,6,5}=15
            // Source config: [0,0,0,1,1,1] (elements 0,1,2 in group 0; 3,4,5 in group 1)
            //
            // Target: 6 element tasks + 1 filler = 7 tasks
            // Schedule for source config [0,0,0,1,1,1]:
            //   Slot 0 [0,15): tasks 0(len=4), 1(len=5), 2(len=6) -> times [0,4), [4,9), [9,15)
            //   Filler [15,16): task 6(len=1)
            //   Slot 1 [16,31): tasks 3(len=4), 4(len=6), 5(len=5) -> times [16,20), [20,26), [26,31)
            // Permutation: [0,1,2,6,3,4,5]
            // Lehmer code: [0,0,0,3,0,0,0]
            //   remaining=[0,1,2,3,4,5,6], pick 0 -> 0, remaining=[1,2,3,4,5,6]
            //   remaining=[1,2,3,4,5,6], pick 0 -> 1, remaining=[2,3,4,5,6]
            //   remaining=[2,3,4,5,6], pick 0 -> 2, remaining=[3,4,5,6]
            //   remaining=[3,4,5,6], pick 3 -> 6, remaining=[3,4,5]
            //   remaining=[3,4,5], pick 0 -> 3, remaining=[4,5]
            //   remaining=[4,5], pick 0 -> 4, remaining=[5]
            //   remaining=[5], pick 0 -> 5
            crate::example_db::specs::rule_example_with_witness::<
                _,
                SequencingWithReleaseTimesAndDeadlines,
            >(
                ThreePartition::new(vec![4, 5, 6, 4, 6, 5], 15),
                SolutionPair {
                    source_config: vec![0, 0, 0, 1, 1, 1],
                    target_config: vec![0, 0, 0, 3, 0, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/threepartition_sequencingwithreleasetimesanddeadlines.rs"]
mod tests;
