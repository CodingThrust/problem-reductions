//! Sequencing to Minimize Weighted Completion Time problem implementation.
//!
//! A classical NP-hard single-machine scheduling problem (SS4 from
//! Garey & Johnson, 1979) where tasks with processing times, weights,
//! and precedence constraints must be scheduled to minimize the total
//! weighted completion time.
//!
//! This model accepts zero-length tasks in addition to positive-length
//! tasks. That choice matches the standard Lawler reduction from
//! Optimal Linear Arrangement, which uses zero-length edge jobs instead
//! of padding them to unit length.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SequencingToMinimizeWeightedCompletionTime",
        display_name: "Sequencing to Minimize Weighted Completion Time",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Schedule tasks with lengths, weights, and precedence constraints to minimize total weighted completion time",
        fields: SequencingToMinimizeWeightedCompletionTimeCreateSpec::FIELDS,
    }
}

/// Sequencing to Minimize Weighted Completion Time problem.
///
/// Given tasks with nonnegative processing times `l(t)`, weights `w(t)`, and precedence
/// constraints, find a single-machine schedule that respects the precedences
/// and minimizes `sum_t w(t) * C(t)`, where `C(t)` is the completion time of
/// task `t`.
///
/// Configurations use Lehmer code with `dims() = [n, n-1, ..., 1]`.
#[derive(Debug, Clone, Serialize)]
pub struct SequencingToMinimizeWeightedCompletionTime {
    lengths: Vec<i64>,
    weights: Vec<i64>,
    precedences: Vec<(usize, usize)>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct SequencingToMinimizeWeightedCompletionTimeCreateSpec {
    lengths: Vec<i64>,
    weights: Vec<i64>,
    precedences: Option<Vec<(usize, usize)>>,
}

impl TryFrom<SequencingToMinimizeWeightedCompletionTimeCreateSpec>
    for SequencingToMinimizeWeightedCompletionTime
{
    type Error = crate::registry::ConstructionError;

    fn try_from(
        spec: SequencingToMinimizeWeightedCompletionTimeCreateSpec,
    ) -> Result<Self, Self::Error> {
        let precedences = spec.precedences.unwrap_or_default();
        Self::validate(&spec.lengths, &spec.weights, &precedences)?;
        Ok(Self::new(spec.lengths, spec.weights, precedences))
    }
}

#[derive(Deserialize)]
struct SequencingToMinimizeWeightedCompletionTimeSerde {
    lengths: Vec<i64>,
    weights: Vec<i64>,
    precedences: Vec<(usize, usize)>,
}

impl SequencingToMinimizeWeightedCompletionTime {
    fn validate(
        lengths: &[i64],
        weights: &[i64],
        precedences: &[(usize, usize)],
    ) -> Result<(), crate::registry::ConstructionError> {
        if lengths.len() != weights.len() {
            return Err("lengths length must equal weights length"
                .to_string()
                .into());
        }

        let num_tasks = lengths.len();
        for &(pred, succ) in precedences {
            if pred >= num_tasks {
                return Err(format!(
                    "predecessor index {} out of range (num_tasks = {})",
                    pred, num_tasks
                )
                .into());
            }
            if succ >= num_tasks {
                return Err(format!(
                    "successor index {} out of range (num_tasks = {})",
                    succ, num_tasks
                )
                .into());
            }
        }

        Ok(())
    }

    /// Create a new sequencing instance.
    ///
    /// # Panics
    ///
    /// Panics if `lengths.len() != weights.len()` or if any precedence
    /// endpoint is out of range.
    pub fn new(lengths: Vec<i64>, weights: Vec<i64>, precedences: Vec<(usize, usize)>) -> Self {
        Self::validate(&lengths, &weights, &precedences).unwrap_or_else(|err| panic!("{err}"));

        Self {
            lengths,
            weights,
            precedences,
        }
    }

    /// Returns the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.lengths.len()
    }

    /// Returns the processing times.
    pub fn lengths(&self) -> &[i64] {
        &self.lengths
    }

