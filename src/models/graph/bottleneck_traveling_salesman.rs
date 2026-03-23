//! Bottleneck Traveling Salesman problem implementation.
//!
//! The Bottleneck Traveling Salesman problem asks for a Hamiltonian cycle
//! minimizing the maximum selected edge weight.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "BottleneckTravelingSalesman",
        display_name: "Bottleneck Traveling Salesman",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a Hamiltonian cycle minimizing the maximum selected edge weight",
        fields: &[
            FieldInfo { name: "graph", type_name: "SimpleGraph", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "edge_weights", type_name: "Vec<i32>", description: "Edge weights w: E -> Z" },
        ],
    }
}

/// The Bottleneck Traveling Salesman problem on a simple weighted graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckTravelingSalesman {
    graph: SimpleGraph,
    edge_weights: Vec<i32>,
}

impl BottleneckTravelingSalesman {
    /// Create a BottleneckTravelingSalesman problem from a graph with edge weights.
    pub fn new(graph: SimpleGraph, edge_weights: Vec<i32>) -> Self {
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
    pub fn weights(&self) -> Vec<i32> {
        self.edge_weights.clone()
    }

    /// Set new weights for the problem.
    pub fn set_weights(&mut self, weights: Vec<i32>) {
        assert_eq!(weights.len(), self.graph.num_edges());
        self.edge_weights = weights;
    }

    /// Get all edges with their weights.
    pub fn edges(&self) -> Vec<(usize, usize, i32)> {
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
    type Value = SolutionSize<i32>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<i32> {
        if config.len() != self.graph.num_edges() {
            return SolutionSize::Invalid;
        }

        let selected: Vec<bool> = config.iter().map(|&s| s == 1).collect();
        if !super::traveling_salesman::is_hamiltonian_cycle(&self.graph, &selected) {
            return SolutionSize::Invalid;
        }

        let bottleneck = config
            .iter()
            .zip(self.edge_weights.iter())
            .filter_map(|(&selected, &weight)| (selected == 1).then_some(weight))
            .max()
            .expect("valid Hamiltonian cycle selects at least one edge");

        SolutionSize::Valid(bottleneck)
    }
}

impl OptimizationProblem for BottleneckTravelingSalesman {
    type Objective = i32;

    fn direction(&self) -> Direction {
        Direction::Minimize
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
        optimal_value: serde_json::json!({"Valid": 4}),
    }]
}

crate::declare_variants! {
    default opt BottleneckTravelingSalesman => "num_vertices^2 * 2^num_vertices",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/bottleneck_traveling_salesman.rs"]
mod tests;
