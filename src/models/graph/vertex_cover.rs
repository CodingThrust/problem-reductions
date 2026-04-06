//! Vertex Cover (decision version) problem implementation.
//!
//! Given a graph G = (V, E) and a positive integer k, determine whether there
//! exists a vertex cover of size at most k. This is the decision version of
//! MinimumVertexCover — one of Karp's 21 NP-complete problems.

use super::minimum_vertex_cover::is_vertex_cover_config;
use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "VertexCover",
        display_name: "Vertex Cover",
        aliases: &["VC"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Determine whether a vertex cover of size at most k exists",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "k", type_name: "usize", description: "Maximum allowed cover size" },
        ],
    }
}

/// Vertex Cover (decision version).
///
/// Given graph G = (V, E) and positive integer k, determine whether there
/// exists a vertex cover of size at most k — a subset V' ⊆ V with |V'| ≤ k
/// such that every edge has at least one endpoint in V'.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::VertexCover;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Triangle with pendant: edges {0,1}, {1,2}, {0,2}, {2,3}
/// let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (0, 2), (2, 3)]);
/// let problem = VertexCover::new(graph, 2);
///
/// let solver = BruteForce::new();
/// let witness = solver.find_witness(&problem);
/// assert!(witness.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexCover<G> {
    /// The underlying graph.
    graph: G,
    /// Maximum cover size threshold.
    k: usize,
}

impl<G: Graph> VertexCover<G> {
    /// Create a new VertexCover problem.
    pub fn new(graph: G, k: usize) -> Self {
        assert!(k > 0, "k must be positive");
        assert!(k <= graph.num_vertices(), "k must be at most num_vertices");
        Self { graph, k }
    }

    /// Get a reference to the graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the cover size threshold.
    pub fn k(&self) -> usize {
        self.k
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check if a configuration is a valid vertex cover of size ≤ k.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        if config.len() != self.graph.num_vertices() {
            return false;
        }
        let count: usize = config.iter().filter(|&&v| v == 1).count();
        if count > self.k {
            return false;
        }
        is_vertex_cover_config(&self.graph, config)
    }
}

impl<G> Problem for VertexCover<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "VertexCover";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or(self.is_valid_solution(config))
    }
}

crate::declare_variants! {
    default VertexCover<SimpleGraph> => "1.1996^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "vertex_cover_simplegraph",
        instance: Box::new(VertexCover::new(
            SimpleGraph::new(4, vec![(0, 1), (1, 2), (0, 2), (2, 3)]),
            2,
        )),
        optimal_config: vec![1, 0, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/vertex_cover.rs"]
mod tests;
