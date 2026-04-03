//! Reduction from ExactCoverBy3Sets to MinimumWeightSolutionToLinearEquations.
//!
//! The incidence matrix has one row per universe element and one column per set.
//! Selecting a set corresponds to setting its variable to 1. The equation
//! system enforces that each universe element is covered exactly once, and the
//! sparsity bound enforces that at most `|U|/3` sets are selected.

use crate::models::algebraic::MinimumWeightSolutionToLinearEquations;
use crate::models::set::ExactCoverBy3Sets;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionX3CToMinimumWeightSolutionToLinearEquations {
    target: MinimumWeightSolutionToLinearEquations,
}

impl ReductionResult for ReductionX3CToMinimumWeightSolutionToLinearEquations {
    type Source = ExactCoverBy3Sets;
    type Target = MinimumWeightSolutionToLinearEquations;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = {
    num_variables = "num_sets",
    num_equations = "universe_size",
})]
impl ReduceTo<MinimumWeightSolutionToLinearEquations> for ExactCoverBy3Sets {
    type Result = ReductionX3CToMinimumWeightSolutionToLinearEquations;

    fn reduce_to(&self) -> Self::Result {
        let mut coefficients = vec![vec![0i64; self.num_sets()]; self.universe_size()];
        for (set_index, set) in self.sets().iter().enumerate() {
            for &element in set {
                coefficients[element][set_index] = 1;
            }
        }

        ReductionX3CToMinimumWeightSolutionToLinearEquations {
            target: MinimumWeightSolutionToLinearEquations::new(
                coefficients,
                vec![1; self.universe_size()],
                self.universe_size() / 3,
            ),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "exactcoverby3sets_to_minimumweightsolutiontolinearequations",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<
                _,
                MinimumWeightSolutionToLinearEquations,
            >(
                ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]),
                SolutionPair {
                    source_config: vec![1, 1, 0],
                    target_config: vec![1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/exactcoverby3sets_minimumweightsolutiontolinearequations.rs"]
mod tests;
