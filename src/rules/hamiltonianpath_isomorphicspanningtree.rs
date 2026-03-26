//! Reduction from HamiltonianPath to IsomorphicSpanningTree.
//!
//! A Hamiltonian path in G exists iff G has a spanning tree isomorphic to the
//! path graph P_n. The reduction keeps G unchanged as the host graph and
//! constructs T = P_n (the path on n vertices: edges {0,1},{1,2},...,{n-2,n-1}).

use crate::models::graph::{HamiltonianPath, IsomorphicSpanningTree};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;

/// Result of reducing HamiltonianPath to IsomorphicSpanningTree.
#[derive(Debug, Clone)]
pub struct ReductionHPToIST {
    target: IsomorphicSpanningTree,
}

impl ReductionResult for ReductionHPToIST {
    type Source = HamiltonianPath<SimpleGraph>;
    type Target = IsomorphicSpanningTree;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: identity mapping.
    ///
    /// The IST config maps tree vertex i to graph vertex config[i]. Since the
    /// tree is P_n (path 0-1-2-...-n-1), this mapping directly gives the
    /// vertex ordering of the Hamiltonian path.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices",
        num_graph_edges = "num_edges",
        num_tree_edges = "num_vertices - 1",
    }
)]
impl ReduceTo<IsomorphicSpanningTree> for HamiltonianPath<SimpleGraph> {
    type Result = ReductionHPToIST;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();

        // Host graph: keep G unchanged
        let graph = self.graph().clone();

        // Target tree: path graph P_n with edges {0,1},{1,2},...,{n-2,n-1}
        let path_edges: Vec<(usize, usize)> =
            (0..n.saturating_sub(1)).map(|i| (i, i + 1)).collect();
        let tree = SimpleGraph::new(n, path_edges);

        ReductionHPToIST {
            target: IsomorphicSpanningTree::new(graph, tree),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltonianpath_to_isomorphicspanningtree",
        build: || {
            // Path graph 0-1-2-3-4 has a trivial Hamiltonian path
            let source =
                HamiltonianPath::new(SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]));
            crate::example_db::specs::rule_example_with_witness::<_, IsomorphicSpanningTree>(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 2, 3, 4],
                    target_config: vec![0, 1, 2, 3, 4],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltonianpath_isomorphicspanningtree.rs"]
mod tests;
