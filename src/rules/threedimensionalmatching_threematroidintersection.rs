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
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
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

#[cfg(test)]
#[path = "../unit_tests/rules/threedimensionalmatching_threematroidintersection.rs"]
mod tests;
