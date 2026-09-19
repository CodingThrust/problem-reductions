//! Shortest Weight-Constrained Path problem implementation.
//!
//! The Shortest Weight-Constrained Path problem finds a simple path from a
//! source vertex to a target vertex that minimizes total length while keeping
//! the total weight within a prescribed bound.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{is_simple_st_path, Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ShortestWeightConstrainedPath",
        display_name: "Shortest Weight-Constrained Path",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i64", &["i64"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find a simple s-t path minimizing total length subject to a weight budget",
        fields: ShortestWeightConstrainedPathCreateSpec::FIELDS,
    }
}

/// The Shortest Weight-Constrained Path problem.
///
/// Given a graph G = (V, E) with positive edge lengths l(e) and edge weights
/// w(e), designated vertices s and t, and a weight bound W, find a simple
/// path from s to t that minimizes total length subject to total weight at
/// most W.
///
/// # Representation
///
/// Each edge is assigned a binary variable:
/// - 0: edge is not in the selected path
/// - 1: edge is in the selected path
///
/// A valid configuration must:
/// - form a single simple path from `source_vertex` to `target_vertex`
/// - use only edges present in the graph
/// - satisfy the weight bound
///
/// The objective value is the total length of the path (`Min<N::Sum>`).
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
/// * `N` - The edge length / weight type (e.g., `i64`, `f64`)
#[derive(Debug, Clone, Serialize)]
pub struct ShortestWeightConstrainedPath<G, N: WeightElement> {
    /// The underlying graph.
    graph: G,
    /// Length for each edge in graph-edge order.
    edge_lengths: Vec<N>,
    /// Weight for each edge in graph-edge order.
    edge_weights: Vec<N>,
    /// Source vertex s.
    source_vertex: usize,
    /// Target vertex t.
    target_vertex: usize,
    /// Upper bound W on total path weight.
    weight_bound: N::Sum,
}

#[derive(Deserialize)]
#[serde(bound(
    deserialize = "G: Graph + Deserialize<'de>, N: WeightElement + Deserialize<'de>, N::Sum: Deserialize<'de>"
))]
struct ShortestWeightConstrainedPathData<G, N: WeightElement> {
    graph: G,
    edge_lengths: Vec<N>,
    edge_weights: Vec<N>,
    source_vertex: usize,
    target_vertex: usize,
    weight_bound: N::Sum,
}

impl<'de, G, N> Deserialize<'de> for ShortestWeightConstrainedPath<G, N>
where
    G: Graph + Deserialize<'de>,
    N: WeightElement + Deserialize<'de>,
    N::Sum: Deserialize<'de>,
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let data = ShortestWeightConstrainedPathData::<G, N>::deserialize(deserializer)?;
        Self::try_new(
            data.graph,
            data.edge_lengths,
            data.edge_weights,
            data.source_vertex,
            data.target_vertex,
            data.weight_bound,
        )
        .map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct ShortestWeightConstrainedPathCreateSpec {
    /// The underlying graph G=(V,E).
    graph: SimpleGraph,
    /// Positive edge lengths in graph edge order.
    edge_lengths: Vec<i64>,
    /// Positive edge weights in graph edge order.
    edge_weights: Vec<i64>,
    /// Source vertex s.
    source_vertex: usize,
    /// Target vertex t.
    target_vertex: usize,
    /// Positive upper bound on total path weight.
    weight_bound: i64,
}

impl TryFrom<ShortestWeightConstrainedPathCreateSpec>
    for ShortestWeightConstrainedPath<SimpleGraph, i64>
{
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: ShortestWeightConstrainedPathCreateSpec) -> Result<Self, Self::Error> {
        Self::try_new(
            spec.graph,
            spec.edge_lengths,
            spec.edge_weights,
            spec.source_vertex,
            spec.target_vertex,
            spec.weight_bound,
        )
    }
}

impl<G: Graph, N: WeightElement> ShortestWeightConstrainedPath<G, N> {
    fn check_edge_values(
        graph: &G,
        values: &[N],
        label: &str,
    ) -> Result<(), crate::registry::ConstructionError> {
        if values.len() != graph.num_edges() {
            return Err(format!("{label} length must match num_edges").into());
        }
        if !values.iter().all(|value| value.to_sum() > N::Sum::zero()) {
            return Err(format!("All {label} must be positive (> 0)").into());
        }
        Ok(())
    }

    /// Create a new ShortestWeightConstrainedPath instance.
    ///
    /// # Panics
    ///
    /// Panics if either edge vector length does not match the graph's edge
    /// count, or if the source / target vertices are out of bounds.
    pub fn new(
        graph: G,
        edge_lengths: Vec<N>,
        edge_weights: Vec<N>,
        source_vertex: usize,
        target_vertex: usize,
        weight_bound: N::Sum,
    ) -> Self {
        Self::try_new(
            graph,
            edge_lengths,
            edge_weights,
            source_vertex,
            target_vertex,
            weight_bound,
        )
        .unwrap_or_else(|error| panic!("{error}"))
    }

