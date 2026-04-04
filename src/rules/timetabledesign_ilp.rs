//! Reduction from TimetableDesign to ILP<bool>.
//!
//! The source witness is a binary craftsman-task-period incidence table,
//! and all feasibility conditions are already linear: availability forcing,
//! per-period exclusivity, and exact pairwise work requirements.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::TimetableDesign;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing TimetableDesign to ILP<bool>.
///
/// Variable layout: x_{c,t,h} at index `((c * num_tasks) + t) * num_periods + h`
/// exactly matching the source configuration layout.
#[derive(Debug, Clone)]
pub struct ReductionTDToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionTDToILP {
    type Source = TimetableDesign;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract: direct identity mapping — the ILP variable layout matches the
    /// source configuration layout exactly.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = {
    num_vars = "num_craftsmen * num_tasks * num_periods",
    num_constraints = "num_craftsmen * num_periods + num_tasks * num_periods + num_craftsmen * num_tasks",
})]
impl ReduceTo<ILP<bool>> for TimetableDesign {
    type Result = ReductionTDToILP;

    fn reduce_to(&self) -> Self::Result {
        let nc = self.num_craftsmen();
        let nt = self.num_tasks();
        let nh = self.num_periods();
        let num_vars = nc * nt * nh;

        let var = |c: usize, t: usize, h: usize| -> usize { ((c * nt) + t) * nh + h };

        let mut constraints = Vec::new();

        // 1. Availability: x_{c,t,h} = 0 whenever craftsman c or task t is unavailable in h
        for c in 0..nc {
            for t in 0..nt {
                for h in 0..nh {
                    if !self.craftsman_avail()[c][h] || !self.task_avail()[t][h] {
                        constraints.push(LinearConstraint::eq(vec![(var(c, t, h), 1.0)], 0.0));
                    }
                }
            }
        }

        // 2. Each craftsman works on at most one task per period: Σ_t x_{c,t,h} <= 1 for all c, h
        for c in 0..nc {
            for h in 0..nh {
                let terms: Vec<(usize, f64)> = (0..nt).map(|t| (var(c, t, h), 1.0)).collect();
                constraints.push(LinearConstraint::le(terms, 1.0));
            }
        }

        // 3. Each task worked on by at most one craftsman per period: Σ_c x_{c,t,h} <= 1 for all t, h
        for t in 0..nt {
            for h in 0..nh {
                let terms: Vec<(usize, f64)> = (0..nc).map(|c| (var(c, t, h), 1.0)).collect();
                constraints.push(LinearConstraint::le(terms, 1.0));
            }
        }

        // 4. Exact requirements: Σ_h x_{c,t,h} = r_{c,t} for all c, t
        for c in 0..nc {
            for t in 0..nt {
                let terms: Vec<(usize, f64)> = (0..nh).map(|h| (var(c, t, h), 1.0)).collect();
                constraints.push(LinearConstraint::eq(
                    terms,
                    self.requirements()[c][t] as f64,
                ));
            }
        }

        ReductionTDToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "timetabledesign_to_ilp",
        build: || {
            // Small 2-craftsman, 2-task, 2-period instance
            let source = TimetableDesign::new(
                2,
                2,
                2,
                vec![vec![true, true], vec![true, true]],
                vec![vec![true, true], vec![true, true]],
                vec![vec![1, 0], vec![0, 1]],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/timetabledesign_ilp.rs"]
mod tests;
