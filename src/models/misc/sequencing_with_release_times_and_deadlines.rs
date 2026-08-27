//! Sequencing with Release Times and Deadlines problem implementation.
//!
//! Given a set of tasks each with a processing time, release time, and deadline,
//! determine whether all tasks can be non-preemptively scheduled on one processor
//! such that each task starts after its release time and finishes by its deadline.
//! Strongly NP-complete (Garey & Johnson, A5 SS1).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SequencingWithReleaseTimesAndDeadlines",
        display_name: "Sequencing with Release Times and Deadlines",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Single-machine scheduling feasibility: can all tasks be scheduled within their release-deadline windows without overlap?",
        fields: &[
            FieldInfo { name: "lengths", type_name: "Vec<i64>", description: "Processing time l(t) for each task (positive)" },
            FieldInfo { name: "release_times", type_name: "Vec<i64>", description: "Release time r(t) for each task (non-negative)" },
            FieldInfo { name: "deadlines", type_name: "Vec<i64>", description: "Deadline d(t) for each task (positive)" },
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
/// use problemreductions::{Problem, BruteForce};
///
/// let problem = SequencingWithReleaseTimesAndDeadlines::new(
///     vec![1, 2, 1],
///     vec![0, 0, 2],
///     vec![3, 3, 4],
/// );
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencingWithReleaseTimesAndDeadlines {
    lengths: Vec<i64>,
    release_times: Vec<i64>,
    deadlines: Vec<i64>,
}

impl SequencingWithReleaseTimesAndDeadlines {
    /// Create a new instance.
    ///
    /// # Panics
    ///
    /// Panics if the three vectors have different lengths.
    pub fn new(lengths: Vec<i64>, release_times: Vec<i64>, deadlines: Vec<i64>) -> Self {
        assert_eq!(lengths.len(), release_times.len());
        assert_eq!(lengths.len(), deadlines.len());
        assert!(
            lengths.iter().all(|&length| length >= 0),
            "task lengths must be nonnegative"
        );
        assert!(
            release_times.iter().all(|&release| release >= 0),
            "release times must be nonnegative"
        );
        assert!(
            deadlines.iter().all(|&deadline| deadline >= 0),
            "deadlines must be nonnegative"
        );
        Self {
            lengths,
            release_times,
            deadlines,
        }
    }

    /// Returns the processing times.
    pub fn lengths(&self) -> &[i64] {
        &self.lengths
    }

    /// Returns the release times.
    pub fn release_times(&self) -> &[i64] {
        &self.release_times
    }

    /// Returns the deadlines.
    pub fn deadlines(&self) -> &[i64] {
        &self.deadlines
    }

    /// Returns the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.lengths.len()
    }

    /// Returns the time horizon (maximum deadline).
    pub fn time_horizon(&self) -> i64 {
        self.deadlines.iter().copied().max().unwrap_or(0)
    }
}

impl Problem for SequencingWithReleaseTimesAndDeadlines {
    const NAME: &'static str = "SequencingWithReleaseTimesAndDeadlines";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_size![("num_tasks", num_tasks), ("time_horizon", time_horizon),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        let n = self.num_tasks();
        if config.len() != n {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "schedule length does not match the tasks".into(),
            ));
        }
        if config.iter().any(|&task| task >= n) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "schedule contains an out-of-range task".into(),
            ));
        }
        Ok({
            crate::types::Or({
                let Some(schedule) = super::decode_permutation(config, self.num_tasks()) else {
                    return Ok(crate::types::Or(false));
                };

                // Schedule tasks left-to-right: each task starts at max(release_time, current_time).
                let mut current_time: i64 = 0;
                for &task in &schedule {
                    let start = current_time.max(self.release_times[task]);
                    let finish = start + self.lengths[task];
                    if finish > self.deadlines[task] {
                        return Ok(crate::types::Or(false));
                    }
                    current_time = finish;
                }

                true
            })
        })
    }
}

impl crate::solvers::BruteForceProblem for SequencingWithReleaseTimesAndDeadlines {
    fn dimensions(&self) -> Vec<usize> {
        super::lehmer_dims(self.num_tasks())
    }
}

crate::declare_variants! {
    default SequencingWithReleaseTimesAndDeadlines => "2^num_tasks * num_tasks",
}

crate::register_brute_force! {
    SequencingWithReleaseTimesAndDeadlines decode |problem: &SequencingWithReleaseTimesAndDeadlines, indices: Vec<usize>| super::decode_lehmer(&indices, problem.num_tasks()).expect("enumerated Lehmer digits are valid"),
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
        optimal_config: serde_json::json!(vec![3, 0, 1, 2, 4]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/sequencing_with_release_times_and_deadlines.rs"]
mod tests;
