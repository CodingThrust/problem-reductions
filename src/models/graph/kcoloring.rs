//! Graph K-Coloring problem implementation.
//!
//! The K-Coloring problem asks whether a graph can be colored with K colors
//! such that no two adjacent vertices have the same color.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use crate::variant::{KValue, VariantParam};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "KColoring",
        module_path: module_path!(),
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
/// * `K` - KValue type representing the number of colors (e.g., K3 for 3-coloring)
/// * `G` - Graph type (e.g., SimpleGraph, GridGraph)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::KColoring;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::variant::K3;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Triangle graph needs at least 3 colors
/// let problem = KColoring::<K3, SimpleGraph>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
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
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct KColoring<K: KValue, G> {
    /// The underlying graph.
    graph: G,
    #[serde(skip)]
    _phantom: std::marker::PhantomData<K>,
}

impl<K: KValue> KColoring<K, SimpleGraph> {
    /// Create a new K-Coloring problem.
    ///
    /// # Arguments
    /// * `num_vertices` - Number of vertices
    /// * `edges` - List of edges as (u, v) pairs
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self {
        let graph = SimpleGraph::new(num_vertices, edges);
        Self {
            graph,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<K: KValue, G: Graph> KColoring<K, G> {
    /// Create a K-Coloring problem from an existing graph.
    pub fn from_graph(graph: G) -> Self {
        Self {
            graph,
            _phantom: std::marker::PhantomData,
        }
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
        K::K.expect("KN cannot be used as problem instance")
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

impl<K: KValue, G> Problem for KColoring<K, G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "KColoring";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![K, G]
    }

    fn dims(&self) -> Vec<usize> {
        let k = K::K.expect("KN cannot be used as problem instance");
        vec![k; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.is_valid_coloring(config)
    }
}

impl<K: KValue, G: Graph + VariantParam> SatisfactionProblem for KColoring<K, G> {}

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
