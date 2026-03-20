//! Minimum Graph Bandwidth problem implementation.
//!
//! The Graph Bandwidth problem asks for a vertex ordering that minimizes the
//! maximum span of any edge.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumGraphBandwidth",
        display_name: "Minimum Graph Bandwidth",
        aliases: &["GraphBandwidth", "Bandwidth"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Find a vertex ordering minimizing the maximum edge span",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The undirected graph G=(V,E)" },
        ],
    }
}

/// The Minimum Graph Bandwidth problem.
///
/// Given an undirected graph `G = (V, E)`, find a bijection
/// `f: V -> {0, 1, ..., |V|-1}` minimizing
/// `max_{(u,v) in E} |f(u) - f(v)|`.
///
/// # Representation
///
/// Each vertex is assigned one variable indicating its position on the line.
/// Variable `i` takes a value in `{0, 1, ..., n-1}`, and a valid configuration
/// must be a permutation of those positions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct MinimumGraphBandwidth<G> {
    /// The underlying graph.
    graph: G,
}

impl<G: Graph> MinimumGraphBandwidth<G> {
    /// Create a new Minimum Graph Bandwidth problem.
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

    /// Check whether a configuration is a permutation of `{0, ..., n-1}`.
    fn is_valid_permutation(&self, config: &[usize]) -> bool {
        let n = self.graph.num_vertices();
        if config.len() != n {
            return false;
        }

        let mut seen = vec![false; n];
        for &position in config {
            if position >= n || seen[position] {
                return false;
            }
            seen[position] = true;
        }
        true
    }

    /// Compute the maximum edge span for a vertex ordering.
    ///
    /// Returns `None` if `config` is not a valid permutation.
    pub fn max_edge_span(&self, config: &[usize]) -> Option<usize> {
        if !self.is_valid_permutation(config) {
            return None;
        }

        let mut bandwidth = 0usize;
        for (u, v) in self.graph.edges() {
            bandwidth = bandwidth.max(config[u].abs_diff(config[v]));
        }
        Some(bandwidth)
    }

    /// Check if a configuration is a valid ordering.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.is_valid_permutation(config)
    }
}

impl<G> Problem for MinimumGraphBandwidth<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumGraphBandwidth";
    type Metric = SolutionSize<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.graph.num_vertices();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<usize> {
        match self.max_edge_span(config) {
            Some(bandwidth) => SolutionSize::Valid(bandwidth),
            None => SolutionSize::Invalid,
        }
    }
}

impl<G> OptimizationProblem for MinimumGraphBandwidth<G>
where
    G: Graph + crate::variant::VariantParam,
{
    type Value = usize;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

crate::declare_variants! {
    default opt MinimumGraphBandwidth<SimpleGraph> => "4.473^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_graph_bandwidth",
        instance: Box::new(MinimumGraphBandwidth::new(SimpleGraph::new(
            6,
            vec![(0, 1), (1, 2), (0, 3), (1, 4), (2, 5), (3, 4), (4, 5)],
        ))),
        optimal_config: vec![0, 2, 4, 1, 3, 5],
        optimal_value: serde_json::json!({"Valid": 2}),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_graph_bandwidth.rs"]
mod tests;
