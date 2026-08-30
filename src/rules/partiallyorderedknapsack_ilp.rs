//! Reduction from PartiallyOrderedKnapsack to ILP (Integer Linear Programming).
//!
//! Binary variable x_i per item. Capacity constraint Σ w_i·x_i ≤ C.
//! Precedence constraints: ∀ (a,b): x_b ≤ x_a. Maximize Σ v_i·x_i.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::PartiallyOrderedKnapsack;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::i64_to_exact_f64;

#[derive(Debug, Clone)]
pub struct ReductionPOKToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionPOKToILP {
    type Source = PartiallyOrderedKnapsack;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.iter().map(|&value| value == 1).collect())
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_items",
        num_constraints = "num_precedences + 1",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for PartiallyOrderedKnapsack {
    type Result = ReductionPOKToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_items();
        let mut constraints = Vec::new();
        let weights = self.weights();
        let values = self
            .values()
            .iter()
            .copied()
            .map(i64_to_exact_f64)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                crate::rules::ReductionError::inexact_float_conversion::<
                    PartiallyOrderedKnapsack,
                    ILP<bool>,
                >(error)
            })?;
        let capacity = self.capacity();

        // Capacity constraint: Σ w_i·x_i ≤ capacity
        let cap_terms: Vec<(usize, i64)> = weights
            .iter()
            .enumerate()
            .map(|(item, &weight)| (item, weight))
            .collect();
        constraints.push(LinearConstraint::le(cap_terms, capacity));

        // Precedence constraints: ∀ (a,b): x_b - x_a ≤ 0
        for &(a, b) in self.precedences() {
            constraints.push(LinearConstraint::le(vec![(b, 1), (a, -1)], 0));
        }

        // Objective: Maximize Σ v_i·x_i
        let objective = values.into_iter().enumerate().collect();

        let target = ILP::new(n, constraints, objective, ObjectiveSense::Maximize)
            .map_err(Self::target_construction)?;
        Ok(ReductionPOKToILP { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partiallyorderedknapsack_to_ilp",
        build: || {
            let source =
                PartiallyOrderedKnapsack::new(vec![2, 3, 1], vec![3, 4, 2], vec![(0, 1)], 4);
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partiallyorderedknapsack_ilp.rs"]
mod tests;
