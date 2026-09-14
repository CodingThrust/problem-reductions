//! Reduction from PartiallyOrderedKnapsack to ILP (Integer Linear Programming).
//!
//! Binary variable x_i per item. Capacity constraint Σ w_i·x_i ≤ C.
//! Precedence constraints: ∀ (a,b): x_b ≤ x_a. Maximize Σ v_i·x_i.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::PartiallyOrderedKnapsack;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;
use crate::solvers::SolveOutcome;

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

    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        match target {
            SolveOutcome::Infeasible => Ok(SolveOutcome::Infeasible),
            SolveOutcome::Optimal { solution, .. } => {
                let solution = self.map_solution(&solution)?;
                Ok(SolveOutcome::optimal(source, solution)?)
            }
            SolveOutcome::Feasible { solution, .. } => {
                let solution = self.map_solution(&solution)?;
                Ok(SolveOutcome::feasible(source, solution)?)
            }
        }
    }
}

impl ReductionPOKToILP {
    fn map_solution(
        &self,
        target_solution: &<<Self as ReductionResult>::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<
        <<Self as ReductionResult>::Source as crate::traits::Problem>::Solution,
    > {
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
        let values = self.values();
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
        let objective = values.iter().copied().enumerate().collect();

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
