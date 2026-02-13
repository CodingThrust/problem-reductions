//! Maximal Independent Set problem implementation.
//!
//! The Maximal Independent Set problem asks for an independent set that
//! cannot be extended by adding any other vertex.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximalIS",
        description: "Find maximum weight maximal independent set",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
        ],
    }
}

/// The Maximal Independent Set problem.
///
/// Given a graph G = (V, E), find an independent set S that is maximal,
/// meaning no vertex can be added to S while keeping it independent.
///
/// This is different from Maximum Independent Set - maximal means locally
/// optimal (cannot extend), while maximum means globally optimal (largest).
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaximalIS;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Path graph 0-1-2
/// let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_best(&problem);
///
/// // Maximal independent sets: {0, 2} or {1}
/// for sol in &solutions {
///     assert!(problem.evaluate(sol).is_valid());
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximalIS<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each vertex.
    weights: Vec<W>,
}

impl<W: Clone + Default> MaximalIS<SimpleGraph, W> {
    /// Create a new Maximal Independent Set problem with unit weights.
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        let graph = SimpleGraph::new(num_vertices, edges);
        let weights = vec![W::from(1); num_vertices];
        Self { graph, weights }
    }

    /// Create a new Maximal Independent Set problem with custom weights.
    pub fn with_weights(num_vertices: usize, edges: Vec<(usize, usize)>, weights: Vec<W>) -> Self {
        assert_eq!(weights.len(), num_vertices);
        let graph = SimpleGraph::new(num_vertices, edges);
        Self { graph, weights }
    }
}

impl<G: Graph, W: Clone + Default> MaximalIS<G, W> {
    /// Create a new Maximal Independent Set problem from a graph with custom weights.
    pub fn from_graph(graph: G, weights: Vec<W>) -> Self {
        assert_eq!(weights.len(), graph.num_vertices());
        Self { graph, weights }
    }

    /// Create a new Maximal Independent Set problem from a graph with unit weights.
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

    /// Check if a configuration is an independent set.
    fn is_independent(&self, config: &[usize]) -> bool {
        for (u, v) in self.graph.edges() {
            if config.get(u).copied().unwrap_or(0) == 1 && config.get(v).copied().unwrap_or(0) == 1
            {
                return false;
            }
        }
        true
    }

    /// Check if an independent set is maximal (cannot be extended).
    fn is_maximal(&self, config: &[usize]) -> bool {
        if !self.is_independent(config) {
            return false;
        }

        let n = self.graph.num_vertices();
        for v in 0..n {
            if config.get(v).copied().unwrap_or(0) == 1 {
                continue; // Already in set
            }

            // Check if v can be added
            let neighbors = self.graph.neighbors(v);
            let can_add = neighbors
                .iter()
                .all(|&u| config.get(u).copied().unwrap_or(0) == 0);

            if can_add {
                return false; // Set is not maximal
            }
        }

        true
    }
}

impl<G, W> Problem for MaximalIS<G, W>
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
    const NAME: &'static str = "MaximalIS";
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
        if !self.is_maximal(config) {
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

impl<G, W> OptimizationProblem for MaximalIS<G, W>
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
        Direction::Maximize
    }
}

/// Check if a set is a maximal independent set.
pub fn is_maximal_independent_set(
    num_vertices: usize,
    edges: &[(usize, usize)],
    selected: &[bool],
) -> bool {
    if selected.len() != num_vertices {
        return false;
    }

    // Build adjacency
    let mut adj: Vec<Vec<usize>> = vec![vec![]; num_vertices];
    for &(u, v) in edges {
        if u < num_vertices && v < num_vertices {
            adj[u].push(v);
            adj[v].push(u);
        }
    }

    // Check independence
    for &(u, v) in edges {
        if u < selected.len() && v < selected.len() && selected[u] && selected[v] {
            return false;
        }
    }

    // Check maximality
    for v in 0..num_vertices {
        if selected[v] {
            continue;
        }
        let can_add = adj[v].iter().all(|&u| !selected[u]);
        if can_add {
            return false;
        }
    }

    true
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/maximal_is.rs"]
mod tests;
