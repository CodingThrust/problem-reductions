//! Maximum Edge-Weighted k-Clique problem implementation.
//!
//! Given a simple undirected graph G = (V, E), edge weights w: E -> R, and an
//! integer k with 0 <= k <= |V|, find a subset S ⊆ V with |S| = k such that
//! every two distinct vertices in S are adjacent in G and the total weight of
//! the induced clique edges is maximized:
//!
//! maximize  Σ_{{u,v} ⊆ S, {u,v} ∈ E} w_{uv}.
//!
//! Edge weights may be positive, zero, or negative. Cliques of size 0 and 1
//! are allowed when `k` takes those values, with objective value 0 because no
//! pair of selected vertices is induced.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Max, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumEdgeWeightedKClique",
        display_name: "Maximum Edge-Weighted k-Clique",
        aliases: &[],
        dimensions: &[VariantDimension::new("weight", "i32", &["i32", "f64"])],
        module_path: module_path!(),
        description: "Select exactly k pairwise-adjacent vertices maximizing the total weight of induced clique edges",
        fields: &[
            FieldInfo { name: "graph", type_name: "SimpleGraph", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "edge_weights", type_name: "Vec<W>", description: "Edge weights in graph edge order" },
            FieldInfo { name: "k", type_name: "usize", description: "Required clique size" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "MaximumEdgeWeightedKClique",
        fields: &["num_vertices", "num_edges"],
    }
}

/// The Maximum Edge-Weighted k-Clique problem.
///
/// Given a simple undirected graph `G = (V, E)`, edge weights
/// `w: E -> R`, and an integer `k` with `0 <= k <= |V|`, find a subset
/// `S ⊆ V` with `|S| = k` such that every two distinct vertices in `S`
/// are adjacent in `G` and the sum of induced edge weights is maximized.
///
/// # Type Parameters
///
/// * `W` - Edge weight type (e.g., `i32`, `f64`). The graph is fixed to
///   [`SimpleGraph`] in the current registered variants.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaximumEdgeWeightedKClique;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::types::Max;
/// use problemreductions::{BruteForce, Problem, Solver};
///
/// // Graph from issue #1020: 4 vertices, triangles {0,1,2} and {0,1,3}.
/// let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]);
/// let weights = vec![5_i32, 4, -1, 1, 0];
/// let problem = MaximumEdgeWeightedKClique::new(graph, weights, 3);
/// assert_eq!(BruteForce::new().solve(&problem), Max(Some(8)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximumEdgeWeightedKClique<W: WeightElement> {
    /// The underlying graph.
    graph: SimpleGraph,
    /// Edge weights, in the graph's edge iteration order.
    edge_weights: Vec<W>,
    /// Required clique size.
    k: usize,
}

impl<W: WeightElement> MaximumEdgeWeightedKClique<W> {
    /// Create a new MaximumEdgeWeightedKClique instance.
    ///
    /// # Panics
    /// Panics if `edge_weights.len()` does not match `graph.num_edges()`, or
    /// if `k > graph.num_vertices()`.
    pub fn new(graph: SimpleGraph, edge_weights: Vec<W>, k: usize) -> Self {
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match graph num_edges"
        );
        assert!(
            k <= graph.num_vertices(),
            "k = {} must be <= num_vertices = {}",
            k,
            graph.num_vertices()
        );
        Self {
            graph,
            edge_weights,
            k,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &SimpleGraph {
        &self.graph
    }

    /// Get a reference to the edge weights.
    pub fn edge_weights(&self) -> &[W] {
        &self.edge_weights
    }

    /// Get the required clique size.
    pub fn k(&self) -> usize {
        self.k
    }

    /// Number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check whether the selected vertices form a clique of size exactly `k`.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_k_clique_config(&self.graph, config, self.k)
    }
}

impl<W> Problem for MaximumEdgeWeightedKClique<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MaximumEdgeWeightedKClique";
    type Value = Max<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> Max<W::Sum> {
        if !is_k_clique_config(&self.graph, config, self.k) {
            return Max(None);
        }
        // Sum weights of edges whose both endpoints are selected.
        let mut total = W::Sum::zero();
        for ((u, v), weight) in self.graph.edges().iter().zip(self.edge_weights.iter()) {
            if config.get(*u).copied().unwrap_or(0) == 1
                && config.get(*v).copied().unwrap_or(0) == 1
            {
                total += weight.to_sum();
            }
        }
        Max(Some(total))
    }
}

/// Check whether `config` selects exactly `k` vertices that form a clique.
fn is_k_clique_config(graph: &SimpleGraph, config: &[usize], k: usize) -> bool {
    let n = graph.num_vertices();
    if config.len() != n {
        return false;
    }
    let selected: Vec<usize> = config
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect();
    if selected.len() != k {
        return false;
    }
    for i in 0..selected.len() {
        for j in (i + 1)..selected.len() {
            if !graph.has_edge(selected[i], selected[j]) {
                return false;
            }
        }
    }
    true
}

crate::declare_variants! {
    default MaximumEdgeWeightedKClique<i32> => "2^num_vertices",
    MaximumEdgeWeightedKClique<f64>         => "2^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "maximum_edge_weighted_k_clique_simplegraph_i32",
        instance: Box::new(MaximumEdgeWeightedKClique::<i32>::new(
            SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
            vec![5, 4, -1, 1, 0],
            3,
        )),
        optimal_config: vec![1, 1, 1, 0],
        optimal_value: serde_json::json!(8),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/maximum_edge_weighted_k_clique.rs"]
mod tests;
