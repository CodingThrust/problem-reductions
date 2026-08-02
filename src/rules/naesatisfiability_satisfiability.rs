//! Reduction from Not-All-Equal Satisfiability to Satisfiability.

use crate::models::formula::{CNFClause, NAESatisfiability, Satisfiability};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing NAE-SAT to SAT.
#[derive(Debug, Clone)]
pub struct ReductionNAESATToSAT {
    target: Satisfiability,
}

impl ReductionResult for ReductionNAESATToSAT {
    type Source = NAESatisfiability;
    type Target = Satisfiability;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = {
    num_vars = "num_vars",
    num_clauses = "2 * num_clauses",
    num_literals = "2 * num_literals",
})]
impl ReduceTo<Satisfiability> for NAESatisfiability {
    type Result = ReductionNAESATToSAT;

    fn reduce_to(&self) -> Self::Result {
        let clauses = self
            .clauses()
            .iter()
            .flat_map(|clause| {
                [
                    clause.clone(),
                    CNFClause::new(clause.literals.iter().map(|literal| -literal).collect()),
                ]
            })
            .collect();

        ReductionNAESATToSAT {
            target: Satisfiability::new(self.num_vars(), clauses),
        }
    }
}

#[cfg(any(test, feature = "example-db"))]
fn canonical_source() -> NAESatisfiability {
    NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 2]),
            CNFClause::new(vec![1, -2, -3]),
        ],
    )
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "naesatisfiability_to_satisfiability",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Satisfiability>(
                canonical_source(),
                SolutionPair {
                    source_config: vec![0, 0, 1],
                    target_config: vec![0, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/naesatisfiability_satisfiability.rs"]
mod tests;
