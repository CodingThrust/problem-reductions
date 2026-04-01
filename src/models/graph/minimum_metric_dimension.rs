//! Minimum Metric Dimension problem implementation.
//!
//! Given a graph G = (V, E), find a minimum resolving set — a smallest subset
//! V' ⊆ V such that for all distinct u, v ∈ V, there exists w ∈ V' with
//! d(u, w) ≠ d(v, w), where d denotes shortest-path distance.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumMetricDimension",
        display_name: "Minimum Metric Dimension",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Find minimum resolving set of a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
        ],
    }
}

/// Compute BFS shortest-path distances from a single source vertex.
///
/// Returns a vector where `dist[v]` is the shortest-path distance from
/// `source` to `v`, or `usize::MAX` if `v` is unreachable.
pub fn bfs_distances<G: Graph>(graph: &G, source: usize) -> Vec<usize> {
    let n = graph.num_vertices();
    let mut dist = vec![usize::MAX; n];
    dist[source] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(source);
    while let Some(u) = queue.pop_front() {
        for v in graph.neighbors(u) {
            if dist[v] == usize::MAX {
                dist[v] = dist[u] + 1;
                queue.push_back(v);
            }
        }
    }
    dist
}

/// The Minimum Metric Dimension problem.
///
/// Given a graph G = (V, E), find a minimum-size resolving set V' ⊆ V such
/// that for every pair of distinct vertices u, v ∈ V, there exists at least
/// one vertex w ∈ V' with d(u, w) ≠ d(v, w).
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumMetricDimension;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // House graph: vertices 0–4
/// let graph = SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]);
/// let problem = MinimumMetricDimension::new(graph);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem).unwrap();
/// let value = problem.evaluate(&solution);
/// assert!(value.is_valid());
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MinimumMetricDimension<G> {
    /// The underlying graph.
    graph: G,
    /// Precomputed all-pairs shortest-path distances.
    #[serde(skip)]
    dist_matrix: Vec<Vec<usize>>,
}

impl<'de, G: Graph + Deserialize<'de>> Deserialize<'de> for MinimumMetricDimension<G> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper<G> {
            graph: G,
        }
        let helper = Helper::<G>::deserialize(deserializer)?;
        Ok(Self::new(helper.graph))
    }
}

impl<G: Graph> MinimumMetricDimension<G> {
    /// Create a MinimumMetricDimension problem from a graph.
    pub fn new(graph: G) -> Self {
        let n = graph.num_vertices();
        let dist_matrix = (0..n).map(|v| bfs_distances(&graph, v)).collect();
        Self { graph, dist_matrix }
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

    /// Check whether a configuration (binary vertex selection) forms a resolving set.
    ///
    /// A set S ⊆ V is resolving if for every pair of distinct vertices u, v ∈ V,
    /// there exists some w ∈ S such that d(u, w) ≠ d(v, w).
    pub fn is_resolving(&self, config: &[usize]) -> bool {
        let n = self.graph.num_vertices();
        let selected: Vec<usize> = (0..n).filter(|&i| config[i] == 1).collect();
        if selected.is_empty() {
            return false;
        }

        // Check that all pairs of distinct vertices have different distance vectors
        // using precomputed all-pairs distances
        for u in 0..n {
            for v in (u + 1)..n {
                let all_same = selected
                    .iter()
                    .all(|&w| self.dist_matrix[w][u] == self.dist_matrix[w][v]);
                if all_same {
                    return false;
                }
            }
        }

        true
    }
}

impl<G> Problem for MinimumMetricDimension<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumMetricDimension";
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        if !self.is_resolving(config) {
            return Min(None);
        }
        let count = config.iter().filter(|&&x| x == 1).count();
        Min(Some(count))
    }
}

crate::declare_variants! {
    default MinimumMetricDimension<SimpleGraph> => "2^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_metric_dimension_simplegraph",
        instance: Box::new(MinimumMetricDimension::new(SimpleGraph::new(
            5,
            vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)],
        ))),
        optimal_config: vec![1, 1, 0, 0, 0],
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_metric_dimension.rs"]
mod tests;
