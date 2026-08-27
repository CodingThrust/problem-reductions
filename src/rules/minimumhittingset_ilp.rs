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
        num_vars = "universe_size",
        num_constraints = "num_sets",
    },
)]
impl ReduceTo<ILP<bool>> for MinimumHittingSet {
    type Result = ReductionHSToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_vars = self.universe_size();
        let constraints: Vec<LinearConstraint> = self
            .sets()
            .iter()
            .map(|set| {
                let terms: Vec<(usize, i64)> = set.iter().map(|&e| (e, 1)).collect();
                LinearConstraint::ge(terms, 1)
            })
            .collect();
        let objective: Vec<(usize, f64)> = (0..num_vars).map(|i| (i, 1.0)).collect();
        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        Ok(ReductionHSToILP { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumhittingset_to_ilp",
        build: || {
            let source = MinimumHittingSet::new(4, vec![vec![0, 1], vec![2, 3], vec![1, 2]]);
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumhittingset_ilp.rs"]
mod tests;
