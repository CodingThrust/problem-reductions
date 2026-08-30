//! Precedence Constrained Scheduling problem implementation.
//!
//! Given unit-length tasks with precedence constraints, m processors, and a
//! deadline D, determine whether all tasks can be scheduled to meet D while
//! respecting precedences. NP-complete via reduction from 3SAT (Ullman, 1975).

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "PrecedenceConstrainedScheduling",
        display_name: "Precedence Constrained Scheduling",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Schedule unit-length tasks on m processors by deadline D respecting precedence constraints",
        fields: PrecedenceConstrainedSchedulingCreateSpec::FIELDS,
    }
}

/// The Precedence Constrained Scheduling problem.
///
/// Given `n` unit-length tasks with precedence constraints (a partial order),
/// `m` processors, and a deadline `D`, determine whether there exists a schedule
/// assigning each task to a time slot in `{0, ..., D-1}` such that:
/// - At most `m` tasks are assigned to any single time slot
/// - For each precedence `(i, j)`: task `j` starts after task `i` completes,
///   i.e., `slot(j) >= slot(i) + 1`
///
/// # Representation
///
/// Each task has a variable in `{0, ..., D-1}` representing its assigned time slot.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::PrecedenceConstrainedScheduling;
/// use problemreductions::{Problem, BruteForce};
///
/// // 4 tasks, 2 processors, deadline 3, with t0 < t2 and t1 < t3
/// let problem = PrecedenceConstrainedScheduling::new(4, 2, 3, vec![(0, 2), (1, 3)]);
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecedenceConstrainedScheduling {
    num_tasks: usize,
    num_processors: usize,
    deadline: i64,
    precedences: Vec<(usize, usize)>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct PrecedenceConstrainedSchedulingCreateSpec {
    num_tasks: usize,
    num_processors: usize,
    deadline: i64,
    precedences: Option<Vec<(usize, usize)>>,
}

impl TryFrom<PrecedenceConstrainedSchedulingCreateSpec> for PrecedenceConstrainedScheduling {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: PrecedenceConstrainedSchedulingCreateSpec) -> Result<Self, Self::Error> {
        if spec.num_tasks > 0 && spec.num_processors == 0 {
            return Err("num_processors must be positive when there are tasks"
                .to_string()
                .into());
        }
        if spec.num_tasks > 0 && spec.deadline == 0 {
            return Err("deadline must be positive when there are tasks"
                .to_string()
                .into());
        }
        if spec.deadline < 0 || usize::try_from(spec.deadline).is_err() {
            return Err("deadline must be nonnegative and fit usize"
                .to_string()
                .into());
        }
        let precedences = spec.precedences.unwrap_or_default();
        if let Some(&(pred, succ)) = precedences
            .iter()
            .find(|&&(pred, succ)| pred >= spec.num_tasks || succ >= spec.num_tasks)
        {
            return Err(format!(
                "precedence ({pred}, {succ}) is out of range for {} tasks",
                spec.num_tasks
            )
            .into());
        }
        Ok(Self::new(
            spec.num_tasks,
            spec.num_processors,
            spec.deadline,
            precedences,
        ))
    }
}

impl PrecedenceConstrainedScheduling {
    /// Create a new Precedence Constrained Scheduling instance.
    ///
    /// # Panics
    ///
    /// Panics if `num_processors` or `deadline` is zero (when `num_tasks > 0`),
    /// or if any precedence index is out of bounds (>= num_tasks).
    pub fn new(
        num_tasks: usize,
        num_processors: usize,
        deadline: i64,
        precedences: Vec<(usize, usize)>,
    ) -> Self {
        if num_tasks > 0 {
            assert!(
                num_processors > 0,
                "num_processors must be > 0 when there are tasks"
            );
            assert!(deadline > 0, "deadline must be > 0 when there are tasks");
        }
        assert!(
            deadline >= 0 && usize::try_from(deadline).is_ok(),
            "deadline must be nonnegative and fit usize"
        );
        for &(i, j) in &precedences {
            assert!(
                i < num_tasks && j < num_tasks,
                "Precedence ({}, {}) out of bounds for {} tasks",
                i,
                j,
                num_tasks
            );
        }
        Self {
            num_tasks,
            num_processors,
            deadline,
            precedences,
        }
    }

    /// Get the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.num_tasks
    }

    /// Get the number of processors.
    pub fn num_processors(&self) -> usize {
        self.num_processors
    }

    /// Get the deadline.
    pub fn deadline(&self) -> i64 {
        self.deadline
    }

    /// Get the precedence constraints.
    pub fn precedences(&self) -> &[(usize, usize)] {
        &self.precedences
    }

    /// Return the number of precedence relations.
    pub fn num_precedences(&self) -> usize {
        self.precedences.len()
    }
}

impl Problem for PrecedenceConstrainedScheduling {
    const NAME: &'static str = "PrecedenceConstrainedScheduling";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_parameters![
        ("deadline", deadline),
        ("num_precedences", num_precedences),
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
                if config.len() != self.num_tasks {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        "schedule length does not match the tasks".into(),
                    ));
                }
                let deadline =
                    usize::try_from(self.deadline).expect("validated deadline must fit usize");
                if config.iter().any(|&v| v >= deadline) {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        "schedule contains an out-of-range time slot".into(),
                    ));
                }
                // Check processor capacity: at most num_processors tasks per time slot
                let mut slot_count = vec![0usize; deadline];
                for &slot in config {
                    slot_count[slot] += 1;
                    if slot_count[slot] > self.num_processors {
                        return Ok(crate::types::Or(false));
                    }
                }
                // Check precedence constraints: for (i, j), slot[j] >= slot[i] + 1
                for &(i, j) in &self.precedences {
                    if config[j] < config[i] + 1 {
                        return Ok(crate::types::Or(false));
                    }
                }
                true
            })
        })
    }
}

impl crate::solvers::BruteForceProblem for PrecedenceConstrainedScheduling {
    fn dimensions(&self) -> Vec<usize> {
        vec![
            usize::try_from(self.deadline).expect("validated deadline must fit usize");
            self.num_tasks
        ]
    }
}

crate::declare_variants! {
    default PrecedenceConstrainedScheduling => "2^num_tasks" create PrecedenceConstrainedSchedulingCreateSpec,
}

crate::register_brute_force! {
    PrecedenceConstrainedScheduling,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "precedence_constrained_scheduling",
        // Issue #501 example: 8 tasks, 3 processors, deadline 4
        instance: Box::new(PrecedenceConstrainedScheduling::new(
            8,
            3,
            4,
            vec![
                (0, 2),
                (0, 3),
                (1, 3),
                (1, 4),
                (2, 5),
                (3, 6),
                (4, 6),
                (5, 7),
                (6, 7),
            ],
        )),
        // Valid schedule: slot 0: {t0,t1}, slot 1: {t2,t3,t4}, slot 2: {t5,t6}, slot 3: {t7}
        optimal_config: serde_json::json!(vec![0, 0, 1, 1, 1, 2, 2, 3]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/precedence_constrained_scheduling.rs"]
mod tests;
