//! Reduction from KClique to SubgraphIsomorphism.
//!
//! Given a KClique instance (G, k), we construct a SubgraphIsomorphism instance
//! where the host graph is G and the pattern graph is K_k (the complete graph on
//! k vertices). G contains a k-clique if and only if G contains a subgraph
//! isomorphic to K_k.

use crate::models::graph::{KClique, SubgraphIsomorphism};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing KClique to SubgraphIsomorphism.
///
/// The host graph is the original graph G and the pattern graph is K_k.
/// A subgraph isomorphism mapping f: V(K_k) -> V(G) identifies the k vertices
/// that form a clique in G.
#[derive(Debug, Clone)]
pub struct ReductionKCliqueToSubIso {
    target: SubgraphIsomorphism,
    num_source_vertices: usize,
}

impl ReductionResult for ReductionKCliqueToSubIso {
    type Source = KClique<SimpleGraph>;
    type Target = SubgraphIsomorphism;

    fn target_problem(&self) -> &SubgraphIsomorphism {
        &self.target
    }

    /// Extract KClique solution from SubgraphIsomorphism solution.
    ///
    /// The SubgraphIsomorphism config maps each pattern vertex (0..k-1) to a
    /// host vertex. We create a binary vector of length n and set positions
    /// f(0), f(1), ..., f(k-1) to 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let mut config = vec![0; self.num_source_vertices];
        for &host_vertex in target_solution {
            config[host_vertex] = 1;
        }
        config
    }
}

#[reduction(
    overhead = {
        num_host_vertices = "num_vertices",
        num_host_edges = "num_edges",
        num_pattern_vertices = "k",
        num_pattern_edges = "k * (k - 1) / 2",
    }
)]
impl ReduceTo<SubgraphIsomorphism> for KClique<SimpleGraph> {
    type Result = ReductionKCliqueToSubIso;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();
        let k = self.k();

        // Build the complete graph K_k as the pattern
        let mut pattern_edges = Vec::new();
        for i in 0..k {
            for j in (i + 1)..k {
                pattern_edges.push((i, j));
            }
        }
        let pattern = SimpleGraph::new(k, pattern_edges);

        // Host graph is the original graph, cloned into a SimpleGraph
        let host_edges = self.graph().edges();
        let host = SimpleGraph::new(n, host_edges);

        let target = SubgraphIsomorphism::new(host, pattern);

        ReductionKCliqueToSubIso {
            target,
            num_source_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "kclique_to_subgraphisomorphism",
        build: || {
            // 5-vertex graph with a known 3-clique on vertices {2, 3, 4}
            let source = KClique::new(
                SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]),
                3,
            );
            crate::example_db::specs::rule_example_with_witness::<_, SubgraphIsomorphism>(
                source,
                SolutionPair {
                    source_config: vec![0, 0, 1, 1, 1],
                    target_config: vec![2, 3, 4],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/kclique_subgraphisomorphism.rs"]
mod tests;
