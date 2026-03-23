//! Partial Feedback Edge Set problem implementation.
//!
//! The Partial Feedback Edge Set problem asks whether removing at most `K`
//! edges can hit every cycle of length at most `L`.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};
#[cfg(feature = "example-db")]
use std::collections::BTreeSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "PartialFeedbackEdgeSet",
        display_name: "Partial Feedback Edge Set",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Remove at most K edges so that every cycle of length at most L is hit",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "budget", type_name: "usize", description: "Maximum number K of edges that may be removed" },
            FieldInfo { name: "max_cycle_length", type_name: "usize", description: "Cycle length bound L; every cycle with length at most L must be hit" },
        ],
    }
}

/// The Partial Feedback Edge Set problem.
///
/// Given an undirected graph `G = (V, E)`, a budget `K`, and a cycle-length
/// bound `L`, determine whether there exists a subset `E' ⊆ E` such that:
/// - `|E'| <= K`
/// - every simple cycle in `G` with length at most `L` contains an edge in `E'`
///
/// Each edge has one binary decision variable:
/// - `0`: keep the edge
/// - `1`: remove the edge
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct PartialFeedbackEdgeSet<G> {
    graph: G,
    budget: usize,
    max_cycle_length: usize,
}

impl<G: Graph> PartialFeedbackEdgeSet<G> {
    /// Create a new Partial Feedback Edge Set instance.
    pub fn new(graph: G, budget: usize, max_cycle_length: usize) -> Self {
        Self {
            graph,
            budget,
            max_cycle_length,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the edge-removal budget `K`.
    pub fn budget(&self) -> usize {
        self.budget
    }

    /// Get the cycle-length bound `L`.
    pub fn max_cycle_length(&self) -> usize {
        self.max_cycle_length
    }

    /// Get the number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check whether a configuration is a satisfying partial feedback edge set.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        if config.len() != self.num_edges() || config.iter().any(|&value| value > 1) {
            return false;
        }

        let removed_edges = config.iter().filter(|&&value| value == 1).count();
        if removed_edges > self.budget {
            return false;
        }

        let kept_edges: Vec<bool> = config.iter().map(|&value| value == 0).collect();
        !has_cycle_with_length_at_most(&self.graph, &kept_edges, self.max_cycle_length)
    }
}

impl<G> Problem for PartialFeedbackEdgeSet<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "PartialFeedbackEdgeSet";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or(self.is_valid_solution(config))
    }
}

fn has_cycle_with_length_at_most<G: Graph>(
    graph: &G,
    kept_edges: &[bool],
    max_cycle_length: usize,
) -> bool {
    if kept_edges.len() != graph.num_edges() || max_cycle_length < 3 || graph.num_vertices() < 3 {
        return false;
    }

    let mut adjacency = vec![Vec::new(); graph.num_vertices()];
    for (keep, (u, v)) in kept_edges.iter().copied().zip(graph.edges()) {
        if keep {
            adjacency[u].push(v);
            adjacency[v].push(u);
        }
    }

    let mut visited = vec![false; graph.num_vertices()];
    for start in 0..graph.num_vertices() {
        visited[start] = true;
        for &neighbor in &adjacency[start] {
            if neighbor <= start {
                continue;
            }
            visited[neighbor] = true;
            if dfs_short_cycle(
                &adjacency,
                start,
                neighbor,
                1,
                max_cycle_length,
                &mut visited,
            ) {
                return true;
            }
            visited[neighbor] = false;
        }
        visited[start] = false;
    }

    false
}

fn dfs_short_cycle(
    adjacency: &[Vec<usize>],
    start: usize,
    current: usize,
    path_length: usize,
    max_cycle_length: usize,
    visited: &mut [bool],
) -> bool {
    for &neighbor in &adjacency[current] {
        if neighbor == start {
            let cycle_length = path_length + 1;
            if cycle_length >= 3 && cycle_length <= max_cycle_length {
                return true;
            }
            continue;
        }

        if visited[neighbor] || neighbor <= start || path_length + 1 >= max_cycle_length {
            continue;
        }

        visited[neighbor] = true;
        if dfs_short_cycle(
            adjacency,
            start,
            neighbor,
            path_length + 1,
            max_cycle_length,
            visited,
        ) {
            return true;
        }
        visited[neighbor] = false;
    }

    false
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let graph = SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (1, 2),
            (2, 0),
            (2, 3),
            (3, 4),
            (4, 2),
            (3, 5),
            (5, 4),
            (0, 3),
        ],
    );
    let chosen: BTreeSet<_> = [(0, 2), (2, 3), (3, 4)]
        .into_iter()
        .map(|(u, v)| normalize_edge(u, v))
        .collect();
    let optimal_config = graph
        .edges()
        .into_iter()
        .map(|(u, v)| usize::from(chosen.contains(&normalize_edge(u, v))))
        .collect();

    vec![crate::example_db::specs::ModelExampleSpec {
        id: "partial_feedback_edge_set_simplegraph",
        instance: Box::new(PartialFeedbackEdgeSet::new(graph, 3, 4)),
        optimal_config,
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(any(feature = "example-db", test))]
fn normalize_edge(u: usize, v: usize) -> (usize, usize) {
    if u <= v {
        (u, v)
    } else {
        (v, u)
    }
}

crate::declare_variants! {
    default PartialFeedbackEdgeSet<SimpleGraph> => "2^num_edges",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/partial_feedback_edge_set.rs"]
mod tests;
