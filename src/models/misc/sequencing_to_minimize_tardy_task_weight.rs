//! Sequencing to Minimize Tardy Task Weight problem implementation.
//!
//! A classical NP-hard single-machine scheduling problem (SS8 from
//! Garey & Johnson, 1979) where tasks with processing times, weights,
//! and deadlines must be scheduled to minimize the total weight of tardy tasks.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SequencingToMinimizeTardyTaskWeight",
        display_name: "Sequencing to Minimize Tardy Task Weight",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Schedule tasks with lengths, weights, and deadlines to minimize total weight of tardy tasks",
        fields: &[
            FieldInfo { name: "lengths", type_name: "Vec<u64>", description: "Processing time for each task" },
            FieldInfo { name: "weights", type_name: "Vec<u64>", description: "Weight w(t) for each task" },
            FieldInfo { name: "deadlines", type_name: "Vec<u64>", description: "Deadline d(t) for each task" },
        ],
    }
}

/// Sequencing to Minimize Tardy Task Weight problem.
///
/// Given tasks with processing times `l(t)`, weights `w(t)`, and deadlines
/// `d(t)`, find a single-machine schedule that minimizes `sum_{t tardy} w(t)`,
/// where task `t` is tardy if its completion time `C(t) > d(t)`.
///
/// This is the weighted generalization of minimizing the number of tardy tasks
/// (problem SS8 in Garey & Johnson, 1979, written $1 || sum w_j U_j$).
///
/// Configurations are direct permutation encodings with `dims() = [n; n]`:
/// each position holds the index of the task scheduled at that position.
/// A configuration is valid iff it is a permutation of `0..n`.
#[derive(Debug, Clone, Serialize)]
pub struct SequencingToMinimizeTardyTaskWeight {
    lengths: Vec<u64>,
    weights: Vec<u64>,
    deadlines: Vec<u64>,
}

#[derive(Deserialize)]
struct SequencingToMinimizeTardyTaskWeightSerde {
    lengths: Vec<u64>,
    weights: Vec<u64>,
    deadlines: Vec<u64>,
}

impl SequencingToMinimizeTardyTaskWeight {
    fn validate(lengths: &[u64], weights: &[u64], deadlines: &[u64]) -> Result<(), String> {
        if lengths.len() != weights.len() {
            return Err("lengths length must equal weights length".to_string());
        }
        if lengths.len() != deadlines.len() {
            return Err("lengths length must equal deadlines length".to_string());
        }
        if lengths.contains(&0) {
            return Err("task lengths must be positive".to_string());
        }
        if weights.contains(&0) {
            return Err("task weights must be positive".to_string());
        }
        Ok(())
    }

    /// Create a new sequencing instance.
    ///
    /// # Panics
    ///
    /// Panics if `lengths`, `weights`, and `deadlines` are not all the same
    /// length, or if any length or weight is zero.
    pub fn new(lengths: Vec<u64>, weights: Vec<u64>, deadlines: Vec<u64>) -> Self {
        Self::validate(&lengths, &weights, &deadlines).unwrap_or_else(|err| panic!("{err}"));
        Self {
            lengths,
            weights,
            deadlines,
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

    /// Returns the task deadlines.
    pub fn deadlines(&self) -> &[u64] {
        &self.deadlines
    }

    /// Decode a direct permutation configuration.
    ///
    /// Returns the schedule as `Some(Vec<usize>)` if the config is a valid
    /// permutation of `0..n`, or `None` otherwise.
    fn decode_permutation(config: &[usize], n: usize) -> Option<Vec<usize>> {
        if config.len() != n {
            return None;
        }
        let mut seen = vec![false; n];
        for &task in config {
            if task >= n || seen[task] {
                return None;
            }
            seen[task] = true;
        }
        Some(config.to_vec())
    }

    fn tardy_task_weight(&self, schedule: &[usize]) -> Min<u64> {
        let mut elapsed: u64 = 0;
        let mut total: u64 = 0;
        for &task in schedule {
            elapsed = elapsed
                .checked_add(self.lengths[task])
                .expect("total processing time overflowed u64");
            if elapsed > self.deadlines[task] {
                total = total
                    .checked_add(self.weights[task])
                    .expect("tardy task weight overflowed u64");
            }
        }
        Min(Some(total))
    }
}

impl TryFrom<SequencingToMinimizeTardyTaskWeightSerde> for SequencingToMinimizeTardyTaskWeight {
    type Error = String;

    fn try_from(value: SequencingToMinimizeTardyTaskWeightSerde) -> Result<Self, Self::Error> {
        Self::validate(&value.lengths, &value.weights, &value.deadlines)?;
        Ok(Self {
            lengths: value.lengths,
            weights: value.weights,
            deadlines: value.deadlines,
        })
    }
}

impl<'de> Deserialize<'de> for SequencingToMinimizeTardyTaskWeight {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = SequencingToMinimizeTardyTaskWeightSerde::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}

impl Problem for SequencingToMinimizeTardyTaskWeight {
    const NAME: &'static str = "SequencingToMinimizeTardyTaskWeight";
    type Value = Min<u64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.num_tasks();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> Min<u64> {
        let n = self.num_tasks();
        let Some(schedule) = Self::decode_permutation(config, n) else {
            return Min(None);
        };
        self.tardy_task_weight(&schedule)
    }
}

crate::declare_variants! {
    default SequencingToMinimizeTardyTaskWeight => "factorial(num_tasks)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "sequencing_to_minimize_tardy_task_weight",
        // 5 tasks, lengths [3,2,4,1,2], weights [5,3,7,2,4], deadlines [6,4,10,2,8]
        // Optimal schedule: [t4,t1,t5,t3,t2] = config [3,0,4,2,1]
        // Start times: t4 starts 0, completes 1 (tardy: C=1 <= d=2, ok)
        // t1 starts 1, completes 4 (tardy: C=4 <= d=6, ok)
        // t5 starts 4, completes 6 (tardy: C=6 <= d=8, ok)
        // t3 starts 6, completes 10 (tardy: C=10 <= d=10, ok)
        // t2 starts 10, completes 12 (tardy: C=12 > d=4, tardy weight 3)
        // Total tardy weight = 3
        instance: Box::new(SequencingToMinimizeTardyTaskWeight::new(
            vec![3, 2, 4, 1, 2],
            vec![5, 3, 7, 2, 4],
            vec![6, 4, 10, 2, 8],
        )),
        optimal_config: vec![3, 0, 4, 2, 1],
        optimal_value: serde_json::json!(3),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/sequencing_to_minimize_tardy_task_weight.rs"]
mod tests;
