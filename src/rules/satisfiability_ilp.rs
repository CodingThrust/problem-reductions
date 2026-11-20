//! One binary variable per SAT variable and one inequality per clause.

use crate::models::algebraic::{ILP, LinearConstraint, ObjectiveSense};
use crate::models::formula::Satisfiability;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::rules::{ExtractionResult, ReductionError};

#[derive(Debug, Clone)]
pub struct ReductionSATToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionSATToILP {
    type Source = Satisfiability;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, solution: &Vec<i64>) -> ExtractionResult<Vec<bool>> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;
        Ok(solution.iter().map(|&value| value == 1).collect())
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionSATToILP {}

#[reduction(transform = {
    exact {
        num_vars = "num_vars",
        num_constraints = "num_clauses",
    },
    upper_bound {
        num_nonzeros = "num_literals",
        max_constraint_magnitude_bits = "num_literals + 1",
    },
})]
impl ReduceTo<ILP<bool>> for Satisfiability {
    type Result = ReductionSATToILP;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        let mut constraints = Vec::with_capacity(self.num_clauses());
        for clause in self.clauses() {
            let negatives = clause
                .literals
                .iter()
                .filter(|&&literal| literal < 0)
                .count();
            let negatives = <Self as ReduceTo<ILP<bool>>>::exact_i64(
                negatives,
                "counting negative clause literals",
            )?;
            // ILP normalization merges duplicate terms and cancels opposite terms.
            let terms = clause
                .variables()
                .into_iter()
                .zip(clause.literals.iter().map(|literal| literal.signum()))
                .collect();
            constraints.push(LinearConstraint::ge(terms, 1 - negatives));
        }
        let target = ILP::new(
            self.num_vars(),
            constraints,
            vec![],
            ObjectiveSense::Minimize,
        )
        .map_err(<Self as ReduceTo<ILP<bool>>>::target_construction)?;
        Ok(Self::Result { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::models::formula::CNFClause;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "satisfiability_to_ilp",
        build: || {
            let source = Satisfiability::new(
                3,
                vec![
                    CNFClause::new(vec![1, 2, 2]),
                    CNFClause::new(vec![-1, 3]),
                    CNFClause::new(vec![-2, -3]),
                    CNFClause::new(vec![1, -1]),
                ],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/satisfiability_ilp.rs"]
mod tests;
