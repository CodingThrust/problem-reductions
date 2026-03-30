//! Reduction from MinimumVertexCover (unit-weight) to MinimumHittingSet.
//!
//! Each edge becomes a 2-element subset and vertices become universe elements.
//! A vertex cover of G is exactly a hitting set for the edge-subset collection.

use crate::models::graph::MinimumVertexCover;
use crate::models::set::MinimumHittingSet;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::One;

/// Result of reducing MinimumVertexCover<SimpleGraph, One> to MinimumHittingSet.
#[derive(Debug, Clone)]
pub struct ReductionVCToHS {
    target: MinimumHittingSet,
}

impl ReductionResult for ReductionVCToHS {
    type Source = MinimumVertexCover<SimpleGraph, One>;
    type Target = MinimumHittingSet;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: variables correspond 1:1.
    /// Element i in the hitting set corresponds to vertex i in the vertex cover.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        universe_size = "num_vertices",
        num_sets = "num_edges",
    }
)]
impl ReduceTo<MinimumHittingSet> for MinimumVertexCover<SimpleGraph, One> {
    type Result = ReductionVCToHS;

    fn reduce_to(&self) -> Self::Result {
        let edges = self.graph().edges();
        let num_vertices = self.graph().num_vertices();

        // For each edge (u, v), create a 2-element subset {u, v}.
        let sets: Vec<Vec<usize>> = edges.iter().map(|&(u, v)| vec![u, v]).collect();

        let target = MinimumHittingSet::new(num_vertices, sets);

        ReductionVCToHS { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumvertexcover_to_minimumhittingset",
        build: || {
            // 6-vertex graph from the issue example
            let source = MinimumVertexCover::new(
                SimpleGraph::new(
                    6,
                    vec![
                        (0, 1),
                        (0, 2),
                        (1, 3),
                        (2, 3),
                        (2, 4),
                        (3, 5),
                        (4, 5),
                        (1, 4),
                    ],
                ),
                vec![One; 6],
            );
            crate::example_db::specs::rule_example_with_witness::<_, MinimumHittingSet>(
                source,
                SolutionPair {
                    source_config: vec![1, 0, 0, 1, 1, 0],
                    target_config: vec![1, 0, 0, 1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_minimumhittingset.rs"]
mod tests;
