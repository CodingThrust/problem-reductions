//! Bottleneck Traveling Salesman problem implementation.
//!
//! The Bottleneck Traveling Salesman problem asks for a Hamiltonian cycle
//! minimizing the maximum selected edge weight.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "BottleneckTravelingSalesman",
        display_name: "Bottleneck Traveling Salesman",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find a Hamiltonian cycle minimizing the maximum selected edge weight",
        fields: BottleneckTravelingSalesmanCreateSpec::FIELDS,
    }
}

/// The Bottleneck Traveling Salesman problem on a simple weighted graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckTravelingSalesman {
    graph: SimpleGraph,
    edge_weights: Vec<i64>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct BottleneckTravelingSalesmanCreateSpec {
    #[create(codec = "edge-list")]
    graph: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
    #[create(codec = "comma-separated")]
    edge_weights: Option<Vec<i64>>,
}

impl TryFrom<BottleneckTravelingSalesmanCreateSpec> for BottleneckTravelingSalesman {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: BottleneckTravelingSalesmanCreateSpec) -> Result<Self, Self::Error> {
        let graph = simple_graph_from_create(spec.graph, spec.num_vertices)?;
        let edge_weights = spec
            .edge_weights
            .unwrap_or_else(|| vec![1; graph.num_edges()]);
        if edge_weights.len() != graph.num_edges() {
            return Err(format!(
                "edge_weights has length {}, expected {}",
                edge_weights.len(),
                graph.num_edges()
            )
            .into());
        }
        Ok(Self::new(graph, edge_weights))
    }
}

fn simple_graph_from_create(
    edges: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
) -> Result<SimpleGraph, crate::registry::ConstructionError> {
    if edges.is_empty() && num_vertices.is_none() {
        return Err("num_vertices is required for an empty graph"
            .to_string()
            .into());
    }
    for (index, &(u, v)) in edges.iter().enumerate() {
        if u == v {
            return Err(format!("graph edge {index} is a self-loop at vertex {u}").into());
        }
    }
    let inferred = edges
        .iter()
        .flat_map(|&(u, v)| [u, v])
        .max()
        .map(|vertex| vertex.checked_add(1).ok_or("vertex count overflows usize"))
        .transpose()?
        .unwrap_or(0);
    let num_vertices = num_vertices.unwrap_or(inferred);
    if num_vertices < inferred {
        return Err(format!(
            "num_vertices {num_vertices} is too small for graph endpoints; need at least {inferred}"
        )
        .into());
    }
    Ok(SimpleGraph::new(num_vertices, edges))
}

impl BottleneckTravelingSalesman {
    /// Create a BottleneckTravelingSalesman problem from a graph with edge weights.
    pub fn new(graph: SimpleGraph, edge_weights: Vec<i64>) -> Self {
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        Self {
            graph,
            edge_weights,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &SimpleGraph {
        &self.graph
    }

    /// Get the weights for the problem.
    pub fn weights(&self) -> Vec<i64> {
        self.edge_weights.clone()
    }

    /// Set new weights for the problem.
    pub fn set_weights(&mut self, weights: Vec<i64>) {
        assert_eq!(weights.len(), self.graph.num_edges());
        self.edge_weights = weights;
    }

    /// Get all edges with their weights.
    pub fn edges(&self) -> Vec<(usize, usize, i64)> {
        self.graph
            .edges()
            .into_iter()
            .zip(self.edge_weights.iter().copied())
            .map(|((u, v), w)| (u, v, w))
            .collect()
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// This model is always weighted.
    pub fn is_weighted(&self) -> bool {
        true
    }

    /// Check if a configuration is a valid Hamiltonian cycle.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        if config.len() != self.graph.num_edges() {
            return false;
        }
        let selected: Vec<bool> = config.iter().map(|&s| s == 1).collect();
        super::traveling_salesman::is_hamiltonian_cycle(&self.graph, &selected)
    }
}

impl Problem for BottleneckTravelingSalesman {
    const NAME: &'static str = "BottleneckTravelingSalesman";
    type Value = Min<i64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> Result<Min<i64>, crate::traits::EvaluationError> {
        Ok({
            if config.len() != self.graph.num_edges() {
                return Ok(Min(None));
            }

            let selected: Vec<bool> = config.iter().map(|&s| s == 1).collect();
            if !super::traveling_salesman::is_hamiltonian_cycle(&self.graph, &selected) {
                return Ok(Min(None));
            }

            let bottleneck = config
                .iter()
                .zip(self.edge_weights.iter())
                .filter_map(|(&selected, &weight)| (selected == 1).then_some(weight))
                .max()
                .expect("valid Hamiltonian cycle selects at least one edge");

            Min(Some(bottleneck))
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "bottleneck_traveling_salesman",
        instance: Box::new(BottleneckTravelingSalesman::new(
            SimpleGraph::new(
                5,
                vec![
                    (0, 1),
                    (0, 2),
                    (0, 3),
                    (0, 4),
                    (1, 2),
                    (1, 3),
                    (1, 4),
                    (2, 3),
                    (2, 4),
                    (3, 4),
                ],
            ),
            vec![5, 4, 4, 5, 4, 1, 2, 1, 5, 4],
        )),
        optimal_config: vec![0, 1, 1, 0, 1, 0, 1, 0, 0, 1],
        optimal_value: serde_json::json!(4),
    }]
}

crate::impl_random_generate!(
    BottleneckTravelingSalesman,
    crate::random::SimpleGraphRandomSpec,
    |spec| {
        let graph = spec.graph()?;
        let weights = vec![1; graph.num_edges()];
        Ok(BottleneckTravelingSalesman::new(graph, weights))
    }
);

crate::declare_variants! {
    default BottleneckTravelingSalesman => "num_vertices^2 * 2^num_vertices" create BottleneckTravelingSalesmanCreateSpec random,
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/bottleneck_traveling_salesman.rs"]
mod tests;
