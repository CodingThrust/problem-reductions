//! Reduction from Satisfiability to NonTautology via negation.
//!
//! Negating a CNF formula with De Morgan's law turns each clause into a DNF
//! disjunct whose literals all have their signs flipped.

use crate::models::formula::{NonTautology, Satisfiability};
use crate::reduction;
use crate::rules::traits::{recover_preserving_status, ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;

/// Result of reducing SAT to NonTautology.
#[derive(Debug, Clone)]
pub struct ReductionSATToNonTautology {
    target: NonTautology,
}

impl ReductionResult for ReductionSATToNonTautology {
    type Source = Satisfiability;
    type Target = NonTautology;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        recover_preserving_status(source, target, |solution| Ok(solution.clone()))
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_vars",
        num_disjuncts = "num_clauses",
    })]
impl ReduceTo<NonTautology> for Satisfiability {
    type Result = ReductionSATToNonTautology;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let disjuncts = self
            .clauses()
            .iter()
            .map(|clause| clause.literals.iter().map(|&lit| -lit).collect())
            .collect();

        Ok(ReductionSATToNonTautology {
            target: NonTautology::new(self.num_vars(), disjuncts).map_err(|error| {
                crate::rules::ReductionError::construction::<Satisfiability, NonTautology>(error)
            })?,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "satisfiability_to_nontautology",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, NonTautology>(
                Satisfiability::new(
                    3,
                    vec![
                        CNFClause::new(vec![1, 2]),
                        CNFClause::new(vec![-1, 3]),
                        CNFClause::new(vec![-2, -3]),
                    ],
                ),
                SolutionPair {
                    source_config: serde_json::json!(vec![true, false, true]),
                    target_config: serde_json::json!(vec![true, false, true]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/satisfiability_nontautology.rs"]
mod tests;
