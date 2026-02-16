//! Independent Set problem implementation.
//!
//! The Independent Set problem asks for a maximum weight subset of vertices
//! such that no two vertices in the subset are adjacent.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::Graph;
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
/// * `G` - The graph type (e.g., `SimpleGraph`, `KingsSubgraph`, `UnitDiskGraph`)
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
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
/// let problem = MaximumIndependentSet::new(graph, vec![1; 3]);
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

impl<G: Graph, W: Clone + Default> MaximumIndependentSet<G, W> {
    /// Create an Independent Set problem from a graph with given weights.
    pub fn new(graph: G, weights: Vec<W>) -> Self {
        assert_eq!(
            weights.len(),
            graph.num_vertices(),
            "weights length must match graph num_vertices"
        );
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

    /// Check if the problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool
    where
        W: WeightElement,
    {
        !W::IS_UNIT
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
