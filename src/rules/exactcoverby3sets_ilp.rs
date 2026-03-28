//! Reduction from ExactCoverBy3Sets to ILP (Integer Linear Programming).
//!
//! Binary variable x_j per triple; for each element e, require Σ x_j = 1
//! (exact cover). Additional constraint Σ x_j = universe_size/3.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::set::ExactCoverBy3Sets;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionX3CToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionX3CToILP {
    type Source = ExactCoverBy3Sets;
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
        num_vars = "num_subsets",
        num_constraints = "universe_size + 1",
    }
)]
impl ReduceTo<ILP<bool>> for ExactCoverBy3Sets {
    type Result = ReductionX3CToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_subsets();
        let mut constraints = Vec::new();

        // For each element e: Σ_{j: e ∈ triple_j} x_j = 1
        for element in 0..self.universe_size() {
            let terms: Vec<(usize, f64)> = self
                .subsets()
                .iter()
                .enumerate()
                .filter(|(_, subset)| subset.contains(&element))
                .map(|(j, _)| (j, 1.0))
                .collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Σ x_j = universe_size / 3
        let cardinality_terms: Vec<(usize, f64)> = (0..num_vars).map(|j| (j, 1.0)).collect();
        constraints.push(LinearConstraint::eq(
            cardinality_terms,
            (self.universe_size() / 3) as f64,
        ));

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);
        ReductionX3CToILP { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "exactcoverby3sets_to_ilp",
        build: || {
            let source =
                ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4], [1, 2, 5]]);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![1, 1, 0, 0],
                    target_config: vec![1, 1, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/exactcoverby3sets_ilp.rs"]
mod tests;
