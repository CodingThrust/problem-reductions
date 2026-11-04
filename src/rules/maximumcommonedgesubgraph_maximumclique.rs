//! Reduction from MaximumCommonEdgeSubgraph to MaximumClique via an association graph.
//!
//! Every pair of a source arc `(u, l, v)` and a target arc `(a, l, b)` with the
//! same label and the same loop status becomes a vertex that proposes the
//! assignments `u -> a` and `v -> b`. Two vertices are adjacent exactly when the
//! union of their assignments is a partial injection. A clique therefore
//! describes a partial injective vertex map that preserves at least as many
//! arcs as the clique has vertices, and the maximum clique size equals the
//! optimum of the source problem.

use crate::models::graph::{MaximumClique, MaximumCommonEdgeSubgraph};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult, validate_target_witness};
use crate::rules::{ExtractionResult, ReductionError};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::One;
use std::collections::HashSet;

/// Endpoint assignments `(source vertex, target vertex)` proposed by one candidate.
type Assignments = [(usize, usize); 2];

/// Result of reducing MaximumCommonEdgeSubgraph to MaximumClique.
///
/// Target vertex `k` is the `k`-th retained arc pair, ordered by source arc
/// index and then by target arc index.
#[derive(Debug, Clone)]
pub struct ReductionMCESToClique {
    target: MaximumClique<SimpleGraph, One>,
    num_vertices_1: usize,
    num_vertices_2: usize,
    assignments: Vec<Assignments>,
}

impl ReductionResult for ReductionMCESToClique {
    type Source = MaximumCommonEdgeSubgraph;
    type Target = MaximumClique<SimpleGraph, One>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Start with every source vertex unmatched (sentinel `|V2|`), then write
    /// the endpoint assignments of each selected candidate. Pairwise adjacency
    /// guarantees that the writes agree.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as Problem>::Solution,
    ) -> ExtractionResult<<Self::Source as Problem>::Solution> {
        validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.0.is_some(),
            "selected vertices are not a clique",
        )?;

        let mut mapping = vec![self.num_vertices_2; self.num_vertices_1];
        for (selected, assignment) in target_solution.iter().zip(&self.assignments) {
            if *selected {
                for &(source_vertex, target_vertex) in assignment {
                    mapping[source_vertex] = target_vertex;
                }
            }
        }
        Ok(mapping)
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::traits::AggregateReductionResult for ReductionMCESToClique {}

/// Two candidates are compatible when equal source vertices carry equal target
/// vertices and different source vertices carry different target vertices.
fn compatible(left: &Assignments, right: &Assignments) -> bool {
    left.iter()
        .all(|&(s, t)| right.iter().all(|&(s2, t2)| (s == s2) == (t == t2)))
}

// At most one vertex per (source arc, target arc) pair, and at most one edge per
// vertex pair; the label and loop filters only remove vertices and edges.
#[reduction(transform = upper_bound {
    num_vertices = "num_arcs_1 * num_arcs_2",
    num_edges = "num_arcs_1 * num_arcs_2 * (num_arcs_1 * num_arcs_2 - 1) / 2",
})]
impl ReduceTo<MaximumClique<SimpleGraph, One>> for MaximumCommonEdgeSubgraph {
    type Result = ReductionMCESToClique;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Target = MaximumClique<SimpleGraph, One>;
        let invalid = |message: String| ReductionError::invalid_target::<Self, Target>(message);
        let (n1, n2) = (self.num_vertices_1(), self.num_vertices_2());
        for (name, graph, bound) in [
            ("first", self.graph_1(), n1),
            ("second", self.graph_2(), n2),
        ] {
            let mut seen = HashSet::new();
            for arc in graph.arcs() {
                if arc.src >= bound || arc.dst >= bound {
                    return Err(invalid(format!(
                        "an arc of the {name} graph has an endpoint outside 0..{bound}"
                    )));
                }
                if !seen.insert(*arc) {
                    return Err(invalid(format!("the {name} graph repeats an arc")));
                }
            }
        }

        let mut assignments: Vec<Assignments> = Vec::new();
        for source_arc in self.graph_1().arcs() {
            for target_arc in self.graph_2().arcs() {
                let same_loop_status =
                    (source_arc.src == source_arc.dst) == (target_arc.src == target_arc.dst);
                if source_arc.label == target_arc.label && same_loop_status {
                    assignments.push([
                        (source_arc.src, target_arc.src),
                        (source_arc.dst, target_arc.dst),
                    ]);
                }
            }
        }

        let mut edges = Vec::new();
        for (p, left) in assignments.iter().enumerate() {
            for (offset, right) in assignments[p + 1..].iter().enumerate() {
                if compatible(left, right) {
                    edges.push((p, p + 1 + offset));
                }
            }
        }

        let graph = SimpleGraph::try_new(assignments.len(), edges)
            .map_err(<Self as ReduceTo<Target>>::target_construction)?;
        let weights = vec![One; assignments.len()];
        Ok(ReductionMCESToClique {
            target: MaximumClique::new(graph, weights),
            num_vertices_1: n1,
            num_vertices_2: n2,
            assignments,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::graph::{LabelledArc, LabelledDigraph};

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximumcommonedgesubgraph_to_maximumclique",
        build: || {
            let arcs = |triples: &[(usize, usize, usize)]| {
                triples
                    .iter()
                    .map(|&(u, label, v)| LabelledArc::new(u, label, v))
                    .collect::<Vec<_>>()
            };
            let source = MaximumCommonEdgeSubgraph::new(
                LabelledDigraph::new(4, arcs(&[(0, 0, 1), (1, 1, 2), (2, 2, 3), (3, 3, 0)])),
                LabelledDigraph::new(
                    5,
                    arcs(&[(0, 0, 1), (1, 1, 3), (3, 2, 4), (4, 3, 0), (2, 0, 3)]),
                ),
            );
            crate::example_db::specs::rule_example_with_witness::<_, MaximumClique<SimpleGraph, One>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!([0, 1, 3, 4]),
                    target_config: serde_json::json!([true, false, true, true, true]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumcommonedgesubgraph_maximumclique.rs"]
mod tests;
