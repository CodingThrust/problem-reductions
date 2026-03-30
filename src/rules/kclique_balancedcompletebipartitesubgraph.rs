//! Reduction from KClique to BalancedCompleteBipartiteSubgraph.
//!
//! Classical reduction attributed to Garey and Johnson (GT24) and published in
//! Johnson (1987). Given a KClique instance (G, k), constructs a bipartite graph
//! where Part A = padded vertex set and Part B = edge elements + padding elements,
//! with non-incidence adjacency encoding.

use crate::models::graph::{BalancedCompleteBipartiteSubgraph, KClique};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{BipartiteGraph, Graph, SimpleGraph};

/// Result of reducing KClique to BalancedCompleteBipartiteSubgraph.
///
/// Stores the target problem and the number of original vertices for
/// solution extraction.
#[derive(Debug, Clone)]
pub struct ReductionKCliqueToBCBS {
    target: BalancedCompleteBipartiteSubgraph,
    /// Number of vertices in the original graph (before padding).
    num_original_vertices: usize,
}

impl ReductionResult for ReductionKCliqueToBCBS {
    type Source = KClique<SimpleGraph>;
    type Target = BalancedCompleteBipartiteSubgraph;

    fn target_problem(&self) -> &BalancedCompleteBipartiteSubgraph {
        &self.target
    }

    /// Extract KClique solution from BalancedCompleteBipartiteSubgraph solution.
    ///
    /// The k-clique is S = {v in V : v not in A'}, i.e., the original vertices
    /// NOT selected on the left side. For each original vertex v (0..n-1):
    /// source_config[v] = 1 - target_config[v].
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        (0..self.num_original_vertices)
            .map(|v| 1 - target_solution[v])
            .collect()
    }
}

#[reduction(
    overhead = {
        left_size = "num_vertices + k * (k - 1) / 2",
        right_size = "num_edges + num_vertices - k",
        k = "num_vertices + k * (k - 1) / 2 - k",
    }
)]
impl ReduceTo<BalancedCompleteBipartiteSubgraph> for KClique<SimpleGraph> {
    type Result = ReductionKCliqueToBCBS;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let k = self.k();
        let edges: Vec<(usize, usize)> = self.graph().edges();
        let m = edges.len();

        // C(k, 2) = k*(k-1)/2 — number of edges in a k-clique
        let ck2 = k * (k - 1) / 2;

        // Part A (left partition): n' = n + C(k,2) vertices
        let left_size = n + ck2;

        // Part B (right partition): m edge elements + (n - k) padding elements
        let num_padding = n - k;
        let right_size = m + num_padding;

        // Target biclique parameter: K' = n' - k
        let target_k = left_size - k;

        // Build bipartite edges using non-incidence encoding
        let mut bip_edges = Vec::new();

        for v in 0..left_size {
            // Edge elements: add edge (v, j) if v is NOT an endpoint of edges[j]
            for (j, &(u, w)) in edges.iter().enumerate() {
                if v != u && v != w {
                    // For padded vertices (v >= n), they are never endpoints
                    // of any original edge, so they always connect.
                    bip_edges.push((v, j));
                }
            }

            // Padding elements: always connected
            for p in 0..num_padding {
                bip_edges.push((v, m + p));
            }
        }

        let graph = BipartiteGraph::new(left_size, right_size, bip_edges);
        let target = BalancedCompleteBipartiteSubgraph::new(graph, target_k);

        ReductionKCliqueToBCBS {
            target,
            num_original_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "kclique_to_balancedcompletebipartitesubgraph",
        build: || {
            // 4-vertex graph with edges {0,1}, {0,2}, {1,2}, {2,3}, k=3
            // Known 3-clique: {0, 1, 2}
            let source = KClique::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (2, 3)]), 3);
            // Source config: vertices {0,1,2} selected = [1,1,1,0]
            // Target: left_size=7, right_size=5, k'=4
            // Left side: NOT selecting clique vertices -> select {3,4,5,6}
            // target_config for left: [0,0,0,1,1,1,1]
            // Right side: select edge elements for clique edges + padding
            //   e0={0,1}, e1={0,2}, e2={1,2} are clique edges -> select them
            //   e3={2,3} is not a clique edge -> don't select
            //   w0 is padding -> select
            // target_config for right: [1,1,1,0,1]
            // Full target config: [0,0,0,1,1,1,1, 1,1,1,0,1]
            crate::example_db::specs::rule_example_with_witness::<
                _,
                BalancedCompleteBipartiteSubgraph,
            >(
                source,
                SolutionPair {
                    source_config: vec![1, 1, 1, 0],
                    target_config: vec![0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/kclique_balancedcompletebipartitesubgraph.rs"]
mod tests;
