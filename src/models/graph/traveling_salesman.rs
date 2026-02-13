//! Traveling Salesman problem implementation.
//!
//! The Traveling Salesman problem asks for a minimum-weight cycle
//! that visits every vertex exactly once.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "TravelingSalesman",
        module_path: module_path!(),
        description: "Find minimum weight Hamiltonian cycle in a graph (Traveling Salesman Problem)",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "edge_weights", type_name: "Vec<W>", description: "Edge weights w: E -> R" },
        ],
    }
}

/// The Traveling Salesman problem.
///
/// Given a weighted graph G = (V, E) with edge weights w_e,
/// find a cycle that visits every vertex exactly once and
/// minimizes the total edge weight.
///
/// # Representation
///
/// Each edge is assigned a binary variable:
/// - 0: edge is not in the cycle
/// - 1: edge is in the cycle
///
/// A valid Hamiltonian cycle requires:
/// - Exactly 2 selected edges incident to each vertex (degree constraint)
/// - Selected edges form a single connected cycle (no subtours)
/// - Exactly |V| edges are selected
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`, `GridGraph`)
/// * `W` - The weight type for edges (e.g., `i32`, `f64`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelingSalesman<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each edge (in edge index order).
    edge_weights: Vec<W>,
}

impl<W: Clone + Default> TravelingSalesman<SimpleGraph, W> {
    /// Create a new TravelingSalesman problem.
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize, W)>) -> Self {
        let mut edge_list = Vec::new();
        let mut edge_weights = Vec::new();
        for (u, v, w) in edges {
            edge_list.push((u, v));
            edge_weights.push(w);
        }
        let graph = SimpleGraph::new(num_vertices, edge_list);
        Self {
            graph,
            edge_weights,
        }
    }

    /// Create a TravelingSalesman problem with unit weights.
    pub fn unweighted(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        let edge_weights = vec![W::from(1); edges.len()];
        let graph = SimpleGraph::new(num_vertices, edges);
        Self {
            graph,
            edge_weights,
        }
    }
}

impl<G: Graph, W: Clone + Default> TravelingSalesman<G, W> {
    /// Create a TravelingSalesman problem from a graph with given edge weights.
    pub fn from_graph(graph: G, edge_weights: Vec<W>) -> Self {
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        Self {
            graph,
            edge_weights,
        }
    }

    /// Create a TravelingSalesman problem from a graph with unit weights.
    pub fn from_graph_unit_weights(graph: G) -> Self
    where
        W: From<i32>,
    {
        let edge_weights = vec![W::from(1); graph.num_edges()];
        Self {
            graph,
            edge_weights,
        }
    }

    /// Get a reference to the underlying graph.
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

    /// Get all edges with their weights.
    pub fn edges(&self) -> Vec<(usize, usize, W)> {
        self.graph
            .edges()
            .into_iter()
            .zip(self.edge_weights.iter().cloned())
            .map(|((u, v), w)| (u, v, w))
            .collect()
    }

    /// Set new weights for the problem.
    pub fn set_weights(&mut self, weights: Vec<W>) {
        assert_eq!(weights.len(), self.graph.num_edges());
        self.edge_weights = weights;
    }

    /// Get the weights for the problem.
    pub fn weights(&self) -> Vec<W> {
        self.edge_weights.clone()
    }

    /// Check if the problem has non-uniform weights.
    pub fn is_weighted(&self) -> bool
    where
        W: PartialEq,
    {
        if self.edge_weights.is_empty() {
            return false;
        }
        let first = &self.edge_weights[0];
        !self.edge_weights.iter().all(|w| w == first)
    }

    /// Check if a configuration forms a valid Hamiltonian cycle.
    fn is_valid_hamiltonian_cycle(&self, config: &[usize]) -> bool {
        let edges = self.graph.edges();
        let selected: Vec<bool> = config.iter().map(|&s| s == 1).collect();
        is_hamiltonian_cycle(self.graph.num_vertices(), &edges, &selected)
    }
}

impl<G, W> Problem for TravelingSalesman<G, W>
where
    G: Graph,
    W: WeightElement,
{
    const NAME: &'static str = "TravelingSalesman";
    type Metric = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", G::NAME),
            ("weight", crate::variant::short_type_name::<W>()),
        ]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        if !self.is_valid_hamiltonian_cycle(config) {
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

impl<G, W> OptimizationProblem for TravelingSalesman<G, W>
where
    G: Graph,
    W: WeightElement,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

/// Check if a selection of edges forms a valid Hamiltonian cycle.
pub fn is_hamiltonian_cycle(
    num_vertices: usize,
    edges: &[(usize, usize)],
    selected: &[bool],
) -> bool {
    if selected.len() != edges.len() {
        return false;
    }

    let n = num_vertices;
    let mut degree = vec![0usize; n];
    let mut selected_count = 0;
    let mut first_vertex = None;

    for (idx, &sel) in selected.iter().enumerate() {
        if sel {
            let (u, v) = edges[idx];
            if u >= n || v >= n {
                return false;
            }
            degree[u] += 1;
            degree[v] += 1;
            selected_count += 1;
            if first_vertex.is_none() {
                first_vertex = Some(u);
            }
        }
    }

    if selected_count != n {
        return false;
    }

    if degree.iter().any(|&d| d != 2) {
        return false;
    }

    let first = match first_vertex {
        Some(v) => v,
        None => return false,
    };

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    for (idx, &sel) in selected.iter().enumerate() {
        if sel {
            let (u, v) = edges[idx];
            adj[u].push(v);
            adj[v].push(u);
        }
    }

    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();
    visited[first] = true;
    queue.push_back(first);
    let mut visit_count = 1;

    while let Some(node) = queue.pop_front() {
        for &neighbor in &adj[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                visit_count += 1;
                queue.push_back(neighbor);
            }
        }
    }

    visit_count == n
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/traveling_salesman.rs"]
mod tests;
