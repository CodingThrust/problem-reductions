//! Precedence Constrained Scheduling problem implementation.
//!
//! Given unit-length tasks with precedence constraints, m processors, and a
//! deadline D, determine whether all tasks can be scheduled to meet D while
//! respecting precedences. NP-complete via reduction from 3SAT (Ullman, 1975).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "PrecedenceConstrainedScheduling",
        display_name: "Precedence Constrained Scheduling",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Schedule unit-length tasks on m processors by deadline D respecting precedence constraints",
        fields: &[
            FieldInfo { name: "num_tasks", type_name: "usize", description: "Number of tasks n = |T|" },
            FieldInfo { name: "num_processors", type_name: "usize", description: "Number of processors m" },
            FieldInfo { name: "deadline", type_name: "usize", description: "Global deadline D" },
            FieldInfo { name: "precedences", type_name: "Vec<(usize, usize)>", description: "Precedence pairs (i, j) meaning task i must finish before task j starts" },
        ],
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
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 4 tasks, 2 processors, deadline 3, with t0 < t2 and t1 < t3
/// let problem = PrecedenceConstrainedScheduling::new(4, 2, 3, vec![(0, 2), (1, 3)]);
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecedenceConstrainedScheduling {
    num_tasks: usize,
    num_processors: usize,
    deadline: usize,
    precedences: Vec<(usize, usize)>,
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
        deadline: usize,
        precedences: Vec<(usize, usize)>,
    ) -> Self {
        if num_tasks > 0 {
            assert!(
                num_processors > 0,
                "num_processors must be > 0 when there are tasks"
            );
            assert!(deadline > 0, "deadline must be > 0 when there are tasks");
        }
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
    pub fn deadline(&self) -> usize {
        self.deadline
    }

    /// Get the precedence constraints.
    pub fn precedences(&self) -> &[(usize, usize)] {
        &self.precedences
    }
}

impl Problem for PrecedenceConstrainedScheduling {
    const NAME: &'static str = "PrecedenceConstrainedScheduling";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.deadline; self.num_tasks]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.num_tasks {
            return false;
        }
        // Check all values are valid time slots
        if config.iter().any(|&v| v >= self.deadline) {
            return false;
        }
        // Check processor capacity: at most num_processors tasks per time slot
        let mut slot_count = vec![0usize; self.deadline];
        for &slot in config {
            slot_count[slot] += 1;
            if slot_count[slot] > self.num_processors {
                return false;
            }
        }
        // Check precedence constraints: for (i, j), slot[j] >= slot[i] + 1
        for &(i, j) in &self.precedences {
            if config[j] < config[i] + 1 {
                return false;
            }
        }
        true
    }
}

impl SatisfactionProblem for PrecedenceConstrainedScheduling {}

crate::declare_variants! {
    default sat PrecedenceConstrainedScheduling => "deadline ^ num_tasks",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "precedence_constrained_scheduling",
        build: || {
            use crate::solvers::BruteForce;
            // Issue #501 example: 8 tasks, 3 processors, deadline 4
            let problem = PrecedenceConstrainedScheduling::new(
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
            );
            let sample = BruteForce::new()
                .find_all_satisfying(&problem)
                .into_iter()
                .next()
                .expect("precedence_constrained_scheduling example should solve");
            crate::example_db::specs::satisfaction_example(problem, vec![sample])
        },
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/precedence_constrained_scheduling.rs"]
mod tests;