    fn try_new(
        graph: G,
        edge_lengths: Vec<N>,
        edge_weights: Vec<N>,
        source_vertex: usize,
        target_vertex: usize,
        weight_bound: N::Sum,
    ) -> Result<Self, crate::registry::ConstructionError> {
        Self::check_edge_values(&graph, &edge_lengths, "edge lengths")?;
        Self::check_edge_values(&graph, &edge_weights, "edge weights")?;
        if source_vertex >= graph.num_vertices() {
            return Err(format!(
                "source_vertex {} out of bounds (graph has {} vertices)",
                source_vertex,
                graph.num_vertices()
            )
            .into());
        }
        if target_vertex >= graph.num_vertices() {
            return Err(format!(
                "target_vertex {} out of bounds (graph has {} vertices)",
                target_vertex,
                graph.num_vertices()
            )
            .into());
        }
        if weight_bound.partial_cmp(&N::Sum::zero()) != Some(std::cmp::Ordering::Greater) {
            return Err("weight_bound must be positive (> 0)".into());
        }
        Ok(Self {
            graph,
            edge_lengths,
            edge_weights,
            source_vertex,
            target_vertex,
            weight_bound,
        })
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the edge lengths.
    pub fn edge_lengths(&self) -> &[N] {
        &self.edge_lengths
    }

    /// Get the edge weights.
    pub fn edge_weights(&self) -> &[N] {
        &self.edge_weights
    }

    /// Set new edge lengths.
    pub fn set_lengths(&mut self, edge_lengths: Vec<N>) {
        Self::check_edge_values(&self.graph, &edge_lengths, "edge lengths")
            .unwrap_or_else(|error| panic!("{error}"));
        self.edge_lengths = edge_lengths;
    }

    /// Set new edge weights.
    pub fn set_weights(&mut self, edge_weights: Vec<N>) {
        Self::check_edge_values(&self.graph, &edge_weights, "edge weights")
            .unwrap_or_else(|error| panic!("{error}"));
        self.edge_weights = edge_weights;
    }

    /// Get the source vertex.
    pub fn source_vertex(&self) -> usize {
        self.source_vertex
    }

    /// Get the target vertex.
    pub fn target_vertex(&self) -> usize {
        self.target_vertex
    }

    /// Get the weight bound.
    pub fn weight_bound(&self) -> &N::Sum {
        &self.weight_bound
    }

    /// Check whether this problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool {
        !N::IS_UNIT
    }

    /// Get the number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check if a configuration is a valid weight-constrained s-t path.
    ///
    /// Returns `Some(total_length)` for a valid simple s-t path whose total
    /// weight is within the weight bound, or `None` otherwise.
    pub fn is_valid_solution(
        &self,
        config: &[bool],
    ) -> Result<Option<N::Sum>, crate::traits::EvaluationError> {
        if config.len() != self.graph.num_edges() {
            return Ok(None);
        }

        if self.source_vertex == self.target_vertex {
            if config.contains(&true) {
                return Ok(None);
            }
            return Ok(Some(N::Sum::zero()));
        }

        let mut total_length = N::Sum::zero();
        let mut total_weight = N::Sum::zero();

        for (idx, &selected) in config.iter().enumerate() {
            if !selected {
                continue;
            }
            total_length = N::checked_add_to_sum(
                total_length,
                self.edge_lengths[idx].to_sum(),
                "summing constrained path edge lengths",
            )?;
            total_weight = N::checked_add_to_sum(
                total_weight,
                self.edge_weights[idx].to_sum(),
                "summing constrained path edge weights",
            )?;
        }

        if total_weight > self.weight_bound.clone() {
            return Ok(None);
        }
        if !is_simple_st_path(
            self.graph.num_vertices(),
            &self.graph.edges(),
            self.source_vertex,
            self.target_vertex,
            config,
        ) {
            Ok(None)
        } else {
            Ok(Some(total_length))
        }
    }
}

impl<G, N> Problem for ShortestWeightConstrainedPath<G, N>
where
    G: Graph + crate::variant::VariantParam,
    N: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "ShortestWeightConstrainedPath";
    type Solution = Vec<bool>;
    type Value = Min<N::Sum>;

    crate::problem_parameters![("num_edges", num_edges), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, N]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<N::Sum>, crate::traits::EvaluationError> {
        if config.len() != self.graph.num_edges() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "edge-selection length does not match the graph".into(),
            ));
        }
        Ok(Min(self.is_valid_solution(config)?))
    }
}

impl<G, N> crate::solvers::BruteForceProblem for ShortestWeightConstrainedPath<G, N>
where
    G: Graph + crate::variant::VariantParam,
    N: WeightElement + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "shortest_weight_constrained_path_simplegraph",
        instance: Box::new(ShortestWeightConstrainedPath::new(
            SimpleGraph::new(
                6,
                vec![
                    (0, 1),
                    (0, 2),
                    (1, 3),
                    (2, 3),
                    (2, 4),
                    (3, 5),
                    (4, 5),
                    (1, 4),
                ],
            ),
            vec![2, 4, 3, 1, 5, 4, 2, 6],
            vec![5, 1, 2, 3, 2, 3, 1, 1],
            0,
            5,
            8,
        )),
        optimal_config: serde_json::json!(vec![
            false, true, false, true, false, true, false, false
        ]),
        optimal_value: serde_json::json!(9),
    }]
}

crate::declare_variants! {
    default ShortestWeightConstrainedPath<SimpleGraph, i64> => "2^num_edges" create ShortestWeightConstrainedPathCreateSpec,
}

crate::register_brute_force! {
    ShortestWeightConstrainedPath<SimpleGraph, i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/shortest_weight_constrained_path.rs"]
mod tests;
