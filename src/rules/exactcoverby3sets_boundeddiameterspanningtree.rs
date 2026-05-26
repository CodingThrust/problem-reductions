//! Reduction from ExactCoverBy3Sets to BoundedDiameterSpanningTree.
//!
//! Given an X3C instance with universe X (|X| = 3q) and collection
//! C = [S_0, ..., S_{m-1}] of 3-element subsets of X, build a weighted graph
//! and parameters (B, D) so that the BDST instance is feasible iff the source
//! has an exact cover.
//!
//! Construction (Garey & Johnson, ND4):
//!
//! * Vertex set V = {r, v_1, v_2} ∪ {s_0, ..., s_{m-1}} ∪ {e_0, ..., e_{3q-1}}.
//!   Indices: r = 0, v_1 = 1, v_2 = 2, s_i = 3 + i, e_j = 3 + m + j.
//! * Edges (all weights ∈ {1, 2}):
//!     - Forced-center path: (r, v_1) weight 1, (v_1, v_2) weight 1.
//!     - Root-to-set: (r, s_i) weight 2 for every i ∈ [0, m).
//!     - Set-to-element: (s_i, e_j) weight 1 whenever j ∈ S_i.
//!     - Set clique: (s_i, s_j) weight 1 for all 0 ≤ i < j < m.
//! * Diameter bound D = 4.
//! * Weight bound B = 4q + m + 2.
//!
//! Forward direction: an exact cover C' = {S_{i_1}, ..., S_{i_q}} yields a
//! spanning tree of total weight 2q + (m - q) + 3q + 2 = 4q + m + 2 = B and
//! every vertex within distance 2 of r, so the diameter is ≤ 4.
//!
//! Backward direction: any feasible spanning tree must keep every vertex
//! within distance 2 of r (otherwise dist to v_2 exceeds 4). Element vertices
//! only attach through set vertices, so each e_j sits at depth 2 below some
//! s_i that is directly attached to r. Budget counting then forces the number
//! of root-to-set edges to be exactly q, and pigeonhole on the 3q element
//! attachments forces the q chosen sets to be pairwise disjoint -- an exact
//! cover.
//!
//! The solution extractor reads the binary indicator of each root-to-set edge.

use crate::models::graph::BoundedDiameterSpanningTree;
use crate::models::set::ExactCoverBy3Sets;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;

/// Result of reducing ExactCoverBy3Sets to BoundedDiameterSpanningTree.
#[derive(Debug, Clone)]
pub struct ReductionX3CToBoundedDiameterSpanningTree {
    target: BoundedDiameterSpanningTree<SimpleGraph, i32>,
    source_num_subsets: usize,
}

impl ReductionResult for ReductionX3CToBoundedDiameterSpanningTree {
    type Source = ExactCoverBy3Sets;
    type Target = BoundedDiameterSpanningTree<SimpleGraph, i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract the chosen source subsets from the spanning-tree configuration.
    ///
    /// The construction places the m root-to-set edges (r, s_i) at edge indices
    /// 2..2+m (right after the forced-center path edges). For a YES-instance,
    /// the optimal target witness selects exactly q of these edges, which
    /// correspond to the q chosen subsets.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let m = self.source_num_subsets;
        let root_to_set_offset = 2;
        (0..m)
            .map(|i| {
                usize::from(
                    target_solution
                        .get(root_to_set_offset + i)
                        .copied()
                        .unwrap_or(0)
                        == 1,
                )
            })
            .collect()
    }
}

#[reduction(overhead = {
    num_vertices = "num_subsets + universe_size + 3",
    num_edges = "2 + 4 * num_subsets + num_subsets * (num_subsets - 1) / 2",
    weight_bound = "4 * universe_size / 3 + num_subsets + 2",
    diameter_bound = "4",
})]
impl ReduceTo<BoundedDiameterSpanningTree<SimpleGraph, i32>> for ExactCoverBy3Sets {
    type Result = ReductionX3CToBoundedDiameterSpanningTree;

    fn reduce_to(&self) -> Self::Result {
        let universe_size = self.universe_size();
        let m = self.num_subsets();
        let q = universe_size / 3;

        // Vertex indexing matches the docstring.
        let s_index = |i: usize| 3 + i;
        let e_index = |j: usize| 3 + m + j;
        let num_vertices = 3 + m + universe_size;

        let mut edges: Vec<(usize, usize)> = Vec::new();
        let mut weights: Vec<i32> = Vec::new();

        // Forced-center path edges (indices 0 and 1).
        edges.push((0, 1)); // (r, v_1)
        weights.push(1);
        edges.push((1, 2)); // (v_1, v_2)
        weights.push(1);

        // Root-to-set edges (indices 2..2+m). The extractor relies on this layout.
        for i in 0..m {
            edges.push((0, s_index(i)));
            weights.push(2);
        }

        // Set-to-element edges. Subsets are already sorted in `ExactCoverBy3Sets::new`.
        for (i, subset) in self.subsets().iter().enumerate() {
            for &j in subset {
                edges.push((s_index(i), e_index(j)));
                weights.push(1);
            }
        }

        // Set clique edges.
        for i in 0..m {
            for j in (i + 1)..m {
                edges.push((s_index(i), s_index(j)));
                weights.push(1);
            }
        }

        let weight_bound: i32 = (4 * q + m + 2) as i32;
        let diameter_bound: usize = 4;

        let graph = SimpleGraph::new(num_vertices, edges);
        let target = BoundedDiameterSpanningTree::new(graph, weights, weight_bound, diameter_bound);

        ReductionX3CToBoundedDiameterSpanningTree {
            target,
            source_num_subsets: m,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "exactcoverby3sets_to_boundeddiameterspanningtree",
        build: || {
            // q = 2, m = 2: X = {0..5}, C = [{0,1,2}, {3,4,5}].
            // Exact cover: both subsets. Target has 11 vertices and 11 edges,
            // so brute-force solving stays well under the 5s test budget.
            let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5]]);

            // Source: select both subsets.
            let source_config = vec![1, 1];

            // Target spanning tree (n = 11, n-1 = 10 edges):
            //   (r,v1)=idx 0, (v1,v2)=idx 1, (r,s0)=idx 2, (r,s1)=idx 3,
            //   (s0,e0)=idx 4, (s0,e1)=idx 5, (s0,e2)=idx 6,
            //   (s1,e3)=idx 7, (s1,e4)=idx 8, (s1,e5)=idx 9,
            //   (s0,s1)=idx 10 (unused clique edge).
            //
            // Layout (from `reduce_to`):
            //   edge order = forced(2) + root-to-set(m=2) + set-to-element(6) + clique(1)
            //              = indices 0..1, 2..3, 4..9, 10
            // Select every edge except the clique edge.
            let target_config = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0];

            crate::example_db::specs::rule_example_with_witness::<
                _,
                BoundedDiameterSpanningTree<SimpleGraph, i32>,
            >(
                source,
                SolutionPair {
                    source_config,
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/exactcoverby3sets_boundeddiameterspanningtree.rs"]
mod tests;
