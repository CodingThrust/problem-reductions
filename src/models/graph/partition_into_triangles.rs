//! Partition Into Triangles problem implementation.
//!
//! Given a graph G = (V, E) where |V| = 3q, determine whether V can be
//! partitioned into q triples, each forming a triangle (K3) in G.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use crate::variant::VariantParam;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "PartitionIntoTriangles",
        module_path: module_path!(),
        description: "Partition vertices into triangles (K3 subgraphs)",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E) with |V| divisible by 3" },
        ],
    }
}

/// The Partition Into Triangles problem.
///
/// Given a graph G = (V, E) where |V| = 3q, determine whether V can be
/// partitioned into q triples, each forming a triangle (K3) in G.
///
/// # Type Parameters
///
/// * `G` - Graph type (e.g., SimpleGraph)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::PartitionIntoTriangles;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Triangle graph: 3 vertices forming a single triangle
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
/// let problem = PartitionIntoTriangles::new(graph);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct PartitionIntoTriangles<G> {
    /// The underlying graph.
    graph: G,
}

impl<G: Graph> PartitionIntoTriangles<G> {
    /// Create a new Partition Into Triangles problem from a graph.
    ///
    /// # Panics
    /// Panics if the number of vertices is not divisible by 3.
    pub fn new(graph: G) -> Self {
        assert!(
            graph.num_vertices().is_multiple_of(3),
            "Number of vertices ({}) must be divisible by 3",
            graph.num_vertices()
        );
        Self { graph }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
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

impl<G> Problem for PartitionIntoTriangles<G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "PartitionIntoTriangles";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        let q = self.graph.num_vertices() / 3;
        vec![q; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let n = self.graph.num_vertices();
        let q = n / 3;

        // Check config length
        if config.len() != n {
            return false;
        }

        // Check all values are in range [0, q)
        if config.iter().any(|&c| c >= q) {
            return false;
        }

        // Count vertices per group
        let mut counts = vec![0usize; q];
        for &c in config {
            counts[c] += 1;
        }

        // Each group must have exactly 3 vertices
        if counts.iter().any(|&c| c != 3) {
            return false;
        }

        // Build per-group vertex lists in a single pass over config.
        let mut group_verts = vec![[0usize; 3]; q];
        let mut group_pos = vec![0usize; q];

        for (v, &g) in config.iter().enumerate() {
            let pos = group_pos[g];
            group_verts[g][pos] = v;
            group_pos[g] = pos + 1;
        }

        // Check each group forms a triangle
        for verts in &group_verts {
            if !self.graph.has_edge(verts[0], verts[1]) {
                return false;
            }
            if !self.graph.has_edge(verts[0], verts[2]) {
                return false;
            }
            if !self.graph.has_edge(verts[1], verts[2]) {
                return false;
            }
        }

        true
    }
}

impl<G: Graph + VariantParam> SatisfactionProblem for PartitionIntoTriangles<G> {}

crate::declare_variants! {
    PartitionIntoTriangles<SimpleGraph> => "2^num_vertices",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/partition_into_triangles.rs"]
mod tests;
