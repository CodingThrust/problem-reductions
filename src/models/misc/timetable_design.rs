//! Timetable Design problem implementation.
//!
//! Decide whether craftsmen can be assigned to tasks across work periods while
//! respecting availability, per-period exclusivity, and exact pairwise work
//! requirements.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "TimetableDesign",
        display_name: "Timetable Design",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Assign craftsmen to tasks over work periods subject to availability and exact pairwise requirements",
        fields: TimetableDesignCreateSpec::FIELDS,
    }
}

/// The Timetable Design problem.
///
/// A configuration is a flattened binary tensor `f(c,t,h)` in craftsman-major,
/// task-next, period-last order:
/// `idx = ((c * num_tasks) + t) * num_periods + h`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "TimetableDesignCreateSpec")]
pub struct TimetableDesign {
    num_periods: usize,
    num_craftsmen: usize,
    num_tasks: usize,
    craftsman_avail: Vec<Vec<bool>>,
    task_avail: Vec<Vec<bool>>,
    requirements: Vec<Vec<i64>>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct TimetableDesignCreateSpec {
    /// Number of work periods.
    num_periods: usize,
    /// Number of craftsmen.
    num_craftsmen: usize,
    /// Number of tasks.
    num_tasks: usize,
    /// Craftsman availability matrix.
    craftsman_avail: Vec<Vec<bool>>,
    /// Task availability matrix.
    task_avail: Vec<Vec<bool>>,
    /// Required work periods for each craftsman-task pair.
    requirements: Vec<Vec<i64>>,
}
impl TryFrom<TimetableDesignCreateSpec> for TimetableDesign {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: TimetableDesignCreateSpec) -> Result<Self, Self::Error> {
        Self::try_new(
            spec.num_periods,
            spec.num_craftsmen,
            spec.num_tasks,
            spec.craftsman_avail,
            spec.task_avail,
            spec.requirements,
        )
    }
}

impl TimetableDesign {
    /// Create a new Timetable Design instance.
    ///
    /// # Panics
    ///
    /// Panics if any matrix dimensions do not match the declared counts.
    pub fn new(
        num_periods: usize,
        num_craftsmen: usize,
        num_tasks: usize,
        craftsman_avail: Vec<Vec<bool>>,
        task_avail: Vec<Vec<bool>>,
        requirements: Vec<Vec<i64>>,
    ) -> Self {
        Self::try_new(
            num_periods,
            num_craftsmen,
            num_tasks,
            craftsman_avail,
            task_avail,
            requirements,
        )
        .unwrap_or_else(|error| panic!("{error}"))
    }

    fn try_new(
        num_periods: usize,
        num_craftsmen: usize,
        num_tasks: usize,
        craftsman_avail: Vec<Vec<bool>>,
        task_avail: Vec<Vec<bool>>,
        requirements: Vec<Vec<i64>>,
    ) -> Result<Self, crate::registry::ConstructionError> {
        if craftsman_avail.len() != num_craftsmen {
            return Err(format!(
                "craftsman_avail has {} rows, expected {}",
                craftsman_avail.len(),
                num_craftsmen
            )
            .into());
        }
        for (craftsman, row) in craftsman_avail.iter().enumerate() {
            if row.len() != num_periods {
                return Err(format!(
                    "craftsman {} availability has {} periods, expected {}",
                    craftsman,
                    row.len(),
                    num_periods
                )
                .into());
            }
        }

        if task_avail.len() != num_tasks {
            return Err(format!(
                "task_avail has {} rows, expected {}",
                task_avail.len(),
                num_tasks
            )
            .into());
        }
        for (task, row) in task_avail.iter().enumerate() {
            if row.len() != num_periods {
                return Err(format!(
                    "task {} availability has {} periods, expected {}",
                    task,
                    row.len(),
                    num_periods
                )
                .into());
            }
        }

        if requirements.len() != num_craftsmen {
            return Err(format!(
                "requirements has {} rows, expected {}",
                requirements.len(),
                num_craftsmen
            )
            .into());
        }
        for (craftsman, row) in requirements.iter().enumerate() {
            if row.len() != num_tasks {
                return Err(format!(
                    "requirements row {} has {} tasks, expected {}",
                    craftsman,
                    row.len(),
                    num_tasks
                )
                .into());
            }
        }

        Ok(Self {
            num_periods,
            num_craftsmen,
            num_tasks,
            craftsman_avail,
            task_avail,
            requirements,
        })
    }

    /// Get the number of periods.
    pub fn num_periods(&self) -> usize {
        self.num_periods
    }

    /// Get the number of craftsmen.
    pub fn num_craftsmen(&self) -> usize {
        self.num_craftsmen
    }

    /// Get the number of tasks.
    pub fn num_tasks(&self) -> usize {
        self.num_tasks
    }

    /// Get craftsman availability.
    pub fn craftsman_avail(&self) -> &[Vec<bool>] {
        &self.craftsman_avail
    }

    /// Get task availability.
    pub fn task_avail(&self) -> &[Vec<bool>] {
        &self.task_avail
    }

    /// Get the pairwise work requirements.
    pub fn requirements(&self) -> &[Vec<i64>] {
        &self.requirements
    }

    fn config_len(&self) -> usize {
        self.num_craftsmen * self.num_tasks * self.num_periods
    }

