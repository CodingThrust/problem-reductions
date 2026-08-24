//! Scheduling With Individual Deadlines problem implementation.
//!
//! Given unit-length tasks with precedence constraints and per-task deadlines,
//! determine whether they can be scheduled on `m` identical processors so that
//! every task finishes by its own deadline.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

inventory::submit! {
    ProblemSchemaEntry {
        name: "SchedulingWithIndividualDeadlines",
        display_name: "Scheduling With Individual Deadlines",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Determine whether unit-length tasks can be scheduled on m processors while meeting individual deadlines",
        fields: SchedulingWithIndividualDeadlinesCreateSpec::FIELDS,
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
    deadlines: Vec<i64>,
    precedences: Vec<(usize, usize)>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct SchedulingWithIndividualDeadlinesCreateSpec {
    /// Number of tasks.
    num_tasks: usize,
    /// Number of identical processors.
    num_processors: usize,
    /// Deadline for each task.
    deadlines: Vec<i64>,
    /// Precedence pairs.
    precedences: Option<Vec<(usize, usize)>>,
}
impl TryFrom<SchedulingWithIndividualDeadlinesCreateSpec> for SchedulingWithIndividualDeadlines {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: SchedulingWithIndividualDeadlinesCreateSpec) -> Result<Self, Self::Error> {
        if spec.deadlines.len() != spec.num_tasks {
            return Err(format!(
                "deadlines has {} entries, expected {}",
                spec.deadlines.len(),
                spec.num_tasks
            )
            .into());
        }
        if spec.deadlines.iter().any(|&deadline| deadline < 0) {
            return Err("deadlines must be nonnegative".to_string().into());
        }
        if spec
            .deadlines
            .iter()
            .any(|&deadline| usize::try_from(deadline).is_err())
        {
            return Err("deadlines must fit usize to define schedule slots"
                .to_string()
                .into());
        }
        let precedences = spec.precedences.unwrap_or_default();
        if let Some(&(pred, succ)) = precedences
            .iter()
            .find(|&&(p, s)| p >= spec.num_tasks || s >= spec.num_tasks)
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
            spec.deadlines,
            precedences,
        ))
    }
}

impl SchedulingWithIndividualDeadlines {
    pub fn new(
        num_tasks: usize,
        num_processors: usize,
        deadlines: Vec<i64>,
        precedences: Vec<(usize, usize)>,
    ) -> Self {
        assert_eq!(
            deadlines.len(),
            num_tasks,
            "deadlines length must equal num_tasks"
        );
        assert!(
            deadlines.iter().all(|&deadline| deadline >= 0),
            "deadlines must be nonnegative"
        );
        assert!(
            deadlines
                .iter()
                .all(|&deadline| usize::try_from(deadline).is_ok()),
            "deadlines must fit usize to define schedule slots"
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

    pub fn deadlines(&self) -> &[i64] {
        &self.deadlines
    }

    pub fn precedences(&self) -> &[(usize, usize)] {
        &self.precedences
    }

    pub fn num_precedences(&self) -> usize {
        self.precedences.len()
    }

    pub fn max_deadline(&self) -> i64 {
        self.deadlines.iter().copied().max().unwrap_or(0)
    }
}

impl Problem for SchedulingWithIndividualDeadlines {
    const NAME: &'static str = "SchedulingWithIndividualDeadlines";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        self.deadlines
            .iter()
            .map(|&deadline| usize::try_from(deadline).expect("validated deadline must fit usize"))
            .collect()
    }

    fn evaluate(
        &self,
        config: &[usize],
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        Ok({
            crate::types::Or({
                if config.len() != self.num_tasks {
                    return Ok(crate::types::Or(false));
                }

                for (&start, &deadline) in config.iter().zip(&self.deadlines) {
                    let deadline =
                        usize::try_from(deadline).expect("validated deadline must fit usize");
                    if start >= deadline {
                        return Ok(crate::types::Or(false));
                    }
                }

                for &(pred, succ) in &self.precedences {
                    if config[pred] + 1 > config[succ] {
                        return Ok(crate::types::Or(false));
                    }
                }

                let mut slot_loads = BTreeMap::new();
                for &start in config {
                    let load = slot_loads.entry(start).or_insert(0usize);
                    *load += 1;
                    if *load > self.num_processors {
                        return Ok(crate::types::Or(false));
                    }
                }

                true
            })
        })
    }
}

crate::declare_variants! {
    default SchedulingWithIndividualDeadlines => "max_deadline^num_tasks" create SchedulingWithIndividualDeadlinesCreateSpec,
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
