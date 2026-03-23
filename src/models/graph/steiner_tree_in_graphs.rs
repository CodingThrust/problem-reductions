//! Steiner Tree in Graphs problem implementation.
//!
//! The Steiner Tree problem asks for a minimum-weight subtree of a graph
//! that connects all terminal vertices.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, One, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SteinerTreeInGraphs",
        display_name: "Steiner Tree in Graphs",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i32", &["One", "i32"]),
        ],
        module_path: module_path!(),
        description: "Find minimum weight subtree connecting all terminal vertices",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "terminals", type_name: "Vec<usize>", description: "Required terminal vertices R ⊆ V" },
            FieldInfo { name: "edge_weights", type_name: "Vec<W>", description: "Edge weights w: E -> R" },
        ],
    }
}

/// The Steiner Tree in Graphs problem.
///
/// Given a weighted graph G = (V, E) with edge weights w_e and a
/// subset R ⊆ V of required terminal vertices, find a subtree T of G
/// that includes all vertices of R and minimizes the total edge weight
/// Σ_{e ∈ T} w(e).
///
/// # Representation
///
/// Each edge is assigned a binary variable:
/// - 0: edge is not in the tree
/// - 1: edge is in the tree
///
/// A valid Steiner tree requires:
/// - All terminal vertices are connected through selected edges
/// - Selected edges form a connected subgraph (optimally a tree)
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
/// * `W` - The weight type for edges (e.g., `i32`, `f64`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::SteinerTreeInGraphs;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Path graph 0-1-2-3, terminals {0, 3}
/// let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
/// let problem = SteinerTreeInGraphs::new(graph, vec![0, 3], vec![1, 1, 1]);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_best(&problem).unwrap();
/// // Optimal: select all 3 edges (the only path from 0 to 3)
/// assert_eq!(solution, vec![1, 1, 1]);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteinerTreeInGraphs<G, W> {
    /// The underlying graph.
    graph: G,
    /// Required terminal vertices.
    terminals: Vec<usize>,
    /// Weights for each edge (in edge index order).
    edge_weights: Vec<W>,
}

impl<G: Graph, W: Clone + Default> SteinerTreeInGraphs<G, W> {
    /// Create a SteinerTreeInGraphs problem from a graph, terminals, and edge weights.
    ///
    /// # Panics
    /// Panics if `edge_weights.len() != graph.num_edges()` or any terminal index is out of bounds.
    pub fn new(graph: G, terminals: Vec<usize>, edge_weights: Vec<W>) -> Self {
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        for &t in &terminals {
            assert!(
                t < graph.num_vertices(),
                "terminal vertex {} out of bounds (num_vertices = {})",
                t,
                graph.num_vertices()
            );
        }
        Self {
            graph,
            terminals,
            edge_weights,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the terminal vertices.
    pub fn terminals(&self) -> &[usize] {
        &self.terminals
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

    /// Check if a configuration is a valid Steiner tree.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        if config.len() != self.graph.num_edges() {
            return false;
        }
        let selected: Vec<bool> = config.iter().map(|&s| s == 1).collect();
        is_steiner_tree(&self.graph, &self.terminals, &selected)
    }
}

impl<G: Graph, W: WeightElement> SteinerTreeInGraphs<G, W> {
    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph().num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph().num_edges()
    }

    /// Get the number of terminal vertices.
    pub fn num_terminals(&self) -> usize {
        self.terminals.len()
    }
}

impl<G, W> Problem for SteinerTreeInGraphs<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "SteinerTreeInGraphs";
    type Value = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        if config.len() != self.graph.num_edges() {
            return SolutionSize::Invalid;
        }
        let selected: Vec<bool> = config.iter().map(|&s| s == 1).collect();
        if !is_steiner_tree(&self.graph, &self.terminals, &selected) {
            return SolutionSize::Invalid;
        }
        let mut total = W::Sum::zero();
        for (idx, &sel) in config.iter().enumerate() {
            if sel == 1 {
                if let Some(w) = self.edge_weights.get(idx) {
                    total += w.to_sum();
                }
            }
        }
        SolutionSize::Valid(total)
    }
}

impl<G, W> OptimizationProblem for SteinerTreeInGraphs<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    type Objective = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

/// Check if a selection of edges forms a valid Steiner tree (connected subgraph spanning all terminals).
///
/// A valid Steiner tree requires:
/// 1. All terminal vertices are reachable from each other through selected edges.
/// 2. The selected edges form a connected subgraph that includes all terminals.
///
/// Note: The optimal solution is always a tree, but we accept any connected subgraph
/// spanning all terminals (the brute-force solver will find the minimum-weight one).
///
/// # Panics
/// Panics if `selected.len() != graph.num_edges()`.
pub(crate) fn is_steiner_tree<G: Graph>(graph: &G, terminals: &[usize], selected: &[bool]) -> bool {
    assert_eq!(
        selected.len(),
        graph.num_edges(),
        "selected length must match num_edges"
    );

    // If no terminals, any selection is trivially valid (including empty)
    if terminals.is_empty() {
        return true;
    }

    // If only one terminal, it's valid as long as that terminal exists
    // (no edges needed to connect a single vertex)
    if terminals.len() == 1 {
        return true;
    }

    // Build adjacency list from selected edges
    let n = graph.num_vertices();
    let edges = graph.edges();
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];

    let mut has_any_edge = false;
    for (idx, &sel) in selected.iter().enumerate() {
        if sel {
            let (u, v) = edges[idx];
            adj[u].push(v);
            adj[v].push(u);
            has_any_edge = true;
        }
    }

    if !has_any_edge {
        return false;
    }

    // BFS from the first terminal to check connectivity of all terminals
    let start = terminals[0];
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();
    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        for &neighbor in &adj[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    // All terminals must be reachable
    terminals.iter().all(|&t| visited[t])
}

crate::declare_variants! {
    default opt SteinerTreeInGraphs<SimpleGraph, i32> => "2^num_terminals * num_vertices^3",
    opt SteinerTreeInGraphs<SimpleGraph, One> => "2^num_terminals * num_vertices^3",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "steiner_tree_in_graphs_simplegraph_i32",
        instance: Box::new(SteinerTreeInGraphs::new(
            SimpleGraph::new(
                6,
                vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 5), (3, 4), (4, 5)],
            ),
            vec![0, 3, 5],
            vec![3, 2, 4, 1, 2, 3, 1],
        )),
        // Optimal: edges {0,2}(w=2), {2,3}(w=1), {2,5}(w=2) = weight 5
        optimal_config: vec![0, 1, 0, 1, 1, 0, 0],
        optimal_value: serde_json::json!({"Valid": 5}),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/steiner_tree_in_graphs.rs"]
mod tests;
