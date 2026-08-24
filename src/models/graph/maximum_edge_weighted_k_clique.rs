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

use crate::registry::{
    ConstructionError, CreateSpec, ProblemSchemaEntry, ProblemSizeFieldEntry, VariantDimension,
};
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
        dimensions: &[VariantDimension::new("weight", "i64", &["i64", "f64"])],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Select exactly k pairwise-adjacent vertices maximizing the total weight of induced clique edges",
        fields: MaximumEdgeWeightedKCliqueCreateSpec::<i64>::FIELDS,
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
/// * `W` - Edge weight type (e.g., `i64`, `f64`). The graph is fixed to
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
/// let weights = vec![5_i64, 4, -1, 1, 0];
/// let problem = MaximumEdgeWeightedKClique::new(graph, weights, 3).unwrap();
/// assert_eq!(BruteForce::new().solve(&problem).unwrap(), Max(Some(8)));
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MaximumEdgeWeightedKClique<W: WeightElement> {
    /// The underlying graph.
    graph: SimpleGraph,
    /// Edge weights, in the graph's edge iteration order.
    edge_weights: Vec<W>,
    /// Required clique size.
    k: usize,
}

#[derive(Deserialize)]
struct MaximumEdgeWeightedKCliqueData<W> {
    graph: SimpleGraph,
    edge_weights: Vec<W>,
    k: usize,
}

impl<'de, W> Deserialize<'de> for MaximumEdgeWeightedKClique<W>
where
    W: WeightElement + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = MaximumEdgeWeightedKCliqueData::deserialize(deserializer)?;
        Self::new(data.graph, data.edge_weights, data.k).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct MaximumEdgeWeightedKCliqueCreateSpec<W> {
    /// The underlying graph.
    graph: SimpleGraph,
    /// Edge weights; defaults to one per edge.
    edge_weights: Option<Vec<W>>,
    /// Required clique size.
    k: usize,
}
impl<W> TryFrom<MaximumEdgeWeightedKCliqueCreateSpec<W>> for MaximumEdgeWeightedKClique<W>
where
    W: WeightElement,
{
    type Error = ConstructionError;
    fn try_from(spec: MaximumEdgeWeightedKCliqueCreateSpec<W>) -> Result<Self, Self::Error> {
        let count = spec.graph.num_edges();
        let edge_weights = spec
            .edge_weights
            .unwrap_or_else(|| (0..count).map(|_| W::unit()).collect());
        Self::new(spec.graph, edge_weights, spec.k)
    }
}

impl<W: WeightElement> MaximumEdgeWeightedKClique<W> {
    /// Create a new MaximumEdgeWeightedKClique instance.
    ///
    pub fn new(
        graph: SimpleGraph,
        edge_weights: Vec<W>,
        k: usize,
    ) -> Result<Self, ConstructionError> {
        if edge_weights.len() != graph.num_edges() {
            return Err(ConstructionError::Conversion(
                "edge_weights length must match graph num_edges".into(),
            ));
        }
        for (index, weight) in edge_weights.iter().enumerate() {
            weight.validate_element(&format!("edge weight at index {index}"))?;
        }
        if k > graph.num_vertices() {
            return Err(ConstructionError::Conversion(format!(
                "k = {k} must be <= num_vertices = {}",
                graph.num_vertices()
            )));
        }
        Ok(Self {
            graph,
            edge_weights,
            k,
        })
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

    fn evaluate(&self, config: &[usize]) -> Result<Max<W::Sum>, crate::traits::EvaluationError> {
        Ok({
            if !is_k_clique_config(&self.graph, config, self.k) {
                return Ok(Max(None));
            }
            // Sum weights of edges whose both endpoints are selected.
            let mut total = W::Sum::zero();
            for ((u, v), weight) in self.graph.edges().iter().zip(self.edge_weights.iter()) {
                if config.get(*u).copied().unwrap_or(0) == 1
                    && config.get(*v).copied().unwrap_or(0) == 1
                {
                    total = W::checked_add_to_sum(
                        total,
                        weight.to_sum(),
                        "summing selected clique-edge weights",
                    )?;
                }
            }
            Max(Some(total))
        })
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
    default MaximumEdgeWeightedKClique<i64> => "2^num_vertices" create MaximumEdgeWeightedKCliqueCreateSpec<i64>,
    MaximumEdgeWeightedKClique<f64>         => "2^num_vertices" create MaximumEdgeWeightedKCliqueCreateSpec<f64>,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "maximum_edge_weighted_k_clique_simplegraph_i64",
        instance: Box::new(
            MaximumEdgeWeightedKClique::<i64>::new(
                SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
                vec![5, 4, -1, 1, 0],
                3,
            )
            .unwrap(),
        ),
        optimal_config: vec![1, 1, 1, 0],
        optimal_value: serde_json::json!(8),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/maximum_edge_weighted_k_clique.rs"]
mod tests;
