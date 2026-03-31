//! Reduction from ExactCoverBy3Sets to MaximumSetPacking.
//!
//! Given an X3C instance with universe X (|X| = 3q) and collection C of
//! 3-element subsets, construct a MaximumSetPacking<One> instance where each
//! triple becomes a variable-length set with unit weight. An exact cover
//! of q disjoint triples corresponds to a maximum packing of value q.

use crate::models::set::{ExactCoverBy3Sets, MaximumSetPacking};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::One;

/// Result of reducing ExactCoverBy3Sets to MaximumSetPacking<One>.
#[derive(Debug, Clone)]
pub struct ReductionXC3SToMaximumSetPacking {
    target: MaximumSetPacking<One>,
}

impl ReductionResult for ReductionXC3SToMaximumSetPacking {
    type Source = ExactCoverBy3Sets;
    type Target = MaximumSetPacking<One>;

    fn target_problem(&self) -> &MaximumSetPacking<One> {
        &self.target
    }

    /// Extract X3C solution from MaximumSetPacking solution.
    ///
    /// The configuration is identity (same binary selection vector).
    /// A packing of q disjoint 3-sets over a 3q-element universe is necessarily
    /// an exact cover, so no additional checking is needed.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = {
    num_sets = "num_subsets",
})]
impl ReduceTo<MaximumSetPacking<One>> for ExactCoverBy3Sets {
    type Result = ReductionXC3SToMaximumSetPacking;

    fn reduce_to(&self) -> Self::Result {
        let sets: Vec<Vec<usize>> = self
            .subsets()
            .iter()
            .map(|triple| triple.to_vec())
            .collect();

        ReductionXC3SToMaximumSetPacking {
            target: MaximumSetPacking::<One>::new(sets),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "exactcoverby3sets_to_maximumsetpacking",
        build: || {
            // Universe {0,1,2,3,4,5}, subsets [{0,1,2}, {0,1,3}, {3,4,5}, {2,4,5}, {1,3,5}]
            // Exact cover: S0={0,1,2} + S2={3,4,5}
            let source = ExactCoverBy3Sets::new(
                6,
                vec![[0, 1, 2], [0, 1, 3], [3, 4, 5], [2, 4, 5], [1, 3, 5]],
            );
            crate::example_db::specs::rule_example_with_witness::<_, MaximumSetPacking<One>>(
                source,
                SolutionPair {
                    source_config: vec![1, 0, 1, 0, 0],
                    target_config: vec![1, 0, 1, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/exactcoverby3sets_maximumsetpacking.rs"]
mod tests;
