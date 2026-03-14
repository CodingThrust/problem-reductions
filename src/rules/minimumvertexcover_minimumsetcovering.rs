//! Reduction from MinimumVertexCover to MinimumSetCovering.
//!
//! Each vertex becomes a set containing the edges it covers.
//! The universe is the set of all edges (labeled 0 to num_edges-1).

use crate::models::graph::MinimumVertexCover;
use crate::models::set::MinimumSetCovering;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::WeightElement;

/// Result of reducing MinimumVertexCover to MinimumSetCovering.
#[derive(Debug, Clone)]
pub struct ReductionVCToSC<W> {
    target: MinimumSetCovering<W>,
}

impl<W> ReductionResult for ReductionVCToSC<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    type Source = MinimumVertexCover<SimpleGraph, W>;
    type Target = MinimumSetCovering<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: variables correspond 1:1.
    /// Vertex i in VC corresponds to set i in SC.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    id = "minimumvertexcover_to_minimumsetcovering_simplegraph_i32",
    overhead = {
        num_sets = "num_vertices",
        universe_size = "num_edges",
    }
)]
impl ReduceTo<MinimumSetCovering<i32>> for MinimumVertexCover<SimpleGraph, i32> {
    type Result = ReductionVCToSC<i32>;

    fn reduce_to(&self) -> Self::Result {
        let edges = self.graph().edges();
        let num_edges = edges.len();
        let num_vertices = self.graph().num_vertices();

        // For each vertex, create a set of edge indices that it covers.
        // An edge (u, v) with index i is covered by vertex j if j == u or j == v.
        let sets: Vec<Vec<usize>> = (0..num_vertices)
            .map(|vertex| {
                edges
                    .iter()
                    .enumerate()
                    .filter(|(_, (u, v))| *u == vertex || *v == vertex)
                    .map(|(edge_idx, _)| edge_idx)
                    .collect()
            })
            .collect();

        let target = MinimumSetCovering::with_weights(num_edges, sets, self.weights().to_vec());

        ReductionVCToSC { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumvertexcover_to_minimumsetcovering",
        build: || {
            let (n, edges) = crate::topology::small_graphs::petersen();
            let source = MinimumVertexCover::new(SimpleGraph::new(n, edges), vec![1i32; 10]);
            crate::example_db::specs::direct_best_example::<_, MinimumSetCovering<i32>, _>(
                source,
                |_, _| true,
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_minimumsetcovering.rs"]
mod tests;
