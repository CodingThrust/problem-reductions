//! Reduction from Satisfiability to NonTautology via negation.
//!
//! Negating a CNF formula with De Morgan's law turns each clause into a DNF
//! disjunct whose literals all have their signs flipped.

use crate::models::formula::{NonTautology, Satisfiability};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

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

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = {
    num_vars = "num_vars",
    num_disjuncts = "num_clauses",
})]
impl ReduceTo<NonTautology> for Satisfiability {
    type Result = ReductionSATToNonTautology;

    fn reduce_to(&self) -> Self::Result {
        let disjuncts = self
            .clauses()
            .iter()
            .map(|clause| clause.literals.iter().map(|&lit| -lit).collect())
            .collect();

        ReductionSATToNonTautology {
            target: NonTautology::new(self.num_vars(), disjuncts),
        }
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
                    source_config: vec![1, 0, 1],
                    target_config: vec![1, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/satisfiability_nontautology.rs"]
mod tests;
