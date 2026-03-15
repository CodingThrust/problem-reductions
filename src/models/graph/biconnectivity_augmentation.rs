//! Biconnectivity augmentation problem implementation.
//!
//! Given a graph, weighted potential edges, and a budget, determine whether
//! adding some subset of the potential edges can make the graph biconnected
//! without exceeding the budget.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use crate::types::WeightElement;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "BiconnectivityAugmentation",
        module_path: module_path!(),
        description: "Add weighted potential edges to make a graph biconnected within budget",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "potential_weights", type_name: "Vec<(usize, usize, W)>", description: "Potential edges with augmentation weights" },
            FieldInfo { name: "budget", type_name: "W::Sum", description: "Maximum total augmentation weight B" },
        ],
    }
}

/// The Biconnectivity Augmentation problem.
///
/// Given a graph `G = (V, E)`, weighted potential edges, and a budget `B`,
/// determine whether there exists a subset of potential edges `E'` such that:
/// - `sum_{e in E'} w(e) <= B`
/// - `(V, E union E')` is biconnected
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(
    serialize = "G: serde::Serialize, W: serde::Serialize, W::Sum: serde::Serialize",
    deserialize = "G: serde::Deserialize<'de>, W: serde::Deserialize<'de>, W::Sum: serde::Deserialize<'de>"
))]
pub struct BiconnectivityAugmentation<G, W>
where
    W: WeightElement,
{
    /// The underlying graph.
    graph: G,
    /// Potential augmentation edges with their weights.
    potential_weights: Vec<(usize, usize, W)>,
    /// Maximum total weight of selected potential edges.
    budget: W::Sum,
}

impl<G: Graph, W: WeightElement> BiconnectivityAugmentation<G, W> {
    /// Create a new biconnectivity augmentation instance.
    ///
    /// # Panics
    /// Panics if any potential edge references a vertex index outside the graph.
    pub fn new(graph: G, potential_weights: Vec<(usize, usize, W)>, budget: W::Sum) -> Self {
        let num_vertices = graph.num_vertices();
        for &(u, v, _) in &potential_weights {
            assert!(
                u < num_vertices && v < num_vertices,
                "potential edge ({}, {}) references vertex >= num_vertices ({})",
                u,
                v,
                num_vertices
            );
        }

        Self {
            graph,
            potential_weights,
            budget,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the weighted potential edges.
    pub fn potential_weights(&self) -> &[(usize, usize, W)] {
        &self.potential_weights
    }

    /// Get the budget.
    pub fn budget(&self) -> &W::Sum {
        &self.budget
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Get the number of potential augmentation edges.
    pub fn num_potential_edges(&self) -> usize {
        self.potential_weights.len()
    }

    /// Check if the problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
    }

    fn augmented_graph(&self, config: &[usize]) -> Option<SimpleGraph> {
        if config.len() != self.num_potential_edges() || config.iter().any(|&value| value >= 2) {
            return None;
        }

        let mut total = W::Sum::zero();
        let mut edges = BTreeSet::new();

        for (u, v) in self.graph.edges() {
            edges.insert(normalize_edge(u, v));
        }

        for (selected, &(u, v, ref weight)) in config.iter().zip(&self.potential_weights) {
            if *selected == 1 {
                total += weight.to_sum();
                if total > self.budget.clone() {
                    return None;
                }
                edges.insert(normalize_edge(u, v));
            }
        }

        Some(SimpleGraph::new(
            self.num_vertices(),
            edges.into_iter().collect(),
        ))
    }
}

impl<G, W> Problem for BiconnectivityAugmentation<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "BiconnectivityAugmentation";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_potential_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.augmented_graph(config)
            .is_some_and(|graph| is_biconnected(&graph))
    }
}

impl<G, W> SatisfactionProblem for BiconnectivityAugmentation<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
}

fn normalize_edge(u: usize, v: usize) -> (usize, usize) {
    if u <= v {
        (u, v)
    } else {
        (v, u)
    }
}

struct DfsState {
    visited: Vec<bool>,
    discovery_time: Vec<usize>,
    low: Vec<usize>,
    parent: Vec<Option<usize>>,
    time: usize,
    has_articulation_point: bool,
}

fn dfs_articulation_points<G: Graph>(graph: &G, vertex: usize, state: &mut DfsState) {
    if state.has_articulation_point {
        return;
    }

    state.visited[vertex] = true;
    state.time += 1;
    state.discovery_time[vertex] = state.time;
    state.low[vertex] = state.time;

    let mut child_count = 0;
    for neighbor in graph.neighbors(vertex) {
        if !state.visited[neighbor] {
            child_count += 1;
            state.parent[neighbor] = Some(vertex);
            dfs_articulation_points(graph, neighbor, state);
            state.low[vertex] = state.low[vertex].min(state.low[neighbor]);

            if state.parent[vertex].is_none() && child_count > 1 {
                state.has_articulation_point = true;
                return;
            }

            if state.parent[vertex].is_some() && state.low[neighbor] >= state.discovery_time[vertex]
            {
                state.has_articulation_point = true;
                return;
            }
        } else if state.parent[vertex] != Some(neighbor) {
            state.low[vertex] = state.low[vertex].min(state.discovery_time[neighbor]);
        }
    }
}

fn is_biconnected<G: Graph>(graph: &G) -> bool {
    let num_vertices = graph.num_vertices();
    if num_vertices <= 1 {
        return true;
    }

    let mut state = DfsState {
        visited: vec![false; num_vertices],
        discovery_time: vec![0; num_vertices],
        low: vec![0; num_vertices],
        parent: vec![None; num_vertices],
        time: 0,
        has_articulation_point: false,
    };

    dfs_articulation_points(graph, 0, &mut state);

    !state.has_articulation_point && state.visited.into_iter().all(|seen| seen)
}

crate::declare_variants! {
    BiconnectivityAugmentation<SimpleGraph, i32> => "2^num_potential_edges",
}

#[cfg(test)]
pub(crate) fn canonical_model_example_specs() -> (
    SimpleGraph,
    Vec<(usize, usize, i32)>,
    i32,
    Vec<usize>,
) {
    (
        SimpleGraph::path(6),
        vec![
            (0, 2, 1),
            (0, 3, 2),
            (0, 4, 3),
            (1, 3, 1),
            (1, 4, 2),
            (1, 5, 3),
            (2, 4, 1),
            (2, 5, 2),
            (3, 5, 1),
        ],
        4,
        vec![1, 0, 0, 1, 0, 0, 1, 0, 1],
    )
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/biconnectivity_augmentation.rs"]
mod tests;
