//! Reduction from Knapsack to ILP (Integer Linear Programming).
//!
//! The standard 0-1 knapsack formulation is already a binary ILP:
//! - Variables: one binary variable per item
//! - Constraint: the total selected weight must not exceed capacity
//! - Objective: maximize the total selected value

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::Knapsack;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::i64_to_exact_f64;

/// Result of reducing Knapsack to ILP.
#[derive(Debug, Clone)]
pub struct ReductionKnapsackToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionKnapsackToILP {
    type Source = Knapsack;
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
    size = exact {
        num_vars = "num_items",
        num_constraints = "1",
    },
)]
impl ReduceTo<ILP<bool>> for Knapsack {
    type Result = ReductionKnapsackToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_vars = self.num_items();
        let weights = self.weights();
        let values = self
            .values()
            .iter()
            .copied()
            .map(i64_to_exact_f64)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                crate::rules::ReductionError::inexact_float_conversion::<Knapsack, ILP<bool>>(error)
            })?;
        let capacity = self.capacity();
        let constraints = vec![LinearConstraint::le(
            weights
                .iter()
                .enumerate()
                .map(|(item, &weight)| (item, weight))
                .collect(),
            capacity,
        )];
        let objective = values.into_iter().enumerate().collect();
        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize)
            .map_err(<Self as ReduceTo<ILP<bool>>>::target_construction)?;

        Ok(ReductionKnapsackToILP { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "knapsack_to_ilp",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                Knapsack::new(vec![1, 3, 4, 5], vec![1, 4, 5, 7], 7),
                SolutionPair {
                    source_config: serde_json::json!(vec![false, true, true, false]),
                    target_config: serde_json::json!(vec![0, 1, 1, 0]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/knapsack_ilp.rs"]
mod tests;
