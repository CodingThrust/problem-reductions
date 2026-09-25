//! Biconnectivity augmentation problem implementation.
//!
//! Given a graph, weighted potential edges, and a budget, determine whether
//! adding some subset of the potential edges can make the graph biconnected
//! without exceeding the budget.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::WeightElement;
use num_traits::Zero;
use petgraph::algo::{articulation_points::articulation_points, connected_components};
use petgraph::graph::{NodeIndex, UnGraph};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "BiconnectivityAugmentation",
        display_name: "Biconnectivity Augmentation",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i64", &["i64"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Add weighted potential edges to make a graph biconnected within budget",
        fields: BiconnectivityAugmentationCreateSpec::FIELDS,
    }
}

/// The Biconnectivity Augmentation problem.
///
/// Given a graph `G = (V, E)`, weighted potential edges, and a budget `B`,
/// determine whether there exists a subset of potential edges `E'` such that:
/// - `sum_{e in E'} w(e) <= B`
/// - `(V, E union E')` is biconnected
#[derive(Debug, Clone, Serialize)]
#[serde(bound(serialize = "G: serde::Serialize, W: serde::Serialize, W::Sum: serde::Serialize"))]
pub struct BiconnectivityAugmentation<G, W>
where
    W: WeightElement,
{
    /// The underlying graph.
    graph: G,
    /// Potential augmentation edges with their weights.
    potential_weights: Vec<(usize, usize, W)>,
    /// Maximum total weight of selected potential edges.
    budget: W::Sum,
}

#[derive(Deserialize)]
#[serde(bound(
    deserialize = "G: Graph + Deserialize<'de>, W: WeightElement + Deserialize<'de>, W::Sum: Deserialize<'de>"
))]
struct BiconnectivityAugmentationData<G, W: WeightElement> {
    graph: G,
    potential_weights: Vec<(usize, usize, W)>,
    budget: W::Sum,
}

impl<'de, G, W> Deserialize<'de> for BiconnectivityAugmentation<G, W>
where
    G: Graph + Deserialize<'de>,
    W: WeightElement + Deserialize<'de>,
    W::Sum: Deserialize<'de>,
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let data = BiconnectivityAugmentationData::<G, W>::deserialize(deserializer)?;
        Self::try_new(data.graph, data.potential_weights, data.budget)
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct BiconnectivityAugmentationCreateSpec {
    #[create(codec = "edge-list")]
    graph: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
    potential_weights: Vec<(usize, usize, i64)>,
    budget: i64,
}

impl TryFrom<BiconnectivityAugmentationCreateSpec>
    for BiconnectivityAugmentation<SimpleGraph, i64>
{
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: BiconnectivityAugmentationCreateSpec) -> Result<Self, Self::Error> {
        if spec.graph.is_empty() && spec.num_vertices.is_none() {
            return Err("num_vertices is required for an empty graph".into());
        }
        for &(u, v) in &spec.graph {
            if u == v {
                return Err(format!("self-loop {u}-{v} is not allowed").into());
            }
        }
        let inferred = spec
            .graph
            .iter()
            .flat_map(|&(u, v)| [u, v])
            .max()
            .map(|v| v.checked_add(1).ok_or("vertex count overflows usize"))
            .transpose()?
            .unwrap_or(0);
        let count = spec.num_vertices.unwrap_or(inferred);
        if count < inferred {
            return Err("num_vertices is too small for graph endpoints".into());
        }
        let graph = SimpleGraph::new(count, spec.graph);
        Self::try_new(graph, spec.potential_weights, spec.budget)
    }
}

impl<G: Graph, W: WeightElement> BiconnectivityAugmentation<G, W> {
    /// Create a new biconnectivity augmentation instance.
    ///
    /// # Panics
    /// Panics if any potential edge references a vertex index outside the graph,
    /// is a self-loop, duplicates another candidate edge, or already exists in
    /// the input graph.
    pub fn new(graph: G, potential_weights: Vec<(usize, usize, W)>, budget: W::Sum) -> Self {
        Self::try_new(graph, potential_weights, budget).unwrap_or_else(|error| panic!("{error}"))
    }

