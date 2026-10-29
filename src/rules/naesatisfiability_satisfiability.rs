//! Reduction from NAE-Satisfiability to Satisfiability.
//!
//! A not-all-equal clause `(l_1, ..., l_r)` holds exactly when at least one
//! literal is true and at least one literal is false. Each NAE clause therefore
//! becomes two CNF clauses over the same variables: `(l_1 ∨ ... ∨ l_r)` excludes
//! the all-false assignment and `(¬l_1 ∨ ... ∨ ¬l_r)` excludes the all-true one.

use crate::models::formula::{CNFClause, NAESatisfiability, Satisfiability};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing NAE-Satisfiability to Satisfiability.
#[derive(Debug, Clone)]
pub struct ReductionNAESATToSAT {
    /// The target SAT problem.
    target: Satisfiability,
}

impl ReductionResult for ReductionNAESATToSAT {
    type Source = NAESatisfiability;
    type Target = Satisfiability;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.0,
            "target assignment does not satisfy CNF clauses",
        )?;

        Ok(target_solution.clone())
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionNAESATToSAT {}

#[reduction(transform = exact {
    num_vars = "num_vars",
    num_clauses = "2 * num_clauses",
    num_literals = "2 * num_literals",
})]
impl ReduceTo<Satisfiability> for NAESatisfiability {
    type Result = ReductionNAESATToSAT;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let clauses: Vec<CNFClause> = self
            .clauses()
            .iter()
            .flat_map(|clause| {
                let negated = clause.literals.iter().map(|&lit| -lit).collect();
                [
                    CNFClause::new(clause.literals.clone()),
                    CNFClause::new(negated),
                ]
            })
            .collect();

        let target = Satisfiability::try_new(self.num_vars(), clauses).map_err(
            crate::rules::ReductionError::construction::<NAESatisfiability, Satisfiability>,
        )?;

        Ok(ReductionNAESATToSAT { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "naesatisfiability_to_satisfiability",
        build: || {
            let source = NAESatisfiability::new(
                3,
                vec![
                    CNFClause::new(vec![1, 2, 3]),
                    CNFClause::new(vec![-1, 2]),
                    CNFClause::new(vec![1, -2, -3]),
                ],
            );
            crate::example_db::specs::rule_example_with_witness::<_, Satisfiability>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![false, false, true]),
                    target_config: serde_json::json!(vec![false, false, true]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/naesatisfiability_satisfiability.rs"]
mod tests;
