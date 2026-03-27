//! Longest Circuit problem implementation.
//!
//! The Longest Circuit problem asks for a simple circuit in a graph
//! that maximizes the total edge length.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Max, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "LongestCircuit",
        display_name: "Longest Circuit",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i32", &["i32"]),
        ],
        module_path: module_path!(),
        description: "Find a simple circuit in a graph that maximizes total edge length",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "edge_lengths", type_name: "Vec<W>", description: "Positive edge lengths l: E -> Z_(> 0)" },
        ],
    }
}

/// The Longest Circuit problem.
///
/// Given an undirected graph `G = (V, E)` with positive edge lengths `l(e)`,
/// find a simple circuit in `G` that maximizes the total edge-length sum.
///
/// # Representation
///
/// Each edge is assigned a binary variable:
/// - `0`: edge is not in the circuit
/// - `1`: edge is in the circuit
///
/// A valid configuration must select edges that form exactly one connected
/// simple circuit using only edges from `graph`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongestCircuit<G, W: WeightElement> {
    graph: G,
    edge_lengths: Vec<W>,
}

impl<G: Graph, W: WeightElement> LongestCircuit<G, W> {
    /// Create a new LongestCircuit instance.
    ///
    /// # Panics
    ///
    /// Panics if the number of edge lengths does not match the graph's edge
    /// count, or if any edge length is non-positive.
    pub fn new(graph: G, edge_lengths: Vec<W>) -> Self {
        assert_eq!(
            edge_lengths.len(),
            graph.num_edges(),
            "edge_lengths length must match num_edges"
        );
        let zero = W::Sum::zero();
        assert!(
            edge_lengths
                .iter()
                .all(|length| length.to_sum() > zero.clone()),
            "All edge lengths must be positive (> 0)"
        );
        Self {
            graph,
            edge_lengths,
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

    /// Replace the edge lengths.
    pub fn set_lengths(&mut self, edge_lengths: Vec<W>) {
        assert_eq!(
            edge_lengths.len(),
            self.graph.num_edges(),
            "edge_lengths length must match num_edges"
        );
        let zero = W::Sum::zero();
        assert!(
            edge_lengths
                .iter()
                .all(|length| length.to_sum() > zero.clone()),
            "All edge lengths must be positive (> 0)"
        );
        self.edge_lengths = edge_lengths;
    }

    /// Replace the edge lengths via the generic weight-management naming.
    pub fn set_weights(&mut self, weights: Vec<W>) {
        self.set_lengths(weights);
    }

    /// Get the edge lengths as a cloned vector.
    pub fn weights(&self) -> Vec<W> {
        self.edge_lengths.clone()
    }

    /// Get the number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check if the problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
    }

    /// Check whether a configuration is a valid simple circuit.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_simple_circuit(&self.graph, config)
    }
}

impl<G, W> Problem for LongestCircuit<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "LongestCircuit";
    type Value = Max<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> Max<W::Sum> {
        if !is_simple_circuit(&self.graph, config) {
            return Max(None);
        }
        let mut total = W::Sum::zero();
        for (idx, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.edge_lengths[idx].to_sum();
            }
        }
        Max(Some(total))
    }
}

/// Check whether a binary edge-selection encodes exactly one simple circuit.
pub(crate) fn is_simple_circuit<G: Graph>(graph: &G, config: &[usize]) -> bool {
    if config.len() != graph.num_edges() || config.iter().any(|&value| value > 1) {
        return false;
    }

    let edges = graph.edges();
    let n = graph.num_vertices();
    let mut degree = vec![0usize; n];
    let mut adjacency = vec![Vec::new(); n];
    let mut selected_count = 0usize;
    let mut start = None;

    for (idx, &selected) in config.iter().enumerate() {
        if selected == 0 {
            continue;
        }
        let (u, v) = edges[idx];
        degree[u] += 1;
        degree[v] += 1;
        adjacency[u].push(v);
        adjacency[v].push(u);
        selected_count += 1;
        if start.is_none() {
            start = Some(u);
        }
    }

    if selected_count < 3 {
        return false;
    }

    let selected_vertices: Vec<usize> = degree
        .iter()
        .enumerate()
        .filter_map(|(vertex, &deg)| (deg > 0).then_some(vertex))
        .collect();

    if selected_vertices.is_empty() || selected_vertices.iter().any(|&vertex| degree[vertex] != 2) {
        return false;
    }

    let start = match start {
        Some(vertex) => vertex,
        None => return false,
    };

    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    visited[start] = true;
    queue.push_back(start);
    let mut visited_selected_vertices = 0usize;

    while let Some(vertex) = queue.pop_front() {
        visited_selected_vertices += 1;
        for &neighbor in &adjacency[vertex] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    visited_selected_vertices == selected_vertices.len()
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "longest_circuit_simplegraph_i32",
        instance: Box::new(LongestCircuit::new(
            SimpleGraph::new(
                6,
                vec![
                    (0, 1),
                    (1, 2),
                    (2, 3),
                    (3, 4),
                    (4, 5),
                    (5, 0),
                    (0, 3),
                    (1, 4),
                    (2, 5),
                    (3, 5),
                ],
            ),
            vec![3, 2, 4, 1, 5, 2, 3, 2, 1, 2],
        )),
        optimal_config: vec![1, 0, 1, 0, 1, 0, 1, 1, 1, 0],
        optimal_value: serde_json::json!(18),
    }]
}

crate::declare_variants! {
    default LongestCircuit<SimpleGraph, i32> => "2^num_vertices * num_vertices^2",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/longest_circuit.rs"]
mod tests;
