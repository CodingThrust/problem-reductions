//! Longest Path problem implementation.
//!
//! The Longest Path problem asks for a simple path between two distinguished
//! vertices that maximizes the total edge length.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, One, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "LongestPath",
        display_name: "Longest Path",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i32", &["i32", "One"]),
        ],
        module_path: module_path!(),
        description: "Find a simple s-t path of maximum total edge length",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "edge_lengths", type_name: "Vec<W>", description: "Positive edge lengths l: E -> ZZ_(> 0)" },
            FieldInfo { name: "source_vertex", type_name: "usize", description: "Source vertex s" },
            FieldInfo { name: "target_vertex", type_name: "usize", description: "Target vertex t" },
        ],
    }
}

/// The Longest Path problem.
///
/// Given a graph `G = (V, E)` with positive edge lengths `l(e)` and
/// distinguished vertices `s` and `t`, find a simple path from `s` to `t`
/// maximizing the total length of its selected edges.
///
/// # Representation
///
/// Each edge is assigned a binary variable:
/// - `0`: the edge is not selected
/// - `1`: the edge is selected
///
/// A valid configuration must select exactly the edges of one simple
/// undirected path from `source_vertex` to `target_vertex`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongestPath<G, W: WeightElement> {
    graph: G,
    edge_lengths: Vec<W>,
    source_vertex: usize,
    target_vertex: usize,
}

impl<G: Graph, W: WeightElement> LongestPath<G, W> {
    fn assert_positive_edge_lengths(edge_lengths: &[W]) {
        let zero = W::Sum::zero();
        assert!(
            edge_lengths
                .iter()
                .all(|length| length.to_sum() > zero.clone()),
            "All edge lengths must be positive (> 0)"
        );
    }

    /// Create a new LongestPath instance.
    pub fn new(graph: G, edge_lengths: Vec<W>, source_vertex: usize, target_vertex: usize) -> Self {
        assert_eq!(
            edge_lengths.len(),
            graph.num_edges(),
            "edge_lengths length must match num_edges"
        );
        Self::assert_positive_edge_lengths(&edge_lengths);
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
        Self {
            graph,
            edge_lengths,
            source_vertex,
            target_vertex,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the edge lengths.
    pub fn edge_lengths(&self) -> &[W] {
        &self.edge_lengths
    }

    /// Replace the edge lengths with a new vector.
    pub fn set_lengths(&mut self, edge_lengths: Vec<W>) {
        assert_eq!(
            edge_lengths.len(),
            self.graph.num_edges(),
            "edge_lengths length must match num_edges"
        );
        Self::assert_positive_edge_lengths(&edge_lengths);
        self.edge_lengths = edge_lengths;
    }

    /// Get the source vertex.
    pub fn source_vertex(&self) -> usize {
        self.source_vertex
    }

    /// Get the target vertex.
    pub fn target_vertex(&self) -> usize {
        self.target_vertex
    }

    /// Check whether this problem uses non-unit edge lengths.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
    }

    /// Get the number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check if a configuration encodes a valid simple source-target path.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_simple_st_path(&self.graph, self.source_vertex, self.target_vertex, config)
    }
}

impl<G, W> Problem for LongestPath<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "LongestPath";
    type Value = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        if !self.is_valid_solution(config) {
            return SolutionSize::Invalid;
        }

        let mut total = W::Sum::zero();
        for (idx, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.edge_lengths[idx].to_sum();
            }
        }
        SolutionSize::Valid(total)
    }
}

impl<G, W> OptimizationProblem for LongestPath<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    type Objective = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

fn is_simple_st_path<G: Graph>(
    graph: &G,
    source_vertex: usize,
    target_vertex: usize,
    config: &[usize],
) -> bool {
    if config.len() != graph.num_edges() || config.iter().any(|&value| value > 1) {
        return false;
    }

    if source_vertex == target_vertex {
        return config.iter().all(|&value| value == 0);
    }

    let edges = graph.edges();
    let mut degree = vec![0usize; graph.num_vertices()];
    let mut adjacency = vec![Vec::new(); graph.num_vertices()];
    let mut selected_edge_count = 0usize;

    for (idx, &selected) in config.iter().enumerate() {
        if selected == 0 {
            continue;
        }
        let (u, v) = edges[idx];
        degree[u] += 1;
        degree[v] += 1;
        if degree[u] > 2 || degree[v] > 2 {
            return false;
        }
        adjacency[u].push(v);
        adjacency[v].push(u);
        selected_edge_count += 1;
    }

    if selected_edge_count == 0 {
        return false;
    }
    if degree[source_vertex] != 1 || degree[target_vertex] != 1 {
        return false;
    }

    let mut selected_vertex_count = 0usize;
    for (vertex, &vertex_degree) in degree.iter().enumerate() {
        if vertex_degree == 0 {
            continue;
        }
        selected_vertex_count += 1;
        if vertex != source_vertex && vertex != target_vertex && vertex_degree != 2 {
            return false;
        }
    }

    if selected_edge_count != selected_vertex_count.saturating_sub(1) {
        return false;
    }

    let mut visited = vec![false; graph.num_vertices()];
    let mut queue = VecDeque::new();
    visited[source_vertex] = true;
    queue.push_back(source_vertex);

    while let Some(vertex) = queue.pop_front() {
        for &neighbor in &adjacency[vertex] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    visited[target_vertex]
        && degree
            .iter()
            .enumerate()
            .all(|(vertex, &vertex_degree)| vertex_degree == 0 || visited[vertex])
}

crate::declare_variants! {
    default opt LongestPath<SimpleGraph, i32> => "num_vertices * 2^num_vertices",
    opt LongestPath<SimpleGraph, One> => "num_vertices * 2^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "longest_path_simplegraph_i32",
        instance: Box::new(LongestPath::new(
            SimpleGraph::new(
                7,
                vec![
                    (0, 1),
                    (0, 2),
                    (1, 3),
                    (2, 3),
                    (2, 4),
                    (3, 5),
                    (4, 5),
                    (4, 6),
                    (5, 6),
                    (1, 6),
                ],
            ),
            vec![3, 2, 4, 1, 5, 2, 3, 2, 4, 1],
            0,
            6,
        )),
        optimal_config: vec![1, 0, 1, 1, 1, 0, 1, 0, 1, 0],
        optimal_value: serde_json::json!({"Valid": 20}),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/longest_path.rs"]
mod tests;
