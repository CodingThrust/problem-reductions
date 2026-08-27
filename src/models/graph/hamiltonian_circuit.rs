//! Hamiltonian Circuit problem implementation.
//!
//! The Hamiltonian Circuit problem asks whether a graph contains a cycle
//! that visits every vertex exactly once and returns to the starting vertex.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::variant::VariantParam;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "HamiltonianCircuit",
        display_name: "Hamiltonian Circuit",
        aliases: &["HC"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
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
/// use problemreductions::{Problem, BruteForce};
///
/// // Square graph (4-cycle) has a Hamiltonian circuit
/// let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]);
/// let problem = HamiltonianCircuit::new(graph);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem).unwrap();
///
/// // Verify all solutions are valid Hamiltonian circuits
/// for sol in &solutions {
///     assert!(problem.evaluate(sol).unwrap());
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

    /// Check if a configuration is a valid Hamiltonian circuit.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_valid_hamiltonian_circuit(&self.graph, config)
    }
}

impl<G> Problem for HamiltonianCircuit<G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "HamiltonianCircuit";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_size![("num_edges", num_edges), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        let n = self.graph.num_vertices();
        if config.len() != n {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "circuit ordering length does not match the graph vertices".into(),
            ));
        }
        if config.iter().any(|&vertex| vertex >= n) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "circuit ordering contains an out-of-range vertex".into(),
            ));
        }
        Ok(crate::types::Or(is_valid_hamiltonian_circuit(
            &self.graph,
            config,
        )))
    }
}

impl<G> crate::solvers::BruteForceProblem for HamiltonianCircuit<G>
where
    G: Graph + VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        let n = self.graph.num_vertices();
        vec![n; n]
    }
}

/// Check if a configuration represents a valid Hamiltonian circuit in the graph.
///
/// A valid Hamiltonian circuit is a permutation of the vertices such that
/// consecutive vertices in the permutation are adjacent in the graph,
/// including a closing edge from the last vertex back to the first.
pub(crate) fn is_valid_hamiltonian_circuit<G: Graph>(graph: &G, config: &[usize]) -> bool {
    let n = graph.num_vertices();
    if n < 3 || config.len() != n {
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
        if !graph.has_edge(u, v) {
            return false;
        }
    }

    true
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "hamiltonian_circuit_simplegraph",
        // Prism graph (triangular prism): 6 vertices, 9 edges
        instance: Box::new(HamiltonianCircuit::new(SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (1, 2),
                (2, 0),
                (3, 4),
                (4, 5),
                (5, 3),
                (0, 3),
                (1, 4),
                (2, 5),
            ],
        ))),
        optimal_config: serde_json::json!(vec![0, 1, 2, 5, 4, 3]),
        optimal_value: serde_json::json!(true),
    }]
}

crate::impl_random_generate!(
    HamiltonianCircuit<SimpleGraph>,
    crate::random::SimpleGraphRandomSpec,
    |spec| { Ok(HamiltonianCircuit::new(spec.graph()?)) }
);

crate::declare_variants! {
    default HamiltonianCircuit<SimpleGraph> => "1.657^num_vertices" random,
}

crate::register_brute_force! {
    HamiltonianCircuit<SimpleGraph>,
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/hamiltonian_circuit.rs"]
mod tests;
