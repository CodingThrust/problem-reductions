//! Disjoint Connecting Paths problem implementation.
//!
//! The problem asks whether an undirected graph contains pairwise
//! vertex-disjoint paths connecting a prescribed collection of terminal pairs.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::variant::VariantParam;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "DisjointConnectingPaths",
        display_name: "Disjoint Connecting Paths",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Find pairwise vertex-disjoint paths connecting given terminal pairs",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "terminal_pairs", type_name: "Vec<(usize, usize)>", description: "Disjoint terminal pairs (s_i, t_i)" },
        ],
    }
}

/// Disjoint Connecting Paths on an undirected graph.
///
/// A configuration uses one binary variable per edge in the graph's canonical
/// sorted edge list. A valid solution selects exactly the edges of one simple
/// path for each terminal pair, with all such paths pairwise vertex-disjoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct DisjointConnectingPaths<G> {
    graph: G,
    terminal_pairs: Vec<(usize, usize)>,
}

impl<G: Graph> DisjointConnectingPaths<G> {
    /// Create a new Disjoint Connecting Paths instance.
    ///
    /// # Panics
    ///
    /// Panics if no terminal pairs are provided, if a pair uses invalid or
    /// repeated endpoints, or if any terminal appears in more than one pair.
    pub fn new(graph: G, terminal_pairs: Vec<(usize, usize)>) -> Self {
        assert!(
            !terminal_pairs.is_empty(),
            "terminal_pairs must contain at least one pair"
        );

        let num_vertices = graph.num_vertices();
        let mut used = vec![false; num_vertices];
        for &(source, sink) in &terminal_pairs {
            assert!(source < num_vertices, "terminal pair source out of bounds");
            assert!(sink < num_vertices, "terminal pair sink out of bounds");
            assert_ne!(source, sink, "terminal pair endpoints must be distinct");
            assert!(
                !used[source],
                "terminal vertices must be pairwise disjoint across pairs"
            );
            assert!(
                !used[sink],
                "terminal vertices must be pairwise disjoint across pairs"
            );
            used[source] = true;
            used[sink] = true;
        }

        Self {
            graph,
            terminal_pairs,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the terminal pairs.
    pub fn terminal_pairs(&self) -> &[(usize, usize)] {
        &self.terminal_pairs
    }

    /// Get the number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Get the number of terminal pairs.
    pub fn num_pairs(&self) -> usize {
        self.terminal_pairs.len()
    }

    /// Return the canonical lexicographically sorted undirected edge list.
    pub fn ordered_edges(&self) -> Vec<(usize, usize)> {
        canonical_edges(&self.graph)
    }

    /// Check whether a configuration is a valid solution.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_valid_disjoint_connecting_paths(&self.graph, &self.terminal_pairs, config)
    }
}

impl<G> Problem for DisjointConnectingPaths<G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "DisjointConnectingPaths";
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

fn canonical_edges<G: Graph>(graph: &G) -> Vec<(usize, usize)> {
    let mut edges = graph
        .edges()
        .into_iter()
        .map(|(u, v)| if u <= v { (u, v) } else { (v, u) })
        .collect::<Vec<_>>();
    edges.sort_unstable();
    edges
}

fn normalize_edge(u: usize, v: usize) -> (usize, usize) {
    if u <= v {
        (u, v)
    } else {
        (v, u)
    }
}

fn is_valid_disjoint_connecting_paths<G: Graph>(
    graph: &G,
    terminal_pairs: &[(usize, usize)],
    config: &[usize],
) -> bool {
    let edges = canonical_edges(graph);
    if config.len() != edges.len() {
        return false;
    }
    if config.iter().any(|&value| value > 1) {
        return false;
    }

    let num_vertices = graph.num_vertices();
    let mut adjacency = vec![Vec::new(); num_vertices];
    let mut degrees = vec![0usize; num_vertices];
    for (index, &chosen) in config.iter().enumerate() {
        if chosen == 1 {
            let (u, v) = edges[index];
            adjacency[u].push(v);
            adjacency[v].push(u);
            degrees[u] += 1;
            degrees[v] += 1;
        }
    }

    let mut terminal_vertices = vec![false; num_vertices];
    let required_pairs = terminal_pairs
        .iter()
        .map(|&(u, v)| {
            terminal_vertices[u] = true;
            terminal_vertices[v] = true;
            normalize_edge(u, v)
        })
        .collect::<BTreeSet<_>>();
    let mut matched_pairs = BTreeSet::new();
    let mut visited = vec![false; num_vertices];
    let mut component_count = 0usize;

    for start in 0..num_vertices {
        if degrees[start] == 0 || visited[start] {
            continue;
        }

        component_count += 1;
        let mut stack = vec![start];
        let mut vertices = Vec::new();
        let mut degree_sum = 0usize;
        visited[start] = true;

        while let Some(vertex) = stack.pop() {
            vertices.push(vertex);
            degree_sum += degrees[vertex];
            for &neighbor in &adjacency[vertex] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    stack.push(neighbor);
                }
            }
        }

        let edge_count = degree_sum / 2;
        if edge_count + 1 != vertices.len() {
            return false;
        }

        let mut endpoints = Vec::new();
        for &vertex in &vertices {
            match degrees[vertex] {
                1 => endpoints.push(vertex),
                2 => {
                    if terminal_vertices[vertex] {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        if endpoints.len() != 2 {
            return false;
        }

        let realized_pair = normalize_edge(endpoints[0], endpoints[1]);
        if !required_pairs.contains(&realized_pair) || !matched_pairs.insert(realized_pair) {
            return false;
        }
    }

    component_count == terminal_pairs.len() && matched_pairs.len() == terminal_pairs.len()
}

crate::declare_variants! {
    default DisjointConnectingPaths<SimpleGraph> => "2^num_edges",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "disjoint_connecting_paths_simplegraph",
        instance: Box::new(DisjointConnectingPaths::new(
            SimpleGraph::new(
                6,
                vec![(0, 1), (1, 3), (0, 2), (1, 4), (2, 4), (3, 5), (4, 5)],
            ),
            vec![(0, 3), (2, 5)],
        )),
        optimal_config: vec![1, 0, 1, 0, 1, 0, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/disjoint_connecting_paths.rs"]
mod tests;
