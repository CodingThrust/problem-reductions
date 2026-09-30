//! One binary variable per SAT variable and one equality per exact-one clause.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::formula::OneInThreeSatisfiability;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionOneInThreeSatisfiabilityToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionOneInThreeSatisfiabilityToILP {
    type Source = OneInThreeSatisfiability;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, solution: &Vec<i64>) -> crate::rules::ExtractionResult<Vec<bool>> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            solution,
            |value| value.value.is_some(),
            "target ILP assignment does not satisfy the exact-one clauses",
        )?;
        Ok(solution.iter().map(|&value| value == 1).collect())
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionOneInThreeSatisfiabilityToILP {}

#[reduction(transform = {
    exact {
        num_vars = "num_vars",
        num_constraints = "num_clauses",
    },
    upper_bound {
        num_nonzeros = "3 * num_clauses",
        max_constraint_magnitude_bits = "2",
    },
})]
impl ReduceTo<ILP<bool>> for OneInThreeSatisfiability {
    type Result = ReductionOneInThreeSatisfiabilityToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let mut constraints = Vec::new();
        for clause in self.clauses() {
            let mut terms = Vec::new();
            let mut rhs = 1;
            for (variable, &literal) in clause.variables().into_iter().zip(&clause.literals) {
                if literal > 0 {
                    terms.push((variable, 1));
                } else {
                    terms.push((variable, -1));
                    rhs -= 1;
                }
            }
            constraints.push(LinearConstraint::eq(terms, rhs));
        }
        let target = ILP::new(
            self.num_vars(),
            constraints,
            vec![],
            ObjectiveSense::Minimize,
        )
        .map_err(Self::target_construction)?;
        Ok(Self::Result { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::models::formula::CNFClause;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "oneinthreesatisfiability_to_ilp",
        build: || {
            let source = OneInThreeSatisfiability::new(
                3,
                vec![
                    CNFClause::new(vec![1, 2, 3]),
                    CNFClause::new(vec![-1, -2, 3]),
                ],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/oneinthreesatisfiability_ilp.rs"]
mod tests;
