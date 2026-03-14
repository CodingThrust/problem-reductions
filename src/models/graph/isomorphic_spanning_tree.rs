//! Isomorphic Spanning Tree problem implementation.
//!
//! Given a graph G and a tree T with |V(G)| = |V(T)|, determine whether G
//! contains a spanning tree isomorphic to T. This is a classical NP-complete
//! problem (Garey & Johnson, ND8) that generalizes Hamiltonian Path.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "IsomorphicSpanningTree",
        display_name: "Isomorphic Spanning Tree",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Does graph G contain a spanning tree isomorphic to tree T?",
        fields: &[
            FieldInfo { name: "graph", type_name: "SimpleGraph", description: "The host graph G" },
            FieldInfo { name: "tree", type_name: "SimpleGraph", description: "The target tree T (must be a tree with |V(T)| = |V(G)|)" },
        ],
    }
}

/// Isomorphic Spanning Tree problem.
///
/// Given an undirected graph G = (V, E) and a tree T = (V_T, E_T) with
/// |V| = |V_T|, determine if there exists a bijection π: V_T → V such that
/// for every edge {u, v} in E_T, {π(u), π(v)} is an edge in E.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::IsomorphicSpanningTree;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Host graph: triangle 0-1-2-0
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
/// // Tree: path 0-1-2
/// let tree = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
/// let problem = IsomorphicSpanningTree::new(graph, tree);
///
/// let solver = BruteForce::new();
/// let sol = solver.find_satisfying(&problem);
/// assert!(sol.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsomorphicSpanningTree {
    graph: SimpleGraph,
    tree: SimpleGraph,
}

impl IsomorphicSpanningTree {
    /// Create a new IsomorphicSpanningTree problem.
    ///
    /// # Panics
    ///
    /// Panics if |V(G)| != |V(T)| or if T is not a tree (not connected or
    /// wrong number of edges).
    pub fn new(graph: SimpleGraph, tree: SimpleGraph) -> Self {
        let n = graph.num_vertices();
        assert_eq!(
            n,
            tree.num_vertices(),
            "graph and tree must have the same number of vertices"
        );
        if n > 0 {
            assert_eq!(tree.num_edges(), n - 1, "tree must have exactly n-1 edges");
            assert!(Self::is_connected(&tree), "tree must be connected");
        }
        Self { graph, tree }
    }

    /// Get a reference to the host graph.
    pub fn graph(&self) -> &SimpleGraph {
        &self.graph
    }

    /// Get a reference to the target tree.
    pub fn tree(&self) -> &SimpleGraph {
        &self.tree
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the host graph.
    pub fn num_graph_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Get the number of edges in the target tree.
    pub fn num_tree_edges(&self) -> usize {
        self.tree.num_edges()
    }

    /// Check if a graph is connected using BFS.
    fn is_connected(graph: &SimpleGraph) -> bool {
        let n = graph.num_vertices();
        if n == 0 {
            return true;
        }
        let mut visited = vec![false; n];
        let mut queue = std::collections::VecDeque::new();
        visited[0] = true;
        queue.push_back(0);
        let mut count = 1;
        while let Some(v) = queue.pop_front() {
            for u in graph.neighbors(v) {
                if !visited[u] {
                    visited[u] = true;
                    count += 1;
                    queue.push_back(u);
                }
            }
        }
        count == n
    }
}

impl Problem for IsomorphicSpanningTree {
    const NAME: &'static str = "IsomorphicSpanningTree";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        let n = self.graph.num_vertices();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let n = self.graph.num_vertices();
        if config.len() != n {
            return false;
        }

        // Check that config is a valid permutation: all values in 0..n, all distinct
        let mut seen = vec![false; n];
        for &v in config {
            if v >= n || seen[v] {
                return false;
            }
            seen[v] = true;
        }

        // Check that every tree edge maps to a graph edge under the permutation
        // config[i] = π(i): tree vertex i maps to graph vertex config[i]
        for (u, v) in self.tree.edges() {
            if !self.graph.has_edge(config[u], config[v]) {
                return false;
            }
        }

        true
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for IsomorphicSpanningTree {}

crate::declare_variants! {
    default sat IsomorphicSpanningTree => "factorial(num_vertices)",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/isomorphic_spanning_tree.rs"]
mod tests;
