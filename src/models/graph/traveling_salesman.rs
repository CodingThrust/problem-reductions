//! Traveling Salesman problem implementation.
//!
//! The Traveling Salesman problem asks for a minimum-weight cycle
//! that visits every vertex exactly once.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::Graph;
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
/// * `G` - The graph type (e.g., `SimpleGraph`, `KingsSubgraph`)
/// * `W` - The weight type for edges (e.g., `i32`, `f64`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelingSalesman<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each edge (in edge index order).
    edge_weights: Vec<W>,
}

impl<G: Graph, W: Clone + Default> TravelingSalesman<G, W> {
    /// Create a TravelingSalesman problem from a graph with given edge weights.
    pub fn new(graph: G, edge_weights: Vec<W>) -> Self {
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

    /// Create a TravelingSalesman problem with unit weights.
    pub fn unit_weights(graph: G) -> Self
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

    /// Check if the problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool
    where
        W: WeightElement,
    {
        !W::IS_UNIT
    }

    /// Check if a configuration forms a valid Hamiltonian cycle.
    fn is_valid_hamiltonian_cycle(&self, config: &[usize]) -> bool {
        if config.len() != self.graph.num_edges() {
            return false;
        }
        let selected: Vec<bool> = config.iter().map(|&s| s == 1).collect();
        is_hamiltonian_cycle(&self.graph, &selected)
    }
}

impl<G, W> Problem for TravelingSalesman<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "TravelingSalesman";
    type Metric = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
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

    fn problem_size_names() -> &'static [&'static str] {
        &["num_vertices", "num_edges"]
    }
    fn problem_size_values(&self) -> Vec<usize> {
        vec![self.graph().num_vertices(), self.graph().num_edges()]
    }
}

impl<G, W> OptimizationProblem for TravelingSalesman<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

/// Check if a selection of edges forms a valid Hamiltonian cycle.
///
/// # Panics
/// Panics if `selected.len() != graph.num_edges()`.
pub fn is_hamiltonian_cycle<G: Graph>(graph: &G, selected: &[bool]) -> bool {
    assert_eq!(
        selected.len(),
        graph.num_edges(),
        "selected length must match num_edges"
    );

    let n = graph.num_vertices();
    let edges = graph.edges();
    let mut degree = vec![0usize; n];
    let mut selected_count = 0;
    let mut first_vertex = None;

    for (idx, &sel) in selected.iter().enumerate() {
        if sel {
            let (u, v) = edges[idx];
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
