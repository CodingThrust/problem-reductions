//! Degree-Constrained Spanning Tree problem implementation.
//!
//! Given an undirected graph, determine whether it contains a spanning tree
//! whose maximum vertex degree is at most a prescribed bound.

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
        description: "Does graph G contain a spanning tree with maximum degree at most K?",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "max_degree", type_name: "usize", description: "Upper bound K on the degree of every vertex in the spanning tree" },
        ],
    }
}

/// Degree-Constrained Spanning Tree.
///
/// Given an undirected graph `G = (V, E)` and an integer bound `K`, determine
/// whether there exists a spanning tree `T` of `G` such that every vertex has
/// degree at most `K` in `T`.
///
/// # Representation
///
/// A configuration is a binary vector of length `|E|`. Entry `config[e]` is 1
/// exactly when the corresponding edge of `G` is selected into the candidate
/// spanning tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct DegreeConstrainedSpanningTree<G> {
    graph: G,
    max_degree: usize,
}

impl<G: Graph> DegreeConstrainedSpanningTree<G> {
    /// Create a new DegreeConstrainedSpanningTree instance.
    pub fn new(graph: G, max_degree: usize) -> Self {
        Self { graph, max_degree }
    }

    /// Get the underlying graph.
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

    /// Get the maximum allowed degree.
    pub fn max_degree(&self) -> usize {
        self.max_degree
    }

    /// Check whether a configuration is a valid degree-constrained spanning tree.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_degree_constrained_spanning_tree(&self.graph, self.max_degree, config)
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
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or(is_degree_constrained_spanning_tree(
            &self.graph,
            self.max_degree,
            config,
        ))
    }
}

pub(crate) fn is_degree_constrained_spanning_tree<G: Graph>(
    graph: &G,
    max_degree: usize,
    config: &[usize],
) -> bool {
    let edges = graph.edges();
    if config.len() != edges.len() || config.iter().any(|&value| value > 1) {
        return false;
    }

    let num_vertices = graph.num_vertices();
    let selected_count = config.iter().filter(|&&value| value == 1).count();
    if selected_count != num_vertices.saturating_sub(1) {
        return false;
    }

    if num_vertices <= 1 {
        return true;
    }

    let mut adjacency = vec![Vec::new(); num_vertices];
    let mut degree = vec![0usize; num_vertices];

    for ((u, v), &selected) in edges.iter().copied().zip(config.iter()) {
        if selected == 0 {
            continue;
        }
        degree[u] += 1;
        degree[v] += 1;
        if degree[u] > max_degree || degree[v] > max_degree {
            return false;
        }
        adjacency[u].push(v);
        adjacency[v].push(u);
    }

    let mut visited = vec![false; num_vertices];
    let mut queue = VecDeque::new();
    visited[0] = true;
    queue.push_back(0);

    while let Some(vertex) = queue.pop_front() {
        for &neighbor in &adjacency[vertex] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    visited.into_iter().all(|seen| seen)
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "degree_constrained_spanning_tree_simplegraph",
        instance: Box::new(DegreeConstrainedSpanningTree::new(SimpleGraph::path(4), 2)),
        optimal_config: vec![1, 1, 1],
        optimal_value: serde_json::json!(true),
    }]
}

crate::declare_variants! {
    default DegreeConstrainedSpanningTree<SimpleGraph> => "2^num_edges",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/degree_constrained_spanning_tree.rs"]
mod tests;
