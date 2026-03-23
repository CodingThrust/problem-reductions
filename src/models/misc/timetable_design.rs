//! Timetable Design problem implementation.
//!
//! Decide whether craftsmen can be assigned to tasks across work periods while
//! respecting availability, per-period exclusivity, and exact pairwise work
//! requirements.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "TimetableDesign",
        display_name: "Timetable Design",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Assign craftsmen to tasks over work periods subject to availability and exact pairwise requirements",
        fields: &[
            FieldInfo { name: "num_periods", type_name: "usize", description: "Number of work periods |H|" },
            FieldInfo { name: "num_craftsmen", type_name: "usize", description: "Number of craftsmen |C|" },
            FieldInfo { name: "num_tasks", type_name: "usize", description: "Number of tasks |T|" },
            FieldInfo { name: "craftsman_avail", type_name: "Vec<Vec<bool>>", description: "Availability matrix A(c) for craftsmen (|C| x |H|)" },
            FieldInfo { name: "task_avail", type_name: "Vec<Vec<bool>>", description: "Availability matrix A(t) for tasks (|T| x |H|)" },
            FieldInfo { name: "requirements", type_name: "Vec<Vec<u64>>", description: "Required work periods R(c,t) for each craftsman-task pair (|C| x |T|)" },
        ],
    }
}

/// The Timetable Design problem.
///
/// A configuration is a flattened binary tensor `f(c,t,h)` in craftsman-major,
/// task-next, period-last order:
/// `idx = ((c * num_tasks) + t) * num_periods + h`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimetableDesign {
    num_periods: usize,
    num_craftsmen: usize,
    num_tasks: usize,
    craftsman_avail: Vec<Vec<bool>>,
    task_avail: Vec<Vec<bool>>,
    requirements: Vec<Vec<u64>>,
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
        requirements: Vec<Vec<u64>>,
    ) -> Self {
        assert_eq!(
            craftsman_avail.len(),
            num_craftsmen,
            "craftsman_avail has {} rows, expected {}",
            craftsman_avail.len(),
            num_craftsmen
        );
        for (craftsman, row) in craftsman_avail.iter().enumerate() {
            assert_eq!(
                row.len(),
                num_periods,
                "craftsman {} availability has {} periods, expected {}",
                craftsman,
                row.len(),
                num_periods
            );
        }

        assert_eq!(
            task_avail.len(),
            num_tasks,
            "task_avail has {} rows, expected {}",
            task_avail.len(),
            num_tasks
        );
        for (task, row) in task_avail.iter().enumerate() {
            assert_eq!(
                row.len(),
                num_periods,
                "task {} availability has {} periods, expected {}",
                task,
                row.len(),
                num_periods
            );
        }

        assert_eq!(
            requirements.len(),
            num_craftsmen,
            "requirements has {} rows, expected {}",
            requirements.len(),
            num_craftsmen
        );
        for (craftsman, row) in requirements.iter().enumerate() {
            assert_eq!(
                row.len(),
                num_tasks,
                "requirements row {} has {} tasks, expected {}",
                craftsman,
                row.len(),
                num_tasks
            );
        }

        Self {
            num_periods,
            num_craftsmen,
            num_tasks,
            craftsman_avail,
            task_avail,
            requirements,
        }
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
    pub fn requirements(&self) -> &[Vec<u64>] {
        &self.requirements
    }

    fn config_len(&self) -> usize {
        self.num_craftsmen * self.num_tasks * self.num_periods
    }

    fn index(&self, craftsman: usize, task: usize, period: usize) -> usize {
        ((craftsman * self.num_tasks) + task) * self.num_periods + period
    }

    #[cfg(feature = "ilp-solver")]
    pub(crate) fn solve_via_required_assignments(&self) -> Option<Vec<usize>> {
        #[derive(Clone)]
        struct PairRequirement {
            craftsman: usize,
            task: usize,
            required: usize,
            allowed_periods: Vec<usize>,
        }

        let mut craftsman_demand = vec![0usize; self.num_craftsmen];
        let mut task_demand = vec![0usize; self.num_tasks];
        let mut pairs = Vec::new();

        for (craftsman, requirement_row) in self.requirements.iter().enumerate() {
            for (task, required_u64) in requirement_row.iter().enumerate() {
                let required = usize::try_from(*required_u64).ok()?;
                craftsman_demand[craftsman] += required;
                task_demand[task] += required;

                if required == 0 {
                    continue;
                }

                let allowed_periods = (0..self.num_periods)
                    .filter(|&period| {
                        self.craftsman_avail[craftsman][period] && self.task_avail[task][period]
                    })
                    .collect::<Vec<_>>();

                if allowed_periods.len() < required {
                    return None;
                }

                pairs.push(PairRequirement {
                    craftsman,
                    task,
                    required,
                    allowed_periods,
                });
            }
        }

        if craftsman_demand
            .iter()
            .zip(&self.craftsman_avail)
            .any(|(demand, avail)| *demand > avail.iter().filter(|&&v| v).count())
        {
            return None;
        }

        if task_demand
            .iter()
            .zip(&self.task_avail)
            .any(|(demand, avail)| *demand > avail.iter().filter(|&&v| v).count())
        {
            return None;
        }

        pairs.sort_by_key(|pair| (pair.allowed_periods.len(), pair.required));

        struct SearchState<'a> {
            problem: &'a TimetableDesign,
            pairs: &'a [PairRequirement],
            craftsman_busy: Vec<Vec<bool>>,
            task_busy: Vec<Vec<bool>>,
            config: Vec<usize>,
        }

        impl SearchState<'_> {
            fn search_pair(
                &mut self,
                pair_index: usize,
                period_offset: usize,
                remaining: usize,
            ) -> bool {
                if pair_index == self.pairs.len() {
                    return true;
                }

                let pair = &self.pairs[pair_index];
                if remaining == 0 {
                    return self.search_pair(
                        pair_index + 1,
                        0,
                        self.pairs
                            .get(pair_index + 1)
                            .map_or(0, |next| next.required),
                    );
                }

                let feasible_remaining = pair.allowed_periods[period_offset..]
                    .iter()
                    .filter(|&&period| {
                        !self.craftsman_busy[pair.craftsman][period]
                            && !self.task_busy[pair.task][period]
                    })
                    .count();
                if feasible_remaining < remaining {
                    return false;
                }

                for candidate_index in period_offset..pair.allowed_periods.len() {
                    let period = pair.allowed_periods[candidate_index];
                    if self.craftsman_busy[pair.craftsman][period]
                        || self.task_busy[pair.task][period]
                    {
                        continue;
                    }

                    self.craftsman_busy[pair.craftsman][period] = true;
                    self.task_busy[pair.task][period] = true;
                    self.config[self.problem.index(pair.craftsman, pair.task, period)] = 1;

                    if self.search_pair(pair_index, candidate_index + 1, remaining - 1) {
                        return true;
                    }

                    self.config[self.problem.index(pair.craftsman, pair.task, period)] = 0;
                    self.task_busy[pair.task][period] = false;
                    self.craftsman_busy[pair.craftsman][period] = false;
                }

                false
            }
        }

        let mut state = SearchState {
            problem: self,
            pairs: &pairs,
            craftsman_busy: vec![vec![false; self.num_periods]; self.num_craftsmen],
            task_busy: vec![vec![false; self.num_periods]; self.num_tasks],
            config: vec![0; self.config_len()],
        };

        if state.search_pair(0, 0, pairs.first().map_or(0, |pair| pair.required)) {
            Some(state.config)
        } else {
            None
        }
    }
}

