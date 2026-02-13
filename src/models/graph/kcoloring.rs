//! Graph K-Coloring problem implementation.
//!
//! The K-Coloring problem asks whether a graph can be colored with K colors
//! such that no two adjacent vertices have the same color.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "KColoring",
        description: "Find valid k-coloring of a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
        ],
    }
}

/// The Graph K-Coloring problem.
///
/// Given a graph G = (V, E) and K colors, find an assignment of colors
/// to vertices such that no two adjacent vertices have the same color.
///
/// # Type Parameters
///
/// * `K` - Number of colors (const generic)
/// * `G` - Graph type (e.g., SimpleGraph, GridGraph)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::KColoring;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Triangle graph needs at least 3 colors
/// let problem = KColoring::<3, SimpleGraph>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_satisfying(&problem);
///
/// // Verify all solutions are valid colorings
/// for sol in &solutions {
///     assert!(problem.evaluate(sol));
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KColoring<const K: usize, G> {
    /// The underlying graph.
    graph: G,
}

impl<const K: usize> KColoring<K, SimpleGraph> {
    /// Create a new K-Coloring problem.
    ///
    /// # Arguments
    /// * `num_vertices` - Number of vertices
    /// * `edges` - List of edges as (u, v) pairs
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self {
        let graph = SimpleGraph::new(num_vertices, edges);
        Self { graph }
    }
}

impl<const K: usize, G: Graph> KColoring<K, G> {
    /// Create a K-Coloring problem from an existing graph.
    pub fn from_graph(graph: G) -> Self {
        Self { graph }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Get the number of colors.
    pub fn num_colors(&self) -> usize {
        K
    }

    /// Get the edges as a list of (u, v) pairs.
    pub fn edges(&self) -> Vec<(usize, usize)> {
        self.graph.edges()
    }

    /// Check if a coloring is valid.
    fn is_valid_coloring(&self, config: &[usize]) -> bool {
        for (u, v) in self.graph.edges() {
            let color_u = config.get(u).copied().unwrap_or(0);
            let color_v = config.get(v).copied().unwrap_or(0);
            if color_u == color_v {
                return false;
            }
        }
        true
    }
}

impl<const K: usize, G> Problem for KColoring<K, G>
where
    G: Graph,
{
    const NAME: &'static str = "KColoring";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("k", crate::variant::const_usize_str::<K>()),
            ("graph", crate::variant::short_type_name::<G>()),
        ]
    }

    fn dims(&self) -> Vec<usize> {
        vec![K; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.is_valid_coloring(config)
    }
}

/// Check if a coloring is valid for a graph.
pub fn is_valid_coloring(
    num_vertices: usize,
    edges: &[(usize, usize)],
    coloring: &[usize],
    num_colors: usize,
) -> bool {
    if coloring.len() != num_vertices {
        return false;
    }
    // Check all colors are valid
    if coloring.iter().any(|&c| c >= num_colors) {
        return false;
    }
    // Check no adjacent vertices have same color
    for &(u, v) in edges {
        if u < coloring.len() && v < coloring.len() && coloring[u] == coloring[v] {
            return false;
        }
    }
    true
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/kcoloring.rs"]
mod tests;
