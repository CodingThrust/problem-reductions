//! Shortest Weight-Constrained Path problem implementation.
//!
//! The Shortest Weight-Constrained Path problem finds a simple path from a
//! source vertex to a target vertex that minimizes total length while keeping
//! the total weight within a prescribed bound.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "ShortestWeightConstrainedPath",
        display_name: "Shortest Weight-Constrained Path",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i32", &["i32"]),
        ],
        module_path: module_path!(),
        description: "Find a simple s-t path minimizing total length subject to a weight budget",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "edge_lengths", type_name: "Vec<W>", description: "Edge lengths l: E -> ZZ_(> 0)" },
            FieldInfo { name: "edge_weights", type_name: "Vec<W>", description: "Edge weights w: E -> ZZ_(> 0)" },
            FieldInfo { name: "source_vertex", type_name: "usize", description: "Source vertex s" },
            FieldInfo { name: "target_vertex", type_name: "usize", description: "Target vertex t" },
            FieldInfo { name: "weight_bound", type_name: "W::Sum", description: "Upper bound W on total path weight" },
        ],
    }
}

/// The Shortest Weight-Constrained Path problem.
///
/// Given a graph G = (V, E) with positive edge lengths l(e) and edge weights
/// w(e), designated vertices s and t, and a weight bound W, find a simple
/// path from s to t that minimizes total length subject to total weight at
/// most W.
///
/// # Representation
///
/// Each edge is assigned a binary variable:
/// - 0: edge is not in the selected path
/// - 1: edge is in the selected path
///
/// A valid configuration must:
/// - form a single simple path from `source_vertex` to `target_vertex`
/// - use only edges present in the graph
/// - satisfy the weight bound
///
/// The objective value is the total length of the path (`Min<N::Sum>`).
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
/// * `N` - The edge length / weight type (e.g., `i32`, `f64`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortestWeightConstrainedPath<G, N: WeightElement> {
    /// The underlying graph.
    graph: G,
    /// Length for each edge in graph-edge order.
    edge_lengths: Vec<N>,
    /// Weight for each edge in graph-edge order.
    edge_weights: Vec<N>,
    /// Source vertex s.
    source_vertex: usize,
    /// Target vertex t.
    target_vertex: usize,
    /// Upper bound W on total path weight.
    weight_bound: N::Sum,
}

impl<G: Graph, N: WeightElement> ShortestWeightConstrainedPath<G, N> {
    fn assert_positive_edge_values(values: &[N], label: &str) {
        let zero = N::Sum::zero();
        assert!(
            values.iter().all(|value| value.to_sum() > zero.clone()),
            "All {label} must be positive (> 0)"
        );
    }

    fn assert_positive_bound(bound: &N::Sum, label: &str) {
        let zero = N::Sum::zero();
        assert!(bound > &zero, "{label} must be positive (> 0)");
    }

