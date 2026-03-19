//! Steiner Tree problem implementation.
//!
//! Given a weighted graph and a set of terminal vertices, find a minimum-weight
//! tree that connects all terminals.

use std::collections::BTreeSet;

use num_traits::Zero;
use serde::{Deserialize, Serialize};

use crate::{
    registry::{FieldInfo, ProblemSchemaEntry, VariantDimension},
    topology::{Graph, SimpleGraph},
    traits::{OptimizationProblem, Problem},
    types::{Direction, One, SolutionSize, WeightElement},
};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SteinerTree",
        display_name: "Steiner Tree",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i32", &["One", "i32"]),
        ],
        module_path: module_path!(),
        description: "Find minimum weight tree connecting terminal vertices",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "edge_weights", type_name: "Vec<W>", description: "Edge weights w: E -> R" },
            FieldInfo { name: "terminals", type_name: "Vec<usize>", description: "Terminal vertices T that must be connected" },
        ],
    }
}

/// The Steiner Tree problem.
///
/// Given a weighted graph G = (V, E) with edge weights w_e and a set
/// of terminal vertices T, find a tree S in G such that T is a subset
/// of V(S), minimizing the total edge weight of S.
///
/// # Representation
///
/// Each edge is assigned a binary variable:
/// - 0: edge is not in the Steiner tree
/// - 1: edge is in the Steiner tree
///
/// A valid Steiner tree requires:
/// - Selected edges form a tree (connected + acyclic)
/// - All terminal vertices are included
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
/// * `W` - The weight type for edges (e.g., `i32`, `f64`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteinerTree<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each edge (in edge index order).
    edge_weights: Vec<W>,
    /// Terminal vertices that must be connected.
    terminals: Vec<usize>,
}

impl<G: Graph, W: Clone + Default> SteinerTree<G, W> {
    /// Create a SteinerTree problem from a graph, edge weights, and terminals.
    pub fn new(graph: G, edge_weights: Vec<W>, terminals: Vec<usize>) -> Self {
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        let n = graph.num_vertices();
        let distinct_terminals: BTreeSet<_> = terminals.iter().copied().collect();
        assert_eq!(
            distinct_terminals.len(),
            terminals.len(),
            "terminals must be distinct"
        );
        for &t in &terminals {
            assert!(t < n, "terminal {t} out of range (num_vertices = {n})");
        }
        assert!(terminals.len() >= 2, "at least 2 terminals required");
        Self {
            graph,
            edge_weights,
            terminals,
        }
    }

    /// Create a SteinerTree problem with unit edge weights.
    pub fn unit_weights(graph: G, terminals: Vec<usize>) -> Self
    where
        W: From<i32>,
    {
        let edge_weights = vec![W::from(1); graph.num_edges()];
        Self::new(graph, edge_weights, terminals)
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get a reference to the edge weights.
    pub fn edge_weights(&self) -> &[W] {
        &self.edge_weights
    }

    /// Set new edge weights.
    pub fn set_weights(&mut self, weights: Vec<W>) {
        assert_eq!(weights.len(), self.graph.num_edges());
        self.edge_weights = weights;
    }

    /// Get the edge weights as a Vec.
    pub fn weights(&self) -> Vec<W> {
        self.edge_weights.clone()
    }

    /// Get the terminal vertices.
    pub fn terminals(&self) -> &[usize] {
        &self.terminals
    }

    /// Check if the problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool
    where
        W: WeightElement,
    {
        !W::IS_UNIT
    }

    /// Check if a configuration is a valid Steiner tree.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_valid_steiner_tree(&self.graph, &self.terminals, config)
    }
}

impl<G: Graph, W: WeightElement> SteinerTree<G, W> {
    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Get the number of terminal vertices.
    pub fn num_terminals(&self) -> usize {
        self.terminals.len()
    }
}

/// Check if a configuration forms a valid Steiner tree:
/// 1. Selected edges form a connected subgraph containing all terminals
/// 2. Selected edges are acyclic (tree property)
fn is_valid_steiner_tree<G: Graph>(graph: &G, terminals: &[usize], config: &[usize]) -> bool {
    let n = graph.num_vertices();
    let edges = graph.edges();
    if config.len() != edges.len() {
        return false;
    }

    // Build adjacency list from selected edges
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    let mut selected_count = 0usize;
    let mut involved = vec![false; n];
    for (idx, &sel) in config.iter().enumerate() {
        if sel == 1 {
            let (u, v) = edges[idx];
            adj[u].push(v);
            adj[v].push(u);
            involved[u] = true;
            involved[v] = true;
            selected_count += 1;
        }
    }

    if selected_count == 0 {
        return false;
    }

    // BFS from first terminal to check connectivity
    let start = terminals[0];
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();
    visited[start] = true;
    queue.push_back(start);
    while let Some(v) = queue.pop_front() {
        for &u in &adj[v] {
            if !visited[u] {
                visited[u] = true;
                queue.push_back(u);
            }
        }
    }

    // All terminals must be reachable
    if !terminals.iter().all(|&t| visited[t]) {
        return false;
    }

    // All involved vertices must be in one connected component
    if (0..n).any(|i| involved[i] && !visited[i]) {
        return false;
    }

    // Tree property: #edges == #involved_vertices - 1
    let involved_count = involved.iter().filter(|&&x| x).count();
    selected_count == involved_count - 1
}

impl<G, W> Problem for SteinerTree<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "SteinerTree";
    type Metric = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        if !is_valid_steiner_tree(&self.graph, &self.terminals, config) {
            return SolutionSize::Invalid;
        }
        let mut total = W::Sum::zero();
        for (idx, &selected) in config.iter().enumerate() {
            if selected == 1 {
                if let Some(w) = self.edge_weights.get(idx) {
                    total += w.to_sum();
                }
            }
        }
        SolutionSize::Valid(total)
    }
}

impl<G, W> OptimizationProblem for SteinerTree<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

crate::declare_variants! {
    default opt SteinerTree<SimpleGraph, i32> => "3^num_terminals * num_vertices + 2^num_terminals * num_vertices^2",
    opt SteinerTree<SimpleGraph, One> => "3^num_terminals * num_vertices + 2^num_terminals * num_vertices^2",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "steiner_tree_simplegraph_i32",
        instance: Box::new(SteinerTree::new(
            SimpleGraph::new(
                5,
                vec![(0, 1), (0, 3), (1, 2), (1, 3), (2, 3), (2, 4), (3, 4)],
            ),
            vec![2, 5, 2, 1, 5, 6, 1],
            vec![0, 2, 4],
        )),
        optimal_config: vec![1, 0, 1, 1, 0, 0, 1],
        optimal_value: serde_json::json!({"Valid": 6}),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/steiner_tree.rs"]
mod tests;
