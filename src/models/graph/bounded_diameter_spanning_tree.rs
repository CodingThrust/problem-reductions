//! Bounded Diameter Spanning Tree problem implementation.
//!
//! Given a graph G = (V, E) with edge weights, a weight bound B, and a diameter
//! bound D, determine whether G has a spanning tree with total weight at most B
//! and diameter (longest shortest path in edges) at most D.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::WeightElement;
use crate::variant::VariantParam;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "BoundedDiameterSpanningTree",
        display_name: "Bounded Diameter Spanning Tree",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i32", &["i32"]),
        ],
        module_path: module_path!(),
        description: "Does G have a spanning tree with total weight <= B and diameter <= D?",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "edge_weights", type_name: "Vec<W>", description: "Edge weights w: E -> ZZ_(> 0)" },
            FieldInfo { name: "weight_bound", type_name: "W::Sum", description: "Upper bound B on total tree weight" },
            FieldInfo { name: "diameter_bound", type_name: "usize", description: "Upper bound D on tree diameter (in edges)" },
        ],
    }
}

/// Bounded Diameter Spanning Tree problem.
///
/// Given an undirected graph G = (V, E) with positive edge weights w(e), a
/// weight bound B, and a diameter bound D, determine whether G contains a
/// spanning tree T such that the total weight of T is at most B and the
/// diameter of T (the longest shortest path measured in number of edges) is
/// at most D.
///
/// Each configuration entry corresponds to an edge (in the order returned by
/// `graph.edges()`), with value 0 (not selected) or 1 (selected).
///
/// # Type Parameters
///
/// * `G` - Graph type (e.g., SimpleGraph)
/// * `W` - Edge weight type (e.g., i32)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::BoundedDiameterSpanningTree;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let graph = SimpleGraph::new(5, vec![(0,1),(0,2),(0,3),(1,2),(1,4),(2,3),(3,4)]);
/// let problem = BoundedDiameterSpanningTree::new(graph, vec![1,2,1,1,2,1,1], 5, 3);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(
    deserialize = "G: serde::Deserialize<'de>, W: serde::Deserialize<'de>, W::Sum: serde::Deserialize<'de>"
))]
pub struct BoundedDiameterSpanningTree<G, W: WeightElement> {
    /// The underlying graph.
    graph: G,
    /// Weight for each edge in graph-edge order.
    edge_weights: Vec<W>,
    /// Upper bound B on total tree weight.
    weight_bound: W::Sum,
    /// Upper bound D on tree diameter (in edges).
    diameter_bound: usize,
    /// Ordered edge list (mirrors `graph.edges()` order).
    edge_list: Vec<(usize, usize)>,
}

impl<G: Graph, W: WeightElement> BoundedDiameterSpanningTree<G, W> {
    /// Create a new Bounded Diameter Spanning Tree instance.
    ///
    /// # Panics
    /// Panics if `edge_weights` length does not match the graph's edge count,
    /// if any edge weight is not positive, or if `diameter_bound` is zero.
    pub fn new(
        graph: G,
        edge_weights: Vec<W>,
        weight_bound: W::Sum,
        diameter_bound: usize,
    ) -> Self {
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        let zero = W::Sum::zero();
        assert!(
            edge_weights.iter().all(|w| w.to_sum() > zero.clone()),
            "All edge weights must be positive (> 0)"
        );
        assert!(weight_bound > zero, "weight_bound must be positive (> 0)");
        assert!(diameter_bound >= 1, "diameter_bound must be at least 1");
        let edge_list = graph.edges();
        Self {
            graph,
            edge_weights,
            weight_bound,
            diameter_bound,
            edge_list,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the edge weights.
    pub fn edge_weights(&self) -> &[W] {
        &self.edge_weights
    }

    /// Set new edge weights.
    pub fn set_weights(&mut self, edge_weights: Vec<W>) {
        assert_eq!(
            edge_weights.len(),
            self.graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        let zero = W::Sum::zero();
        assert!(
            edge_weights.iter().all(|w| w.to_sum() > zero.clone()),
            "All edge weights must be positive (> 0)"
        );
        self.edge_weights = edge_weights;
    }

    /// Get the weight bound B.
    pub fn weight_bound(&self) -> &W::Sum {
        &self.weight_bound
    }

    /// Get the diameter bound D.
    pub fn diameter_bound(&self) -> usize {
        self.diameter_bound
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

    /// Check whether this problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
    }

    /// Compute the diameter of a tree given its adjacency list.
    /// The diameter is the length (in number of edges) of the longest shortest
    /// path between any two vertices in the tree.
    fn tree_diameter(adj: &[Vec<usize>], n: usize) -> usize {
        let mut max_dist = 0;
        for start in 0..n {
            if adj[start].is_empty() {
                continue;
            }
            let mut dist = vec![usize::MAX; n];
            dist[start] = 0;
            let mut queue = VecDeque::new();
            queue.push_back(start);
            while let Some(v) = queue.pop_front() {
                for &u in &adj[v] {
                    if dist[u] == usize::MAX {
                        dist[u] = dist[v] + 1;
                        if dist[u] > max_dist {
                            max_dist = dist[u];
                        }
                        queue.push_back(u);
                    }
                }
            }
        }
        max_dist
    }
}

impl<G, W> Problem for BoundedDiameterSpanningTree<G, W>
where
    G: Graph + VariantParam,
    W: WeightElement + VariantParam,
{
    const NAME: &'static str = "BoundedDiameterSpanningTree";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
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
            let selected_indices: Vec<usize> = config
                .iter()
                .enumerate()
                .filter(|(_, &v)| v == 1)
                .map(|(i, _)| i)
                .collect();

            // A spanning tree on n vertices must have exactly n-1 edges
            if n == 0 {
                return crate::types::Or(selected_indices.is_empty());
            }
            if selected_indices.len() != n - 1 {
                return crate::types::Or(false);
            }

            // Build adjacency list and compute total weight
            let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
            let mut total_weight = W::Sum::zero();
            for &idx in &selected_indices {
                let (u, v) = self.edge_list[idx];
                adj[u].push(v);
                adj[v].push(u);
                total_weight += self.edge_weights[idx].to_sum();
            }

            // Check weight bound
            if total_weight > self.weight_bound.clone() {
                return crate::types::Or(false);
            }

            // Check connectivity using BFS
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

            if count != n {
                return crate::types::Or(false);
            }

            // Check diameter bound (BFS from each vertex)
            let diameter = Self::tree_diameter(&adj, n);
            diameter <= self.diameter_bound
        })
    }
}

crate::declare_variants! {
    default BoundedDiameterSpanningTree<SimpleGraph, i32> => "num_vertices ^ num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 5 vertices, 7 edges with weights: (0,1,1),(0,2,2),(0,3,1),(1,2,1),(1,4,2),(2,3,1),(3,4,1)
    // B=5, D=3
    // Tree: edges (0,1),(0,3),(2,3),(3,4) → edge indices 0,2,5,6
    // Config: [1,0,1,0,0,1,1] → weight = 1+1+1+1 = 4 ≤ 5, diameter = 3 ≤ 3
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "bounded_diameter_spanning_tree_simplegraph_i32",
        instance: Box::new(BoundedDiameterSpanningTree::new(
            SimpleGraph::new(
                5,
                vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 4), (2, 3), (3, 4)],
            ),
            vec![1, 2, 1, 1, 2, 1, 1],
            5,
            3,
        )),
        optimal_config: vec![1, 0, 1, 0, 0, 1, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/bounded_diameter_spanning_tree.rs"]
mod tests;
