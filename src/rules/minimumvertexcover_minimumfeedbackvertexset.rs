//! Reduction from MinimumVertexCover to MinimumFeedbackVertexSet.
//!
//! Each undirected edge becomes a directed 2-cycle, so a vertex cover is
//! exactly a feedback vertex set in the constructed digraph.

use crate::models::graph::{MinimumFeedbackVertexSet, MinimumVertexCover};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{DirectedGraph, Graph, SimpleGraph};
use crate::types::WeightElement;

/// Result of reducing MinimumVertexCover to MinimumFeedbackVertexSet.
#[derive(Debug, Clone)]
pub struct ReductionVCToFVS<W> {
    target: MinimumFeedbackVertexSet<W>,
}

impl<W> ReductionResult for ReductionVCToFVS<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    type Source = MinimumVertexCover<SimpleGraph, W>;
    type Target = MinimumFeedbackVertexSet<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices",
        num_arcs = "2 * num_edges",
    }
)]
impl ReduceTo<MinimumFeedbackVertexSet<i32>> for MinimumVertexCover<SimpleGraph, i32> {
    type Result = ReductionVCToFVS<i32>;

    fn reduce_to(&self) -> Self::Result {
        let arcs = self
            .graph()
            .edges()
            .into_iter()
            .flat_map(|(u, v)| [(u, v), (v, u)])
            .collect();

        let target = MinimumFeedbackVertexSet::new(
            DirectedGraph::new(self.graph().num_vertices(), arcs),
            self.weights().to_vec(),
        );

        ReductionVCToFVS { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumvertexcover_to_minimumfeedbackvertexset",
        build: || {
            let source = MinimumVertexCover::new(
                SimpleGraph::new(
                    7,
                    vec![
                        (0, 1),
                        (0, 2),
                        (0, 3),
                        (1, 2),
                        (1, 3),
                        (3, 4),
                        (4, 5),
                        (5, 6),
                    ],
                ),
                vec![1i32; 7],
            );

            crate::example_db::specs::rule_example_with_witness::<_, MinimumFeedbackVertexSet<i32>>(
                source,
                SolutionPair {
                    source_config: vec![1, 1, 0, 1, 0, 1, 0],
                    target_config: vec![1, 1, 0, 1, 0, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_minimumfeedbackvertexset.rs"]
mod tests;
