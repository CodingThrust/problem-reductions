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
    /// Each color class of size exactly 2 in `complement(G)` is an independent
    /// set there and thus a clique in `G`. For bipartite `G` such a clique has
    /// size 2, i.e., a source edge. A source edge `(u, v)` belongs to the
    /// extracted matching iff `u` and `v` share a color, which we detect in a
    /// single pass over `source_edges`.
    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            self.source_edges
                .iter()
                .map(|&(u, v)| usize::from(target_solution[u] == target_solution[v]))
                .collect()
        })
    }
}

#[reduction(
    exact = {
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
            // "T" tree (spider with three legs at the centre v1):
            //   v0 -- v1 -- v2 -- v3
            //         |
            //         v4
            //
            // Five vertices and four edges. Bipartite with
            // A = {v0, v2, v4} and B = {v1, v3}.
            //
            // BipartiteGraph encoding (left_size = 3, right_size = 2):
            //   left  local 0 -> v0, local 1 -> v2, local 2 -> v4
            //   right local 0 -> v1, local 1 -> v3
            //   edges (left_idx, right_idx):
            //     (v0, v1) -> (0, 0)
            //     (v1, v2) -> (1, 0)
            //     (v2, v3) -> (1, 1)
            //     (v1, v4) -> (2, 0)
            //
            // Unified vertex labels:
            //   0 = v0 (left 0), 1 = v2 (left 1), 2 = v4 (left 2),
            //   3 = v1 (right 0), 4 = v3 (right 1).
            //
            // Unified edges from Graph::edges() (in source order):
            //   (0, 3), (1, 3), (1, 4), (2, 3).
            //
            // Maximal matchings of G:
            //   - {(v1, v2)}                 size 1   <-- mm(G) = 1
            //   - {(v0, v1), (v2, v3)}       size 2   (suboptimal 1)
            //   - {(v1, v4), (v2, v3)}       size 2   (suboptimal 2)
            // So the canonical example exhibits >=2 suboptimal maximal
            // matchings besides the optimum.
            //
            // Canonical optimum: pick the central edge (v1, v2), which is
            // source edge index 1. source_config = [0, 1, 0, 0].
            //
            // complement(G) on K_5: 10 - 4 = 6 edges, namely
            //   (0, 1), (0, 2), (0, 4), (1, 2), (2, 4), (3, 4).
            //
            // Canonical achromatic 4-coloring of complement(G):
            //   v0 (idx 0) -> color 1
            //   v2 (idx 1) -> color 0  (paired with v1 = G-edge (v1, v2))
            //   v4 (idx 2) -> color 3
            //   v1 (idx 3) -> color 0
            //   v3 (idx 4) -> color 2
            // target_config = [1, 0, 3, 0, 2]; psi(H) = |V| - mm(G) = 4.
            let source = MinimumMaximalMatching::new(BipartiteGraph::new(
                3,
                2,
                vec![(0, 0), (1, 0), (1, 1), (2, 0)],
            ));
            crate::example_db::specs::rule_example_with_witness::<
                _,
                MaximumAchromaticNumber<SimpleGraph>,
            >(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 0, 0],
                    target_config: vec![1, 0, 3, 0, 2],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimummaximalmatching_maximumachromaticnumber.rs"]
mod tests;
