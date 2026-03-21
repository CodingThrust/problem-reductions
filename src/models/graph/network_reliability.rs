//! Network Reliability problem implementation.
//!
//! The model treats each configuration as a single edge-survival pattern.
//! Exact reliability is then obtained by summing the probabilities of the
//! configurations that keep all terminals connected.

use std::collections::{BTreeSet, VecDeque};

use crate::config::DimsIterator;
use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "NetworkReliability",
        display_name: "Network Reliability",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether terminal connectivity survives edge failures with probability at least q",
        fields: &[
            FieldInfo { name: "graph", type_name: "SimpleGraph", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "terminals", type_name: "Vec<usize>", description: "Terminal vertices T subset of V that must remain connected" },
            FieldInfo { name: "failure_probs", type_name: "Vec<f64>", description: "Independent edge failure probabilities p: E -> [0,1]" },
            FieldInfo { name: "threshold", type_name: "f64", description: "Required reliability threshold q in [0,1]" },
        ],
    }
}

/// The Network Reliability decision problem.
///
/// Each binary configuration indicates which edges survive:
/// - `0`: the edge fails
/// - `1`: the edge survives
///
/// `evaluate(config)` checks whether the surviving-edge subgraph keeps all
/// terminal vertices connected. The overall decision question
/// `R(G, T, p) >= q` is exposed via [`NetworkReliability::reliability`] and
/// [`NetworkReliability::meets_threshold`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkReliability {
    graph: SimpleGraph,
    terminals: Vec<usize>,
    failure_probs: Vec<f64>,
    threshold: f64,
}

impl NetworkReliability {
    /// Create a Network Reliability instance.
    ///
    /// # Panics
    /// Panics if the edge probability vector length does not match `num_edges`,
    /// terminals are invalid, or any probability/threshold lies outside `[0, 1]`.
    pub fn new(
        graph: SimpleGraph,
        terminals: Vec<usize>,
        failure_probs: Vec<f64>,
        threshold: f64,
    ) -> Self {
        assert_eq!(
            failure_probs.len(),
            graph.num_edges(),
            "failure_probs length must match num_edges"
        );

        let n = graph.num_vertices();
        let distinct_terminals: BTreeSet<_> = terminals.iter().copied().collect();
        assert_eq!(
            distinct_terminals.len(),
            terminals.len(),
            "terminals must be distinct"
        );
        assert!(terminals.len() >= 2, "at least 2 terminals required");
        for &terminal in &terminals {
            assert!(
                terminal < n,
                "terminal {} out of range (num_vertices = {})",
                terminal,
                n
            );
        }

        for (index, &prob) in failure_probs.iter().enumerate() {
            assert!(
                prob.is_finite() && (0.0..=1.0).contains(&prob),
                "failure probability at edge {} must be in [0, 1]",
                index
            );
        }
        assert!(
            threshold.is_finite() && (0.0..=1.0).contains(&threshold),
            "threshold must be in [0, 1]"
        );

        Self {
            graph,
            terminals,
            failure_probs,
            threshold,
        }
    }

    /// Get the underlying graph.
    pub fn graph(&self) -> &SimpleGraph {
        &self.graph
    }

    /// Get the terminal vertices.
    pub fn terminals(&self) -> &[usize] {
        &self.terminals
    }

    /// Get the independent edge failure probabilities.
    pub fn failure_probs(&self) -> &[f64] {
        &self.failure_probs
    }

    /// Get the reliability threshold.
    pub fn threshold(&self) -> f64 {
        self.threshold
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Get the number of terminals.
    pub fn num_terminals(&self) -> usize {
        self.terminals.len()
    }

    /// Check whether a configuration keeps all terminals connected.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.evaluate(config)
    }

    /// Compute the probability mass of a single edge-survival configuration.
    pub fn configuration_probability(&self, config: &[usize]) -> f64 {
        if config.len() != self.num_edges() || config.iter().any(|&bit| bit > 1) {
            return 0.0;
        }

        config
            .iter()
            .zip(self.failure_probs.iter())
            .map(|(&bit, &failure_prob)| {
                if bit == 1 {
                    1.0 - failure_prob
                } else {
                    failure_prob
                }
            })
            .product()
    }

    /// Sum the probabilities of all surviving-edge patterns that connect the terminals.
    pub fn reliability(&self) -> f64 {
        DimsIterator::new(self.dims())
            .filter(|config| self.evaluate(config))
            .map(|config| self.configuration_probability(&config))
            .sum()
    }

    /// Return whether the exact reliability meets the instance threshold.
    pub fn meets_threshold(&self) -> bool {
        const EPSILON: f64 = 1e-12;
        self.reliability() + EPSILON >= self.threshold
    }
}

impl Problem for NetworkReliability {
    const NAME: &'static str = "NetworkReliability";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        terminals_connected_with_surviving_edges(&self.graph, &self.terminals, config)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for NetworkReliability {}

fn terminals_connected_with_surviving_edges(
    graph: &SimpleGraph,
    terminals: &[usize],
    config: &[usize],
) -> bool {
    if config.len() != graph.num_edges() || config.iter().any(|&bit| bit > 1) {
        return false;
    }

    let mut adjacency = vec![Vec::new(); graph.num_vertices()];
    for ((u, v), &bit) in graph.edges().iter().zip(config.iter()) {
        if bit == 1 {
            adjacency[*u].push(*v);
            adjacency[*v].push(*u);
        }
    }

    let start = terminals[0];
    let mut visited = vec![false; graph.num_vertices()];
    let mut queue = VecDeque::from([start]);
    visited[start] = true;

    while let Some(vertex) = queue.pop_front() {
        for &neighbor in &adjacency[vertex] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    terminals.iter().all(|&terminal| visited[terminal])
}

crate::declare_variants! {
    default sat NetworkReliability => "2^num_edges * num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "network_reliability",
        instance: Box::new(NetworkReliability::new(
            SimpleGraph::new(
                6,
                vec![
                    (0, 1),
                    (0, 2),
                    (1, 3),
                    (2, 3),
                    (1, 4),
                    (3, 4),
                    (3, 5),
                    (4, 5),
                ],
            ),
            vec![0, 5],
            vec![0.1; 8],
            0.95,
        )),
        optimal_config: vec![1, 0, 1, 0, 0, 0, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/network_reliability.rs"]
mod tests;
