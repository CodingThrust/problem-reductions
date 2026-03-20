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
        display_name: "Flow Shop Scheduling",
        aliases: &[],
        dimensions: &[],
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
/// Configurations use Lehmer code encoding with `dims() = [n, n-1, ..., 1]`.
/// A config `[c_0, c_1, ..., c_{n-1}]` where `c_i < n - i` is decoded by
/// maintaining a list of available jobs and picking the `c_i`-th element:
///
/// For 3 jobs, config `[2, 0, 0]`: available=`[0,1,2]`, pick index 2 → job 2;
/// available=`[0,1]`, pick index 0 → job 0; available=`[1]`, pick index 0 → job 1.
/// Result: job order `[2, 0, 1]`.
///
/// Given a job order, start times are determined greedily (as early as possible).
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
        assert_eq!(
            n,
            self.task_lengths.len(),
            "job_order length ({}) does not match num_jobs ({})",
            n,
            self.task_lengths.len()
        );
        for (k, &job) in job_order.iter().enumerate() {
            assert!(
                job < self.task_lengths.len(),
                "job_order[{}] = {} is out of range (num_jobs = {})",
                k,
                job,
                self.task_lengths.len()
            );
        }
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
        (0..n).rev().map(|i| i + 1).collect()
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let n = self.num_jobs();
        if config.len() != n {
            return false;
        }

        // Decode Lehmer code into a permutation.
        // config[i] must be < n - i (the domain size for position i).
        let mut available: Vec<usize> = (0..n).collect();
        let mut job_order = Vec::with_capacity(n);
        for &c in config.iter() {
            if c >= available.len() {
                return false;
            }
            job_order.push(available.remove(c));
        }

        let makespan = self.compute_makespan(&job_order);
        makespan <= self.deadline
    }
}

impl SatisfactionProblem for FlowShopScheduling {}

crate::declare_variants! {
    default sat FlowShopScheduling => "factorial(num_jobs)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "flow_shop_scheduling",
        instance: Box::new(FlowShopScheduling::new(
            3,
            vec![
                vec![3, 4, 2],
                vec![2, 3, 5],
                vec![4, 1, 3],
                vec![1, 5, 4],
                vec![3, 2, 3],
            ],
            25,
        )),
        // Job order [3,0,4,2,1] = Lehmer code [3,0,2,1,0], makespan 23
        optimal_config: vec![3, 0, 2, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/flow_shop_scheduling.rs"]
mod tests;
