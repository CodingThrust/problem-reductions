//! Dominating Set problem implementation.
//!
//! The Dominating Set problem asks for a minimum weight subset of vertices
//! such that every vertex is either in the set or adjacent to a vertex in the set.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::Graph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumDominatingSet",
        module_path: module_path!(),
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
/// let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]);
/// let problem = MinimumDominatingSet::new(graph, vec![1; 4]);
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

impl<G: Graph, W: Clone + Default> MinimumDominatingSet<G, W> {
    /// Create a Dominating Set problem from a graph with given weights.
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

    /// Get a reference to the weights slice.
    pub fn weights(&self) -> &[W] {
        &self.weights
    }

    /// Check if a configuration is a valid dominating set.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.is_dominating(config)
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
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumDominatingSet";
    type Metric = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        if !self.is_dominating(config) {
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

    fn problem_size_names() -> &'static [&'static str] {
        &["num_vertices", "num_edges"]
    }
    fn problem_size_values(&self) -> Vec<usize> {
        vec![self.graph().num_vertices(), self.graph().num_edges()]
    }
}

impl<G, W> OptimizationProblem for MinimumDominatingSet<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

/// Check if a set of vertices is a dominating set.
///
/// # Panics
/// Panics if `selected.len() != graph.num_vertices()`.
#[cfg(test)]
pub(crate) fn is_dominating_set<G: Graph>(graph: &G, selected: &[bool]) -> bool {
    assert_eq!(
        selected.len(),
        graph.num_vertices(),
        "selected length must match num_vertices"
    );

    // Check each vertex is dominated
    for v in 0..graph.num_vertices() {
        if selected[v] {
            continue; // v dominates itself
        }
        // Check if any neighbor of v is selected
        if !graph.neighbors(v).iter().any(|&u| selected[u]) {
            return false;
        }
    }

    true
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_dominating_set.rs"]
mod tests;
