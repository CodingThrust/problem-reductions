//! Optimal Linear Arrangement problem implementation.
//!
//! The Optimal Linear Arrangement problem asks whether there exists a one-to-one
//! function f: V -> {0, 1, ..., |V|-1} such that the total edge length
//! sum_{{u,v} in E} |f(u) - f(v)| is at most K.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "OptimalLinearArrangement",
        module_path: module_path!(),
        description: "Find a vertex ordering on a line with total edge length at most K",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The undirected graph G=(V,E)" },
            FieldInfo { name: "bound", type_name: "usize", description: "Upper bound K on total edge length" },
        ],
    }
}

/// The Optimal Linear Arrangement problem.
///
/// Given an undirected graph G = (V, E) and a non-negative integer K,
/// determine whether there exists a one-to-one function f: V -> {0, 1, ..., |V|-1}
/// such that sum_{{u,v} in E} |f(u) - f(v)| <= K.
///
/// This is the decision (satisfaction) version of the problem, following the
/// Garey & Johnson formulation (GT42).
///
/// # Representation
///
/// Each vertex is assigned a variable representing its position in the arrangement.
/// Variable i takes a value in {0, 1, ..., n-1}, and a valid configuration must be
/// a permutation (all positions are distinct) with total edge length at most K.
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
/// // Path graph: 0-1-2-3 with bound 3
/// let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
/// let problem = OptimalLinearArrangement::new(graph, 3);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct OptimalLinearArrangement<G> {
    /// The underlying graph.
    graph: G,
    /// Upper bound K on total edge length.
    bound: usize,
}

impl<G: Graph> OptimalLinearArrangement<G> {
    /// Create a new Optimal Linear Arrangement problem.
    ///
    /// # Arguments
    /// * `graph` - The undirected graph G = (V, E)
    /// * `bound` - The upper bound K on total edge length
    pub fn new(graph: G, bound: usize) -> Self {
        Self { graph, bound }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the bound K.
    pub fn bound(&self) -> usize {
        self.bound
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check if a configuration is a valid permutation with total edge length at most K.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        match self.total_edge_length(config) {
            Some(length) => length <= self.bound,
            None => false,
        }
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
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.graph.num_vertices();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.is_valid_solution(config)
    }
}

impl<G: Graph + crate::variant::VariantParam> SatisfactionProblem for OptimalLinearArrangement<G> {}

crate::declare_variants! {
    default OptimalLinearArrangement<SimpleGraph> => "2^num_vertices",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/optimal_linear_arrangement.rs"]
mod tests;
