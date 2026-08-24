//! Sequencing to Minimize Tardy Task Weight problem implementation.
//!
//! A classical NP-hard single-machine scheduling problem (SS8 from
//! Garey & Johnson, 1979) where tasks with processing times, weights,
//! and deadlines must be scheduled to minimize the total weight of tardy tasks.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SequencingToMinimizeTardyTaskWeight",
        display_name: "Sequencing to Minimize Tardy Task Weight",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Schedule tasks with lengths, weights, and deadlines to minimize total weight of tardy tasks",
        fields: SequencingToMinimizeTardyTaskWeightCreateSpec::FIELDS,
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
    lengths: Vec<i64>,
    weights: Vec<i64>,
    deadlines: Vec<i64>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct SequencingToMinimizeTardyTaskWeightCreateSpec {
    /// Processing time for each task.
    lengths: Vec<i64>,
    /// Task weights; defaults to one per task.
    weights: Option<Vec<i64>>,
    /// Deadline for each task.
    deadlines: Vec<i64>,
}
impl TryFrom<SequencingToMinimizeTardyTaskWeightCreateSpec>
    for SequencingToMinimizeTardyTaskWeight
{
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: SequencingToMinimizeTardyTaskWeightCreateSpec) -> Result<Self, Self::Error> {
        let count = spec.lengths.len();
        if spec.deadlines.len() != count {
            return Err("deadlines length must equal lengths length"
                .to_string()
                .into());
        }
        let weights = spec.weights.unwrap_or_else(|| vec![1; count]);
        if weights.len() != count {
            return Err("weights length must equal lengths length"
                .to_string()
                .into());
        }
        Ok(Self::new(spec.lengths, weights, spec.deadlines))
    }
}

#[derive(Deserialize)]
struct SequencingToMinimizeTardyTaskWeightSerde {
    lengths: Vec<i64>,
    weights: Vec<i64>,
    deadlines: Vec<i64>,
}

impl SequencingToMinimizeTardyTaskWeight {
    fn validate(
        lengths: &[i64],
        weights: &[i64],
        deadlines: &[i64],
    ) -> Result<(), crate::registry::ConstructionError> {
        if lengths.len() != weights.len() {
            return Err("lengths length must equal weights length"
                .to_string()
                .into());
        }
        if lengths.len() != deadlines.len() {
            return Err("lengths length must equal deadlines length"
                .to_string()
                .into());
        }
        if lengths.contains(&0) {
            return Err("task lengths must be positive".to_string().into());
        }
        if weights.contains(&0) {
            return Err("task weights must be positive".to_string().into());
        }
        Ok(())
    }

    /// Create a new sequencing instance.
    ///
    /// # Panics
    ///
    /// Panics if `lengths`, `weights`, and `deadlines` are not all the same
    /// length, or if any length or weight is zero.
    pub fn new(lengths: Vec<i64>, weights: Vec<i64>, deadlines: Vec<i64>) -> Self {
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
    pub fn lengths(&self) -> &[i64] {
        &self.lengths
    }

    /// Returns the task weights.
    pub fn weights(&self) -> &[i64] {
        &self.weights
    }

    /// Returns the task deadlines.
    pub fn deadlines(&self) -> &[i64] {
        &self.deadlines
    }

    fn tardy_task_weight(
        &self,
        schedule: &[usize],
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        let mut elapsed: i64 = 0;
        let mut total: i64 = 0;
        for &task in schedule {
            elapsed = elapsed.checked_add(self.lengths[task]).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing tardiness sequencing processing times".to_string(),
                )
            })?;
            if elapsed > self.deadlines[task] {
                total = total.checked_add(self.weights[task]).ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "summing tardy task weights".to_string(),
                    )
                })?;
            }
        }
        Ok(Min(Some(total)))
    }
}

impl TryFrom<SequencingToMinimizeTardyTaskWeightSerde> for SequencingToMinimizeTardyTaskWeight {
    type Error = crate::registry::ConstructionError;

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
    type Value = Min<i64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.num_tasks();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> Result<Min<i64>, crate::traits::EvaluationError> {
        Ok({
            let n = self.num_tasks();
            let Some(schedule) = super::decode_permutation(config, n) else {
                return Ok(Min(None));
            };
            self.tardy_task_weight(&schedule)?
        })
    }
}

crate::declare_variants! {
    default SequencingToMinimizeTardyTaskWeight => "factorial(num_tasks)" create SequencingToMinimizeTardyTaskWeightCreateSpec,
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
