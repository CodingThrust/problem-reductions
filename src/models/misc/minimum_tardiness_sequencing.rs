//! Minimum Tardiness Sequencing problem implementation.
//!
//! A classical NP-complete single-machine scheduling problem (SS2 from
//! Garey & Johnson, 1979) where unit-length tasks with precedence constraints
//! and deadlines must be scheduled to minimize the number of tardy tasks.
//! Corresponds to scheduling notation `1|prec, pj=1|sum Uj`.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumTardinessSequencing",
        display_name: "Minimum Tardiness Sequencing",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Schedule unit-length tasks with precedence constraints and deadlines to minimize the number of tardy tasks",
        fields: &[
            FieldInfo { name: "num_tasks", type_name: "usize", description: "Number of tasks |T|" },
            FieldInfo { name: "deadlines", type_name: "Vec<usize>", description: "Deadline d(t) for each task (1-indexed finish time)" },
            FieldInfo { name: "precedences", type_name: "Vec<(usize, usize)>", description: "Precedence pairs (predecessor, successor)" },
        ],
    }
}

/// Minimum Tardiness Sequencing problem.
///
/// Given a set T of tasks, each with unit length and a deadline d(t),
/// and a partial order (precedence constraints) on T, find a schedule
/// `sigma: T -> {0, 1, ..., |T|-1}` that is a valid permutation,
/// respects precedence constraints (`sigma(t) < sigma(t')` whenever `t < t'`),
/// and minimizes the number of tardy tasks (`|{t : sigma(t)+1 > d(t)}|`).
///
/// # Representation
///
/// Each task has a variable representing its position in the schedule.
/// A configuration is valid if and only if it is a bijective mapping
/// (permutation) that respects all precedence constraints.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::MinimumTardinessSequencing;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = MinimumTardinessSequencing::new(
///     3,
///     vec![2, 3, 1],
///     vec![(0, 2)],  // task 0 must precede task 2
/// );
/// let solver = BruteForce::new();
/// let solution = solver.find_best(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumTardinessSequencing {
    num_tasks: usize,
    deadlines: Vec<usize>,
    precedences: Vec<(usize, usize)>,
}

impl MinimumTardinessSequencing {
    /// Create a new MinimumTardinessSequencing instance.
    ///
    /// # Arguments
    ///
    /// * `num_tasks` - Number of tasks.
    /// * `deadlines` - Deadline for each task (1-indexed: a task at position `p` finishes at time `p+1`).
    /// * `precedences` - List of `(predecessor, successor)` pairs.
    ///
    /// # Panics
    ///
    /// Panics if `deadlines.len() != num_tasks` or if any task index in `precedences`
    /// is out of range.
    pub fn new(num_tasks: usize, deadlines: Vec<usize>, precedences: Vec<(usize, usize)>) -> Self {
        assert_eq!(
            deadlines.len(),
            num_tasks,
            "deadlines length must equal num_tasks"
        );
        for &(pred, succ) in &precedences {
            assert!(
                pred < num_tasks,
                "predecessor index {} out of range (num_tasks = {})",
                pred,
                num_tasks
            );
            assert!(
                succ < num_tasks,
                "successor index {} out of range (num_tasks = {})",
                succ,
                num_tasks
            );
        }
        Self {
            num_tasks,
            deadlines,
            precedences,
        }
    }

    /// Returns the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.num_tasks
    }

    /// Returns the deadlines.
    pub fn deadlines(&self) -> &[usize] {
        &self.deadlines
    }

    /// Returns the precedence constraints.
    pub fn precedences(&self) -> &[(usize, usize)] {
        &self.precedences
    }

    /// Returns the number of precedence constraints.
    pub fn num_precedences(&self) -> usize {
        self.precedences.len()
    }
}

impl Problem for MinimumTardinessSequencing {
    const NAME: &'static str = "MinimumTardinessSequencing";
    type Metric = SolutionSize<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.num_tasks;
        (0..n).rev().map(|i| i + 1).collect()
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<usize> {
        let n = self.num_tasks;
        if config.len() != n {
            return SolutionSize::Invalid;
        }

        // Decode Lehmer code into a permutation.
        // config[i] must be < n - i (the domain size for position i).
        let mut available: Vec<usize> = (0..n).collect();
        let mut schedule = Vec::with_capacity(n);
        for &c in config.iter() {
            if c >= available.len() {
                return SolutionSize::Invalid;
            }
            schedule.push(available.remove(c));
        }

        // schedule[i] = the task scheduled at position i.
        // We need sigma(task) = position, i.e., the inverse permutation.
        let mut sigma = vec![0usize; n];
        for (pos, &task) in schedule.iter().enumerate() {
            sigma[task] = pos;
        }

        // Check precedence constraints: for each (pred, succ), sigma(pred) < sigma(succ)
        for &(pred, succ) in &self.precedences {
            if sigma[pred] >= sigma[succ] {
                return SolutionSize::Invalid;
            }
        }

        // Count tardy tasks: task t is tardy if sigma(t) + 1 > d(t)
        let tardy_count = sigma
            .iter()
            .enumerate()
            .filter(|&(t, &pos)| pos + 1 > self.deadlines[t])
            .count();

        SolutionSize::Valid(tardy_count)
    }
}

impl OptimizationProblem for MinimumTardinessSequencing {
    type Value = usize;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

crate::declare_variants! {
    default opt MinimumTardinessSequencing => "2^num_tasks",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_tardiness_sequencing",
        build: || {
            // 4 tasks with precedence 0 -> 2 (task 0 before task 2).
            // Deadlines: task 0 by time 2, task 1 by time 3, task 2 by time 1, task 3 by time 4.
            let problem = MinimumTardinessSequencing::new(
                4,
                vec![2, 3, 1, 4],
                vec![(0, 2)],
            );
            // Sample config: Lehmer code [0,0,0,0] = identity permutation (schedule order 0,1,2,3)
            crate::example_db::specs::optimization_example(problem, vec![vec![0, 0, 0, 0]])
        },
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_tardiness_sequencing.rs"]
mod tests;
