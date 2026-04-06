//! Maximum Leaf Spanning Tree problem implementation.
//!
//! Given a connected graph G, find a spanning tree T of G that maximizes
//! the number of leaves (degree-1 vertices) in T.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::Max;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumLeafSpanningTree",
        display_name: "Maximum Leaf Spanning Tree",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Find spanning tree maximizing the number of leaves",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
        ],
    }
}

/// The Maximum Leaf Spanning Tree problem.
///
/// Given a connected graph G = (V, E), find a spanning tree T of G such that
/// the number of leaves (vertices with degree 1 in T) is maximized.
///
/// # Representation
///
/// Each edge is assigned a binary variable:
/// - 0: edge is not in the spanning tree
/// - 1: edge is in the spanning tree
///
/// A valid spanning tree requires exactly n-1 selected edges that form a
/// connected, acyclic subgraph spanning all vertices.
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximumLeafSpanningTree<G> {
    /// The underlying graph.
    graph: G,
}

impl<G: Graph> MaximumLeafSpanningTree<G> {
    /// Create a MaximumLeafSpanningTree problem from a graph.
    ///
    /// The graph must have at least 2 vertices.
    pub fn new(graph: G) -> Self {
        assert!(
            graph.num_vertices() >= 2,
            "graph must have at least 2 vertices"
        );
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

    /// Check if a configuration is a valid spanning tree.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_valid_spanning_tree(&self.graph, config)
    }
}

/// Check if a configuration forms a valid spanning tree:
/// 1. Exactly n-1 edges selected
/// 2. Selected edges form a connected subgraph (which, combined with n-1 edges, implies a tree)
fn is_valid_spanning_tree<G: Graph>(graph: &G, config: &[usize]) -> bool {
    let n = graph.num_vertices();
    let edges = graph.edges();
    if config.len() != edges.len() {
        return false;
    }

    // Count selected edges
    let selected_count: usize = config.iter().sum();
    if selected_count != n - 1 {
        return false;
    }

    // Build adjacency from selected edges and check connectivity via BFS
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    for (idx, &sel) in config.iter().enumerate() {
        if sel == 1 {
            let (u, v) = edges[idx];
            adj[u].push(v);
            adj[v].push(u);
        }
    }

    // BFS from vertex 0
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();
    visited[0] = true;
    queue.push_back(0);
    while let Some(v) = queue.pop_front() {
        for &u in &adj[v] {
            if !visited[u] {
                visited[u] = true;
                queue.push_back(u);
            }
        }
    }

    // All vertices must be reachable
    visited.iter().all(|&v| v)
}

/// Count the number of leaves (degree-1 vertices) in the tree defined by the config.
fn count_leaves<G: Graph>(graph: &G, config: &[usize]) -> usize {
    let n = graph.num_vertices();
    let edges = graph.edges();
    let mut degree = vec![0usize; n];
    for (idx, &sel) in config.iter().enumerate() {
        if sel == 1 {
            let (u, v) = edges[idx];
            degree[u] += 1;
            degree[v] += 1;
        }
    }
    degree.iter().filter(|&&d| d == 1).count()
}

impl<G> Problem for MaximumLeafSpanningTree<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "MaximumLeafSpanningTree";
    type Value = Max<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> Max<usize> {
        if !is_valid_spanning_tree(&self.graph, config) {
            return Max(None);
        }
        Max(Some(count_leaves(&self.graph, config)))
    }
}

crate::declare_variants! {
    default MaximumLeafSpanningTree<SimpleGraph> => "1.8966^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "maximum_leaf_spanning_tree_simplegraph",
        instance: Box::new(MaximumLeafSpanningTree::new(SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 4),
                (2, 4),
                (2, 5),
                (3, 5),
                (4, 5),
                (1, 3),
            ],
        ))),
        // Edges: 0:(0,1), 1:(0,2), 2:(0,3), 3:(1,4), 4:(2,4), 5:(2,5), 6:(3,5), 7:(4,5), 8:(1,3)
        // Tree: {(0,1),(0,2),(0,3),(2,4),(2,5)} = indices 0,1,2,4,5
        // Leaves: 1,3,4,5 (degree 1 each), Internal: 0 (deg 3), 2 (deg 3)
        optimal_config: vec![1, 1, 1, 0, 1, 1, 0, 0, 0],
        optimal_value: serde_json::json!(4),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/maximum_leaf_spanning_tree.rs"]
mod tests;
