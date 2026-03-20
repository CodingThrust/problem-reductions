//! Scheduling With Individual Deadlines problem implementation.
//!
//! Given unit-length tasks with precedence constraints and per-task deadlines,
//! determine whether they can be scheduled on `m` identical processors so that
//! every task finishes by its own deadline.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

inventory::submit! {
    ProblemSchemaEntry {
        name: "SchedulingWithIndividualDeadlines",
        display_name: "Scheduling With Individual Deadlines",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether unit-length tasks can be scheduled on m processors while meeting individual deadlines",
        fields: &[
            FieldInfo { name: "num_tasks", type_name: "usize", description: "Number of tasks |T|" },
            FieldInfo { name: "num_processors", type_name: "usize", description: "Number of identical processors m" },
            FieldInfo { name: "deadlines", type_name: "Vec<usize>", description: "Deadline d(t) for each task" },
            FieldInfo { name: "precedences", type_name: "Vec<(usize, usize)>", description: "Precedence pairs (predecessor, successor)" },
        ],
    }
}

/// Scheduling With Individual Deadlines.
///
/// A configuration assigns each task `t` a start slot `sigma(t)` with domain
/// `0..d(t)`. The schedule is feasible if every precedence pair `(u, v)`
/// satisfies `sigma(u) + 1 <= sigma(v)` and no time slot hosts more than
/// `num_processors` tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingWithIndividualDeadlines {
    num_tasks: usize,
    num_processors: usize,
    deadlines: Vec<usize>,
    precedences: Vec<(usize, usize)>,
}

impl SchedulingWithIndividualDeadlines {
    pub fn new(
        num_tasks: usize,
        num_processors: usize,
        deadlines: Vec<usize>,
        precedences: Vec<(usize, usize)>,
    ) -> Self {
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
            num_processors,
            deadlines,
            precedences,
        }
    }

    pub fn num_tasks(&self) -> usize {
        self.num_tasks
    }

    pub fn num_processors(&self) -> usize {
        self.num_processors
    }

    pub fn deadlines(&self) -> &[usize] {
        &self.deadlines
    }

    pub fn precedences(&self) -> &[(usize, usize)] {
        &self.precedences
    }

    pub fn num_precedences(&self) -> usize {
        self.precedences.len()
    }

    pub fn max_deadline(&self) -> usize {
        self.deadlines.iter().copied().max().unwrap_or(0)
    }
}

impl Problem for SchedulingWithIndividualDeadlines {
    const NAME: &'static str = "SchedulingWithIndividualDeadlines";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        self.deadlines.clone()
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.num_tasks {
            return false;
        }

        for (&start, &deadline) in config.iter().zip(&self.deadlines) {
            if start >= deadline {
                return false;
            }
        }

        for &(pred, succ) in &self.precedences {
            if config[pred] + 1 > config[succ] {
                return false;
            }
        }

        let mut slot_loads = BTreeMap::new();
        for &start in config {
            let load = slot_loads.entry(start).or_insert(0usize);
            *load += 1;
            if *load > self.num_processors {
                return false;
            }
        }

        true
    }
}

impl SatisfactionProblem for SchedulingWithIndividualDeadlines {}

crate::declare_variants! {
    default sat SchedulingWithIndividualDeadlines => "max_deadline^num_tasks",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "scheduling_with_individual_deadlines",
        instance: Box::new(SchedulingWithIndividualDeadlines::new(
            7,
            3,
            vec![2, 1, 2, 2, 3, 3, 2],
            vec![(0, 3), (1, 3), (1, 4), (2, 4), (2, 5)],
        )),
        optimal_config: vec![0, 0, 0, 1, 2, 1, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/scheduling_with_individual_deadlines.rs"]
mod tests;