    /// Returns the task weights.
    pub fn weights(&self) -> &[i64] {
        &self.weights
    }

    /// Returns the precedence constraints.
    pub fn precedences(&self) -> &[(usize, usize)] {
        &self.precedences
    }

    /// Returns the number of precedence constraints.
    pub fn num_precedences(&self) -> usize {
        self.precedences.len()
    }

    fn decode_schedule(&self, config: &[usize]) -> Option<Vec<usize>> {
        super::decode_permutation(config, self.num_tasks())
    }

    fn weighted_completion_time(
        &self,
        schedule: &[usize],
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        let n = self.num_tasks();
        let mut positions = vec![0usize; n];
        let mut completion_times = vec![0i64; n];
        let mut elapsed = 0i64;

        for (position, &task) in schedule.iter().enumerate() {
            positions[task] = position;
            elapsed = elapsed.checked_add(self.lengths[task]).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing sequencing processing times".to_string(),
                )
            })?;
            completion_times[task] = elapsed;
        }

        for &(pred, succ) in &self.precedences {
            if positions[pred] >= positions[succ] {
                return Ok(Min(None));
            }
        }

        let total = completion_times
            .iter()
            .enumerate()
            .try_fold(0i64, |acc, (task, &completion)| -> Option<i64> {
                let weighted_completion = completion.checked_mul(self.weights[task])?;
                acc.checked_add(weighted_completion)
            })
            .ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "computing weighted completion time".to_string(),
                )
            })?;
        Ok(Min(Some(total)))
    }
}

impl TryFrom<SequencingToMinimizeWeightedCompletionTimeSerde>
    for SequencingToMinimizeWeightedCompletionTime
{
    type Error = crate::registry::ConstructionError;

    fn try_from(
        value: SequencingToMinimizeWeightedCompletionTimeSerde,
    ) -> Result<Self, Self::Error> {
        Self::validate(&value.lengths, &value.weights, &value.precedences)?;
        Ok(Self {
            lengths: value.lengths,
            weights: value.weights,
            precedences: value.precedences,
        })
    }
}

impl<'de> Deserialize<'de> for SequencingToMinimizeWeightedCompletionTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = SequencingToMinimizeWeightedCompletionTimeSerde::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}

impl Problem for SequencingToMinimizeWeightedCompletionTime {
    const NAME: &'static str = "SequencingToMinimizeWeightedCompletionTime";
    type Solution = Vec<usize>;
    type Value = Min<i64>;

    crate::problem_parameters![
        ("num_precedences", num_precedences),
        ("num_tasks", num_tasks),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
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
            let Some(schedule) = self.decode_schedule(config) else {
                return Ok(Min(None));
            };
            self.weighted_completion_time(&schedule)?
        })
    }
}

impl crate::solvers::BruteForceProblem for SequencingToMinimizeWeightedCompletionTime {
    fn dimensions(&self) -> Vec<usize> {
        super::lehmer_dims(self.num_tasks())
    }
}

crate::declare_variants! {
    default SequencingToMinimizeWeightedCompletionTime => "factorial(num_tasks)" create SequencingToMinimizeWeightedCompletionTimeCreateSpec,
}

crate::register_brute_force! {
    SequencingToMinimizeWeightedCompletionTime decode |problem: &SequencingToMinimizeWeightedCompletionTime, indices: Vec<usize>| super::decode_lehmer(&indices, problem.num_tasks()).expect("enumerated Lehmer digits are valid"),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "sequencing_to_minimize_weighted_completion_time",
        instance: Box::new(SequencingToMinimizeWeightedCompletionTime::new(
            vec![2, 1, 3, 1, 2],
            vec![3, 5, 1, 4, 2],
            vec![(0, 2), (1, 4)],
        )),
        optimal_config: serde_json::json!(vec![1, 3, 0, 4, 2]),
        optimal_value: serde_json::json!(46),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/sequencing_to_minimize_weighted_completion_time.rs"]
mod tests;
