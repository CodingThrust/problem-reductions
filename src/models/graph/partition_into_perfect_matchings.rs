//! Partition Into Perfect Matchings problem implementation.
//!
//! Given a graph G = (V, E) and a positive integer K <= |V|, determine whether
//! the vertex set can be partitioned into k <= K groups such that the subgraph
//! induced by each group is a perfect matching (every vertex in the group has
//! exactly one neighbor within the group).

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::variant::VariantParam;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "PartitionIntoPerfectMatchings",
        display_name: "Partition into Perfect Matchings",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Partition vertices into K groups each inducing a perfect matching",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "num_matchings", type_name: "usize", description: "num_matchings: maximum number of matching groups K (>= 1)" },
        ],
    }
}

/// The Partition Into Perfect Matchings problem.
///
/// Given a graph G = (V, E) and a positive integer K <= |V|, determine whether
/// the vertices can be partitioned into k <= K groups V_1, ..., V_k such that
/// the subgraph induced by each V_i is a perfect matching: every vertex in V_i
/// has exactly one neighbor also in V_i.
///
/// # Type Parameters
///
/// * `G` - Graph type (e.g., SimpleGraph)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::PartitionIntoPerfectMatchings;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 4 vertices with edges: (0,1),(2,3),(0,2),(1,3)
/// let graph = SimpleGraph::new(4, vec![(0,1),(2,3),(0,2),(1,3)]);
/// let problem = PartitionIntoPerfectMatchings::new(graph, 2);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct PartitionIntoPerfectMatchings<G> {
    /// The underlying graph.
    graph: G,
    /// Maximum number of matching groups.
    num_matchings: usize,
}

impl<G: Graph> PartitionIntoPerfectMatchings<G> {
    /// Create a new Partition Into Perfect Matchings instance.
    ///
    /// # Panics
    /// Panics if `num_matchings` is zero or greater than `graph.num_vertices()`.
    pub fn new(graph: G, num_matchings: usize) -> Self {
        assert!(num_matchings >= 1, "num_matchings must be at least 1");
        assert!(
            num_matchings <= graph.num_vertices(),
            "num_matchings must be at most num_vertices"
        );
        Self {
            graph,
            num_matchings,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the maximum number of matching groups.
    pub fn num_matchings(&self) -> usize {
        self.num_matchings
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

impl<G> Problem for PartitionIntoPerfectMatchings<G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "PartitionIntoPerfectMatchings";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_matchings; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or(is_valid_perfect_matching_partition(
            &self.graph,
            self.num_matchings,
            config,
        ))
    }
}

/// Check whether `config` is a valid K-perfect-matching partition of `graph`.
fn is_valid_perfect_matching_partition<G: Graph>(
    graph: &G,
    num_matchings: usize,
    config: &[usize],
) -> bool {
    let n = graph.num_vertices();

    // Basic validity checks
    if config.len() != n {
        return false;
    }
    if config.iter().any(|&c| c >= num_matchings) {
        return false;
    }

    // For each group, collect the vertices and check every vertex has exactly
    // one neighbor within the group (i.e., the induced subgraph is a perfect matching).
    for group in 0..num_matchings {
        let members: Vec<usize> = (0..n).filter(|&v| config[v] == group).collect();
        // Empty groups are OK
        if members.is_empty() {
            continue;
        }
        // A perfect matching requires an even number of vertices
        if !members.len().is_multiple_of(2) {
            return false;
        }
        // Each member must have exactly one neighbor in the group
        for &v in &members {
            let neighbor_count = members
                .iter()
                .filter(|&&u| u != v && graph.has_edge(v, u))
                .count();
            if neighbor_count != 1 {
                return false;
            }
        }
    }

    true
}

crate::declare_variants! {
    default PartitionIntoPerfectMatchings<SimpleGraph> => "num_matchings^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "partition_into_perfect_matchings_simplegraph",
        instance: Box::new(PartitionIntoPerfectMatchings::new(
            SimpleGraph::new(4, vec![(0, 1), (2, 3), (0, 2), (1, 3)]),
            2,
        )),
        optimal_config: vec![0, 0, 1, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/partition_into_perfect_matchings.rs"]
mod tests;
