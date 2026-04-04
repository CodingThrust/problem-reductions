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

#[cfg(any(test, feature = "example-db"))]
const ISSUE_EXAMPLE_WITNESS: [usize; 6] = [0, 0, 0, 1, 1, 1];

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

#[cfg(any(test, feature = "example-db"))]
fn issue_example() -> GraphPartitioning<SimpleGraph> {
    GraphPartitioning::new(SimpleGraph::new(
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
    ))
}

fn complete_graph_edges_and_weights(graph: &SimpleGraph) -> (Vec<(usize, usize)>, Vec<i32>) {
    let num_vertices = graph.num_vertices();
    let p = penalty_weight(graph.num_edges());
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

fn penalty_weight(num_edges: usize) -> i32 {
    i32::try_from(num_edges)
        .ok()
        .and_then(|num_edges| num_edges.checked_add(1))
        .expect("GraphPartitioning -> MaxCut penalty exceeds i32 range")
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
            crate::example_db::specs::rule_example_with_witness::<_, MaxCut<SimpleGraph, i32>>(
                issue_example(),
                SolutionPair {
                    source_config: ISSUE_EXAMPLE_WITNESS.to_vec(),
                    target_config: ISSUE_EXAMPLE_WITNESS.to_vec(),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/graphpartitioning_maxcut.rs"]
mod tests;
