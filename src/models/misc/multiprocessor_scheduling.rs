//! Multiprocessor Scheduling problem implementation.
//!
//! The Multiprocessor Scheduling problem asks whether a set of tasks
//! can be assigned to identical processors such that no processor's
//! total load exceeds a given deadline.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MultiprocessorScheduling",
        display_name: "Multiprocessor Scheduling",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Assign tasks to processors so that no processor's load exceeds a deadline",
        fields: MultiprocessorSchedulingCreateSpec::FIELDS,
    }
}

/// The Multiprocessor Scheduling problem.
///
/// Given a set T of tasks with processing times, a number m of identical
/// processors, and a deadline D, determine whether there exists an assignment
/// of tasks to processors such that the total load on each processor does
/// not exceed D.
///
/// Because tasks are independent and processors are identical, any feasible
/// schedule can be packed processor-by-processor without idle gaps. This makes
/// the scheduling question equivalent to partitioning tasks among processors
/// with per-processor load at most `D`.
///
/// # Representation
///
/// Each task has a variable in `{0, ..., m-1}` representing its processor assignment.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::MultiprocessorScheduling;
/// use problemreductions::{Problem, BruteForce};
///
/// // 5 tasks with lengths [4, 5, 3, 2, 6], 2 processors, deadline 10
/// let problem = MultiprocessorScheduling::new(vec![4, 5, 3, 2, 6], 2, 10);
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiprocessorScheduling {
    /// Processing time for each task.
    lengths: Vec<i64>,
    /// Number of identical processors.
    #[serde(deserialize_with = "positive_usize::deserialize")]
    num_processors: usize,
    /// Global deadline.
    deadline: i64,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct MultiprocessorSchedulingCreateSpec {
    /// Processing time for each task.
    lengths: Vec<i64>,
    /// Number of identical processors.
    num_processors: usize,
    /// Global deadline.
    deadline: i64,
}
impl TryFrom<MultiprocessorSchedulingCreateSpec> for MultiprocessorScheduling {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: MultiprocessorSchedulingCreateSpec) -> Result<Self, Self::Error> {
        if spec.num_processors == 0 {
            return Err("num_processors must be positive".to_string().into());
        }
        Ok(Self::new(spec.lengths, spec.num_processors, spec.deadline))
    }
}

impl MultiprocessorScheduling {
    /// Create a new Multiprocessor Scheduling instance.
    ///
    /// # Panics
    /// Panics if `num_processors` is zero.
    pub fn new(lengths: Vec<i64>, num_processors: usize, deadline: i64) -> Self {
        assert!(num_processors > 0, "num_processors must be positive");
        assert!(
            lengths.iter().all(|&length| length >= 0),
            "task lengths must be nonnegative"
        );
        assert!(deadline >= 0, "deadline must be nonnegative");
        Self {
            lengths,
            num_processors,
            deadline,
        }
    }

    /// Returns the processing times for each task.
    pub fn lengths(&self) -> &[i64] {
        &self.lengths
    }

    /// Returns the number of processors.
    pub fn num_processors(&self) -> usize {
        self.num_processors
    }

    /// Returns the deadline.
    pub fn deadline(&self) -> i64 {
        self.deadline
    }

    /// Returns the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.lengths.len()
    }

    /// Returns the total processing time of all tasks.
    pub fn total_length(&self) -> i64 {
        self.lengths.iter().sum()
    }
}

impl Problem for MultiprocessorScheduling {
    const NAME: &'static str = "MultiprocessorScheduling";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_parameters![("num_processors", num_processors), ("num_tasks", num_tasks),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        Ok({
            crate::types::Or({
                if config.len() != self.num_tasks() {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        "processor assignment length does not match the tasks".into(),
                    ));
                }
                let m = self.num_processors;
                if config.iter().any(|&processor| processor >= m) {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        "assignment contains an out-of-range processor".into(),
                    ));
                }
                let mut loads = vec![0i64; m];
                for (i, &processor) in config.iter().enumerate() {
                    loads[processor] =
                        loads[processor]
                            .checked_add(self.lengths[i])
                            .ok_or_else(|| {
                                crate::traits::EvaluationError::IntegerOverflow(
                                    "summing multiprocessor load".into(),
                                )
                            })?;
                }
                loads.iter().all(|&load| load <= self.deadline)
            })
        })
    }
}

impl crate::solvers::BruteForceProblem for MultiprocessorScheduling {
    fn dimensions(&self) -> Vec<usize> {
        vec![self.num_processors; self.num_tasks()]
    }
}

crate::declare_variants! {
    default MultiprocessorScheduling => "2^num_tasks" create MultiprocessorSchedulingCreateSpec,
}

crate::register_brute_force! {
    MultiprocessorScheduling,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "multiprocessor_scheduling",
        instance: Box::new(MultiprocessorScheduling::new(vec![4, 5, 3, 2, 6], 2, 10)),
        optimal_config: serde_json::json!(vec![0, 1, 1, 1, 0]),
        optimal_value: serde_json::json!(true),
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
