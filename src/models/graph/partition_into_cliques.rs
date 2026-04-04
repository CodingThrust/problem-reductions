//! Partition Into Cliques problem implementation.
//!
//! Given a graph G = (V, E) and a positive integer K <= |V|, determine whether
//! the vertex set can be partitioned into k <= K groups such that the subgraph
//! induced by each group is a complete subgraph (clique).

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::variant::VariantParam;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "PartitionIntoCliques",
        display_name: "Partition into Cliques",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Partition vertices into K groups each inducing a clique",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "num_cliques", type_name: "usize", description: "num_cliques: maximum number of clique groups K (>= 1)" },
        ],
    }
}

/// The Partition Into Cliques problem.
///
/// Given a graph G = (V, E) and a positive integer K <= |V|, determine whether
/// the vertices can be partitioned into k <= K groups V_1, ..., V_k such that
/// the subgraph induced by each V_i is a complete subgraph (clique).
///
/// # Type Parameters
///
/// * `G` - Graph type (e.g., SimpleGraph)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::PartitionIntoCliques;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Two triangles: 0-1-2-0 and 3-4-5-3
/// let graph = SimpleGraph::new(6, vec![(0,1),(0,2),(1,2),(3,4),(3,5),(4,5)]);
/// let problem = PartitionIntoCliques::new(graph, 3);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct PartitionIntoCliques<G> {
    /// The underlying graph.
    graph: G,
    /// Maximum number of clique groups.
    num_cliques: usize,
}

impl<G: Graph> PartitionIntoCliques<G> {
    /// Create a new Partition Into Cliques instance.
    ///
    /// # Panics
    /// Panics if `num_cliques` is zero or greater than `graph.num_vertices()`.
    pub fn new(graph: G, num_cliques: usize) -> Self {
        assert!(num_cliques >= 1, "num_cliques must be at least 1");
        assert!(
            num_cliques <= graph.num_vertices(),
            "num_cliques must be at most num_vertices"
        );
        Self { graph, num_cliques }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the maximum number of clique groups.
    pub fn num_cliques(&self) -> usize {
        self.num_cliques
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
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
        crate::types::Or(is_valid_clique_partition(
            &self.graph,
            self.num_cliques,
            config,
        ))
    }
}

/// Check whether `config` is a valid K-clique partition of `graph`.
fn is_valid_clique_partition<G: Graph>(graph: &G, num_cliques: usize, config: &[usize]) -> bool {
    let n = graph.num_vertices();

    // Basic validity checks
    if config.len() != n {
        return false;
    }
    if config.iter().any(|&c| c >= num_cliques) {
        return false;
    }

    // For each group, collect the vertices and check all pairs are adjacent.
    for group in 0..num_cliques {
        let members: Vec<usize> = (0..n).filter(|&v| config[v] == group).collect();
        for i in 0..members.len() {
            for j in (i + 1)..members.len() {
                if !graph.has_edge(members[i], members[j]) {
                    return false;
                }
            }
        }
    }

    true
}

crate::declare_variants! {
    default PartitionIntoCliques<SimpleGraph> => "2^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "partition_into_cliques_simplegraph",
        instance: Box::new(PartitionIntoCliques::new(
            SimpleGraph::new(
                6,
                vec![
                    (0, 1),
                    (0, 2),
                    (1, 2),
                    (3, 4),
                    (3, 5),
                    (4, 5),
                    (0, 3),
                    (1, 4),
                    (2, 5),
                ],
            ),
            3,
        )),
        optimal_config: vec![0, 0, 0, 1, 1, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/partition_into_cliques.rs"]
mod tests;
