//! Hamiltonian Circuit problem implementation.
//!
//! The Hamiltonian Circuit problem asks whether a graph contains a cycle
//! that visits every vertex exactly once and returns to the starting vertex.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use crate::variant::VariantParam;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "HamiltonianCircuit",
        module_path: module_path!(),
        description: "Does the graph contain a Hamiltonian circuit?",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The undirected graph G=(V,E)" },
        ],
    }
}

/// The Hamiltonian Circuit problem.
///
/// Given a graph G = (V, E), determine whether there exists a cycle that
/// visits every vertex exactly once and returns to the starting vertex.
///
/// # Type Parameters
///
/// * `G` - Graph type (e.g., SimpleGraph)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::HamiltonianCircuit;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Square graph (4-cycle) has a Hamiltonian circuit
/// let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]);
/// let problem = HamiltonianCircuit::new(graph);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_satisfying(&problem);
///
/// // Verify all solutions are valid Hamiltonian circuits
/// for sol in &solutions {
///     assert!(problem.evaluate(sol));
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct HamiltonianCircuit<G> {
    /// The underlying graph.
    graph: G,
}

impl<G: Graph> HamiltonianCircuit<G> {
    /// Create a new Hamiltonian Circuit problem from a graph.
    pub fn new(graph: G) -> Self {
        Self { graph }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph().num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph().num_edges()
    }
}

impl<G> Problem for HamiltonianCircuit<G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "HamiltonianCircuit";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.graph.num_vertices();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let n = self.graph.num_vertices();
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

        // Check that consecutive vertices (including wrap-around) are connected by edges
        for i in 0..n {
            let u = config[i];
            let v = config[(i + 1) % n];
            if !self.graph.has_edge(u, v) {
                return false;
            }
        }

        true
    }
}

impl<G: Graph + VariantParam> SatisfactionProblem for HamiltonianCircuit<G> {}

crate::declare_variants! {
    HamiltonianCircuit<SimpleGraph> => "num_vertices^2 * 2^num_vertices",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/hamiltonian_circuit.rs"]
mod tests;
