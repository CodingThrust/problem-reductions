//! Reduction from SequencingWithReleaseTimesAndDeadlines to `ILP<bool>`.
//!
//! Time-indexed formulation: binary x_{j,t} = 1 iff task j starts at time t.
//! Each task starts within its admissible window [r_j, d_j - p_j].
//! No two tasks may overlap on the single machine.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::SequencingWithReleaseTimesAndDeadlines;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing SequencingWithReleaseTimesAndDeadlines to `ILP<bool>`.
///
/// Variable layout: x_{j,t} at index `j * T + t` for j in 0..n, t in 0..T,
/// where T = time_horizon (max deadline).
#[derive(Debug, Clone)]
pub struct ReductionSWRTDToILP {
    target: ILP<bool>,
    num_tasks: usize,
    time_horizon: usize,
}

impl ReductionResult for ReductionSWRTDToILP {
    type Source = SequencingWithReleaseTimesAndDeadlines;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract by reading each task's start time and sorting tasks by start time.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let n = self.num_tasks;
            let horizon = self.time_horizon;
            // For each task, find the start time
            let starts =
                crate::rules::ilp_helpers::one_hot_decode_rows(target_solution, n, horizon, 0)?;
            let mut start_times: Vec<_> = starts.into_iter().enumerate().collect();
            // Sort by start time (break ties by task index)
            start_times.sort_by_key(|&(j, t)| (t, j));
            let schedule: Vec<usize> = start_times.iter().map(|&(j, _)| j).collect();
            schedule
        })
    }
}

#[reduction(transform = upper_bound {
    num_vars = "num_tasks * time_horizon",
    num_constraints = "num_tasks * time_horizon + num_tasks + time_horizon",
},
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for SequencingWithReleaseTimesAndDeadlines {
    type Result = ReductionSWRTDToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
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
            let last_start = deadlines[j]
                .checked_sub(lengths[j])
                .and_then(|time| usize::try_from(time).ok());
            let terms: Vec<(usize, i64)> = last_start
                .filter(|&last| r <= last)
                .into_iter()
                .flat_map(|last| r..=last)
                .filter(|&t| t < horizon)
                .map(|t| (var(j, t), 1))
                .collect();
            constraints.push(LinearConstraint::eq(terms, 1));

            // Zero-fix variables outside the admissible window
            for t in 0..horizon {
                if t < r || last_start.is_none_or(|last| t > last) {
                    constraints.push(LinearConstraint::eq(vec![(var(j, t), 1)], 0));
                }
            }
        }

        // 2. No overlap: for each time instant tau in 0..horizon,
        // Σ_{j,t : t <= tau < t + p_j} x_{j,t} <= 1
        for tau in 0..horizon {
            let mut terms: Vec<(usize, i64)> = Vec::new();
            for (j, &len_j) in lengths.iter().enumerate() {
                let p = len_j as usize;
                // Task j started at time t overlaps tau iff t <= tau < t + p_j
                // i.e., tau - p_j + 1 <= t <= tau, where t >= 0
                let t_min = (tau + 1).saturating_sub(p);
                let t_max = tau;
                for t in t_min..=t_max {
                    if t < horizon {
                        terms.push((var(j, t), 1));
                    }
                }
            }
            constraints.push(LinearConstraint::le(terms, 1));
        }

        Ok(ReductionSWRTDToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize)
                .map_err(Self::target_construction)?,
            num_tasks: n,
            time_horizon: horizon,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "sequencingwithreleasetimesanddeadlines_to_ilp",
        build: || {
            let source = SequencingWithReleaseTimesAndDeadlines::new(
                vec![1, 2, 1],
                vec![0, 0, 2],
                vec![3, 3, 4],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sequencingwithreleasetimesanddeadlines_ilp.rs"]
mod tests;
