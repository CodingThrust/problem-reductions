//! Reduction from SetSplitting to ILP (Integer Linear Programming).
//!
//! Binary variable $x_i \in \{0,1\}$ per universe element: 0 means element $i$
//! is placed in part $S_1$, 1 means it is placed in part $S_2$.
//!
//! For each subset $C = \{i_1, \ldots, i_k\}$ we need:
//! - At least one element in $S_2$: $\sum_{j \in C} x_j \geq 1$
//! - At least one element in $S_1$: $\sum_{j \in C} x_j \leq k - 1$
//!
//! Objective: feasibility (minimize 0).

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::set::SetSplitting;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing SetSplitting to ILP.
#[derive(Debug, Clone)]
pub struct ReductionSetSplittingToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionSetSplittingToILP {
    type Source = SetSplitting;
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
        num_vars = "universe_size",
        num_constraints = "2 * num_subsets",
    }
)]
impl ReduceTo<ILP<bool>> for SetSplitting {
    type Result = ReductionSetSplittingToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.universe_size();
        let mut constraints = Vec::new();

        for subset in self.subsets() {
            let k = subset.len();
            let terms: Vec<(usize, f64)> = subset.iter().map(|&e| (e, 1.0)).collect();

            // At least one element in S2: sum >= 1
            constraints.push(LinearConstraint::ge(terms.clone(), 1.0));

            // At least one element in S1: sum <= k - 1
            constraints.push(LinearConstraint::le(terms, (k - 1) as f64));
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);
        ReductionSetSplittingToILP { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "setsplitting_to_ilp",
        build: || {
            let source = SetSplitting::new(
                6,
                vec![vec![0, 1, 2], vec![2, 3, 4], vec![0, 4, 5], vec![1, 3, 5]],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/setsplitting_ilp.rs"]
mod tests;
