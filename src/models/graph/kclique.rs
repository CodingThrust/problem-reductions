//! KClique problem implementation.
//!
//! KClique is the decision version of Clique: determine whether a graph
//! contains a clique of size at least `k`.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "KClique",
        display_name: "k-Clique",
        aliases: &["Clique"],
        dimensions: &[VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"])],
        module_path: module_path!(),
        description: "Determine whether a graph contains a clique of size at least k",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "k", type_name: "usize", description: "Minimum clique size threshold" },
        ],
    }
}

/// The k-Clique decision problem.
///
/// Given a graph `G = (V, E)` and a positive integer `k`, determine whether
/// there exists a subset `K ⊆ V` of size at least `k` such that every pair of
/// distinct vertices in `K` is adjacent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KClique<G> {
    graph: G,
    k: usize,
}

impl<G: Graph> KClique<G> {
    /// Create a new k-Clique problem instance.
    pub fn new(graph: G, k: usize) -> Self {
        assert!(k > 0, "k must be positive");
        assert!(k <= graph.num_vertices(), "k must be <= graph num_vertices");
        Self { graph, k }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the clique-size threshold.
    pub fn k(&self) -> usize {
        self.k
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check whether a configuration is a valid witness.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_kclique_config(&self.graph, config, self.k)
    }
}

impl<G> Problem for KClique<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "KClique";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or(is_kclique_config(&self.graph, config, self.k))
    }
}

fn is_kclique_config<G: Graph>(graph: &G, config: &[usize], k: usize) -> bool {
    if config.len() != graph.num_vertices() {
        return false;
    }

    let selected: Vec<usize> = match config
        .iter()
        .enumerate()
        .map(|(index, &value)| match value {
            0 => Ok(None),
            1 => Ok(Some(index)),
            _ => Err(()),
        })
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(values) => values.into_iter().flatten().collect(),
        Err(()) => return false,
    };

    if selected.len() < k {
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
    default KClique<SimpleGraph> => "1.1996^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "kclique_simplegraph",
        instance: Box::new(KClique::new(
            SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]),
            3,
        )),
        optimal_config: vec![0, 0, 1, 1, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/kclique.rs"]
mod tests;
