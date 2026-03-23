//! Sequencing with Release Times and Deadlines problem implementation.
//!
//! Given a set of tasks each with a processing time, release time, and deadline,
//! determine whether all tasks can be non-preemptively scheduled on one processor
//! such that each task starts after its release time and finishes by its deadline.
//! Strongly NP-complete (Garey & Johnson, A5 SS1).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, WitnessProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SequencingWithReleaseTimesAndDeadlines",
        display_name: "Sequencing with Release Times and Deadlines",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Single-machine scheduling feasibility: can all tasks be scheduled within their release-deadline windows without overlap?",
        fields: &[
            FieldInfo { name: "lengths", type_name: "Vec<u64>", description: "Processing time l(t) for each task (positive)" },
            FieldInfo { name: "release_times", type_name: "Vec<u64>", description: "Release time r(t) for each task (non-negative)" },
            FieldInfo { name: "deadlines", type_name: "Vec<u64>", description: "Deadline d(t) for each task (positive)" },
        ],
    }
}

/// Sequencing with Release Times and Deadlines.
///
/// Given a set of `n` tasks, each with a processing time `l(t)`, release time
/// `r(t)`, and deadline `d(t)`, determine whether there exists a one-processor
/// schedule where each task starts no earlier than its release time and finishes
/// by its deadline, with no two tasks overlapping.
///
/// # Representation
///
/// Uses a permutation encoding (Lehmer code), where `config[i]` selects which
/// remaining task to schedule next from the pool of unscheduled tasks.
/// `dims() = [n, n-1, ..., 2, 1]`. Tasks are scheduled left-to-right: each
/// task starts at `max(release_time, current_time)`. The schedule is feasible
/// iff every task finishes by its deadline.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::SequencingWithReleaseTimesAndDeadlines;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = SequencingWithReleaseTimesAndDeadlines::new(
///     vec![1, 2, 1],
///     vec![0, 0, 2],
///     vec![3, 3, 4],
/// );
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencingWithReleaseTimesAndDeadlines {
    lengths: Vec<u64>,
    release_times: Vec<u64>,
    deadlines: Vec<u64>,
}

impl SequencingWithReleaseTimesAndDeadlines {
    /// Create a new instance.
    ///
    /// # Panics
    ///
    /// Panics if the three vectors have different lengths.
    pub fn new(lengths: Vec<u64>, release_times: Vec<u64>, deadlines: Vec<u64>) -> Self {
        assert_eq!(lengths.len(), release_times.len());
        assert_eq!(lengths.len(), deadlines.len());
        Self {
            lengths,
            release_times,
            deadlines,
        }
    }

    /// Returns the processing times.
    pub fn lengths(&self) -> &[u64] {
        &self.lengths
    }

    /// Returns the release times.
    pub fn release_times(&self) -> &[u64] {
        &self.release_times
    }

    /// Returns the deadlines.
    pub fn deadlines(&self) -> &[u64] {
        &self.deadlines
    }

    /// Returns the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.lengths.len()
    }

    /// Returns the time horizon (maximum deadline).
    pub fn time_horizon(&self) -> u64 {
        self.deadlines.iter().copied().max().unwrap_or(0)
    }
}

impl Problem for SequencingWithReleaseTimesAndDeadlines {
    const NAME: &'static str = "SequencingWithReleaseTimesAndDeadlines";
    type Value = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.num_tasks();
        (0..n).rev().map(|i| i + 1).collect()
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let n = self.num_tasks();
        if config.len() != n {
            return false;
        }

        // Decode Lehmer code into a permutation of task indices.
        let mut available: Vec<usize> = (0..n).collect();
        let mut schedule = Vec::with_capacity(n);
        for &c in config.iter() {
            if c >= available.len() {
                return false;
            }
            schedule.push(available.remove(c));
        }

        // Schedule tasks left-to-right: each task starts at max(release_time, current_time).
        let mut current_time: u64 = 0;
        for &task in &schedule {
            let start = current_time.max(self.release_times[task]);
            let finish = start + self.lengths[task];
            if finish > self.deadlines[task] {
                return false;
            }
            current_time = finish;
        }

        true
    }
}

impl WitnessProblem for SequencingWithReleaseTimesAndDeadlines {}

crate::declare_variants! {
    default sat SequencingWithReleaseTimesAndDeadlines => "2^num_tasks * num_tasks",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "sequencing_with_release_times_and_deadlines",
        // 5 tasks from issue example.
        // Feasible schedule order: t3, t0, t1, t2, t4
        // Lehmer code [3,0,0,0,0] = permutation [3,0,1,2,4]
        instance: Box::new(SequencingWithReleaseTimesAndDeadlines::new(
            vec![3, 2, 4, 1, 2],
            vec![0, 1, 5, 0, 8],
            vec![5, 6, 10, 3, 12],
        )),
        optimal_config: vec![3, 0, 0, 0, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/sequencing_with_release_times_and_deadlines.rs"]
mod tests;
