//! Reduction from TimetableDesign to `ILP<bool>`.
//!
//! The source witness is a binary craftsman-task-period incidence table,
//! and all feasibility conditions are already linear: availability forcing,
//! per-period exclusivity, and exact pairwise work requirements.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::TimetableDesign;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing TimetableDesign to `ILP<bool>`.
#[derive(Debug, Clone)]
pub struct ReductionTDToILP {
    target: ILP<bool>,
    num_craftsmen: usize,
    num_tasks: usize,
    num_periods: usize,
    assignments: Vec<(usize, usize, usize)>,
}

impl ReductionResult for ReductionTDToILP {
    type Source = TimetableDesign;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;

        let mut timetable =
            vec![vec![vec![false; self.num_periods]; self.num_tasks]; self.num_craftsmen];
        for (variable, &(craftsman, task, period)) in self.assignments.iter().enumerate() {
            timetable[craftsman][task][period] = target_solution[variable] == 1;
        }
        Ok(timetable)
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionTDToILP {}

#[reduction(transform = {
    exact {
        num_vars = "num_available_assignments",
        num_nonzeros = "3 * num_available_assignments",
    },
    upper_bound {
        num_constraints = "2 * num_available_assignments + num_nonzero_requirements",
        max_constraint_magnitude_bits = "period_count_bits + 1",
    },
})]
impl ReduceTo<ILP<bool>> for TimetableDesign {
    type Result = ReductionTDToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let nc = self.num_craftsmen();
        let nt = self.num_tasks();
        let nh = self.num_periods();
        let requirements = self.requirements();
        // A pair can work at most nh periods. Keep out-of-range requirements infeasible.
        let max_requirement = Self::exact_i64(nh, "encoding the period count")?.saturating_add(1);
        let mut assignments = Vec::new();
        let mut craftsmen = std::collections::BTreeMap::<_, Vec<_>>::new();
        let mut tasks = std::collections::BTreeMap::<_, Vec<_>>::new();
        let mut pairs = Vec::new();
        for (c, row) in requirements.iter().enumerate() {
            for (t, &requirement) in row.iter().enumerate() {
                if requirement == 0 {
                    continue;
                }
                let mut terms = Vec::new();
                if requirement > 0 {
                    for h in 0..nh {
                        if self.craftsman_avail()[c][h] && self.task_avail()[t][h] {
                            let term = (assignments.len(), 1);
                            assignments.push((c, t, h));
                            terms.push(term);
                            craftsmen.entry((c, h)).or_default().push(term);
                            tasks.entry((t, h)).or_default().push(term);
                        }
                    }
                }
                pairs.push(LinearConstraint::eq(
                    terms,
                    requirement.clamp(-1, max_requirement),
                ));
            }
        }
        let mut constraints: Vec<_> = craftsmen
            .into_values()
            .chain(tasks.into_values())
            .map(|terms| LinearConstraint::le(terms, 1))
            .collect();
        constraints.extend(pairs);
        Ok(ReductionTDToILP {
            target: ILP::new(
                assignments.len(),
                constraints,
                vec![],
                ObjectiveSense::Minimize,
            )
            .map_err(Self::target_construction)?,
            num_craftsmen: nc,
            num_tasks: nt,
            num_periods: nh,
            assignments,
        })
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
