//! Partition Into Forests problem implementation.
//!
//! Given a graph G = (V, E) and a positive integer K, determine whether the
//! vertex set can be partitioned into K subsets such that the subgraph induced
//! by each subset is a forest (acyclic graph).

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::variant::VariantParam;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "PartitionIntoForests",
        display_name: "Partition into Forests",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Partition vertices into K classes each inducing an acyclic subgraph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "num_forests", type_name: "usize", description: "num_forests: number of forest classes K (>= 1)" },
        ],
    }
}

/// The Partition Into Forests problem.
///
/// Given a graph G = (V, E) and a positive integer K, determine whether the
/// vertices can be partitioned into K classes V_1, ..., V_K such that the
/// subgraph induced by each V_i is a forest (contains no cycle).
///
/// # Type Parameters
///
/// * `G` - Graph type (e.g., SimpleGraph)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::PartitionIntoForests;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Graph containing two triangles; K=2 forests suffice
/// let graph = SimpleGraph::new(6, vec![(0,1),(1,2),(2,0),(2,3),(3,4),(4,5),(5,3)]);
/// let problem = PartitionIntoForests::new(graph, 2);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct PartitionIntoForests<G> {
    /// The underlying graph.
    graph: G,
    /// Number of forest classes.
    num_forests: usize,
}

impl<G: Graph> PartitionIntoForests<G> {
    /// Create a new Partition Into Forests instance.
    ///
    /// # Panics
    /// Panics if `num_forests` is zero.
    pub fn new(graph: G, num_forests: usize) -> Self {
        assert!(num_forests >= 1, "num_forests must be at least 1");
        Self { graph, num_forests }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the number of forest classes.
    pub fn num_forests(&self) -> usize {
        self.num_forests
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

impl<G> Problem for PartitionIntoForests<G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "PartitionIntoForests";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_forests; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or(is_valid_forest_partition(
            &self.graph,
            self.num_forests,
            config,
        ))
    }
}

/// Check whether `config` is a valid K-forest partition of `graph`.
fn is_valid_forest_partition<G: Graph>(graph: &G, num_forests: usize, config: &[usize]) -> bool {
    let n = graph.num_vertices();

    // Basic validity checks
    if config.len() != n {
        return false;
    }
    if config.iter().any(|&c| c >= num_forests) {
        return false;
    }

    // For each forest class, verify the induced subgraph is acyclic using union-find.
    // An undirected graph is acyclic iff union-find never sees an edge (u, v) where
    // u and v already share a component.
    let mut parent: Vec<usize> = (0..n).collect();

    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    for (u, v) in graph.edges() {
        if config[u] != config[v] {
            // Edge crosses classes — not in any induced subgraph
            continue;
        }
        // Both u and v are in the same class; check for cycle
        let ru = find(&mut parent, u);
        let rv = find(&mut parent, v);
        if ru == rv {
            return false; // Cycle detected
        }
        parent[ru] = rv; // Union
    }

    true
}

crate::declare_variants! {
    default PartitionIntoForests<SimpleGraph> => "num_forests^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "partition_into_forests_simplegraph",
        instance: Box::new(PartitionIntoForests::new(
            SimpleGraph::new(
                6,
                vec![(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 5), (5, 3)],
            ),
            2,
        )),
        // V0={0,3}: edges from graph in class 0: none among {0,3} → forest
        // V1={1,2,4,5}: edges (1,2),(3,4) but 3∉V1; edges among V1: (1,2),(4,5) → path forest
        optimal_config: vec![0, 1, 1, 0, 1, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/partition_into_forests.rs"]
mod tests;
