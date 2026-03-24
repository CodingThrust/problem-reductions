//! Reduction from SequencingWithReleaseTimesAndDeadlines to ILP<bool>.
//!
//! Time-indexed formulation: binary x_{j,t} = 1 iff task j starts at time t.
//! Each task starts within its admissible window [r_j, d_j - p_j].
//! No two tasks may overlap on the single machine.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::SequencingWithReleaseTimesAndDeadlines;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing SequencingWithReleaseTimesAndDeadlines to ILP<bool>.
///
/// Variable layout: x_{j,t} at index `j * T + t` for j in 0..n, t in 0..T,
/// where T = time_horizon (max deadline).
#[derive(Debug, Clone)]
pub struct ReductionSWRTDToILP {
    target: ILP<bool>,
    num_tasks: usize,
    time_horizon: usize,
}

impl ReductionSWRTDToILP {
    fn encode_schedule_as_lehmer(schedule: &[usize]) -> Vec<usize> {
        let mut available: Vec<usize> = (0..schedule.len()).collect();
        let mut config = Vec::with_capacity(schedule.len());
        for &task in schedule {
            let digit = available
                .iter()
                .position(|&c| c == task)
                .expect("schedule must be a permutation");
            config.push(digit);
            available.remove(digit);
        }
        config
    }
}

impl ReductionResult for ReductionSWRTDToILP {
    type Source = SequencingWithReleaseTimesAndDeadlines;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract: read each task's start time, sort tasks by start time,
    /// encode as Lehmer code.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_tasks;
        let horizon = self.time_horizon;
        // For each task, find the start time
        let mut start_times: Vec<(usize, usize)> = (0..n)
            .map(|j| {
                let start = (0..horizon)
                    .find(|&t| target_solution.get(j * horizon + t).copied().unwrap_or(0) == 1)
                    .unwrap_or(0);
                (j, start)
            })
            .collect();
        // Sort by start time (break ties by task index)
        start_times.sort_by_key(|&(j, t)| (t, j));
        let schedule: Vec<usize> = start_times.iter().map(|&(j, _)| j).collect();
        Self::encode_schedule_as_lehmer(&schedule)
    }
}

#[reduction(overhead = {
    num_vars = "num_tasks * time_horizon",
    num_constraints = "num_tasks + time_horizon",
})]
impl ReduceTo<ILP<bool>> for SequencingWithReleaseTimesAndDeadlines {
    type Result = ReductionSWRTDToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_tasks();
        let horizon = self.time_horizon() as usize;
        let num_vars = n * horizon;

        let var = |j: usize, t: usize| -> usize { j * horizon + t };

        let lengths = self.lengths();
        let release_times = self.release_times();
        let deadlines = self.deadlines();

        let mut constraints = Vec::new();

        // 1. Each task starts exactly once within its admissible window:
        // Σ_{t=r_j}^{d_j-p_j} x_{j,t} = 1 for all j.
        // Also, x_{j,t} = 0 for t outside the window (handled implicitly
        // by not including them; add explicit zero constraints for safety).
        for j in 0..n {
            let r = release_times[j] as usize;
            let last_start = if deadlines[j] >= lengths[j] {
                (deadlines[j] - lengths[j]) as usize
            } else {
                0
            };
            let terms: Vec<(usize, f64)> = (r..=last_start)
                .filter(|&t| t < horizon)
                .map(|t| (var(j, t), 1.0))
                .collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));

            // Zero-fix variables outside the admissible window
            for t in 0..horizon {
                if t < r || t > last_start {
                    constraints.push(LinearConstraint::eq(vec![(var(j, t), 1.0)], 0.0));
                }
            }
        }

        // 2. No overlap: for each time instant tau in 0..horizon,
        // Σ_{j,t : t <= tau < t + p_j} x_{j,t} <= 1
        for tau in 0..horizon {
            let mut terms: Vec<(usize, f64)> = Vec::new();
            for (j, &len_j) in lengths.iter().enumerate() {
                let p = len_j as usize;
                // Task j started at time t overlaps tau iff t <= tau < t + p_j
                // i.e., tau - p_j + 1 <= t <= tau, where t >= 0
                let t_min = (tau + 1).saturating_sub(p);
                let t_max = tau;
                for t in t_min..=t_max {
                    if t < horizon {
                        terms.push((var(j, t), 1.0));
                    }
                }
            }
            constraints.push(LinearConstraint::le(terms, 1.0));
        }

        ReductionSWRTDToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize),
            num_tasks: n,
            time_horizon: horizon,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "sequencingwithreleasetimesanddeadlines_to_ilp",
        build: || {
            let source = SequencingWithReleaseTimesAndDeadlines::new(
                vec![1, 2, 1],
                vec![0, 0, 2],
                vec![3, 3, 4],
            );
            let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
            let ilp_solution = crate::solvers::ILPSolver::new()
                .solve(reduction.target_problem())
                .expect("canonical example must be solvable");
            let source_config = reduction.extract_solution(&ilp_solution);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config,
                    target_config: ilp_solution,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sequencingwithreleasetimesanddeadlines_ilp.rs"]
mod tests;
