//! SubgraphIsomorphism problem implementation.
//!
//! The Subgraph Isomorphism problem asks whether a "pattern" graph H can be
//! found embedded within a "host" graph G as a subgraph — that is, whether
//! there exists an injective mapping f: V(H) -> V(G) such that every edge
//! {u,v} in H maps to an edge {f(u),f(v)} in G.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SubgraphIsomorphism",
        module_path: module_path!(),
        description: "Determine if host graph G contains a subgraph isomorphic to pattern graph H",
        fields: &[
            FieldInfo { name: "graph", type_name: "SimpleGraph", description: "The host graph G = (V_1, E_1) to search in" },
            FieldInfo { name: "pattern", type_name: "SimpleGraph", description: "The pattern graph H = (V_2, E_2) to find as a subgraph" },
        ],
    }
}

/// The Subgraph Isomorphism problem.
///
/// Given a host graph G = (V_1, E_1) and a pattern graph H = (V_2, E_2),
/// determine whether there exists an injective function f: V_2 -> V_1 such
/// that for every edge {u,v} in E_2, {f(u), f(v)} is an edge in E_1.
///
/// This is a satisfaction (decision) problem: the metric is `bool`.
///
/// # Configuration
///
/// A configuration is a vector of length |V_2| where each entry is a value
/// in {0, ..., |V_1|-1} representing the host vertex that each pattern
/// vertex maps to. The configuration is valid (true) if:
/// 1. All mapped host vertices are distinct (injective mapping)
/// 2. Every edge in the pattern graph maps to an edge in the host graph
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::SubgraphIsomorphism;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Host: K4 (complete graph on 4 vertices)
/// let host = SimpleGraph::new(4, vec![(0,1),(0,2),(0,3),(1,2),(1,3),(2,3)]);
/// // Pattern: triangle (K3)
/// let pattern = SimpleGraph::new(3, vec![(0,1),(0,2),(1,2)]);
/// let problem = SubgraphIsomorphism::new(host, pattern);
///
/// // Mapping [0, 1, 2] means pattern vertex 0->host 0, 1->1, 2->2
/// assert!(problem.evaluate(&[0, 1, 2]));
///
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubgraphIsomorphism {
    /// The host graph G = (V_1, E_1).
    host_graph: SimpleGraph,
    /// The pattern graph H = (V_2, E_2).
    pattern_graph: SimpleGraph,
}

impl SubgraphIsomorphism {
    /// Create a new SubgraphIsomorphism problem.
    ///
    /// # Arguments
    /// * `host_graph` - The host graph to search in
    /// * `pattern_graph` - The pattern graph to find as a subgraph
    pub fn new(host_graph: SimpleGraph, pattern_graph: SimpleGraph) -> Self {
        Self {
            host_graph,
            pattern_graph,
        }
    }

    /// Get a reference to the host graph.
    pub fn host_graph(&self) -> &SimpleGraph {
        &self.host_graph
    }

    /// Get a reference to the pattern graph.
    pub fn pattern_graph(&self) -> &SimpleGraph {
        &self.pattern_graph
    }

    /// Get the number of vertices in the host graph.
    pub fn num_host_vertices(&self) -> usize {
        self.host_graph.num_vertices()
    }

    /// Get the number of edges in the host graph.
    pub fn num_host_edges(&self) -> usize {
        self.host_graph.num_edges()
    }

    /// Get the number of vertices in the pattern graph.
    pub fn num_pattern_vertices(&self) -> usize {
        self.pattern_graph.num_vertices()
    }

    /// Get the number of edges in the pattern graph.
    pub fn num_pattern_edges(&self) -> usize {
        self.pattern_graph.num_edges()
    }

    /// Check if a configuration represents a valid subgraph isomorphism.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.evaluate(config)
    }
}

impl Problem for SubgraphIsomorphism {
    const NAME: &'static str = "SubgraphIsomorphism";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        let n_host = self.host_graph.num_vertices();
        let n_pattern = self.pattern_graph.num_vertices();

        if n_pattern > n_host {
            // No injective mapping possible: each variable gets an empty domain.
            vec![0; n_pattern]
        } else {
            vec![n_host; n_pattern]
        }
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let n_pattern = self.pattern_graph.num_vertices();
        let n_host = self.host_graph.num_vertices();

        // If the pattern has more vertices than the host, no injective mapping exists.
        if n_pattern > n_host {
            return false;
        }

        // Config must have one entry per pattern vertex
        if config.len() != n_pattern {
            return false;
        }

        // All values must be valid host vertex indices
        if config.iter().any(|&v| v >= n_host) {
            return false;
        }

        // Check injectivity: all mapped host vertices must be distinct
        for i in 0..n_pattern {
            for j in (i + 1)..n_pattern {
                if config[i] == config[j] {
                    return false;
                }
            }
        }

        // Check edge preservation: every pattern edge must map to a host edge
        for (u, v) in self.pattern_graph.edges() {
            if !self.host_graph.has_edge(config[u], config[v]) {
                return false;
            }
        }

        true
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for SubgraphIsomorphism {}

crate::declare_variants! {
    default sat SubgraphIsomorphism => "num_host_vertices ^ num_pattern_vertices",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/subgraph_isomorphism.rs"]
mod tests;
