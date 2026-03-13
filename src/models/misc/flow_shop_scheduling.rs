//! Flow Shop Scheduling problem implementation.
//!
//! Given m processors and a set of jobs, each consisting of m tasks (one per processor)
//! that must be processed in processor order 1, 2, ..., m, determine if all jobs can
//! be completed by a global deadline D.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "FlowShopScheduling",
        module_path: module_path!(),
        description: "Determine if a flow-shop schedule for jobs on m processors meets a deadline",
        fields: &[
            FieldInfo { name: "num_processors", type_name: "usize", description: "Number of machines m" },
            FieldInfo { name: "task_lengths", type_name: "Vec<Vec<u64>>", description: "task_lengths[j][i] = length of job j's task on machine i" },
            FieldInfo { name: "deadline", type_name: "u64", description: "Global deadline D" },
        ],
    }
}

/// The Flow Shop Scheduling problem.
///
/// Given `m` processors and a set of `n` jobs, each job `j` consists of `m` tasks
/// `t_1[j], t_2[j], ..., t_m[j]` with specified lengths. Tasks must be processed
/// in processor order: job `j` cannot start on machine `i+1` until its task on
/// machine `i` is completed. The question is whether there exists a schedule such
/// that all jobs complete by deadline `D`.
///
/// # Representation
///
/// Each variable represents a job's position in the processing sequence.
/// A valid configuration is a permutation of `{0, ..., n-1}`.
/// Given a permutation, start times are determined greedily (as early as possible).
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::FlowShopScheduling;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 2 machines, 3 jobs, deadline 10
/// let problem = FlowShopScheduling::new(2, vec![vec![2, 3], vec![3, 2], vec![1, 4]], 10);
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowShopScheduling {
    /// Number of processors (machines).
    num_processors: usize,
    /// Task lengths: `task_lengths[j][i]` is the processing time of job `j` on machine `i`.
    task_lengths: Vec<Vec<u64>>,
    /// Global deadline.
    deadline: u64,
}

impl FlowShopScheduling {
    /// Create a new Flow Shop Scheduling instance.
    ///
    /// # Arguments
    /// * `num_processors` - Number of machines m
    /// * `task_lengths` - task_lengths[j][i] = processing time of job j on machine i.
    ///   Each inner Vec must have length `num_processors`.
    /// * `deadline` - Global deadline D
    ///
    /// # Panics
    /// Panics if any job does not have exactly `num_processors` tasks.
    pub fn new(num_processors: usize, task_lengths: Vec<Vec<u64>>, deadline: u64) -> Self {
        for (j, tasks) in task_lengths.iter().enumerate() {
            assert_eq!(
                tasks.len(),
                num_processors,
                "Job {} has {} tasks, expected {}",
                j,
                tasks.len(),
                num_processors
            );
        }
        Self {
            num_processors,
            task_lengths,
            deadline,
        }
    }

    /// Get the number of processors.
    pub fn num_processors(&self) -> usize {
        self.num_processors
    }

    /// Get the task lengths matrix.
    pub fn task_lengths(&self) -> &[Vec<u64>] {
        &self.task_lengths
    }

    /// Get the deadline.
    pub fn deadline(&self) -> u64 {
        self.deadline
    }

    /// Get the number of jobs.
    pub fn num_jobs(&self) -> usize {
        self.task_lengths.len()
    }

    /// Compute the makespan for a given job ordering.
    ///
    /// The job_order slice must be a permutation of `0..num_jobs`.
    /// Returns the completion time of the last job on the last machine.
    pub fn compute_makespan(&self, job_order: &[usize]) -> u64 {
        let n = job_order.len();
        let m = self.num_processors;
        if n == 0 || m == 0 {
            return 0;
        }

        // completion[k][i] = completion time of the k-th job in sequence on machine i
        let mut completion = vec![vec![0u64; m]; n];

        for (k, &job) in job_order.iter().enumerate() {
            for i in 0..m {
                let prev_machine = if i == 0 { 0 } else { completion[k][i - 1] };
                let prev_job = if k == 0 { 0 } else { completion[k - 1][i] };
                let start = prev_machine.max(prev_job);
                completion[k][i] = start + self.task_lengths[job][i];
            }
        }

        completion[n - 1][m - 1]
    }
}

impl Problem for FlowShopScheduling {
    const NAME: &'static str = "FlowShopScheduling";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.num_jobs();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let n = self.num_jobs();
        if config.len() != n {
            return false;
        }

        // Check that config is a valid permutation of 0..n
        let mut seen = vec![false; n];
        for &pos in config {
            if pos >= n || seen[pos] {
                return false;
            }
            seen[pos] = true;
        }

        // config[j] = position of job j in the sequence
        // We need to convert to job_order: job_order[position] = job
        let mut job_order = vec![0usize; n];
        for (job, &pos) in config.iter().enumerate() {
            job_order[pos] = job;
        }

        let makespan = self.compute_makespan(&job_order);
        makespan <= self.deadline
    }
}

impl SatisfactionProblem for FlowShopScheduling {}

crate::declare_variants! {
    FlowShopScheduling => "3^num_jobs",
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/flow_shop_scheduling.rs"]
mod tests;
