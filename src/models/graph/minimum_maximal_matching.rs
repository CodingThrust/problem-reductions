//! MinimumMaximalMatching problem implementation.
//!
//! The Minimum Maximal Matching problem asks for a matching of minimum size
//! that is maximal (cannot be extended by adding any edge).

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumMaximalMatching",
        display_name: "Minimum Maximal Matching",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Find a minimum-size matching that cannot be extended",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
        ],
    }
}

/// The Minimum Maximal Matching problem.
///
/// Given a graph G = (V, E), find a matching M ⊆ E of minimum cardinality
/// such that M is maximal: every edge not in M shares an endpoint with some
/// edge in M (i.e., M cannot be extended by adding any further edge).
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumMaximalMatching;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Path graph P4: 0-1-2-3
/// let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
/// let problem = MinimumMaximalMatching::new(graph);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem).unwrap();
///
/// // Minimum maximal matching has 1 edge (e.g., edge (1,2))
/// let count: usize = solution.iter().sum();
/// assert_eq!(count, 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumMaximalMatching<G> {
    /// The underlying graph.
    graph: G,
}

impl<G: Graph> MinimumMaximalMatching<G> {
    /// Create a MinimumMaximalMatching problem from a graph.
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

    /// Check whether a configuration is a valid maximal matching.
    ///
    /// Returns `true` iff:
    /// 1. The selected edges form a matching (no two share an endpoint).
    /// 2. The matching is maximal (every non-selected edge shares an endpoint
    ///    with some selected edge).
    pub fn is_valid_maximal_matching(&self, config: &[usize]) -> bool {
        let edges = self.graph.edges();
        let n = self.graph.num_vertices();

        // Step 1: Check matching property.
        let mut vertex_used = vec![false; n];
        for (idx, &sel) in config.iter().enumerate() {
            if sel == 1 {
                let (u, v) = edges[idx];
                if vertex_used[u] || vertex_used[v] {
                    return false;
                }
                vertex_used[u] = true;
                vertex_used[v] = true;
            }
        }

        // Step 2: Check maximality — every unselected edge must be blocked.
        for (idx, &sel) in config.iter().enumerate() {
            if sel == 0 {
                let (u, v) = edges[idx];
                // Edge (u,v) is blocked iff u or v is already matched.
                if !vertex_used[u] && !vertex_used[v] {
                    return false;
                }
            }
        }

        true
    }
}

impl<G> Problem for MinimumMaximalMatching<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumMaximalMatching";
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        if config.len() != self.graph.num_edges() {
            return Min(None);
        }
        if !self.is_valid_maximal_matching(config) {
            return Min(None);
        }
        let count = config.iter().filter(|&&x| x == 1).count();
        Min(Some(count))
    }
}

crate::declare_variants! {
    default MinimumMaximalMatching<SimpleGraph> => "1.3160^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // Path graph P6: 6 vertices, edges [(0,1),(1,2),(2,3),(3,4),(4,5)]
    // config [0,1,0,1,0] = edges {(1,2),(3,4)} — a maximal matching of size 2.
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_maximal_matching_simplegraph",
        instance: Box::new(MinimumMaximalMatching::new(SimpleGraph::new(
            6,
            vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)],
        ))),
        optimal_config: vec![0, 1, 0, 1, 0],
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_maximal_matching.rs"]
mod tests;
