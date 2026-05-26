//! Reduction from MinimumMaximalMatching (on a bipartite graph) to
//! MaximumAchromaticNumber.
//!
//! Classical reduction of Yannakakis and Gavril (1980) establishing
//! NP-completeness of Achromatic Number (G&J GT5). For a bipartite graph `G`,
//! the identity `ach(complement(G)) = |V| - mm(G)` holds, where `mm(G)` is the
//! minimum maximal matching size of `G`. The decision-version correspondence
//! used in the reduction is `(G, K) -> (complement(G), |V| - K)`.

use crate::models::graph::{MaximumAchromaticNumber, MinimumMaximalMatching};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{BipartiteGraph, Graph, SimpleGraph};
use std::collections::HashMap;

/// Result of reducing `MinimumMaximalMatching<BipartiteGraph>` to
/// `MaximumAchromaticNumber<SimpleGraph>`.
///
/// Stores the target problem along with the source edge list (in unified vertex
/// coordinates) so that `extract_solution` can map a target coloring back to a
/// maximal matching of the source graph.
#[derive(Debug, Clone)]
pub struct ReductionMMMToAchromatic {
    target: MaximumAchromaticNumber<SimpleGraph>,
    /// Source edges in unified vertex coordinates, in the same order as
    /// `source.graph().edges()` (which determines the source `dims()`).
    source_edges: Vec<(usize, usize)>,
}

impl ReductionResult for ReductionMMMToAchromatic {
    type Source = MinimumMaximalMatching<BipartiteGraph>;
    type Target = MaximumAchromaticNumber<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a maximal matching of the source graph from an achromatic
    /// coloring of `complement(G)`.
    ///
    /// Each color class of size exactly 2 corresponds to a clique-in-G of
    /// size 2, i.e., a single source edge. Marking those edges yields the
    /// maximal matching `M` with `|M| = |V| - k`, where `k` is the number of
    /// colors used.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let num_source_edges = self.source_edges.len();
        let mut source_config = vec![0usize; num_source_edges];

        // Group vertices by color.
        let mut color_to_vertices: HashMap<usize, Vec<usize>> = HashMap::new();
        for (vertex, &color) in target_solution.iter().enumerate() {
            color_to_vertices.entry(color).or_default().push(vertex);
        }

        // Build an edge lookup keyed by canonical (min, max) pairs.
        let mut edge_index: HashMap<(usize, usize), usize> = HashMap::new();
        for (idx, &(u, v)) in self.source_edges.iter().enumerate() {
            let key = if u < v { (u, v) } else { (v, u) };
            edge_index.insert(key, idx);
        }

        // Color classes of size 2 must be edges of G (cliques in G of size 2).
        for vertices in color_to_vertices.values() {
            if vertices.len() == 2 {
                let (a, b) = (vertices[0], vertices[1]);
                let key = if a < b { (a, b) } else { (b, a) };
                if let Some(&idx) = edge_index.get(&key) {
                    source_config[idx] = 1;
                }
            }
        }

        source_config
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices",
        num_edges = "num_vertices * (num_vertices - 1) / 2 - num_edges",
    }
)]
impl ReduceTo<MaximumAchromaticNumber<SimpleGraph>> for MinimumMaximalMatching<BipartiteGraph> {
    type Result = ReductionMMMToAchromatic;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();
        let source_edges = self.graph().edges();

        // Build adjacency lookup over unified coordinates.
        let source_edge_set: std::collections::HashSet<(usize, usize)> = source_edges
            .iter()
            .map(|&(u, v)| if u < v { (u, v) } else { (v, u) })
            .collect();

        // Complement graph: all non-edges of G become edges of H.
        let mut complement_edges = Vec::new();
        for u in 0..n {
            for v in (u + 1)..n {
                if !source_edge_set.contains(&(u, v)) {
                    complement_edges.push((u, v));
                }
            }
        }

        let target = MaximumAchromaticNumber::new(SimpleGraph::new(n, complement_edges));

        ReductionMMMToAchromatic {
            target,
            source_edges,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimummaximalmatching_to_maximumachromaticnumber",
        build: || {
            // Path P4 as a bipartite graph: A = {v0, v2}, B = {v1, v3}.
            //
            // BipartiteGraph encoding (left_size = 2, right_size = 2):
            //   left  local 0 -> v0, local 1 -> v2
            //   right local 0 -> v1, local 1 -> v3
            //   edges (left_idx, right_idx):
            //     (v0, v1) -> (0, 0)
            //     (v1, v2) -> (1, 0)   (v2 is left=1, v1 is right=0)
            //     (v2, v3) -> (1, 1)
            //
            // Unified vertex labels:
            //   0 = v0 (left 0)
            //   1 = v2 (left 1)
            //   2 = v1 (right 0)
            //   3 = v3 (right 1)
            //
            // Unified edges from Graph::edges():
            //   (0, 2), (1, 2), (1, 3)
            //
            // mm(G) = 1, achieved by selecting the middle edge (v1, v2),
            // which is unified edge index 1 (i.e., (1, 2)).
            // So source_config = [0, 1, 0].
            //
            // complement(G) edges: (0, 1), (0, 3), (2, 3).
            //
            // Achromatic 3-coloring of complement(G):
            //   v0 (idx 0)  -> color 0
            //   v2 (idx 1)  -> color 1
            //   v1 (idx 2)  -> color 1   (paired with v2 = G-edge (v1, v2))
            //   v3 (idx 3)  -> color 2
            // target_config = [0, 1, 1, 2].
            let source = MinimumMaximalMatching::new(BipartiteGraph::new(
                2,
                2,
                vec![(0, 0), (1, 0), (1, 1)],
            ));
            crate::example_db::specs::rule_example_with_witness::<
                _,
                MaximumAchromaticNumber<SimpleGraph>,
            >(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 0],
                    target_config: vec![0, 1, 1, 2],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimummaximalmatching_maximumachromaticnumber.rs"]
mod tests;
