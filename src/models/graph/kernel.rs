//! Kernel problem implementation.
//!
//! A kernel in a directed graph is a vertex set that is both independent and
//! absorbing.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "Kernel",
        display_name: "Kernel",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Does the directed graph contain a kernel?",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "The directed graph G=(V,A)" },
        ],
    }
}

/// Kernel in a directed graph.
///
/// Given a directed graph $G = (V, A)$, determine whether there exists a set
/// $K \subseteq V$ such that:
/// - no arc has both endpoints in $K$ (independence)
/// - every vertex outside $K$ has an outgoing arc to some vertex in $K$
///   (absorption)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kernel {
    graph: DirectedGraph,
}

impl Kernel {
    /// Create a new Kernel instance.
    pub fn new(graph: DirectedGraph) -> Self {
        Self { graph }
    }

    /// Get the underlying directed graph.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Get the number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of arcs in the graph.
    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }

    /// Check whether a configuration is a valid kernel.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_kernel_config(&self.graph, config)
    }
}

impl Problem for Kernel {
    const NAME: &'static str = "Kernel";
    type Value = crate::types::Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or(is_kernel_config(&self.graph, config))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

fn is_kernel_config(graph: &DirectedGraph, config: &[usize]) -> bool {
    if config.len() != graph.num_vertices() || config.iter().any(|&bit| bit > 1) {
        return false;
    }

    for (u, v) in graph.arcs() {
        if config[u] == 1 && config[v] == 1 {
            return false;
        }
    }

    for u in 0..graph.num_vertices() {
        if config[u] == 0 && !graph.successors(u).into_iter().any(|v| config[v] == 1) {
            return false;
        }
    }

    true
}

crate::declare_variants! {
    default Kernel => "2^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "kernel",
        instance: Box::new(Kernel::new(DirectedGraph::new(
            3,
            vec![(0, 1), (1, 0), (0, 2), (1, 2)],
        ))),
        optimal_config: vec![0, 0, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/kernel.rs"]
mod tests;