    fn index(&self, craftsman: usize, task: usize, period: usize) -> usize {
        ((craftsman * self.num_tasks) + task) * self.num_periods + period
    }

    /// Available triples belonging to pairs with positive demand.
    pub fn num_available_assignments(&self) -> usize {
        self.requirements
            .iter()
            .enumerate()
            .map(|(c, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, r)| **r > 0)
                    .map(|(t, _)| {
                        self.craftsman_avail[c]
                            .iter()
                            .zip(&self.task_avail[t])
                            .filter(|(a, b)| **a && **b)
                            .count()
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    /// Pair equations needed, including impossible negative demands.
    pub fn num_nonzero_requirements(&self) -> usize {
        self.requirements
            .iter()
            .flatten()
            .filter(|&&r| r != 0)
            .count()
    }

    /// Bit length of the number of periods.
    pub fn period_count_bits(&self) -> u64 {
        crate::types::max_numeric_magnitude_bits([self.num_periods])
    }
}

impl Problem for TimetableDesign {
    const NAME: &'static str = "TimetableDesign";
    type Solution = Vec<Vec<Vec<bool>>>;
    type Value = crate::types::Or;

    crate::problem_parameters![
        ("num_craftsmen", num_craftsmen),
        ("num_available_assignments", num_available_assignments),
        ("num_nonzero_requirements", num_nonzero_requirements),
        ("period_count_bits", period_count_bits),
        ("num_periods", num_periods),
        ("num_tasks", num_tasks),
    ];

    fn evaluate(
        &self,
        solution: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        if solution.len() != self.num_craftsmen
            || solution.iter().any(|craftsman| {
                craftsman.len() != self.num_tasks
                    || craftsman.iter().any(|task| task.len() != self.num_periods)
            })
        {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "timetable dimensions do not match the instance".into(),
            ));
        }
        let config = solution
            .iter()
            .flatten()
            .flatten()
            .copied()
            .collect::<Vec<_>>();
        Ok({
            crate::types::Or({
                if config.len() != self.config_len() {
                    return Ok(crate::types::Or(false));
                }
                let mut craftsman_busy = vec![vec![false; self.num_periods]; self.num_craftsmen];
                let mut task_busy = vec![vec![false; self.num_periods]; self.num_tasks];
                let mut pair_counts = vec![vec![0i64; self.num_tasks]; self.num_craftsmen];

                for craftsman in 0..self.num_craftsmen {
                    for task in 0..self.num_tasks {
                        for period in 0..self.num_periods {
                            if !config[self.index(craftsman, task, period)] {
                                continue;
                            }

                            if !self.craftsman_avail[craftsman][period]
                                || !self.task_avail[task][period]
                            {
                                return Ok(crate::types::Or(false));
                            }

                            if craftsman_busy[craftsman][period] || task_busy[task][period] {
                                return Ok(crate::types::Or(false));
                            }

                            craftsman_busy[craftsman][period] = true;
                            task_busy[task][period] = true;
                            pair_counts[craftsman][task] += 1;
                        }
                    }
                }

                pair_counts == self.requirements
            })
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl crate::solvers::BruteForceProblem for TimetableDesign {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.config_len()]
    }
}

crate::declare_variants! {
    default TimetableDesign => "2^(num_craftsmen * num_tasks * num_periods)" create TimetableDesignCreateSpec,
}

crate::register_brute_force! {
    TimetableDesign decode |problem: &TimetableDesign, indices: Vec<usize>| (0..problem.num_craftsmen()).map(|craftsman| (0..problem.num_tasks()).map(|task| (0..problem.num_periods()).map(|period| indices[problem.index(craftsman, task, period)] != 0).collect()).collect()).collect(),
}

#[cfg(any(test, feature = "example-db"))]
const ISSUE_EXAMPLE_ASSIGNMENTS: &[(usize, usize, usize)] = &[
    (0, 0, 0),
    (1, 4, 0),
    (1, 1, 1),
    (2, 3, 1),
    (0, 2, 2),
    (3, 4, 2),
    (4, 1, 2),
];

#[cfg(any(test, feature = "example-db"))]
fn issue_example_problem() -> TimetableDesign {
    TimetableDesign::new(
        3,
        5,
        5,
        vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![false, true, true],
            vec![true, false, true],
            vec![true, true, true],
        ],
        vec![
            vec![true, true, false],
            vec![false, true, true],
            vec![true, false, true],
            vec![true, true, true],
            vec![true, true, true],
        ],
        vec![
            vec![1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1],
            vec![0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1],
            vec![0, 1, 0, 0, 0],
        ],
    )
}

#[cfg(any(test, feature = "example-db"))]
fn issue_example_config() -> Vec<Vec<Vec<bool>>> {
    let problem = issue_example_problem();
    let mut config = vec![
        vec![vec![false; problem.num_periods()]; problem.num_tasks()];
        problem.num_craftsmen()
    ];
    for &(craftsman, task, period) in ISSUE_EXAMPLE_ASSIGNMENTS {
        config[craftsman][task][period] = true;
    }
    config
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "timetable_design",
        instance: Box::new(issue_example_problem()),
        optimal_config: serde_json::json!(issue_example_config()),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/timetable_design.rs"]
mod tests;
