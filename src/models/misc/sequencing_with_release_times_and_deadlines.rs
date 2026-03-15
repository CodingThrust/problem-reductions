//! Sequencing with Release Times and Deadlines problem implementation.
//!
//! Given a set of tasks each with a processing time, release time, and deadline,
//! determine whether all tasks can be non-preemptively scheduled on one processor
//! such that each task starts after its release time and finishes by its deadline.
//! Strongly NP-complete (Garey & Johnson, A5 SS1).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
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
/// Each variable represents the start time of a task. Variable `i` takes values
/// in `{0, 1, ..., max_deadline - 1}`. A configuration is feasible iff:
/// - `start[i] >= release_times[i]` for all `i`
/// - `start[i] + lengths[i] <= deadlines[i]` for all `i`
/// - No two tasks overlap in time
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
/// let solution = solver.find_satisfying(&problem);
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
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let h = self.time_horizon() as usize;
        if h == 0 {
            return vec![1; self.num_tasks()];
        }
        vec![h; self.num_tasks()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let n = self.num_tasks();
        if config.len() != n {
            return false;
        }

        // Check each task's release time and deadline constraints
        for (i, &start_val) in config.iter().enumerate() {
            let start = start_val as u64;
            if start < self.release_times[i] {
                return false;
            }
            if start + self.lengths[i] > self.deadlines[i] {
                return false;
            }
        }

        // Check no two tasks overlap: for all i != j,
        // either start[i] + length[i] <= start[j] or start[j] + length[j] <= start[i]
        for i in 0..n {
            for j in (i + 1)..n {
                let si = config[i] as u64;
                let ei = si + self.lengths[i];
                let sj = config[j] as u64;
                let ej = sj + self.lengths[j];
                if ei > sj && ej > si {
                    return false;
                }
            }
        }

        true
    }
}

impl SatisfactionProblem for SequencingWithReleaseTimesAndDeadlines {}

crate::declare_variants! {
    default sat SequencingWithReleaseTimesAndDeadlines => "2^num_tasks * num_tasks",
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/sequencing_with_release_times_and_deadlines.rs"]
mod tests;
