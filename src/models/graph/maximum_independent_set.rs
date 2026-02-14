//! Independent Set problem implementation.
//!
//! The Independent Set problem asks for a maximum weight subset of vertices
//! such that no two vertices in the subset are adjacent.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumIndependentSet",
        module_path: module_path!(),
        description: "Find maximum weight independent set in a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
        ],
    }
}

/// The Independent Set problem.
///
/// Given a graph G = (V, E) and weights w_v for each vertex,
/// find a subset S ⊆ V such that:
/// - No two vertices in S are adjacent (independent set constraint)
/// - The total weight Σ_{v ∈ S} w_v is maximized
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`, `GridGraph`, `UnitDiskGraph`)
/// * `W` - The weight type (e.g., `i32`, `f64`, `One`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaximumIndependentSet;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Create a triangle graph (3 vertices, 3 edges)
/// let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_best(&problem);
///
/// // Maximum independent set in a triangle has size 1
/// assert!(solutions.iter().all(|s| s.iter().sum::<usize>() == 1));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximumIndependentSet<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each vertex.
    weights: Vec<W>,
}

impl<W: Clone + Default> MaximumIndependentSet<SimpleGraph, W> {
    /// Create a new Independent Set problem with unit weights.
    ///
    /// # Arguments
    /// * `num_vertices` - Number of vertices in the graph
    /// * `edges` - List of edges as (u, v) pairs
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        let graph = SimpleGraph::new(num_vertices, edges);
        let weights = vec![W::from(1); num_vertices];
        Self { graph, weights }
    }

    /// Create a new Independent Set problem with custom weights.
    pub fn with_weights(num_vertices: usize, edges: Vec<(usize, usize)>, weights: Vec<W>) -> Self {
        assert_eq!(
            weights.len(),
            num_vertices,
            "weights length must match num_vertices"
        );
        let graph = SimpleGraph::new(num_vertices, edges);
        Self { graph, weights }
    }
}

impl<G: Graph, W: Clone + Default> MaximumIndependentSet<G, W> {
    /// Create an Independent Set problem from an existing graph with custom weights.
    pub fn from_graph(graph: G, weights: Vec<W>) -> Self {
        assert_eq!(
            weights.len(),
            graph.num_vertices(),
            "weights length must match graph num_vertices"
        );
        Self { graph, weights }
    }

    /// Create an Independent Set problem from an existing graph with unit weights.
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

    /// Get the edges as a list of (u, v) pairs.
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
}

impl<G, W> Problem for MaximumIndependentSet<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MaximumIndependentSet";
    type Metric = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        if !is_independent_set_config(&self.graph, config) {
            return SolutionSize::Invalid;
        }
        let mut total = W::Sum::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].to_sum();
            }
        }
        SolutionSize::Valid(total)
    }
}

impl<G, W> OptimizationProblem for MaximumIndependentSet<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

/// Check if a configuration forms a valid independent set.
fn is_independent_set_config<G: Graph>(graph: &G, config: &[usize]) -> bool {
    for (u, v) in graph.edges() {
        if config.get(u).copied().unwrap_or(0) == 1 && config.get(v).copied().unwrap_or(0) == 1 {
            return false;
        }
    }
    true
}

/// Check if a set of vertices forms an independent set.
///
/// # Arguments
/// * `num_vertices` - Total number of vertices
/// * `edges` - List of edges as (u, v) pairs
/// * `selected` - Boolean slice indicating which vertices are selected
pub fn is_independent_set(
    num_vertices: usize,
    edges: &[(usize, usize)],
    selected: &[bool],
) -> bool {
    if selected.len() != num_vertices {
        return false;
    }
    for &(u, v) in edges {
        if u < selected.len() && v < selected.len() && selected[u] && selected[v] {
            return false;
        }
    }
    true
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/maximum_independent_set.rs"]
mod tests;
