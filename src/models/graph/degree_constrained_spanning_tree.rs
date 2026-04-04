//! Degree-Constrained Spanning Tree problem implementation.
//!
//! Given a graph G = (V, E) and a positive integer K, determine whether G has
//! a spanning tree in which every vertex has degree at most K.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::variant::VariantParam;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "DegreeConstrainedSpanningTree",
        display_name: "Degree-Constrained Spanning Tree",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Does G have a spanning tree with maximum vertex degree at most K?",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "max_degree", type_name: "usize", description: "max_degree: maximum allowed vertex degree K (>= 1)" },
        ],
    }
}

/// Degree-Constrained Spanning Tree problem.
///
/// Given an undirected graph G = (V, E) and a positive integer K, determine
/// whether G contains a spanning tree T such that every vertex in T has degree
/// at most K.
///
/// Each configuration entry corresponds to an edge (in the order returned by
/// `graph.edges()`), with value 0 (not selected) or 1 (selected).
///
/// # Type Parameters
///
/// * `G` - Graph type (e.g., SimpleGraph)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::DegreeConstrainedSpanningTree;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let graph = SimpleGraph::new(4, vec![(0,1),(1,2),(2,3),(0,3)]);
/// let problem = DegreeConstrainedSpanningTree::new(graph, 2);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct DegreeConstrainedSpanningTree<G> {
    /// The underlying graph.
    graph: G,
    /// Maximum allowed vertex degree in the spanning tree.
    max_degree: usize,
    /// Ordered edge list (mirrors `graph.edges()` order).
    edge_list: Vec<(usize, usize)>,
}

impl<G: Graph> DegreeConstrainedSpanningTree<G> {
    /// Create a new Degree-Constrained Spanning Tree instance.
    ///
    /// # Panics
    /// Panics if `max_degree` is zero.
    pub fn new(graph: G, max_degree: usize) -> Self {
        assert!(max_degree >= 1, "max_degree must be at least 1");
        let edge_list = graph.edges();
        Self {
            graph,
            max_degree,
            edge_list,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the max_degree parameter K.
    pub fn max_degree(&self) -> usize {
        self.max_degree
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Get the ordered edge list.
    pub fn edge_list(&self) -> &[(usize, usize)] {
        &self.edge_list
    }
}

impl<G> Problem for DegreeConstrainedSpanningTree<G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "DegreeConstrainedSpanningTree";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.edge_list.len()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or({
            let n = self.graph.num_vertices();
            if config.len() != self.edge_list.len() {
                return crate::types::Or(false);
            }

            // Collect selected edges
            let selected: Vec<(usize, usize)> = config
                .iter()
                .enumerate()
                .filter(|(_, &v)| v == 1)
                .map(|(i, _)| self.edge_list[i])
                .collect();

            // A spanning tree on n vertices must have exactly n-1 edges
            if n == 0 {
                return crate::types::Or(selected.is_empty());
            }
            if selected.len() != n - 1 {
                return crate::types::Or(false);
            }

            // Check connectivity using BFS on selected edges
            let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
            let mut degree = vec![0usize; n];
            for &(u, v) in &selected {
                adj[u].push(v);
                adj[v].push(u);
                degree[u] += 1;
                degree[v] += 1;
            }

            // Check max degree constraint
            if degree.iter().any(|&d| d > self.max_degree) {
                return crate::types::Or(false);
            }

            // BFS to check connectivity
            let mut visited = vec![false; n];
            let mut queue = VecDeque::new();
            visited[0] = true;
            queue.push_back(0);
            let mut count = 1;
            while let Some(v) = queue.pop_front() {
                for &u in &adj[v] {
                    if !visited[u] {
                        visited[u] = true;
                        count += 1;
                        queue.push_back(u);
                    }
                }
            }

            count == n
        })
    }
}

crate::declare_variants! {
    default DegreeConstrainedSpanningTree<SimpleGraph> => "2^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 5 vertices, 7 edges: (0,1),(0,2),(0,3),(1,2),(1,4),(2,3),(3,4), K=2
    // Spanning tree with max degree 2: edges (0,2),(0,3),(1,2),(1,4)
    //   indices: 1,2,3,4 → config [0,1,1,1,1,0,0]
    //   Degrees: 0→{2,3}=2, 1→{2,4}=2, 2→{0,1}=2, 3→{0}=1, 4→{1}=1
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "degree_constrained_spanning_tree_simplegraph",
        instance: Box::new(DegreeConstrainedSpanningTree::new(
            SimpleGraph::new(
                5,
                vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 4), (2, 3), (3, 4)],
            ),
            2,
        )),
        optimal_config: vec![0, 1, 1, 1, 1, 0, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/degree_constrained_spanning_tree.rs"]
mod tests;
