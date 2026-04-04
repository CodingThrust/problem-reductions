//! Minimum Graph Bandwidth problem implementation.
//!
//! The Minimum Graph Bandwidth problem asks for a bijection
//! f: V -> {0, 1, ..., |V|-1} that minimizes the maximum edge stretch
//! max_{(u,v) in E} |f(u) - f(v)|.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumGraphBandwidth",
        display_name: "Minimum Graph Bandwidth",
        aliases: &["MGB"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Find a vertex ordering minimizing the maximum edge stretch",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The undirected graph G=(V,E)" },
        ],
    }
}

/// The Minimum Graph Bandwidth problem.
///
/// Given an undirected graph G = (V, E), find a bijection f: V -> {0, 1, ..., |V|-1}
/// that minimizes the bandwidth max_{(u,v) in E} |f(u) - f(v)|.
///
/// # Representation
///
/// Each vertex is assigned a variable representing its position in the arrangement.
/// Variable i takes a value in {0, 1, ..., n-1}, and a valid configuration must be
/// a permutation (all positions are distinct). The objective is to minimize the
/// maximum edge stretch.
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumGraphBandwidth;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Star graph S4: center 0 connected to 1, 2, 3
/// let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]);
/// let problem = MinimumGraphBandwidth::new(graph);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct MinimumGraphBandwidth<G> {
    /// The underlying graph.
    graph: G,
}

impl<G: Graph> MinimumGraphBandwidth<G> {
    /// Create a new Minimum Graph Bandwidth problem.
    ///
    /// # Arguments
    /// * `graph` - The undirected graph G = (V, E)
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

    /// Compute the bandwidth (maximum edge stretch) for a given arrangement.
    ///
    /// Returns `None` if the configuration is not a valid permutation.
    pub fn bandwidth(&self, config: &[usize]) -> Option<usize> {
        if !self.is_valid_permutation(config) {
            return None;
        }
        let mut max_stretch = 0usize;
        for (u, v) in self.graph.edges() {
            let stretch = config[u].abs_diff(config[v]);
            max_stretch = max_stretch.max(stretch);
        }
        Some(max_stretch)
    }
}

impl<G> Problem for MinimumGraphBandwidth<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumGraphBandwidth";
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.graph.num_vertices();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        match self.bandwidth(config) {
            Some(bw) => Min(Some(bw)),
            None => Min(None),
        }
    }
}

crate::declare_variants! {
    default MinimumGraphBandwidth<SimpleGraph> => "factorial(num_vertices)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    use crate::topology::SimpleGraph;
    // Star graph S4: center 0 connected to 1, 2, 3
    // Config [1,0,2,3]: f(0)=1, f(1)=0, f(2)=2, f(3)=3
    // Bandwidth = max(|1-0|, |1-2|, |1-3|) = max(1, 1, 2) = 2
    // Optimal bandwidth for S4 is 2 (center must be adjacent to all leaves,
    // placing center at position 1 achieves max stretch 2).
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_graph_bandwidth",
        instance: Box::new(MinimumGraphBandwidth::new(SimpleGraph::new(
            4,
            vec![(0, 1), (0, 2), (0, 3)],
        ))),
        optimal_config: vec![1, 0, 2, 3],
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_graph_bandwidth.rs"]
mod tests;
