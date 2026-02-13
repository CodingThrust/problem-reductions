//! Dominating Set problem implementation.
//!
//! The Dominating Set problem asks for a minimum weight subset of vertices
//! such that every vertex is either in the set or adjacent to a vertex in the set.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumDominatingSet",
        description: "Find minimum weight dominating set in a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
        ],
    }
}

/// The Dominating Set problem.
///
/// Given a graph G = (V, E) and weights w_v for each vertex,
/// find a subset D ⊆ V such that:
/// - Every vertex is either in D or adjacent to a vertex in D (domination)
/// - The total weight Σ_{v ∈ D} w_v is minimized
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumDominatingSet;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Star graph: center dominates all
/// let problem = MinimumDominatingSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (0, 2), (0, 3)]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_best(&problem);
///
/// // Minimum dominating set is just the center vertex
/// assert!(solutions.contains(&vec![1, 0, 0, 0]));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumDominatingSet<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each vertex.
    weights: Vec<W>,
}

impl<W: Clone + Default> MinimumDominatingSet<SimpleGraph, W> {
    /// Create a new Dominating Set problem with unit weights.
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        let graph = SimpleGraph::new(num_vertices, edges);
        let weights = vec![W::from(1); num_vertices];
        Self { graph, weights }
    }

    /// Create a new Dominating Set problem with custom weights.
    pub fn with_weights(num_vertices: usize, edges: Vec<(usize, usize)>, weights: Vec<W>) -> Self {
        assert_eq!(weights.len(), num_vertices);
        let graph = SimpleGraph::new(num_vertices, edges);
        Self { graph, weights }
    }
}

impl<G: Graph, W: Clone + Default> MinimumDominatingSet<G, W> {
    /// Create a Dominating Set problem from a graph with custom weights.
    pub fn from_graph(graph: G, weights: Vec<W>) -> Self {
        assert_eq!(weights.len(), graph.num_vertices());
        Self { graph, weights }
    }

    /// Create a Dominating Set problem from a graph with unit weights.
    pub fn from_graph_unit_weights(graph: G) -> Self
    where
        W: From<i32>,
    {
        let weights = vec![W::from(1); graph.num_vertices()];
        Self { graph, weights }
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

    /// Get edges as a list of (u, v) pairs.
    pub fn edges(&self) -> Vec<(usize, usize)> {
        self.graph.edges()
    }

    /// Check if two vertices are adjacent.
    pub fn has_edge(&self, u: usize, v: usize) -> bool {
        self.graph.has_edge(u, v)
    }

    /// Get neighbors of a vertex.
    pub fn neighbors(&self, v: usize) -> Vec<usize> {
        self.graph.neighbors(v)
    }

    /// Get the closed neighborhood `N[v] = {v} ∪ N(v)`.
    pub fn closed_neighborhood(&self, v: usize) -> HashSet<usize> {
        let mut neighborhood: HashSet<usize> = self.neighbors(v).into_iter().collect();
        neighborhood.insert(v);
        neighborhood
    }

    /// Get a reference to the weights vector.
    pub fn weights_ref(&self) -> &Vec<W> {
        &self.weights
    }

    /// Set new weights for the problem.
    pub fn set_weights(&mut self, weights: Vec<W>) {
        assert_eq!(weights.len(), self.graph.num_vertices());
        self.weights = weights;
    }

    /// Get the weights for the problem.
    pub fn weights(&self) -> Vec<W> {
        self.weights.clone()
    }

    /// Check if the problem has non-uniform weights.
    pub fn is_weighted(&self) -> bool
    where
        W: PartialEq,
    {
        if self.weights.is_empty() {
            return false;
        }
        let first = &self.weights[0];
        !self.weights.iter().all(|w| w == first)
    }

    /// Check if a set of vertices is a dominating set.
    fn is_dominating(&self, config: &[usize]) -> bool {
        let n = self.graph.num_vertices();
        let mut dominated = vec![false; n];

        for (v, &selected) in config.iter().enumerate() {
            if selected == 1 {
                // v dominates itself
                dominated[v] = true;
                // v dominates all its neighbors
                for neighbor in self.neighbors(v) {
                    if neighbor < n {
                        dominated[neighbor] = true;
                    }
                }
            }
        }

        dominated.iter().all(|&d| d)
    }
}

impl<G, W> Problem for MinimumDominatingSet<G, W>
where
    G: Graph,
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static,
{
    const NAME: &'static str = "MinimumDominatingSet";
    type Metric = SolutionSize<W>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", crate::variant::short_type_name::<G>()),
            ("weight", crate::variant::short_type_name::<W>()),
        ]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W> {
        if !self.is_dominating(config) {
            return SolutionSize::Invalid;
        }
        let mut total = W::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].clone();
            }
        }
        SolutionSize::Valid(total)
    }
}

impl<G, W> OptimizationProblem for MinimumDominatingSet<G, W>
where
    G: Graph,
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static,
{
    type Value = W;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

/// Check if a set of vertices is a dominating set.
pub fn is_dominating_set(num_vertices: usize, edges: &[(usize, usize)], selected: &[bool]) -> bool {
    if selected.len() != num_vertices {
        return false;
    }

    // Build adjacency list
    let mut adj: Vec<HashSet<usize>> = vec![HashSet::new(); num_vertices];
    for &(u, v) in edges {
        if u < num_vertices && v < num_vertices {
            adj[u].insert(v);
            adj[v].insert(u);
        }
    }

    // Check each vertex is dominated
    for v in 0..num_vertices {
        if selected[v] {
            continue; // v dominates itself
        }
        // Check if any neighbor of v is selected
        let dominated = adj[v].iter().any(|&u| selected[u]);
        if !dominated {
            return false;
        }
    }

    true
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_dominating_set.rs"]
mod tests;
