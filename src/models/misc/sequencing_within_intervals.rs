//! Sequencing Within Intervals problem implementation.
//!
//! Given a set of tasks, each with a release time, deadline, and processing length,
//! determine whether all tasks can be scheduled non-overlappingly such that each
//! task runs entirely within its allowed time window.

use crate::registry::{ConstructionError, CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SequencingWithinIntervals",
        display_name: "Sequencing Within Intervals",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Schedule tasks non-overlappingly within their time windows",
        fields: SequencingWithinIntervalsCreateSpec::FIELDS,
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
/// use problemreductions::{Problem, BruteForce};
///
/// // 3 tasks: release_times = [0, 2, 4], deadlines = [3, 5, 7], lengths = [2, 2, 2]
/// let problem = SequencingWithinIntervals::new(vec![0, 2, 4], vec![3, 5, 7], vec![2, 2, 2]).unwrap();
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct SequencingWithinIntervals {
    /// Release times for each task.
    release_times: Vec<i64>,
    /// Deadlines for each task.
    deadlines: Vec<i64>,
    /// Processing lengths for each task.
    lengths: Vec<i64>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct SequencingWithinIntervalsCreateSpec {
    /// Release times.
    release_times: Vec<i64>,
    /// Deadlines.
    deadlines: Vec<i64>,
    /// Processing lengths.
    lengths: Vec<i64>,
}
impl TryFrom<SequencingWithinIntervalsCreateSpec> for SequencingWithinIntervals {
    type Error = ConstructionError;
    fn try_from(spec: SequencingWithinIntervalsCreateSpec) -> Result<Self, Self::Error> {
        Self::new(spec.release_times, spec.deadlines, spec.lengths)
    }
}

impl SequencingWithinIntervals {
    /// Create a new SequencingWithinIntervals problem.
    ///
    pub fn new(
        release_times: Vec<i64>,
        deadlines: Vec<i64>,
        lengths: Vec<i64>,
    ) -> Result<Self, ConstructionError> {
        if release_times.len() != deadlines.len() {
            return Err(ConstructionError::Conversion(
                "release_times and deadlines must have the same length".into(),
            ));
        }
        if release_times.len() != lengths.len() {
            return Err(ConstructionError::Conversion(
                "release_times and lengths must have the same length".into(),
            ));
        }
        if release_times.iter().any(|&release| release < 0)
            || deadlines.iter().any(|&deadline| deadline < 0)
            || lengths.iter().any(|&length| length < 0)
        {
            return Err(ConstructionError::Conversion(
                "release times, deadlines, and lengths must be nonnegative".into(),
            ));
        }
        let mut total_slots = 0usize;
        for i in 0..release_times.len() {
            let sum = release_times[i].checked_add(lengths[i]).ok_or_else(|| {
                ConstructionError::IntegerOverflow(format!(
                    "task {i} release time plus length overflows i64"
                ))
            })?;
            if sum > deadlines[i] {
                return Err(ConstructionError::Conversion(format!(
                    "task {i} has an empty time window"
                )));
            }
            let slots = deadlines[i]
                .checked_sub(sum)
                .and_then(|slack| slack.checked_add(1))
                .ok_or_else(|| {
                    ConstructionError::IntegerOverflow(format!(
                        "task {i} start-slot count overflows i64"
                    ))
                })?;
            let slots = usize::try_from(slots).map_err(|_| {
                ConstructionError::IntegerOverflow(format!(
                    "task {i} start-slot count does not fit usize"
                ))
            })?;
            total_slots = total_slots.checked_add(slots).ok_or_else(|| {
                ConstructionError::IntegerOverflow("total start-slot count exceeds usize".into())
            })?;
        }
        Ok(Self {
            release_times,
            deadlines,
            lengths,
        })
    }

    /// Returns the release times.
    pub fn release_times(&self) -> &[i64] {
        &self.release_times
    }

    /// Returns the deadlines.
    pub fn deadlines(&self) -> &[i64] {
        &self.deadlines
    }

    /// Returns the processing lengths.
    pub fn lengths(&self) -> &[i64] {
        &self.lengths
    }

    /// Returns the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.release_times.len()
    }

    /// Return the total number of feasible start slots across all tasks.
    pub fn num_start_slots(&self) -> usize {
        self.release_times
            .iter()
            .zip(&self.deadlines)
            .zip(&self.lengths)
            .map(|((&release, &deadline), &length)| deadline - release - length + 1)
            .fold(0usize, |total, slots| {
                let slots = usize::try_from(slots).expect("start-slot count does not fit usize");
                total
                    .checked_add(slots)
                    .expect("total start-slot count overflow")
            })
    }
}

impl<'de> Deserialize<'de> for SequencingWithinIntervals {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            release_times: Vec<i64>,
            deadlines: Vec<i64>,
            lengths: Vec<i64>,
        }

        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.release_times, raw.deadlines, raw.lengths).map_err(serde::de::Error::custom)
    }
}

