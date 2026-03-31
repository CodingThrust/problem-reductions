//! Hamiltonian Path Between Two Vertices problem implementation.
//!
//! The Hamiltonian Path Between Two Vertices problem asks whether a graph contains a
//! simple path that starts at a specified source vertex, ends at a specified target
//! vertex, and visits every other vertex exactly once.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::variant::VariantParam;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "HamiltonianPathBetweenTwoVertices",
        display_name: "Hamiltonian Path Between Two Vertices",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Find a Hamiltonian path between two specified vertices in a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "source_vertex", type_name: "usize", description: "Source vertex s" },
            FieldInfo { name: "target_vertex", type_name: "usize", description: "Target vertex t" },
        ],
    }
}

/// The Hamiltonian Path Between Two Vertices problem.
///
/// Given a graph G = (V, E) and two distinguished vertices s, t in V,
/// determine whether G contains a Hamiltonian path from s to t, i.e.,
/// a simple path that begins at s, ends at t, and visits every vertex
/// exactly once.
///
/// # Representation
///
/// A configuration is a sequence of `n` vertex indices representing a vertex
/// ordering (permutation). Each position `i` in the configuration holds the
/// vertex visited at step `i`. A valid solution must be a permutation of
/// `0..n` where:
/// - The first element equals `source_vertex`
/// - The last element equals `target_vertex`
/// - Consecutive entries are adjacent in the graph
///
/// The search space has `dims() = [n; n]` (each position can hold any of `n`
/// vertices), so brute-force enumerates `n^n` configurations.
///
/// # Type Parameters
///
/// * `G` - Graph type (e.g., SimpleGraph)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::HamiltonianPathBetweenTwoVertices;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Path graph: 0-1-2-3, source=0, target=3
/// let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
/// let problem = HamiltonianPathBetweenTwoVertices::new(graph, 0, 3);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct HamiltonianPathBetweenTwoVertices<G> {
    graph: G,
    source_vertex: usize,
    target_vertex: usize,
}

impl<G: Graph> HamiltonianPathBetweenTwoVertices<G> {
    /// Create a new Hamiltonian Path Between Two Vertices problem.
    ///
    /// # Panics
    ///
    /// Panics if `source_vertex` or `target_vertex` is out of range, or if they are equal.
    pub fn new(graph: G, source_vertex: usize, target_vertex: usize) -> Self {
        let n = graph.num_vertices();
        assert!(
            source_vertex < n,
            "source_vertex {source_vertex} out of range for graph with {n} vertices"
        );
        assert!(
            target_vertex < n,
            "target_vertex {target_vertex} out of range for graph with {n} vertices"
        );
        assert_ne!(
            source_vertex, target_vertex,
            "source_vertex and target_vertex must be distinct"
        );
        Self {
            graph,
            source_vertex,
            target_vertex,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the source vertex s.
    pub fn source_vertex(&self) -> usize {
        self.source_vertex
    }

    /// Get the target vertex t.
    pub fn target_vertex(&self) -> usize {
        self.target_vertex
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check if a configuration is a valid Hamiltonian s-t path.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_valid_hamiltonian_st_path(&self.graph, config, self.source_vertex, self.target_vertex)
    }
}

impl<G> Problem for HamiltonianPathBetweenTwoVertices<G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "HamiltonianPathBetweenTwoVertices";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.graph.num_vertices();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or(is_valid_hamiltonian_st_path(
            &self.graph,
            config,
            self.source_vertex,
            self.target_vertex,
        ))
    }
}

/// Check if a configuration represents a valid Hamiltonian s-t path in the graph.
///
/// A valid Hamiltonian s-t path is a permutation of all vertices such that:
/// - The first element is `source`
/// - The last element is `target`
/// - Consecutive vertices in the permutation are adjacent in the graph
pub(crate) fn is_valid_hamiltonian_st_path<G: Graph>(
    graph: &G,
    config: &[usize],
    source: usize,
    target: usize,
) -> bool {
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

    // Check endpoint constraints
    if config[0] != source || config[n - 1] != target {
        return false;
    }

    // Check consecutive vertices are adjacent
    for i in 0..n.saturating_sub(1) {
        if !graph.has_edge(config[i], config[i + 1]) {
            return false;
        }
    }

    true
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // Instance from issue #831: 6 vertices, s=0, t=5
    // Hamiltonian s-t path: [0, 3, 2, 1, 4, 5]
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "hamiltonian_path_between_two_vertices_simplegraph",
        instance: Box::new(HamiltonianPathBetweenTwoVertices::new(
            SimpleGraph::new(
                6,
                vec![
                    (0, 1),
                    (0, 3),
                    (1, 2),
                    (1, 4),
                    (2, 5),
                    (3, 4),
                    (4, 5),
                    (2, 3),
                ],
            ),
            0,
            5,
        )),
        optimal_config: vec![0, 3, 2, 1, 4, 5],
        optimal_value: serde_json::json!(true),
    }]
}

// Use Bjorklund (2014) O*(1.657^n) as best known for general undirected graphs
crate::declare_variants! {
    default HamiltonianPathBetweenTwoVertices<SimpleGraph> => "1.657^num_vertices",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/hamiltonian_path_between_two_vertices.rs"]
mod tests;