    /// Create a new ShortestWeightConstrainedPath instance.
    ///
    /// # Panics
    ///
    /// Panics if either edge vector length does not match the graph's edge
    /// count, or if the source / target vertices are out of bounds.
    pub fn new(
        graph: G,
        edge_lengths: Vec<N>,
        edge_weights: Vec<N>,
        source_vertex: usize,
        target_vertex: usize,
        weight_bound: N::Sum,
    ) -> Self {
        assert_eq!(
            edge_lengths.len(),
            graph.num_edges(),
            "edge_lengths length must match num_edges"
        );
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        Self::assert_positive_edge_values(&edge_lengths, "edge lengths");
        Self::assert_positive_edge_values(&edge_weights, "edge weights");
        assert!(
            source_vertex < graph.num_vertices(),
            "source_vertex {} out of bounds (graph has {} vertices)",
            source_vertex,
            graph.num_vertices()
        );
        assert!(
            target_vertex < graph.num_vertices(),
            "target_vertex {} out of bounds (graph has {} vertices)",
            target_vertex,
            graph.num_vertices()
        );
        Self::assert_positive_bound(&weight_bound, "weight_bound");
        Self {
            graph,
            edge_lengths,
            edge_weights,
            source_vertex,
            target_vertex,
            weight_bound,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the edge lengths.
    pub fn edge_lengths(&self) -> &[N] {
        &self.edge_lengths
    }

    /// Get the edge weights.
    pub fn edge_weights(&self) -> &[N] {
        &self.edge_weights
    }

    /// Set new edge lengths.
    pub fn set_lengths(&mut self, edge_lengths: Vec<N>) {
        assert_eq!(
            edge_lengths.len(),
            self.graph.num_edges(),
            "edge_lengths length must match num_edges"
        );
        Self::assert_positive_edge_values(&edge_lengths, "edge lengths");
        self.edge_lengths = edge_lengths;
    }

    /// Set new edge weights.
    pub fn set_weights(&mut self, edge_weights: Vec<N>) {
        assert_eq!(
            edge_weights.len(),
            self.graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        Self::assert_positive_edge_values(&edge_weights, "edge weights");
        self.edge_weights = edge_weights;
    }

    /// Get the source vertex.
    pub fn source_vertex(&self) -> usize {
        self.source_vertex
    }

    /// Get the target vertex.
    pub fn target_vertex(&self) -> usize {
        self.target_vertex
    }

    /// Get the weight bound.
    pub fn weight_bound(&self) -> &N::Sum {
        &self.weight_bound
    }

    /// Check whether this problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool {
        !N::IS_UNIT
    }

    /// Get the number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check if a configuration is a valid weight-constrained s-t path.
    ///
    /// Returns `Some(total_length)` for a valid simple s-t path whose total
    /// weight is within the weight bound, or `None` otherwise.
    pub fn is_valid_solution(&self, config: &[usize]) -> Option<N::Sum> {
        if config.len() != self.graph.num_edges() || config.iter().any(|&value| value > 1) {
            return None;
        }

        if self.source_vertex == self.target_vertex {
            if config.contains(&1) {
                return None;
            }
            return Some(N::Sum::zero());
        }

        let edges = self.graph.edges();
        let mut degree = vec![0usize; self.graph.num_vertices()];
        let mut adjacency = vec![Vec::new(); self.graph.num_vertices()];
        let mut selected_edge_count = 0usize;
        let mut total_length = N::Sum::zero();
        let mut total_weight = N::Sum::zero();

        for (idx, &selected) in config.iter().enumerate() {
            if selected == 0 {
                continue;
            }
            let (u, v) = edges[idx];
            degree[u] += 1;
            degree[v] += 1;
            adjacency[u].push(v);
            adjacency[v].push(u);
            selected_edge_count += 1;
            total_length += self.edge_lengths[idx].to_sum();
            total_weight += self.edge_weights[idx].to_sum();
        }

        if selected_edge_count == 0 {
            return None;
        }

        if total_weight > self.weight_bound.clone() {
            return None;
        }

        if degree[self.source_vertex] != 1 || degree[self.target_vertex] != 1 {
            return None;
        }

        for (vertex, &vertex_degree) in degree.iter().enumerate() {
            if vertex == self.source_vertex || vertex == self.target_vertex {
                continue;
            }
            if vertex_degree != 0 && vertex_degree != 2 {
                return None;
            }
        }

        let mut visited = vec![false; self.graph.num_vertices()];
        let mut queue = VecDeque::new();
        visited[self.source_vertex] = true;
        queue.push_back(self.source_vertex);

        while let Some(vertex) = queue.pop_front() {
            for &neighbor in &adjacency[vertex] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        if !visited[self.target_vertex] {
            return None;
        }

        let used_vertex_count = degree
            .iter()
            .filter(|&&vertex_degree| vertex_degree > 0)
            .count();
        for (vertex, &vertex_degree) in degree.iter().enumerate() {
            if vertex_degree > 0 && !visited[vertex] {
                return None;
            }
        }

        if used_vertex_count == selected_edge_count + 1 {
            Some(total_length)
        } else {
            None
        }
    }
}

impl<G, N> Problem for ShortestWeightConstrainedPath<G, N>
where
    G: Graph + crate::variant::VariantParam,
    N: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "ShortestWeightConstrainedPath";
    type Value = Min<N::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, N]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<N::Sum> {
        Min(self.is_valid_solution(config))
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "shortest_weight_constrained_path_simplegraph_i32",
        instance: Box::new(ShortestWeightConstrainedPath::new(
            SimpleGraph::new(
                6,
                vec![
                    (0, 1),
                    (0, 2),
                    (1, 3),
                    (2, 3),
                    (2, 4),
                    (3, 5),
                    (4, 5),
                    (1, 4),
                ],
            ),
            vec![2, 4, 3, 1, 5, 4, 2, 6],
            vec![5, 1, 2, 3, 2, 3, 1, 1],
            0,
            5,
            8,
        )),
        optimal_config: vec![0, 1, 0, 1, 0, 1, 0, 0],
        optimal_value: serde_json::json!(9),
    }]
}

crate::declare_variants! {
    default ShortestWeightConstrainedPath<SimpleGraph, i32> => "2^num_edges",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/shortest_weight_constrained_path.rs"]
mod tests;
