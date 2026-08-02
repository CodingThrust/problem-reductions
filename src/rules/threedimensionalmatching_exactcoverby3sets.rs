//! Reduction from ThreeDimensionalMatching to ExactCoverBy3Sets.
//!
//! Each coordinate domain is embedded into its own tagged block of a
//! `3q`-element universe. Thus `(w, x, y)` becomes
//! `{w, q + x, 2q + y}`, preserving the source triple index.

use crate::models::set::{ExactCoverBy3Sets, ThreeDimensionalMatching};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing ThreeDimensionalMatching to ExactCoverBy3Sets.
#[derive(Debug, Clone)]
pub struct ReductionThreeDimensionalMatchingToExactCoverBy3Sets {
    target: ExactCoverBy3Sets,
}

impl ReductionResult for ReductionThreeDimensionalMatchingToExactCoverBy3Sets {
    type Source = ThreeDimensionalMatching;
    type Target = ExactCoverBy3Sets;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = {
    universe_size = "3 * universe_size",
    num_subsets = "num_triples",
    num_sets = "num_triples",
})]
impl ReduceTo<ExactCoverBy3Sets> for ThreeDimensionalMatching {
    type Result = ReductionThreeDimensionalMatchingToExactCoverBy3Sets;

    fn reduce_to(&self) -> Self::Result {
        let q = self.universe_size();
        let tagged_subsets = self
            .triples()
            .iter()
            .map(|&(w, x, y)| [w, q + x, 2 * q + y])
            .collect();

        ReductionThreeDimensionalMatchingToExactCoverBy3Sets {
            target: ExactCoverBy3Sets::new(3 * q, tagged_subsets),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "threedimensionalmatching_to_exactcoverby3sets",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, ExactCoverBy3Sets>(
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
#[path = "../unit_tests/rules/threedimensionalmatching_exactcoverby3sets.rs"]
mod tests;
