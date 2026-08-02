//! Reduction from unit-weight MinimumDominatingSet to MinimumHittingSet.
//!
//! Vertices become universe elements, and each vertex contributes its closed
//! neighborhood as a set. A dominating set is exactly a hitting set for this
//! collection.

use crate::models::graph::MinimumDominatingSet;
use crate::models::set::MinimumHittingSet;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::One;

/// Result of reducing MinimumDominatingSet<SimpleGraph, One> to MinimumHittingSet.
#[derive(Debug, Clone)]
pub struct ReductionDominatingSetToHittingSet {
    target: MinimumHittingSet,
}

impl ReductionResult for ReductionDominatingSetToHittingSet {
    type Source = MinimumDominatingSet<SimpleGraph, One>;
    type Target = MinimumHittingSet;

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
impl ReduceTo<MinimumHittingSet> for MinimumDominatingSet<SimpleGraph, One> {
    type Result = ReductionDominatingSetToHittingSet;

    fn reduce_to(&self) -> Self::Result {
        let num_vertices = self.graph().num_vertices();
        let sets = (0..num_vertices)
            .map(|vertex| {
                let mut closed_neighborhood: Vec<_> =
                    self.closed_neighborhood(vertex).into_iter().collect();
                closed_neighborhood.sort_unstable();
                closed_neighborhood
            })
            .collect();

        ReductionDominatingSetToHittingSet {
            target: MinimumHittingSet::new(num_vertices, sets),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumdominatingset_to_minimumhittingset",
        build: || {
            let source = MinimumDominatingSet::new(
                SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
                vec![One; 5],
            );
            crate::example_db::specs::rule_example_with_witness::<_, MinimumHittingSet>(
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
#[path = "../unit_tests/rules/minimumdominatingset_minimumhittingset.rs"]
mod tests;
