//! Reduction from MinimumVertexCover to MinimumWeightAndOrGraph.

use crate::models::graph::MinimumVertexCover;
use crate::models::misc::MinimumWeightAndOrGraph;
use crate::reduction;
use crate::rules::traits::{recover_preserving_status, ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;
use crate::topology::Graph;
use crate::topology::SimpleGraph;

/// Result of reducing MinimumVertexCover to MinimumWeightAndOrGraph.
#[derive(Debug, Clone)]
pub struct ReductionVCToAndOrGraph {
    target: MinimumWeightAndOrGraph,
    sink_arc_start: usize,
    forced_cover: Vec<bool>,
}

impl ReductionResult for ReductionVCToAndOrGraph {
    type Source = MinimumVertexCover<SimpleGraph, i64>;
    type Target = MinimumWeightAndOrGraph;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        recover_preserving_status(source, target, |solution| {
            Ok(self
                .forced_cover
                .iter()
                .enumerate()
                .map(|(j, &forced)| forced || solution[self.sink_arc_start + j])
                .collect())
        })
    }
}

#[reduction(
    transform = upper_bound {
        num_vertices = "1 + num_edges + 2 * num_vertices",
        num_arcs = "3 * num_edges + num_vertices",
    }
)]
impl ReduceTo<MinimumWeightAndOrGraph> for MinimumVertexCover<SimpleGraph, i64> {
    type Result = ReductionVCToAndOrGraph;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.graph().num_vertices();
        // Adding a negative-weight vertex preserves coverage and strictly
        // lowers cost, so every optimum contains all such vertices.
        let forced_cover: Vec<_> = self.weights().iter().map(|&w| w < 0).collect();
        let edges: Vec<_> = self
            .graph()
            .edges()
            .into_iter()
            .filter(|&(u, v)| !forced_cover[u] && !forced_cover[v])
            .collect();
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
            arc_weights.push(weight.max(0));
        }

        let target =
            MinimumWeightAndOrGraph::new(num_target_vertices, arcs, 0, gate_types, arc_weights);

        Ok(ReductionVCToAndOrGraph {
            target,
            sink_arc_start,
            forced_cover,
        })
    }
}

#[cfg(any(test, feature = "example-db"))]
fn issue_example_source() -> MinimumVertexCover<SimpleGraph, i64> {
    MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i64; 3])
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
                    source_config: serde_json::json!(vec![false, true, false]),
                    target_config: serde_json::json!(vec![
                        true, true, false, true, true, false, false, true, false
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_minimumweightandorgraph.rs"]
mod tests;
