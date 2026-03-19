//! Sequencing Within Intervals problem implementation.
//!
//! Given a set of tasks, each with a release time, deadline, and processing length,
//! determine whether all tasks can be scheduled non-overlappingly such that each
//! task runs entirely within its allowed time window.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::de::Error as _;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SequencingWithinIntervals",
        display_name: "Sequencing Within Intervals",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Schedule tasks non-overlappingly within their time windows",
        fields: &[
            FieldInfo { name: "release_times", type_name: "Vec<u64>", description: "Release time r(t) for each task" },
            FieldInfo { name: "deadlines", type_name: "Vec<u64>", description: "Deadline d(t) for each task" },
            FieldInfo { name: "lengths", type_name: "Vec<u64>", description: "Processing length l(t) for each task" },
        ],
    }
}

/// Sequencing Within Intervals problem.
///
/// Given `n` tasks, each with release time `r(t)`, deadline `d(t)`, and processing
/// length `l(t)`, determine whether there exists a schedule `sigma: T -> Z_>=0`
/// such that:
/// - `sigma(t) >= r(t)` (task starts no earlier than its release time)
/// - `sigma(t) + l(t) <= d(t)` (task finishes by its deadline)
/// - No two tasks overlap in time
///
/// This is problem SS1 from Garey & Johnson (1979), NP-complete via Theorem 3.8.
///
/// # Representation
///
/// Each task has a variable representing its start time offset from the release time.
/// Variable `i` takes values in `{0, ..., d(i) - r(i) - l(i)}`, so the actual start
/// time is `r(i) + config[i]`.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::SequencingWithinIntervals;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 3 tasks: release_times = [0, 2, 4], deadlines = [3, 5, 7], lengths = [2, 2, 2]
/// let problem = SequencingWithinIntervals::new(vec![0, 2, 4], vec![3, 5, 7], vec![2, 2, 2]);
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct SequencingWithinIntervals {
    /// Release times for each task.
    release_times: Vec<u64>,
    /// Deadlines for each task.
    deadlines: Vec<u64>,
    /// Processing lengths for each task.
    lengths: Vec<u64>,
}

#[derive(Debug, Deserialize)]
struct SequencingWithinIntervalsUnchecked {
    release_times: Vec<u64>,
    deadlines: Vec<u64>,
    lengths: Vec<u64>,
}

impl SequencingWithinIntervals {
    /// Create a new SequencingWithinIntervals problem.
    ///
    /// # Panics
    /// Panics if the three vectors have different lengths, or if any task has
    /// `r(i) + l(i) > d(i)` (empty time window).
    pub fn new(release_times: Vec<u64>, deadlines: Vec<u64>, lengths: Vec<u64>) -> Self {
        validate_time_windows(&release_times, &deadlines, &lengths);
        Self {
            release_times,
            deadlines,
            lengths,
        }
    }

    /// Returns the release times.
    pub fn release_times(&self) -> &[u64] {
        &self.release_times
    }

    /// Returns the deadlines.
    pub fn deadlines(&self) -> &[u64] {
        &self.deadlines
    }

    /// Returns the processing lengths.
    pub fn lengths(&self) -> &[u64] {
        &self.lengths
    }

    /// Returns the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.release_times.len()
    }
}

impl<'de> Deserialize<'de> for SequencingWithinIntervals {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let unchecked = SequencingWithinIntervalsUnchecked::deserialize(deserializer)?;
        if let Some(message) = time_window_validation_error(
            &unchecked.release_times,
            &unchecked.deadlines,
            &unchecked.lengths,
        ) {
            return Err(D::Error::custom(message));
        }
        Ok(Self {
            release_times: unchecked.release_times,
            deadlines: unchecked.deadlines,
            lengths: unchecked.lengths,
        })
    }
}

fn validate_time_windows(release_times: &[u64], deadlines: &[u64], lengths: &[u64]) {
    if let Some(message) = time_window_validation_error(release_times, deadlines, lengths) {
        panic!("{message}");
    }
}

fn time_window_validation_error(
    release_times: &[u64],
    deadlines: &[u64],
    lengths: &[u64],
) -> Option<String> {
    if release_times.len() != deadlines.len() {
        return Some("release_times and deadlines must have the same length".to_string());
    }
    if release_times.len() != lengths.len() {
        return Some("release_times and lengths must have the same length".to_string());
    }

    for (i, ((&release_time, &deadline), &length)) in release_times
        .iter()
        .zip(deadlines.iter())
        .zip(lengths.iter())
        .enumerate()
    {
        let end = match release_time.checked_add(length) {
            Some(end) => end,
            None => return Some(format!("Task {i}: overflow computing r(i) + l(i)")),
        };
        if end > deadline {
            return Some(format!(
                "Task {i}: r({}) + l({}) > d({}), time window is empty",
                release_time, length, deadline
            ));
        }
    }

    None
}

impl Problem for SequencingWithinIntervals {
    const NAME: &'static str = "SequencingWithinIntervals";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        (0..self.num_tasks())
            .map(|i| (self.deadlines[i] - self.release_times[i] - self.lengths[i] + 1) as usize)
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let n = self.num_tasks();
        if config.len() != n {
            return false;
        }

        // Check each variable is within range and compute start times
        let mut starts = Vec::with_capacity(n);
        for (i, &c) in config.iter().enumerate() {
            let dim = (self.deadlines[i] - self.release_times[i] - self.lengths[i] + 1) as usize;
            if c >= dim {
                return false;
            }
            // start = r[i] + c, and c < dim = d[i] - r[i] - l[i] + 1,
            // so start + l[i] <= d[i] is guaranteed by construction.
            let start = self.release_times[i] + c as u64;
            starts.push(start);
        }

        // Check no two tasks overlap
        for i in 0..n {
            for j in (i + 1)..n {
                let end_i = starts[i] + self.lengths[i];
                let end_j = starts[j] + self.lengths[j];
                // Tasks overlap if neither finishes before the other starts
                if !(end_i <= starts[j] || end_j <= starts[i]) {
                    return false;
                }
            }
        }

        true
    }
}

impl SatisfactionProblem for SequencingWithinIntervals {}

crate::declare_variants! {
    default sat SequencingWithinIntervals => "2^num_tasks",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "sequencing_within_intervals",
        instance: Box::new(SequencingWithinIntervals::new(
            vec![0, 1, 3, 6, 0],
            vec![5, 8, 9, 12, 12],
            vec![2, 2, 2, 3, 2],
        )),
        optimal_config: vec![0, 1, 1, 0, 9],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/sequencing_within_intervals.rs"]
mod tests;
