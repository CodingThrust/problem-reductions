//! Hamiltonian Path problem implementation.
//!
//! The Hamiltonian Path problem asks whether a graph contains a simple path
//! that visits every vertex exactly once.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use crate::variant::VariantParam;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "HamiltonianPath",
        module_path: module_path!(),
        description: "Find a Hamiltonian path in a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
        ],
    }
}

/// The Hamiltonian Path problem.
///
/// Given a graph G = (V, E), determine whether G contains a Hamiltonian path,
/// i.e., a simple path that visits every vertex exactly once.
///
/// # Representation
///
/// A configuration is a sequence of `n` vertex indices representing a vertex
/// ordering (permutation). Each position `i` in the configuration holds the
/// vertex visited at step `i`. A valid solution must be a permutation of
/// `0..n` where consecutive entries are adjacent in the graph.
///
/// The search space has `dims() = [n; n]` (each position can hold any of `n`
/// vertices), so brute-force enumerates `n^n` configurations. Only `n!`
/// permutations can satisfy the constraints, but the encoding avoids complex
/// variable-domain schemes and matches the problem's natural formulation.
///
/// # Type Parameters
///
/// * `G` - Graph type (e.g., SimpleGraph)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::HamiltonianPath;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Path graph: 0-1-2-3
/// let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
/// let problem = HamiltonianPath::new(graph);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct HamiltonianPath<G> {
    graph: G,
}

impl<G: Graph> HamiltonianPath<G> {
    /// Create a new Hamiltonian Path problem from a graph.
    pub fn new(graph: G) -> Self {
        Self { graph }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check if a configuration is a valid Hamiltonian path.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_valid_hamiltonian_path(&self.graph, config)
    }
}

impl<G> Problem for HamiltonianPath<G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "HamiltonianPath";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.graph.num_vertices();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        is_valid_hamiltonian_path(&self.graph, config)
    }
}

impl<G: Graph + VariantParam> SatisfactionProblem for HamiltonianPath<G> {}

/// Check if a configuration represents a valid Hamiltonian path in the graph.
///
/// A valid Hamiltonian path is a permutation of the vertices such that
/// consecutive vertices in the permutation are adjacent in the graph.
pub(crate) fn is_valid_hamiltonian_path<G: Graph>(graph: &G, config: &[usize]) -> bool {
    let n = graph.num_vertices();
    if config.len() != n {
        return false;
    }

    // Check that config is a valid permutation of 0..n
    let mut seen = vec![false; n];
    for &v in config {
        if v >= n || seen[v] {
            return false;
        }
        seen[v] = true;
    }

    // Check consecutive vertices are adjacent
    for i in 0..n.saturating_sub(1) {
        if !graph.has_edge(config[i], config[i + 1]) {
            return false;
        }
    }

    true
}

// Use Bjorklund (2014) O*(1.657^n) as best known for general undirected graphs
crate::declare_variants! {
    default sat HamiltonianPath<SimpleGraph> => "1.657^num_vertices",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/hamiltonian_path.rs"]
mod tests;
