//! Reduction from MinimumVertexCover to MinimumWeightAndOrGraph.

use crate::models::graph::MinimumVertexCover;
use crate::models::misc::MinimumWeightAndOrGraph;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::Graph;
use crate::topology::SimpleGraph;

/// Result of reducing MinimumVertexCover to MinimumWeightAndOrGraph.
#[derive(Debug, Clone)]
pub struct ReductionVCToAndOrGraph {
    target: MinimumWeightAndOrGraph,
    sink_arc_start: usize,
    num_source_vertices: usize,
}

impl ReductionResult for ReductionVCToAndOrGraph {
    type Source = MinimumVertexCover<SimpleGraph, i32>;
    type Target = MinimumWeightAndOrGraph;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        (0..self.num_source_vertices)
            .map(|j| usize::from(target_solution.get(self.sink_arc_start + j) == Some(&1)))
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vertices = "1 + num_edges + 2 * num_vertices",
        num_arcs = "3 * num_edges + num_vertices",
    }
)]
impl ReduceTo<MinimumWeightAndOrGraph> for MinimumVertexCover<SimpleGraph, i32> {
    type Result = ReductionVCToAndOrGraph;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();
        let edges = self.graph().edges();
        let m = edges.len();

        let num_target_vertices = 1 + m + (2 * n);
        let mut gate_types = vec![None; num_target_vertices];
        gate_types[0] = Some(true);
        for gate in gate_types.iter_mut().skip(1).take(m + n) {
            *gate = Some(false);
        }

        let edge_vertex = |i: usize| 1 + i;
        let cover_vertex = |j: usize| 1 + m + j;
        let sink_vertex = |j: usize| 1 + m + n + j;

        let mut arcs = Vec::with_capacity((3 * m) + n);
        let mut arc_weights = Vec::with_capacity((3 * m) + n);

        for i in 0..m {
            arcs.push((0, edge_vertex(i)));
            arc_weights.push(1);
        }

        for (i, &(u, v)) in edges.iter().enumerate() {
            arcs.push((edge_vertex(i), cover_vertex(u)));
            arc_weights.push(1);
            arcs.push((edge_vertex(i), cover_vertex(v)));
            arc_weights.push(1);
        }

        let sink_arc_start = arcs.len();
        for (j, &weight) in self.weights().iter().enumerate() {
            arcs.push((cover_vertex(j), sink_vertex(j)));
            arc_weights.push(weight);
        }

        let target =
            MinimumWeightAndOrGraph::new(num_target_vertices, arcs, 0, gate_types, arc_weights);

        ReductionVCToAndOrGraph {
            target,
            sink_arc_start,
            num_source_vertices: n,
        }
    }
}

#[cfg(any(test, feature = "example-db"))]
fn issue_example_source() -> MinimumVertexCover<SimpleGraph, i32> {
    MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3])
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumvertexcover_to_minimumweightandorgraph",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, MinimumWeightAndOrGraph>(
                issue_example_source(),
                SolutionPair {
                    source_config: vec![0, 1, 0],
                    target_config: vec![1, 1, 0, 1, 1, 0, 0, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_minimumweightandorgraph.rs"]
mod tests;
