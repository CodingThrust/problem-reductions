//! Reduction from Set Splitting to NAE-Satisfiability.
//!
//! Create one Boolean variable for each universe element and one positive-literal
//! NAE clause for each subset. Repeated members are removed in first-occurrence
//! order. An all-repeated subset becomes `(x_u, x_u)`, which is unsatisfiable.

use std::collections::HashSet;

use crate::models::formula::{CNFClause, NAESatisfiability};
use crate::models::set::SetSplitting;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionSetSplittingToNAESatisfiability {
    target: NAESatisfiability,
}

impl ReductionResult for ReductionSetSplittingToNAESatisfiability {
    type Source = SetSplitting;
    type Target = NAESatisfiability;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "universe_size",
        num_clauses = "num_subsets",
        num_literals = "(universe_size + 1) * num_subsets",
    }
)]
impl ReduceTo<NAESatisfiability> for SetSplitting {
    type Result = ReductionSetSplittingToNAESatisfiability;

    fn reduce_to(&self) -> Self::Result {
        let clauses = self
            .subsets()
            .iter()
            .map(|subset| {
                let mut seen = HashSet::new();
                let mut literals: Vec<_> = subset
                    .iter()
                    .copied()
                    .filter(|element| seen.insert(*element))
                    .map(|element| (element + 1) as i32)
                    .collect();

                if literals.len() == 1 {
                    literals.push(literals[0]);
                }

                CNFClause::new(literals)
            })
            .collect();

        ReductionSetSplittingToNAESatisfiability {
            target: NAESatisfiability::new(self.universe_size(), clauses),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "setsplitting_to_naesatisfiability",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, NAESatisfiability>(
                SetSplitting::new(4, vec![vec![0, 1], vec![1, 2, 3], vec![0, 2, 3]]),
                SolutionPair {
                    source_config: vec![0, 1, 0, 1],
                    target_config: vec![0, 1, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/setsplitting_naesatisfiability.rs"]
mod tests;
