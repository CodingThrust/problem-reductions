//! Reduction from GraphPartitioning to MaxCut on a weighted complete graph.

use crate::models::graph::{GraphPartitioning, MaxCut};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing GraphPartitioning to MaxCut.
#[derive(Debug, Clone)]
pub struct ReductionGPToMaxCut {
    target: MaxCut<SimpleGraph, i32>,
}

impl ReductionResult for ReductionGPToMaxCut {
    type Source = GraphPartitioning<SimpleGraph>;
    type Target = MaxCut<SimpleGraph, i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

fn complete_graph_edges_and_weights(graph: &SimpleGraph) -> (Vec<(usize, usize)>, Vec<i32>) {
    let num_vertices = graph.num_vertices();
    let p = graph.num_edges() as i32 + 1;
    let mut edges = Vec::new();
    let mut weights = Vec::new();

    for u in 0..num_vertices {
        for v in (u + 1)..num_vertices {
            edges.push((u, v));
            weights.push(if graph.has_edge(u, v) { p - 1 } else { p });
        }
    }

    (edges, weights)
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices",
        num_edges = "num_vertices * (num_vertices - 1) / 2",
    }
)]
impl ReduceTo<MaxCut<SimpleGraph, i32>> for GraphPartitioning<SimpleGraph> {
    type Result = ReductionGPToMaxCut;

    fn reduce_to(&self) -> Self::Result {
        let (edges, weights) = complete_graph_edges_and_weights(self.graph());
        let target = MaxCut::new(SimpleGraph::new(self.num_vertices(), edges), weights);

        ReductionGPToMaxCut { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "graphpartitioning_to_maxcut",
        build: || {
            let source = GraphPartitioning::new(SimpleGraph::new(
                6,
                vec![
                    (0, 1),
                    (0, 2),
                    (1, 2),
                    (1, 3),
                    (2, 3),
                    (2, 4),
                    (3, 4),
                    (3, 5),
                    (4, 5),
                ],
            ));
            crate::example_db::specs::rule_example_with_witness::<_, MaxCut<SimpleGraph, i32>>(
                source,
                SolutionPair {
                    source_config: vec![0, 0, 0, 1, 1, 1],
                    target_config: vec![0, 0, 0, 1, 1, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/graphpartitioning_maxcut.rs"]
mod tests;
