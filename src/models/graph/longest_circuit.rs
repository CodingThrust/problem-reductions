//! Longest Circuit problem implementation.
//!
//! The Longest Circuit problem asks whether a graph contains a simple circuit
//! whose total edge length is at least a given bound.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use crate::types::WeightElement;
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
        description: "Determine whether a graph contains a simple circuit with total length at least K",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "edge_lengths", type_name: "Vec<W>", description: "Positive edge lengths l: E -> Z_(> 0)" },
            FieldInfo { name: "bound", type_name: "W::Sum", description: "Lower bound K on the total circuit length" },
        ],
    }
}

/// The Longest Circuit problem.
///
/// Given an undirected graph `G = (V, E)` with positive edge lengths `l(e)` and
/// a positive bound `K`, determine whether there exists a simple circuit in `G`
/// whose total edge-length sum is at least `K`.
///
/// # Representation
///
/// Each edge is assigned a binary variable:
/// - `0`: edge is not in the circuit
/// - `1`: edge is in the circuit
///
/// A valid configuration must select edges that:
/// - form exactly one connected simple circuit
/// - use only edges from `graph`
/// - have total selected length at least `bound`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongestCircuit<G, W: WeightElement> {
    graph: G,
    edge_lengths: Vec<W>,
    bound: W::Sum,
}

impl<G: Graph, W: WeightElement> LongestCircuit<G, W> {
    /// Create a new LongestCircuit instance.
    ///
    /// # Panics
    ///
    /// Panics if the number of edge lengths does not match the graph's edge
    /// count, if any edge length is non-positive, or if `bound` is non-positive.
    pub fn new(graph: G, edge_lengths: Vec<W>, bound: W::Sum) -> Self {
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
        assert!(bound > zero, "bound must be positive (> 0)");
        Self {
            graph,
            edge_lengths,
            bound,
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

    /// Get the lower bound K.
    pub fn bound(&self) -> &W::Sum {
        &self.bound
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

    /// Check whether a configuration is a valid satisfying simple circuit.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        if !is_simple_circuit(&self.graph, config) {
            return false;
        }

        let mut total = W::Sum::zero();
        for (idx, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.edge_lengths[idx].to_sum();
            }
        }

        total >= self.bound
    }
}

impl<G, W> Problem for LongestCircuit<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "LongestCircuit";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.is_valid_solution(config)
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

impl<G, W> SatisfactionProblem for LongestCircuit<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
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
            17,
        )),
        optimal_config: vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
        optimal_value: serde_json::json!(true),
    }]
}

crate::declare_variants! {
    default sat LongestCircuit<SimpleGraph, i32> => "2^num_vertices * num_vertices^2",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/longest_circuit.rs"]
mod tests;
