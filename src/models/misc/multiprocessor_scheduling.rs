//! Multiprocessor Scheduling problem implementation.
//!
//! Given a set of tasks with positive processing times, a number of identical
//! processors, and a global deadline, determine whether the tasks can be
//! partitioned among the processors so that every processor's total load is at
//! most the deadline.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MultiprocessorScheduling",
        display_name: "Multiprocessor Scheduling",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Assign tasks to identical processors so that every processor meets a deadline",
        fields: &[
            FieldInfo { name: "lengths", type_name: "Vec<u64>", description: "Processing time l(t) for each task" },
            FieldInfo { name: "num_processors", type_name: "usize", description: "Number of identical processors m" },
            FieldInfo { name: "deadline", type_name: "u64", description: "Global deadline D" },
        ],
    }
}

/// The Multiprocessor Scheduling problem.
///
/// The original Garey-Johnson formulation uses a start-time function
/// `sigma: T -> Z_>=0`. For identical processors with non-preemptive,
/// independent tasks, that feasibility question is equivalent to assigning each
/// task to a processor and requiring every processor's total assigned load to be
/// at most `deadline`, since tasks on the same processor can be packed
/// sequentially without gaps.
///
/// # Representation
///
/// Each task has one variable in `{0, ..., m - 1}` indicating which processor
/// executes it.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::MultiprocessorScheduling;
/// use problemreductions::{BruteForce, Solver};
///
/// let problem = MultiprocessorScheduling::new(vec![4, 5, 3, 2, 6], 2, 10);
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiprocessorScheduling {
    lengths: Vec<u64>,
    #[serde(deserialize_with = "positive_usize::deserialize")]
    num_processors: usize,
    deadline: u64,
}

impl MultiprocessorScheduling {
    /// Create a new MultiprocessorScheduling instance.
    ///
    /// # Panics
    ///
    /// Panics if `num_processors == 0`.
    pub fn new(lengths: Vec<u64>, num_processors: usize, deadline: u64) -> Self {
        assert!(num_processors > 0, "num_processors must be positive");
        Self {
            lengths,
            num_processors,
            deadline,
        }
    }

    /// Returns the task processing times.
    pub fn lengths(&self) -> &[u64] {
        &self.lengths
    }

    /// Returns the number of processors.
    pub fn num_processors(&self) -> usize {
        self.num_processors
    }

    /// Returns the global deadline.
    pub fn deadline(&self) -> u64 {
        self.deadline
    }

    /// Returns the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.lengths.len()
    }

    /// Returns the total processing time of all tasks.
    pub fn total_length(&self) -> u64 {
        self.lengths.iter().sum()
    }
}

impl Problem for MultiprocessorScheduling {
    const NAME: &'static str = "MultiprocessorScheduling";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_processors; self.num_tasks()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.num_tasks() {
            return false;
        }
        if config.iter().any(|&processor| processor >= self.num_processors) {
            return false;
        }

        let mut loads = vec![0u64; self.num_processors];
        for (task, &processor) in config.iter().enumerate() {
            loads[processor] += self.lengths[task];
        }
        loads.iter().all(|&load| load <= self.deadline)
    }
}

impl SatisfactionProblem for MultiprocessorScheduling {}

crate::declare_variants! {
    default sat MultiprocessorScheduling => "2 ^ num_tasks",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "multiprocessor_scheduling",
        build: || {
            let problem = MultiprocessorScheduling::new(vec![4, 5, 3, 2, 6], 2, 10);
            crate::example_db::specs::satisfaction_example(
                problem,
                vec![vec![0, 1, 1, 1, 0], vec![0, 0, 0, 0, 0]],
            )
        },
    }]
}

mod positive_usize {
    use serde::de::Error;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<usize, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = usize::deserialize(deserializer)?;
        if value == 0 {
            return Err(D::Error::custom("expected positive integer, got 0"));
        }
        Ok(value)
    }
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/multiprocessor_scheduling.rs"]
mod tests;
