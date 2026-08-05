//! Reduction from ThreeDimensionalMatching to ThreeMatroidIntersection.

use crate::models::set::{ThreeDimensionalMatching, ThreeMatroidIntersection};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing ThreeDimensionalMatching to ThreeMatroidIntersection.
#[derive(Debug, Clone)]
pub struct ReductionThreeDimensionalMatchingToThreeMatroidIntersection {
    target: ThreeMatroidIntersection,
}

impl ReductionResult for ReductionThreeDimensionalMatchingToThreeMatroidIntersection {
    type Source = ThreeDimensionalMatching;
    type Target = ThreeMatroidIntersection;

    fn target_problem(&self) -> &ThreeMatroidIntersection {
        &self.target
    }

    /// Each target ground-set element is exactly one source triple, so the
    /// witness vector is preserved unchanged.
    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        Ok(target_solution.to_vec())
    }
}

#[reduction(overhead = {
    ground_set_size = "num_triples",
    num_groups = "3 * universe_size",
    bound = "universe_size",
})]
impl ReduceTo<ThreeMatroidIntersection> for ThreeDimensionalMatching {
    type Result = ReductionThreeDimensionalMatchingToThreeMatroidIntersection;

    fn reduce_to(&self) -> Self::Result {
        let mut w_groups = vec![Vec::new(); self.universe_size()];
        let mut x_groups = vec![Vec::new(); self.universe_size()];
        let mut y_groups = vec![Vec::new(); self.universe_size()];

        for (triple_index, &(w, x, y)) in self.triples().iter().enumerate() {
            w_groups[w].push(triple_index);
            x_groups[x].push(triple_index);
            y_groups[y].push(triple_index);
        }

        ReductionThreeDimensionalMatchingToThreeMatroidIntersection {
            target: ThreeMatroidIntersection::new(
                self.num_triples(),
                vec![w_groups, x_groups, y_groups],
                self.universe_size(),
            ),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "threedimensionalmatching_to_threematroidintersection",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, ThreeMatroidIntersection>(
                ThreeDimensionalMatching::new(
                    3,
                    vec![(0, 0, 0), (1, 1, 1), (2, 2, 2), (0, 1, 2), (1, 2, 0)],
                ),
                SolutionPair {
                    source_config: vec![1, 1, 1, 0, 0],
                    target_config: vec![1, 1, 1, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/threedimensionalmatching_threematroidintersection.rs"]
mod tests;
