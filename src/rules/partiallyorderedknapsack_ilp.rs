//! Reduction from PartiallyOrderedKnapsack to ILP (Integer Linear Programming).
//!
//! The partially ordered knapsack is a binary ILP with:
//! - One binary variable per item
//! - A capacity constraint: Σ w_i·x_i ≤ capacity
//! - Precedence constraints: for each (a, b), x_b - x_a ≤ 0 (i.e., x_b ≤ x_a)
//! - Objective: maximize Σ v_i·x_i

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::PartiallyOrderedKnapsack;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing PartiallyOrderedKnapsack to ILP.
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

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_items",
        num_constraints = "num_precedences + 1",
    }
)]
impl ReduceTo<ILP<bool>> for PartiallyOrderedKnapsack {
    type Result = ReductionPOKToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_items();

        let mut constraints = Vec::new();

        // Capacity constraint: Σ w_i·x_i ≤ capacity
        let cap_terms: Vec<(usize, f64)> = self
            .weights()
            .iter()
            .enumerate()
            .map(|(i, &w)| (i, w as f64))
            .collect();
        constraints.push(LinearConstraint::le(cap_terms, self.capacity() as f64));

        // Precedence constraints: ∀ (a, b): x_b - x_a ≤ 0
        // (if item b is selected, item a must also be selected)
        for &(a, b) in self.precedences() {
            constraints.push(LinearConstraint::le(vec![(b, 1.0), (a, -1.0)], 0.0));
        }

        // Objective: Maximize Σ v_i·x_i
        let objective: Vec<(usize, f64)> = self
            .values()
            .iter()
            .enumerate()
            .map(|(i, &v)| (i, v as f64))
            .collect();

        let target = ILP::new(n, constraints, objective, ObjectiveSense::Maximize);
        ReductionPOKToILP { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partiallyorderedknapsack_to_ilp",
        build: || {
            // 3 items: weights [2, 3, 1], values [3, 4, 2], capacity 4
            // Precedence: (0, 1) means item 0 must be taken if item 1 is taken
            // Optimal: select items 0 and 2 (weight 3 ≤ 4, value 5)
            let source = PartiallyOrderedKnapsack::new(
                vec![2, 3, 1],
                vec![3, 4, 2],
                vec![(0, 1)],
                4,
            );
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![1, 0, 1],
                    target_config: vec![1, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partiallyorderedknapsack_ilp.rs"]
mod tests;
