//! Partition Into Cliques problem implementation.
//!
//! Given an undirected graph G = (V, E) and an integer K, determine whether V
//! can be partitioned into at most K groups such that each group induces a
//! clique in G.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::variant::VariantParam;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "PartitionIntoCliques",
        display_name: "Partition Into Cliques",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Partition vertices into at most k cliques",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The undirected graph G=(V,E)" },
            FieldInfo { name: "num_cliques", type_name: "usize", description: "Upper bound K on the number of cliques in the partition" },
        ],
    }
}

/// The Partition Into Cliques problem.
///
/// Given an undirected graph G = (V, E) and an integer K, determine whether the
/// vertices of G can be assigned to at most K groups so that each group induces
/// a clique.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct PartitionIntoCliques<G> {
    graph: G,
    num_cliques: usize,
}

impl<G: Graph> PartitionIntoCliques<G> {
    /// Create a new Partition Into Cliques instance.
    pub fn new(graph: G, num_cliques: usize) -> Self {
        Self { graph, num_cliques }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Get the clique bound K.
    pub fn num_cliques(&self) -> usize {
        self.num_cliques
    }
}

impl<G> Problem for PartitionIntoCliques<G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "PartitionIntoCliques";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_cliques; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        let n = self.graph.num_vertices();

        if config.len() != n {
            return crate::types::Or(false);
        }

        if config.iter().any(|&group| group >= self.num_cliques) {
            return crate::types::Or(false);
        }

        for u in 0..n {
            for v in (u + 1)..n {
                if config[u] == config[v] && !self.graph.has_edge(u, v) {
                    return crate::types::Or(false);
                }
            }
        }

        crate::types::Or(true)
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "partition_into_cliques_simplegraph",
        instance: Box::new(PartitionIntoCliques::new(
            SimpleGraph::new(5, vec![(0, 3), (0, 4), (1, 2), (1, 4)]),
            3,
        )),
        optimal_config: vec![0, 1, 1, 0, 2],
        optimal_value: serde_json::json!(true),
    }]
}

crate::declare_variants! {
    default PartitionIntoCliques<SimpleGraph> => "2^num_vertices",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/partition_into_cliques.rs"]
mod tests;
