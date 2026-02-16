//! Maximal Independent Set problem implementation.
//!
//! The Maximal Independent Set problem asks for an independent set that
//! cannot be extended by adding any other vertex.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::Graph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximalIS",
        module_path: module_path!(),
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
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
/// let problem = MaximalIS::new(graph, vec![1; 3]);
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

impl<G: Graph, W: Clone + Default> MaximalIS<G, W> {
    /// Create a Maximal Independent Set problem from a graph with given weights.
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
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MaximalIS";
    type Metric = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        if !self.is_maximal(config) {
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

impl<G, W> OptimizationProblem for MaximalIS<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

/// Check if a set is a maximal independent set.
///
/// # Panics
/// Panics if `selected.len() != graph.num_vertices()`.
pub fn is_maximal_independent_set<G: Graph>(graph: &G, selected: &[bool]) -> bool {
    assert_eq!(
        selected.len(),
        graph.num_vertices(),
        "selected length must match num_vertices"
    );

    // Check independence
    for (u, v) in graph.edges() {
        if selected[u] && selected[v] {
            return false;
        }
    }

    // Check maximality: no unselected vertex can be added
    for v in 0..graph.num_vertices() {
        if selected[v] {
            continue;
        }
        if graph.neighbors(v).iter().all(|&u| !selected[u]) {
            return false;
        }
    }

    true
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/maximal_is.rs"]
mod tests;
