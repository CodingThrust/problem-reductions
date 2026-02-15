//! Vertex Covering problem implementation.
//!
//! The Vertex Cover problem asks for a minimum weight subset of vertices
//! such that every edge has at least one endpoint in the subset.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumVertexCover",
        module_path: module_path!(),
        description: "Find minimum weight vertex cover in a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
        ],
    }
}

/// The Vertex Covering problem.
///
/// Given a graph G = (V, E) and weights w_v for each vertex,
/// find a subset S ⊆ V such that:
/// - Every edge has at least one endpoint in S (covering constraint)
/// - The total weight Σ_{v ∈ S} w_v is minimized
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumVertexCover;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Create a path graph 0-1-2
/// let problem = MinimumVertexCover::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_best(&problem);
///
/// // Minimum vertex cover is just vertex 1
/// assert!(solutions.contains(&vec![0, 1, 0]));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumVertexCover<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each vertex.
    weights: Vec<W>,
}

impl<W: Clone + Default> MinimumVertexCover<SimpleGraph, W> {
    /// Create a new Vertex Covering problem with unit weights.
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        let graph = SimpleGraph::new(num_vertices, edges);
        let weights = vec![W::from(1); num_vertices];
        Self { graph, weights }
    }

    /// Create a new Vertex Covering problem with custom weights.
    pub fn with_weights(num_vertices: usize, edges: Vec<(usize, usize)>, weights: Vec<W>) -> Self {
        assert_eq!(weights.len(), num_vertices);
        let graph = SimpleGraph::new(num_vertices, edges);
        Self { graph, weights }
    }
}

impl<G: Graph, W: Clone + Default> MinimumVertexCover<G, W> {
    /// Create a Vertex Covering problem from a graph with custom weights.
    pub fn from_graph(graph: G, weights: Vec<W>) -> Self {
        assert_eq!(weights.len(), graph.num_vertices());
        Self { graph, weights }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get a reference to the weights.
    pub fn weights(&self) -> &[W] {
        &self.weights
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

impl<G, W> Problem for MinimumVertexCover<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumVertexCover";
    type Metric = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        if !is_vertex_cover_config(&self.graph, config) {
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

impl<G, W> OptimizationProblem for MinimumVertexCover<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

/// Check if a configuration forms a valid vertex cover.
fn is_vertex_cover_config<G: Graph>(graph: &G, config: &[usize]) -> bool {
    for (u, v) in graph.edges() {
        let u_covered = config.get(u).copied().unwrap_or(0) == 1;
        let v_covered = config.get(v).copied().unwrap_or(0) == 1;
        if !u_covered && !v_covered {
            return false;
        }
    }
    true
}

/// Check if a set of vertices forms a vertex cover.
///
/// # Arguments
/// * `num_vertices` - Total number of vertices
/// * `edges` - List of edges as (u, v) pairs
/// * `selected` - Boolean slice indicating which vertices are selected
pub fn is_vertex_cover(num_vertices: usize, edges: &[(usize, usize)], selected: &[bool]) -> bool {
    if selected.len() != num_vertices {
        return false;
    }
    for &(u, v) in edges {
        if u < selected.len() && v < selected.len() && !selected[u] && !selected[v] {
            return false;
        }
    }
    true
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_vertex_cover.rs"]
mod tests;