    fn try_new(
        graph: G,
        potential_weights: Vec<(usize, usize, W)>,
        budget: W::Sum,
    ) -> Result<Self, crate::registry::ConstructionError> {
        let num_vertices = graph.num_vertices();
        let mut seen_potential_edges = BTreeSet::new();
        for &(u, v, _) in &potential_weights {
            if u >= num_vertices || v >= num_vertices {
                return Err(format!(
                    "potential edge ({}, {}) references vertex >= num_vertices ({})",
                    u, v, num_vertices
                )
                .into());
            }
            if u == v {
                return Err(format!("potential edge ({}, {}) is a self-loop", u, v).into());
            }
            let edge = normalize_edge(u, v);
            if graph.has_edge(edge.0, edge.1) {
                return Err(format!(
                    "potential edge ({}, {}) already exists in the graph",
                    edge.0, edge.1
                )
                .into());
            }
            if !seen_potential_edges.insert(edge) {
                return Err(
                    format!("potential edge ({}, {}) is duplicated", edge.0, edge.1).into(),
                );
            }
        }

        Ok(Self {
            graph,
            potential_weights,
            budget,
        })
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the weighted potential edges.
    pub fn potential_weights(&self) -> &[(usize, usize, W)] {
        &self.potential_weights
    }

    /// Get the budget.
    pub fn budget(&self) -> &W::Sum {
        &self.budget
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Get the number of potential augmentation edges.
    pub fn num_potential_edges(&self) -> usize {
        self.potential_weights.len()
    }

    /// Check if the problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
    }

    fn augmented_graph(
        &self,
        config: &[bool],
    ) -> Result<Option<UnGraph<(), ()>>, crate::traits::EvaluationError> {
        if config.len() != self.num_potential_edges() {
            return Ok(None);
        }

        let mut total = W::Sum::zero();
        let mut edges = BTreeSet::new();

        for (u, v) in self.graph.edges() {
            edges.insert(normalize_edge(u, v));
        }

        for (selected, &(u, v, ref weight)) in config.iter().zip(&self.potential_weights) {
            if *selected {
                total = W::checked_add_to_sum(
                    total,
                    weight.to_sum(),
                    "summing biconnectivity augmentation weights",
                )?;
                edges.insert(normalize_edge(u, v));
            }
        }
        if total > self.budget.clone() {
            return Ok(None);
        }

        let mut graph = UnGraph::new_undirected();
        for _ in 0..self.num_vertices() {
            graph.add_node(());
        }
        for (u, v) in edges {
            graph.add_edge(NodeIndex::new(u), NodeIndex::new(v), ());
        }
        Ok(Some(graph))
    }
}

impl<G, W> Problem for BiconnectivityAugmentation<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "BiconnectivityAugmentation";
    type Solution = Vec<bool>;
    type Value = crate::types::Or;

    crate::problem_parameters![
        ("num_edges", num_edges),
        ("num_potential_edges", num_potential_edges),
        ("num_vertices", num_vertices),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        if config.len() != self.num_potential_edges() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "edge-selection length does not match the candidate edges".into(),
            ));
        }
        Ok({
            crate::types::Or({
                self.augmented_graph(config)?
                    .is_some_and(|graph| is_biconnected(&graph))
            })
        })
    }
}

impl<G, W> crate::solvers::BruteForceProblem for BiconnectivityAugmentation<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.num_potential_edges()]
    }
}

fn normalize_edge(u: usize, v: usize) -> (usize, usize) {
    if u <= v {
        (u, v)
    } else {
        (v, u)
    }
}

fn is_biconnected(graph: &UnGraph<(), ()>) -> bool {
    graph.node_count() <= 1
        || (connected_components(graph) == 1 && articulation_points(graph).is_empty())
}

crate::declare_variants! {
    default BiconnectivityAugmentation<SimpleGraph, i64> => "2^num_potential_edges" create BiconnectivityAugmentationCreateSpec,
}

crate::register_brute_force! {
    BiconnectivityAugmentation<SimpleGraph, i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "biconnectivity_augmentation",
        instance: Box::new(BiconnectivityAugmentation::new(
            SimpleGraph::path(6),
            vec![
                (0, 2, 1),
                (0, 3, 2),
                (0, 4, 3),
                (1, 3, 1),
                (1, 4, 2),
                (1, 5, 3),
                (2, 4, 1),
                (2, 5, 2),
                (3, 5, 1),
            ],
            4,
        )),
        optimal_config: serde_json::json!(vec![
            true, false, false, true, false, false, true, false, true
        ]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
pub(crate) fn example_instance() -> BiconnectivityAugmentation<SimpleGraph, i64> {
    BiconnectivityAugmentation::new(
        SimpleGraph::path(6),
        vec![
            (0, 2, 1),
            (0, 3, 2),
            (0, 4, 3),
            (1, 3, 1),
            (1, 4, 2),
            (1, 5, 3),
            (2, 4, 1),
            (2, 5, 2),
            (3, 5, 1),
        ],
        4,
    )
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/biconnectivity_augmentation.rs"]
mod tests;
