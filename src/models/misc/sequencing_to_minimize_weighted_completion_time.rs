//! Sequencing to Minimize Weighted Completion Time problem implementation.
//!
//! A classical NP-hard single-machine scheduling problem (SS4 from
//! Garey & Johnson, 1979) where tasks with processing times, weights,
//! and precedence constraints must be scheduled to minimize the total
//! weighted completion time.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SequencingToMinimizeWeightedCompletionTime",
        display_name: "Sequencing to Minimize Weighted Completion Time",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Schedule tasks with lengths, weights, and precedence constraints to minimize total weighted completion time",
        fields: &[
            FieldInfo { name: "lengths", type_name: "Vec<u64>", description: "Processing time l(t) for each task" },
            FieldInfo { name: "weights", type_name: "Vec<u64>", description: "Weight w(t) for each task" },
            FieldInfo { name: "precedences", type_name: "Vec<(usize, usize)>", description: "Precedence pairs (predecessor, successor)" },
        ],
    }
}

/// Sequencing to Minimize Weighted Completion Time problem.
///
/// Given tasks with processing times `l(t)`, weights `w(t)`, and precedence
/// constraints, find a single-machine schedule that respects the precedences
/// and minimizes `sum_t w(t) * C(t)`, where `C(t)` is the completion time of
/// task `t`.
///
/// Configurations use Lehmer code with `dims() = [n, n-1, ..., 1]`.
#[derive(Debug, Clone, Serialize)]
pub struct SequencingToMinimizeWeightedCompletionTime {
    lengths: Vec<u64>,
    weights: Vec<u64>,
    precedences: Vec<(usize, usize)>,
}

#[derive(Deserialize)]
struct SequencingToMinimizeWeightedCompletionTimeSerde {
    lengths: Vec<u64>,
    weights: Vec<u64>,
    precedences: Vec<(usize, usize)>,
}

impl SequencingToMinimizeWeightedCompletionTime {
    fn validate(
        lengths: &[u64],
        weights: &[u64],
        precedences: &[(usize, usize)],
    ) -> Result<(), String> {
        if lengths.len() != weights.len() {
            return Err("lengths length must equal weights length".to_string());
        }
        if lengths.contains(&0) {
            return Err("task lengths must be positive".to_string());
        }

        let num_tasks = lengths.len();
        for &(pred, succ) in precedences {
            if pred >= num_tasks {
                return Err(format!(
                    "predecessor index {} out of range (num_tasks = {})",
                    pred, num_tasks
                ));
            }
            if succ >= num_tasks {
                return Err(format!(
                    "successor index {} out of range (num_tasks = {})",
                    succ, num_tasks
                ));
            }
        }

        Ok(())
    }

    /// Create a new sequencing instance.
    ///
    /// # Panics
    ///
    /// Panics if `lengths.len() != weights.len()` or if any precedence endpoint
    /// is out of range.
    pub fn new(lengths: Vec<u64>, weights: Vec<u64>, precedences: Vec<(usize, usize)>) -> Self {
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
    pub fn lengths(&self) -> &[u64] {
        &self.lengths
    }

    /// Returns the task weights.
    pub fn weights(&self) -> &[u64] {
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

    /// Returns the sum of all processing times.
    pub fn total_processing_time(&self) -> u64 {
        self.lengths
            .iter()
            .try_fold(0u64, |acc, &length| acc.checked_add(length))
            .expect("total processing time overflowed u64")
    }

    fn decode_schedule(&self, config: &[usize]) -> Option<Vec<usize>> {
        super::decode_lehmer(config, self.num_tasks())
    }

    fn weighted_completion_time(&self, schedule: &[usize]) -> Min<u64> {
        let n = self.num_tasks();
        let mut positions = vec![0usize; n];
        let mut completion_times = vec![0u64; n];
        let mut elapsed = 0u64;

        for (position, &task) in schedule.iter().enumerate() {
            positions[task] = position;
            elapsed = elapsed
                .checked_add(self.lengths[task])
                .expect("total processing time overflowed u64");
            completion_times[task] = elapsed;
        }

        for &(pred, succ) in &self.precedences {
            if positions[pred] >= positions[succ] {
                return Min(None);
            }
        }

        let total = completion_times
            .iter()
            .enumerate()
            .try_fold(0u64, |acc, (task, &completion)| -> Option<u64> {
                let weighted_completion = completion.checked_mul(self.weights[task])?;
                acc.checked_add(weighted_completion)
            })
            .expect("weighted completion time overflowed u64");
        Min(Some(total))
    }
}

impl TryFrom<SequencingToMinimizeWeightedCompletionTimeSerde>
    for SequencingToMinimizeWeightedCompletionTime
{
    type Error = String;

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
    type Value = Min<u64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        super::lehmer_dims(self.num_tasks())
    }

    fn evaluate(&self, config: &[usize]) -> Min<u64> {
        let Some(schedule) = self.decode_schedule(config) else {
            return Min(None);
        };
        self.weighted_completion_time(&schedule)
    }
}

crate::declare_variants! {
    default SequencingToMinimizeWeightedCompletionTime => "factorial(num_tasks)",
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
        optimal_config: vec![1, 2, 0, 1, 0],
        optimal_value: serde_json::json!(46),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/sequencing_to_minimize_weighted_completion_time.rs"]
mod tests;
