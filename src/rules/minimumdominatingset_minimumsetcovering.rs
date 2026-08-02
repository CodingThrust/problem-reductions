//! Reduction from MinimumDominatingSet to MinimumSetCovering.
//!
//! Each vertex becomes the set containing its closed neighborhood.

use crate::models::graph::MinimumDominatingSet;
use crate::models::set::MinimumSetCovering;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MinimumDominatingSet to MinimumSetCovering.
#[derive(Debug, Clone)]
pub struct ReductionDominatingSetToSetCovering {
    target: MinimumSetCovering<i32>,
}

impl ReductionResult for ReductionDominatingSetToSetCovering {
    type Source = MinimumDominatingSet<SimpleGraph, i32>;
    type Target = MinimumSetCovering<i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        universe_size = "num_vertices",
        num_sets = "num_vertices",
    }
)]
impl ReduceTo<MinimumSetCovering<i32>> for MinimumDominatingSet<SimpleGraph, i32> {
    type Result = ReductionDominatingSetToSetCovering;

    fn reduce_to(&self) -> Self::Result {
        let sets = (0..self.graph().num_vertices())
            .map(|vertex| {
                let mut closed_neighborhood: Vec<_> =
                    self.closed_neighborhood(vertex).into_iter().collect();
                closed_neighborhood.sort_unstable();
                closed_neighborhood
            })
            .collect();
        let target = MinimumSetCovering::with_weights(
            self.graph().num_vertices(),
            sets,
            self.weights().to_vec(),
        );

        ReductionDominatingSetToSetCovering { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumdominatingset_to_minimumsetcovering",
        build: || {
            let source = MinimumDominatingSet::new(SimpleGraph::path(5), vec![3, 1, 4, 1, 3]);
            crate::example_db::specs::rule_example_with_witness::<_, MinimumSetCovering<i32>>(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 0, 1, 0],
                    target_config: vec![0, 1, 0, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumdominatingset_minimumsetcovering.rs"]
mod tests;
