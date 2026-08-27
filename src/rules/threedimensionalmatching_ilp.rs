//! Reduction from ThreeDimensionalMatching to `ILP<bool>`.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::set::ThreeDimensionalMatching;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionThreeDimensionalMatchingToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionThreeDimensionalMatchingToILP {
    type Source = ThreeDimensionalMatching;
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
        num_vars = "num_triples",
        num_constraints = "3 * universe_size",
    },
)]
impl ReduceTo<ILP<bool>> for ThreeDimensionalMatching {
    type Result = ReductionThreeDimensionalMatchingToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_vars = self.num_triples();
        let mut w_constraints = vec![Vec::new(); self.universe_size()];
        let mut x_constraints = vec![Vec::new(); self.universe_size()];
        let mut y_constraints = vec![Vec::new(); self.universe_size()];

        for (triple_index, &(w, x, y)) in self.triples().iter().enumerate() {
            w_constraints[w].push((triple_index, 1));
            x_constraints[x].push((triple_index, 1));
            y_constraints[y].push((triple_index, 1));
        }

        let mut constraints = Vec::with_capacity(3 * self.universe_size());
        constraints.extend(
            w_constraints
                .into_iter()
                .map(|terms| LinearConstraint::eq(terms, 1)),
        );
        constraints.extend(
            x_constraints
                .into_iter()
                .map(|terms| LinearConstraint::eq(terms, 1)),
        );
        constraints.extend(
            y_constraints
                .into_iter()
                .map(|terms| LinearConstraint::eq(terms, 1)),
        );

        Ok(ReductionThreeDimensionalMatchingToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize)
                .map_err(<Self as ReduceTo<ILP<bool>>>::target_construction)?,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "threedimensionalmatching_to_ilp",
        build: || {
            let source = ThreeDimensionalMatching::new(
                3,
                vec![(0, 1, 2), (1, 0, 1), (2, 2, 0), (0, 0, 0), (1, 2, 2)],
            );
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![true, true, true, false, false]),
                    target_config: serde_json::json!(vec![1, 1, 1, 0, 0]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/threedimensionalmatching_ilp.rs"]
mod tests;
