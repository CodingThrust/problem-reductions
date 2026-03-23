//! Partition into Paths of Length 2 problem implementation.
//!
//! Given a graph G = (V, E) with |V| = 3q, determine whether V can be partitioned
//! into q disjoint sets of three vertices each, such that each set induces at least
//! two edges (i.e., a path of length 2 or a triangle).
//!
//! This is a classical NP-complete problem from Garey & Johnson, Chapter 3, Section 3.3, p.76.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, WitnessProblem};
use crate::variant::VariantParam;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "PartitionIntoPathsOfLength2",
        display_name: "Partition into Paths of Length 2",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Partition vertices into triples each inducing at least two edges (P3 or triangle)",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E) with |V| divisible by 3" },
        ],
    }
}

/// Partition into Paths of Length 2 problem.
///
/// Given a graph G = (V, E) with |V| = 3q for a positive integer q,
/// determine whether V can be partitioned into q disjoint sets
/// V_1, V_2, ..., V_q of three vertices each, such that each V_t
/// induces at least two edges in G.
///
/// Each triple must form either a path of length 2 (exactly 2 edges)
/// or a triangle (all 3 edges).
///
/// # Type Parameters
///
/// * `G` - Graph type (e.g., SimpleGraph)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::PartitionIntoPathsOfLength2;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 6-vertex graph with two P3 paths: 0-1-2 and 3-4-5
/// let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]);
/// let problem = PartitionIntoPathsOfLength2::new(graph);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct PartitionIntoPathsOfLength2<G> {
    /// The underlying graph.
    graph: G,
}

impl<G: Graph> PartitionIntoPathsOfLength2<G> {
    /// Create a new PartitionIntoPathsOfLength2 problem from a graph.
    ///
    /// # Panics
    /// Panics if `graph.num_vertices()` is not divisible by 3.
    pub fn new(graph: G) -> Self {
        assert_eq!(
            graph.num_vertices() % 3,
            0,
            "Number of vertices ({}) must be divisible by 3",
            graph.num_vertices()
        );
        Self { graph }
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

    /// Get q = |V| / 3, the number of groups in the partition.
    pub fn num_groups(&self) -> usize {
        self.graph.num_vertices() / 3
    }

    /// Check if a configuration represents a valid partition.
    ///
    /// A valid configuration assigns each vertex to a group (0..q-1) such that:
    /// 1. Each group contains exactly 3 vertices.
    /// 2. Each group induces at least 2 edges.
    pub fn is_valid_partition(&self, config: &[usize]) -> bool {
        let n = self.graph.num_vertices();
        let q = self.num_groups();

        if config.len() != n {
            return false;
        }

        // Check all assignments are in range
        if config.iter().any(|&g| g >= q) {
            return false;
        }

        // Count vertices per group
        let mut group_sizes = vec![0usize; q];
        for &g in config {
            group_sizes[g] += 1;
        }

        // Each group must have exactly 3 vertices
        if group_sizes.iter().any(|&s| s != 3) {
            return false;
        }

        // Check each group induces at least 2 edges (single pass over edges)
        let mut group_edge_counts = vec![0usize; q];
        for (u, v) in self.graph.edges() {
            if config[u] == config[v] {
                group_edge_counts[config[u]] += 1;
            }
        }
        if group_edge_counts.iter().any(|&c| c < 2) {
            return false;
        }

        true
    }
}

impl<G> Problem for PartitionIntoPathsOfLength2<G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "PartitionIntoPathsOfLength2";
    type Value = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        let q = self.num_groups();
        vec![q; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.is_valid_partition(config)
    }
}

impl<G: Graph + VariantParam> WitnessProblem for PartitionIntoPathsOfLength2<G> {}

crate::declare_variants! {
    default sat PartitionIntoPathsOfLength2<SimpleGraph> => "3^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "partition_into_paths_of_length_2_simplegraph",
        instance: Box::new(PartitionIntoPathsOfLength2::new(SimpleGraph::new(
            6,
            vec![(0, 1), (1, 2), (3, 4), (4, 5)],
        ))),
        optimal_config: vec![0, 0, 0, 1, 1, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/partition_into_paths_of_length_2.rs"]
mod tests;
