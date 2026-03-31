//! Reduction from ThreePartition to JobShopScheduling.
//!
//! Given a 3-Partition instance with 3m positive integers (each strictly between
//! B/4 and B/2) that must be partitioned into m triples summing to B, construct a
//! Job-Shop Scheduling instance on 2 processors:
//!
//! - **Element jobs** (3m jobs): job i has tasks [(0, s(a_i)), (1, s(a_i))].
//! - **Separator jobs** (m-1 jobs): job k has a single task [(0, L)] where L = m*B + 1.
//!
//! The separators force m windows of size B on processor 0. A valid 3-partition
//! exists iff the optimal makespan equals the threshold D = m*B + (m-1)*L.
//!
//! Solution extraction: decode the processor-0 Lehmer code to find the task
//! ordering, locate the separator boundaries, and assign each element to the
//! group (window) it occupies.
//!
//! Reference: Garey, Johnson & Sethi (1976). "The complexity of flowshop and
//! jobshop scheduling." Mathematics of Operations Research 1, pp. 117-129.

use crate::models::misc::{JobShopScheduling, ThreePartition};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing ThreePartition to JobShopScheduling.
#[derive(Debug, Clone)]
pub struct ReductionThreePartitionToJSS {
    target: JobShopScheduling,
    /// Number of elements (3m) in the source problem.
    num_elements: usize,
    /// Number of groups (m) in the source problem.
    num_groups: usize,
    /// The makespan threshold: schedules achieving this makespan correspond
    /// to valid 3-partitions.
    threshold: u64,
}

impl ReductionThreePartitionToJSS {
    /// The makespan threshold D: a valid 3-partition exists iff the optimal
    /// makespan of the target JSS instance equals D.
    pub fn threshold(&self) -> u64 {
        self.threshold
    }

    /// Compute the makespan threshold D = m*B + (m-1)*L where L = m*B + 1.
    fn compute_threshold(num_groups: usize, bound: u64) -> u64 {
        let m = num_groups as u64;
        let b = bound;
        let l = m * b + 1;
        m * b + (m - 1) * l
    }

    /// Compute the separator length L = m*B + 1.
    fn separator_length(num_groups: usize, bound: u64) -> u64 {
        (num_groups as u64) * bound + 1
    }
}

impl ReductionResult for ReductionThreePartitionToJSS {
    type Source = ThreePartition;
    type Target = JobShopScheduling;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // The target config encodes Lehmer codes for each machine's tasks.
        // Machine 0 has: 3m element tasks (task index 0 of each element job)
        //                + (m-1) separator tasks
        //              = 3m + (m-1) tasks total
        // Machine 1 has: 3m element tasks (task index 1 of each element job)
        //
        // The config layout is [machine_0_lehmer..., machine_1_lehmer...].
        // machine_0_lehmer has length (3m + m - 1) = 4m - 1.
        //
        // We decode machine 0's ordering to find which group each element
        // belongs to: elements between separators k-1 and k form group k.

        let num_elem = self.num_elements;
        let m = self.num_groups;

        // Number of tasks on machine 0: element tasks + separator tasks
        let machine0_len = num_elem + (m - 1);

        // Decode machine 0 Lehmer code
        let machine0_lehmer = &target_solution[..machine0_len];
        let machine0_order = crate::models::misc::decode_lehmer(machine0_lehmer, machine0_len)
            .expect("valid Lehmer code for machine 0");

        // Task IDs on machine 0:
        // - Element job i contributes task at flat index 2*i (first task of job i).
        // - Separator job k contributes task at flat index 2*num_elem + k.
        //
        // Build mapping: flat task ID -> element index or separator marker.
        let separator_task_ids: Vec<usize> = (0..m - 1).map(|k| 2 * num_elem + k).collect();

