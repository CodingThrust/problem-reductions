//! Scheduling to Minimize Weighted Completion Time problem implementation.
//!
//! An NP-hard multiprocessor scheduling optimization problem (SS13 from
//! Garey & Johnson, 1979) where tasks with processing times and weights
//! must be assigned to identical processors to minimize the total weighted
//! completion time. Within each processor, tasks are ordered by Smith's
//! rule (non-decreasing length-to-weight ratio).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SchedulingToMinimizeWeightedCompletionTime",
        display_name: "Scheduling to Minimize Weighted Completion Time",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Assign tasks to processors to minimize total weighted completion time (Smith's rule ordering)",
        fields: &[
            FieldInfo { name: "lengths", type_name: "Vec<u64>", description: "Processing time l(t) for each task" },
            FieldInfo { name: "weights", type_name: "Vec<u64>", description: "Weight w(t) for each task" },
            FieldInfo { name: "num_processors", type_name: "usize", description: "Number of identical processors m" },
        ],
    }
}

/// Scheduling to Minimize Weighted Completion Time problem.
///
/// Given a set T of tasks with processing times `l(t)` and weights `w(t)`,
/// and a number `m` of identical processors, find an assignment of tasks to
/// processors that minimizes the total weighted completion time
/// `sum_t w(t) * C(t)`, where `C(t) = start_time(t) + l(t)`.
///
/// Within each processor, tasks are ordered by Smith's rule: non-decreasing
/// `l(t)/w(t)` ratio. The only free variables are the processor assignments.
///
/// # Representation
///
/// Each task has a variable in `{0, ..., m-1}` representing its processor
/// assignment, giving `dims() = [m; n]`.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::SchedulingToMinimizeWeightedCompletionTime;
/// use problemreductions::{Problem, Solver, BruteForce};
/// use problemreductions::types::Min;
///
/// // 5 tasks, 2 processors
/// let problem = SchedulingToMinimizeWeightedCompletionTime::new(
///     vec![1, 2, 3, 4, 5], vec![6, 4, 3, 2, 1], 2,
/// );
/// let solver = BruteForce::new();
/// let witness = solver.find_witness(&problem).unwrap();
/// assert_eq!(problem.evaluate(&witness), Min(Some(47)));
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct SchedulingToMinimizeWeightedCompletionTime {
    lengths: Vec<u64>,
    weights: Vec<u64>,
    #[serde(serialize_with = "serialize_num_processors")]
    num_processors: usize,
}

fn serialize_num_processors<S: serde::Serializer>(v: &usize, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u64(*v as u64)
}

#[derive(Deserialize)]
struct SchedulingToMinimizeWeightedCompletionTimeSerde {
    lengths: Vec<u64>,
    weights: Vec<u64>,
    num_processors: usize,
}

impl SchedulingToMinimizeWeightedCompletionTime {
    fn validate(lengths: &[u64], weights: &[u64], num_processors: usize) -> Result<(), String> {
        if lengths.len() != weights.len() {
            return Err("lengths and weights must have the same length".to_string());
        }
        if num_processors == 0 {
            return Err("num_processors must be positive".to_string());
        }
        if lengths.contains(&0) {
            return Err("task lengths must be positive".to_string());
        }
        if weights.contains(&0) {
            return Err("task weights must be positive".to_string());
        }
        Ok(())
    }

    /// Create a new scheduling instance.
    ///
    /// # Panics
    ///
    /// Panics if `lengths.len() != weights.len()`, if `num_processors` is zero,
    /// or if any length or weight is zero.
    pub fn new(lengths: Vec<u64>, weights: Vec<u64>, num_processors: usize) -> Self {
        Self::validate(&lengths, &weights, num_processors).unwrap_or_else(|err| panic!("{err}"));
        Self {
            lengths,
            weights,
            num_processors,
        }
    }

    /// Returns the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.lengths.len()
    }

    /// Returns the number of processors.
    pub fn num_processors(&self) -> usize {
        self.num_processors
    }

    /// Returns the processing times.
    pub fn lengths(&self) -> &[u64] {
        &self.lengths
    }

    /// Returns the task weights.
    pub fn weights(&self) -> &[u64] {
        &self.weights
    }

    /// Compute the total weighted completion time for a given processor
    /// assignment. Tasks on each processor are ordered by Smith's rule
    /// (non-decreasing l(t)/w(t) ratio).
    fn compute_weighted_completion_time(&self, config: &[usize]) -> Min<u64> {
        let n = self.num_tasks();
        let m = self.num_processors;

        if config.len() != n {
            return Min(None);
        }
        if config.iter().any(|&p| p >= m) {
            return Min(None);
        }

        // Group task indices by processor
        let mut processor_tasks: Vec<Vec<usize>> = vec![vec![]; m];
        for (task, &processor) in config.iter().enumerate() {
            processor_tasks[processor].push(task);
        }

        let mut total_weighted_completion = 0u64;

        for tasks in &mut processor_tasks {
            // Smith's rule: sort by non-decreasing l(t)/w(t)
            // Equivalent to: l(i)*w(j) <= l(j)*w(i) (avoids floating point)
            tasks.sort_by(|&a, &b| {
                let lhs = self.lengths[a] as u128 * self.weights[b] as u128;
                let rhs = self.lengths[b] as u128 * self.weights[a] as u128;
                lhs.cmp(&rhs).then(a.cmp(&b))
            });

            let mut elapsed = 0u64;
            for &task in tasks.iter() {
                elapsed = elapsed
                    .checked_add(self.lengths[task])
                    .expect("processing time overflowed u64");
                let contribution = elapsed
                    .checked_mul(self.weights[task])
                    .expect("weighted completion time overflowed u64");
                total_weighted_completion = total_weighted_completion
                    .checked_add(contribution)
                    .expect("total weighted completion time overflowed u64");
            }
        }

        Min(Some(total_weighted_completion))
    }
}

impl TryFrom<SchedulingToMinimizeWeightedCompletionTimeSerde>
    for SchedulingToMinimizeWeightedCompletionTime
{
    type Error = String;

    fn try_from(
        value: SchedulingToMinimizeWeightedCompletionTimeSerde,
    ) -> Result<Self, Self::Error> {
        Self::validate(&value.lengths, &value.weights, value.num_processors)?;
        Ok(Self {
            lengths: value.lengths,
            weights: value.weights,
            num_processors: value.num_processors,
        })
    }
}

impl<'de> Deserialize<'de> for SchedulingToMinimizeWeightedCompletionTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = SchedulingToMinimizeWeightedCompletionTimeSerde::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}

impl Problem for SchedulingToMinimizeWeightedCompletionTime {
    const NAME: &'static str = "SchedulingToMinimizeWeightedCompletionTime";
    type Value = Min<u64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_processors; self.num_tasks()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<u64> {
        self.compute_weighted_completion_time(config)
    }
}

crate::declare_variants! {
    default SchedulingToMinimizeWeightedCompletionTime => "num_processors^num_tasks",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "scheduling_to_minimize_weighted_completion_time",
        instance: Box::new(SchedulingToMinimizeWeightedCompletionTime::new(
            vec![1, 2, 3, 4, 5],
            vec![6, 4, 3, 2, 1],
            2,
        )),
        // P0={t0,t2,t4}, P1={t1,t3} => config [0, 1, 0, 1, 0]
        optimal_config: vec![0, 1, 0, 1, 0],
        optimal_value: serde_json::json!(47),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/scheduling_to_minimize_weighted_completion_time.rs"]
mod tests;