impl Problem for TimetableDesign {
    const NAME: &'static str = "TimetableDesign";
    type Value = bool;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.config_len()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.config_len() {
            return false;
        }
        if config.iter().any(|&value| value > 1) {
            return false;
        }

        let mut craftsman_busy = vec![vec![false; self.num_periods]; self.num_craftsmen];
        let mut task_busy = vec![vec![false; self.num_periods]; self.num_tasks];
        let mut pair_counts = vec![vec![0u64; self.num_tasks]; self.num_craftsmen];

        for craftsman in 0..self.num_craftsmen {
            for task in 0..self.num_tasks {
                for period in 0..self.num_periods {
                    if config[self.index(craftsman, task, period)] == 0 {
                        continue;
                    }

                    if !self.craftsman_avail[craftsman][period] || !self.task_avail[task][period] {
                        return false;
                    }

                    if craftsman_busy[craftsman][period] || task_busy[task][period] {
                        return false;
                    }

                    craftsman_busy[craftsman][period] = true;
                    task_busy[task][period] = true;
                    pair_counts[craftsman][task] += 1;
                }
            }
        }

        pair_counts == self.requirements
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for TimetableDesign {}

crate::declare_variants! {
    default sat TimetableDesign => "2^(num_craftsmen * num_tasks * num_periods)",
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
fn issue_example_config() -> Vec<usize> {
    let problem = issue_example_problem();
    let mut config = vec![0; problem.config_len()];
    for &(craftsman, task, period) in ISSUE_EXAMPLE_ASSIGNMENTS {
        config[problem.index(craftsman, task, period)] = 1;
    }
    config
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "timetable_design",
        instance: Box::new(issue_example_problem()),
        optimal_config: issue_example_config(),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/timetable_design.rs"]
mod tests;