        // machine0_order gives the order of task indices assigned to machine 0.
        // The flatten_tasks() in JobShopScheduling assigns IDs sequentially:
        // job 0 tasks get ids [0, 1], job 1 tasks get [2, 3], ...
        // Element job i (2 tasks): ids [2*i, 2*i+1]
        // Separator job k (1 task): id [2*num_elem + k]
        //
        // Machine 0 tasks are: element task 2*i (for i in 0..num_elem) and
        // separator task 2*num_elem+k (for k in 0..m-1).
        // Machine 1 tasks are: element task 2*i+1 (for i in 0..num_elem).
        //
        // The machine_task_ids for machine 0 are ordered by job index (since
        // flatten_tasks iterates jobs in order): [0, 2, 4, ..., 2*(num_elem-1),
        // 2*num_elem, 2*num_elem+1, ...].
        //
        // machine0_order[j] gives the j-th machine-local index in the Lehmer
        // permutation, which maps to machine_task_ids[machine0_order[j]].

        // Build the machine 0 task id list in the same order as flatten_tasks
        let mut machine0_task_ids: Vec<usize> = Vec::with_capacity(machine0_len);
        for i in 0..num_elem {
            machine0_task_ids.push(2 * i); // element job i, task 0 (on machine 0)
        }
        for k in 0..m - 1 {
            machine0_task_ids.push(2 * num_elem + k); // separator job k
        }

        // The actual ordering of tasks on machine 0:
        let ordered_task_ids: Vec<usize> = machine0_order
            .iter()
            .map(|&local_idx| machine0_task_ids[local_idx])
            .collect();

        // Now assign groups: walk through ordered_task_ids, incrementing group
        // at each separator.
        let mut config = vec![0usize; num_elem];
        let mut current_group = 0usize;

        for &task_id in &ordered_task_ids {
            if separator_task_ids.contains(&task_id) {
                current_group += 1;
            } else {
                // This is an element task with flat id 2*i => element i
                let element_index = task_id / 2;
                config[element_index] = current_group;
            }
        }

        config
    }
}

#[reduction(overhead = {
    num_jobs = "num_elements + num_groups - 1",
    num_tasks = "2 * num_elements + num_groups - 1",
})]
impl ReduceTo<JobShopScheduling> for ThreePartition {
    type Result = ReductionThreePartitionToJSS;

    fn reduce_to(&self) -> Self::Result {
        let num_elements = self.num_elements();
        let m = self.num_groups();
        let bound = self.bound();
        let l = ReductionThreePartitionToJSS::separator_length(m, bound);
        let threshold = ReductionThreePartitionToJSS::compute_threshold(m, bound);

        // Build jobs
        let mut jobs: Vec<Vec<(usize, u64)>> = Vec::with_capacity(num_elements + m - 1);

        // Element jobs: 2 tasks each, one on each processor
        for &size in self.sizes() {
            jobs.push(vec![(0, size), (1, size)]);
        }

        // Separator jobs: 1 task each, on processor 0
        for _ in 0..m.saturating_sub(1) {
            jobs.push(vec![(0, l)]);
        }

        ReductionThreePartitionToJSS {
            target: JobShopScheduling::new(2, jobs),
            num_elements,
            num_groups: m,
            threshold,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "threepartition_to_jobshopscheduling",
        build: || {
            // m=1: sizes [4, 5, 6], bound=15, one group
            // 3 element jobs, 0 separators => 3 jobs, 6 tasks
            // All elements go to group 0: config = [0, 0, 0]
            let source = ThreePartition::new(vec![4, 5, 6], 15);
            let reduction = ReduceTo::<JobShopScheduling>::reduce_to(&source);

            // For m=1, any ordering works. Use identity ordering on both machines.
            // Machine 0: 3 tasks => Lehmer [0, 0, 0]
            // Machine 1: 3 tasks => Lehmer [0, 0, 0]
            let target_config = vec![0, 0, 0, 0, 0, 0];

            crate::example_db::specs::rule_example_with_witness::<_, JobShopScheduling>(
                source,
                SolutionPair {
                    source_config: vec![0, 0, 0],
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/threepartition_jobshopscheduling.rs"]
mod tests;
