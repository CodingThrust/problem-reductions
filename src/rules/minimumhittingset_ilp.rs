//! Reduction from MinimumHittingSet to ILP (Integer Linear Programming).
//!
//! Binary variable x_e per universe element; for each set S,
//! require Σ_{e∈S} x_e ≥ 1 (set is hit). Minimize Σ x_e.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::set::MinimumHittingSet;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionHSToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionHSToILP {
    type Source = MinimumHittingSet;
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
        num_constraints = "num_sets",
    }
)]
impl ReduceTo<ILP<bool>> for MinimumHittingSet {
    type Result = ReductionHSToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.universe_size();
        let constraints: Vec<LinearConstraint> = self
            .sets()
            .iter()
            .map(|set| {
                let terms: Vec<(usize, f64)> = set.iter().map(|&e| (e, 1.0)).collect();
                LinearConstraint::ge(terms, 1.0)
            })
            .collect();
        let objective: Vec<(usize, f64)> = (0..num_vars).map(|i| (i, 1.0)).collect();
        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);
        ReductionHSToILP { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumhittingset_to_ilp",
        build: || {
            let source = MinimumHittingSet::new(4, vec![vec![0, 1], vec![2, 3], vec![1, 2]]);
            let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
            let ilp_solution = crate::solvers::ILPSolver::new()
                .solve(reduction.target_problem())
                .expect("canonical example must be solvable");
            let source_config = reduction.extract_solution(&ilp_solution);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config,
                    target_config: ilp_solution,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumhittingset_ilp.rs"]
mod tests;
