//! Minimum Covering by Cliques problem implementation.
//!
//! Given a graph G = (V, E), find a minimum number of cliques whose union
//! covers every edge in E.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumCoveringByCliques",
        display_name: "Minimum Covering by Cliques",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Find minimum number of cliques covering all edges",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
        ],
    }
}

/// The Minimum Covering by Cliques problem.
///
/// Given a graph G = (V, E), find a collection of cliques C_1, ..., C_k
/// in G such that every edge is contained in at least one clique,
/// and k is minimized.
///
/// Variables: one per edge, each selecting which clique group covers it.
/// Each edge can be assigned to one of at most |E| groups (upper bound).
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumCoveringByCliques;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Triangle: 3 edges can be covered by 1 clique
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
/// let problem = MinimumCoveringByCliques::new(graph);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem).unwrap();
/// let value = problem.evaluate(&solution);
/// assert_eq!(value, problemreductions::types::Min(Some(1)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumCoveringByCliques<G> {
    /// The underlying graph.
    graph: G,
}

impl<G: Graph> MinimumCoveringByCliques<G> {
    /// Create a MinimumCoveringByCliques problem from a graph.
    pub fn new(graph: G) -> Self {
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

    /// Check whether a configuration is a valid edge clique cover.
    ///
    /// For each group index used, the edges assigned to it must form a clique:
    /// all vertices touched by those edges must be pairwise adjacent.
    pub fn is_valid_cover(&self, config: &[usize]) -> bool {
        let edges = self.graph.edges();
        let num_edges = edges.len();

        if config.len() != num_edges {
            return false;
        }

        // Collect vertices per group and check clique property.
        let max_group = match config.iter().max() {
            Some(&m) => m,
            None => return true, // no edges → trivially valid
        };

        for group in 0..=max_group {
            let vertices: HashSet<usize> = config
                .iter()
                .enumerate()
                .filter(|(_, &g)| g == group)
                .flat_map(|(idx, _)| {
                    let (u, v) = edges[idx];
                    [u, v]
                })
                .collect();

            let verts: Vec<usize> = vertices.into_iter().collect();
            for i in 0..verts.len() {
                for j in (i + 1)..verts.len() {
                    if !self.graph.has_edge(verts[i], verts[j]) {
                        return false;
                    }
                }
            }
        }

        true
    }
}

impl<G> Problem for MinimumCoveringByCliques<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumCoveringByCliques";
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.graph.num_edges(); self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        if config.len() != self.graph.num_edges() {
            return Min(None);
        }
        if self.graph.num_edges() == 0 {
            return Min(Some(0));
        }
        if !self.is_valid_cover(config) {
            return Min(None);
        }
        let distinct_groups: HashSet<usize> = config.iter().copied().collect();
        Min(Some(distinct_groups.len()))
    }
}

crate::declare_variants! {
    default MinimumCoveringByCliques<SimpleGraph> => "2^num_edges",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 6 vertices, 9 edges:
    // (0,1),(1,2),(2,3),(3,0),(0,2),(4,0),(4,1),(5,2),(5,3)
    // Optimal: 4 cliques
    // edges 0,1,4 -> group 0 (clique {0,1,2})
    // edges 2,3 -> group 1 (clique {0,2,3}... wait, (2,3) and (3,0) -> vertices {0,2,3})
    // edges 5,6 -> group 2 (clique {0,1,4})
    // edges 7,8 -> group 3 (clique {2,3,5})
    // Config: [0, 0, 1, 1, 0, 2, 2, 3, 3]
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_covering_by_cliques_simplegraph",
        instance: Box::new(MinimumCoveringByCliques::new(SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 0),
                (0, 2),
                (4, 0),
                (4, 1),
                (5, 2),
                (5, 3),
            ],
        ))),
        optimal_config: vec![0, 0, 1, 1, 0, 2, 2, 3, 3],
        optimal_value: serde_json::json!(4),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_covering_by_cliques.rs"]
mod tests;
