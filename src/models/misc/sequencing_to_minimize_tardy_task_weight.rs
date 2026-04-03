//! Sequencing to Minimize Tardy Task Weight problem implementation.
//!
//! A classical NP-hard single-machine scheduling problem (SS3 from
//! Garey & Johnson, 1979) asking for a job order that minimizes the total
//! weight of tardy jobs.
//! Corresponds to scheduling notation `1 || sum w_j U_j`.

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
        description: "Schedule tasks on one machine to minimize the total weight of tardy tasks",
        fields: &[
            FieldInfo { name: "lengths", type_name: "Vec<u64>", description: "Processing time l(t) for each task" },
            FieldInfo { name: "weights", type_name: "Vec<u64>", description: "Weight w(t) for each task" },
            FieldInfo { name: "deadlines", type_name: "Vec<u64>", description: "Deadline d(t) for each task" },
        ],
    }
}

/// Sequencing to Minimize Tardy Task Weight.
///
/// Given tasks with processing times `l(t)`, weights `w(t)`, and deadlines
/// `d(t)`, find a single-machine schedule minimizing the total weight of tasks
/// that finish after their deadlines.
///
/// # Representation
///
/// Configurations use Lehmer code with `dims() = [n, n-1, ..., 1]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencingToMinimizeTardyTaskWeight {
    lengths: Vec<u64>,
    weights: Vec<u64>,
    deadlines: Vec<u64>,
}

impl SequencingToMinimizeTardyTaskWeight {
    /// Create a new sequencing instance.
    ///
    /// # Panics
    ///
    /// Panics if the three vectors do not have the same length.
    pub fn new(lengths: Vec<u64>, weights: Vec<u64>, deadlines: Vec<u64>) -> Self {
        assert_eq!(
            lengths.len(),
            weights.len(),
            "weights length must equal lengths length"
        );
        assert_eq!(
            lengths.len(),
            deadlines.len(),
            "deadlines length must equal lengths length"
        );

        Self {
            lengths,
            weights,
            deadlines,
        }
    }

    /// Returns the processing times.
    pub fn lengths(&self) -> &[u64] {
        &self.lengths
    }

    /// Returns the task weights.
    pub fn weights(&self) -> &[u64] {
        &self.weights
    }

    /// Returns the deadlines.
    pub fn deadlines(&self) -> &[u64] {
        &self.deadlines
    }

    /// Returns the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.lengths.len()
    }

    fn decode_schedule(&self, config: &[usize]) -> Option<Vec<usize>> {
        let n = self.num_tasks();
        if config.len() != n {
            return None;
        }

        let mut available: Vec<usize> = (0..n).collect();
        let mut schedule = Vec::with_capacity(n);
        for &digit in config {
            if digit >= available.len() {
                return None;
            }
            schedule.push(available.remove(digit));
        }
        Some(schedule)
    }

    fn tardy_task_weight(&self, schedule: &[usize]) -> u64 {
        let mut completion_time = 0u64;
        let mut total = 0u64;

        for &task in schedule {
            completion_time = completion_time
                .checked_add(self.lengths[task])
                .expect("completion time overflowed u64");
            if completion_time > self.deadlines[task] {
                total = total
                    .checked_add(self.weights[task])
                    .expect("total tardy weight overflowed u64");
            }
        }

        total
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
        (0..n).rev().map(|i| i + 1).collect()
    }

    fn evaluate(&self, config: &[usize]) -> Min<u64> {
        let Some(schedule) = self.decode_schedule(config) else {
            return Min(None);
        };
        Min(Some(self.tardy_task_weight(&schedule)))
    }
}

crate::declare_variants! {
    default SequencingToMinimizeTardyTaskWeight => "factorial(num_tasks)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "sequencing_to_minimize_tardy_task_weight",
        instance: Box::new(SequencingToMinimizeTardyTaskWeight::new(
            vec![3, 2, 4, 1, 2],
            vec![5, 3, 7, 2, 4],
            vec![6, 4, 10, 2, 8],
        )),
        optimal_config: vec![3, 0, 2, 1, 0],
        optimal_value: serde_json::json!(3),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/sequencing_to_minimize_tardy_task_weight.rs"]
mod tests;
