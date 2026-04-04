//! Reduction from HamiltonianPathBetweenTwoVertices to LongestPath.
//!
//! A Hamiltonian s-t path in G has length n-1 edges (the maximum possible for
//! any simple path). Setting all edge lengths to unit weight and the same
//! source/target vertices, the longest path of length n-1 exactly corresponds
//! to a Hamiltonian s-t path.

use crate::models::graph::{HamiltonianPathBetweenTwoVertices, LongestPath};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::One;

/// Result of reducing HamiltonianPathBetweenTwoVertices to LongestPath.
#[derive(Debug, Clone)]
pub struct ReductionHPBTVToLP {
    target: LongestPath<SimpleGraph, One>,
    /// Cached edge list from the graph, indexed by edge position.
    edges: Vec<(usize, usize)>,
    source_vertex: usize,
    num_vertices: usize,
}

impl ReductionResult for ReductionHPBTVToLP {
    type Source = HamiltonianPathBetweenTwoVertices<SimpleGraph>;
    type Target = LongestPath<SimpleGraph, One>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a vertex-permutation solution from an edge-selection solution.
    ///
    /// The target solution is a binary vector over edges. We walk the selected
    /// edges from the source vertex to reconstruct the vertex ordering.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_vertices;

        // Build adjacency from selected edges
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (idx, &selected) in target_solution.iter().enumerate() {
            if selected == 1 {
                let (u, v) = self.edges[idx];
                adj[u].push(v);
                adj[v].push(u);
            }
        }

        // Walk the path from source
        let mut path = Vec::with_capacity(n);
        let mut current = self.source_vertex;
        let mut prev = usize::MAX; // sentinel for "no previous"
        path.push(current);

        while path.len() < n {
            let next = adj[current]
                .iter()
                .find(|&&neighbor| neighbor != prev)
                .copied();
            match next {
                Some(next_vertex) => {
                    prev = current;
                    current = next_vertex;
                    path.push(current);
                }
                None => break,
            }
        }

        path
    }
}

#[reduction(overhead = {
    num_vertices = "num_vertices",
    num_edges = "num_edges",
})]
impl ReduceTo<LongestPath<SimpleGraph, One>> for HamiltonianPathBetweenTwoVertices<SimpleGraph> {
    type Result = ReductionHPBTVToLP;

    fn reduce_to(&self) -> Self::Result {
        let graph = self.graph().clone();
        let num_edges = graph.num_edges();
        let edges = graph.edges();
        let edge_lengths = vec![One; num_edges];

        let target = LongestPath::new(
            graph,
            edge_lengths,
            self.source_vertex(),
            self.target_vertex(),
        );

        ReductionHPBTVToLP {
            target,
            edges,
            source_vertex: self.source_vertex(),
            num_vertices: self.num_vertices(),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltonianpathbetweentwovertices_to_longestpath",
        build: || {
            // Path graph 0-1-2-3-4 with s=0, t=4
            let source = HamiltonianPathBetweenTwoVertices::new(
                SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
                0,
                4,
            );
            crate::example_db::specs::rule_example_with_witness::<_, LongestPath<SimpleGraph, One>>(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 2, 3, 4],
                    target_config: vec![1, 1, 1, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltonianpathbetweentwovertices_longestpath.rs"]
mod tests;
