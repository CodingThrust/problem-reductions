//! Graph K-Coloring problem implementation.
//!
//! The K-Coloring problem asks whether a graph can be colored with K colors
//! such that no two adjacent vertices have the same color.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::Graph;
use crate::traits::{Problem, SatisfactionProblem};
use crate::variant::{KValue, VariantParam, KN};
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
/// * `G` - Graph type (e.g., SimpleGraph, KingsSubgraph)
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
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
/// let problem = KColoring::<K3, _>::new(graph);
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
    /// Runtime number of colors. Always set; for compile-time K types it equals K::K.
    #[serde(default = "default_num_colors::<K>")]
    num_colors: usize,
    #[serde(skip)]
    _phantom: std::marker::PhantomData<K>,
}

fn default_num_colors<K: KValue>() -> usize {
    K::K.unwrap_or(0)
}

impl<K: KValue, G: Graph> KColoring<K, G> {
    /// Create a new K-Coloring problem from a graph.
    ///
    /// # Panics
    /// Panics if `K` is `KN` (use [`KColoring::<KN, G>::with_k`] instead).
    pub fn new(graph: G) -> Self {
        Self {
            graph,
            num_colors: K::K.expect("KN requires with_k"),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the number of colors.
    pub fn num_colors(&self) -> usize {
        self.num_colors
    }

    /// Check if a configuration is a valid coloring.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_valid_coloring(&self.graph, config, self.num_colors)
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

impl<G: Graph> KColoring<KN, G> {
    /// Create a K-Coloring problem with an explicit number of colors.
    ///
    /// Only available for `KN` (runtime K). For compile-time K types like
    /// `K3`, use [`new`](KColoring::new) which derives K from the type
    /// parameter.
    pub fn with_k(graph: G, num_colors: usize) -> Self {
        Self {
            graph,
            num_colors,
            _phantom: std::marker::PhantomData,
        }
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
        vec![self.num_colors; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.is_valid_coloring(config)
    }

    fn problem_size_names() -> &'static [&'static str] {
        &["num_vertices", "num_edges"]
    }
    fn problem_size_values(&self) -> Vec<usize> {
        vec![self.graph().num_vertices(), self.graph().num_edges()]
    }
}

impl<K: KValue, G: Graph + VariantParam> SatisfactionProblem for KColoring<K, G> {}

/// Check if a coloring is valid for a graph.
///
/// # Panics
/// Panics if `coloring.len() != graph.num_vertices()`.
pub(crate) fn is_valid_coloring<G: Graph>(graph: &G, coloring: &[usize], num_colors: usize) -> bool {
    assert_eq!(
        coloring.len(),
        graph.num_vertices(),
        "coloring length must match num_vertices"
    );
    // Check all colors are valid
    if coloring.iter().any(|&c| c >= num_colors) {
        return false;
    }
    // Check no adjacent vertices have same color
    for (u, v) in graph.edges() {
        if coloring[u] == coloring[v] {
            return false;
        }
    }
    true
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/kcoloring.rs"]
mod tests;