impl Problem for SequencingWithinIntervals {
    const NAME: &'static str = "SequencingWithinIntervals";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_size![
        ("num_start_slots", num_start_slots),
        ("num_tasks", num_tasks),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        Ok({
            crate::types::Or({
                let n = self.num_tasks();
                if config.len() != n {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        "sequence length does not match the tasks".into(),
                    ));
                }

                // Check each variable is within range and compute start times
                let mut starts = Vec::with_capacity(n);
                for (i, &c) in config.iter().enumerate() {
                    let dim =
                        (self.deadlines[i] - self.release_times[i] - self.lengths[i] + 1) as usize;
                    if c >= dim {
                        return Err(crate::traits::EvaluationError::InvalidConfiguration(
                            "schedule contains an out-of-range start offset".into(),
                        ));
                    }
                    // start = r[i] + c, and c < dim = d[i] - r[i] - l[i] + 1,
                    // so start + l[i] <= d[i] is guaranteed by construction.
                    let offset = i64::try_from(c).map_err(|_| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "converting a sequencing start offset to i64".into(),
                        )
                    })?;
                    let start = self.release_times[i].checked_add(offset).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "adding a sequencing start offset".into(),
                        )
                    })?;
                    starts.push(start);
                }
                let ends = starts
                    .iter()
                    .zip(&self.lengths)
                    .map(|(&start, &length)| {
                        start.checked_add(length).ok_or_else(|| {
                            crate::traits::EvaluationError::IntegerOverflow(
                                "computing a sequencing task end".into(),
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                // Check no two tasks overlap
                for i in 0..n {
                    for j in (i + 1)..n {
                        // Tasks overlap if neither finishes before the other starts
                        if !(ends[i] <= starts[j] || ends[j] <= starts[i]) {
                            return Ok(crate::types::Or(false));
                        }
                    }
                }

                true
            })
        })
    }
}

impl crate::solvers::BruteForceProblem for SequencingWithinIntervals {
    fn dimensions(&self) -> Vec<usize> {
        (0..self.num_tasks())
            .map(|i| (self.deadlines[i] - self.release_times[i] - self.lengths[i] + 1) as usize)
            .collect()
    }
}

crate::declare_variants! {
    default SequencingWithinIntervals => "2^num_tasks" create SequencingWithinIntervalsCreateSpec,
}

crate::register_brute_force! {
    SequencingWithinIntervals,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "sequencing_within_intervals",
        instance: Box::new(
            SequencingWithinIntervals::new(
                vec![0, 1, 3, 6, 0],
                vec![5, 8, 9, 12, 12],
                vec![2, 2, 2, 3, 2],
            )
            .expect("canonical sequencing-within-intervals instance must be valid"),
        ),
        optimal_config: serde_json::json!(vec![0, 1, 1, 0, 9]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/sequencing_within_intervals.rs"]
mod tests;
