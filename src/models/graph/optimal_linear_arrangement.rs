//! Optimal Linear Arrangement problem implementation.
//!
//! The Optimal Linear Arrangement problem asks for a permutation of vertices
//! on a line that minimizes the total edge length (sum of |f(u) - f(v)| for all edges).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "OptimalLinearArrangement",
        module_path: module_path!(),
        description: "Find vertex ordering on a line minimizing total edge length",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
        ],
    }
}

/// The Optimal Linear Arrangement problem.
///
/// Given a graph G = (V, E), find a bijection f: V -> {0, 1, ..., |V|-1}
/// that minimizes the total edge length: sum_{(u,v) in E} |f(u) - f(v)|.
///
/// # Representation
///
/// Each vertex is assigned a variable representing its position in the arrangement.
/// Variable i takes a value in {0, 1, ..., n-1}, and a valid configuration must be
/// a permutation (all positions are distinct).
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::OptimalLinearArrangement;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Path graph: 0-1-2
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
/// let problem = OptimalLinearArrangement::new(graph);
///
/// let solver = BruteForce::new();
/// let best = solver.find_best(&problem).unwrap();
/// // Optimal: identity arrangement, cost = 2
/// assert_eq!(problem.evaluate(&best).unwrap(), 2);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimalLinearArrangement<G> {
    /// The underlying graph.
    graph: G,
}

impl<G: Graph> OptimalLinearArrangement<G> {
    /// Create an Optimal Linear Arrangement problem from a graph.
    pub fn new(graph: G) -> Self {
        Self { graph }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check if a configuration is a valid permutation.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.is_valid_permutation(config)
    }

    /// Check if a configuration forms a valid permutation of {0, ..., n-1}.
    fn is_valid_permutation(&self, config: &[usize]) -> bool {
        let n = self.graph.num_vertices();
        if config.len() != n {
            return false;
        }
        let mut seen = vec![false; n];
        for &pos in config {
            if pos >= n || seen[pos] {
                return false;
            }
            seen[pos] = true;
        }
        true
    }

    /// Compute the total edge length for a given arrangement.
    ///
    /// Returns `None` if the configuration is not a valid permutation.
    pub fn total_edge_length(&self, config: &[usize]) -> Option<usize> {
        if !self.is_valid_permutation(config) {
            return None;
        }
        let mut total = 0usize;
        for (u, v) in self.graph.edges() {
            let fu = config[u];
            let fv = config[v];
            total += fu.abs_diff(fv);
        }
        Some(total)
    }
}

impl<G> Problem for OptimalLinearArrangement<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "OptimalLinearArrangement";
    type Metric = SolutionSize<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.graph.num_vertices();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<usize> {
        match self.total_edge_length(config) {
            Some(cost) => SolutionSize::Valid(cost),
            None => SolutionSize::Invalid,
        }
    }
}

impl<G> OptimizationProblem for OptimalLinearArrangement<G>
where
    G: Graph + crate::variant::VariantParam,
{
    type Value = usize;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

crate::declare_variants! {
    OptimalLinearArrangement<SimpleGraph> => "2^num_vertices",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/optimal_linear_arrangement.rs"]
mod tests;
