//! MinimumCutIntoBoundedSets problem implementation.
//!
//! A graph partitioning problem that asks whether vertices can be partitioned
//! into two bounded-size sets (containing designated source and sink vertices)
//! with total cut weight at most K. From Garey & Johnson, A2 ND17.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use crate::types::WeightElement;
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumCutIntoBoundedSets",
        display_name: "Minimum Cut Into Bounded Sets",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i32", &["i32"]),
        ],
        module_path: module_path!(),
        description: "Partition vertices into two bounded-size sets with cut weight at most K",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The undirected graph G = (V, E)" },
            FieldInfo { name: "edge_weights", type_name: "Vec<W>", description: "Edge weights w: E -> Z+" },
            FieldInfo { name: "source", type_name: "usize", description: "Source vertex s (must be in V1)" },
            FieldInfo { name: "sink", type_name: "usize", description: "Sink vertex t (must be in V2)" },
            FieldInfo { name: "size_bound", type_name: "usize", description: "Maximum size B for each partition set" },
            FieldInfo { name: "cut_bound", type_name: "W::Sum", description: "Maximum total cut weight K" },
        ],
    }
}

/// Minimum Cut Into Bounded Sets (Garey & Johnson ND17).
///
/// Given a weighted graph G = (V, E), source vertex s, sink vertex t,
/// size bound B, and cut bound K, determine whether there exists a partition
/// of V into disjoint sets V1 and V2 such that:
/// - s is in V1, t is in V2
/// - |V1| <= B, |V2| <= B
/// - The total weight of edges crossing the partition is at most K
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
/// * `W` - The weight type for edges (e.g., `i32`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumCutIntoBoundedSets;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Simple 4-vertex path graph with unit weights, s=0, t=3
/// let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
/// let problem = MinimumCutIntoBoundedSets::new(graph, vec![1, 1, 1], 0, 3, 3, 2);
///
/// // Partition {0,1} vs {2,3}: cut edge (1,2) with weight 1 <= 2
/// assert!(problem.evaluate(&[0, 0, 1, 1]));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumCutIntoBoundedSets<G, W: WeightElement> {
    /// The underlying graph structure.
    graph: G,
    /// Weights for each edge (in the same order as graph.edges()).
    edge_weights: Vec<W>,
    /// Source vertex s that must be in V1.
    source: usize,
    /// Sink vertex t that must be in V2.
    sink: usize,
    /// Maximum size B for each partition set.
    size_bound: usize,
    /// Maximum total cut weight K.
    cut_bound: W::Sum,
}

impl<G: Graph, W: WeightElement> MinimumCutIntoBoundedSets<G, W> {
    /// Create a new MinimumCutIntoBoundedSets problem.
    ///
    /// # Arguments
    /// * `graph` - The undirected graph
    /// * `edge_weights` - Weights for each edge (must match graph.num_edges())
    /// * `source` - Source vertex s (must be in V1)
    /// * `sink` - Sink vertex t (must be in V2)
    /// * `size_bound` - Maximum size B for each partition set
    /// * `cut_bound` - Maximum total cut weight K
    ///
    /// # Panics
    /// Panics if edge_weights length doesn't match num_edges, if source == sink,
    /// or if source/sink are out of bounds.
    pub fn new(
        graph: G,
        edge_weights: Vec<W>,
        source: usize,
        sink: usize,
        size_bound: usize,
        cut_bound: W::Sum,
    ) -> Self {
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        assert!(source < graph.num_vertices(), "source vertex out of bounds");
        assert!(sink < graph.num_vertices(), "sink vertex out of bounds");
        assert_ne!(source, sink, "source and sink must be different vertices");
        Self {
            graph,
            edge_weights,
            source,
            sink,
            size_bound,
            cut_bound,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the edge weights.
    pub fn edge_weights(&self) -> &[W] {
        &self.edge_weights
    }

    /// Get the source vertex.
    pub fn source(&self) -> usize {
        self.source
    }

    /// Get the sink vertex.
    pub fn sink(&self) -> usize {
        self.sink
    }

    /// Get the size bound B.
    pub fn size_bound(&self) -> usize {
        self.size_bound
    }

    /// Get the cut bound K.
    pub fn cut_bound(&self) -> &W::Sum {
        &self.cut_bound
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

impl<G, W> Problem for MinimumCutIntoBoundedSets<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumCutIntoBoundedSets";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let n = self.graph.num_vertices();
        if config.len() != n {
            return false;
        }

        // Check source is in V1 (config=0) and sink is in V2 (config=1)
        if config[self.source] != 0 {
            return false;
        }
        if config[self.sink] != 1 {
            return false;
        }

        // Check size bounds
        let count_v1 = config.iter().filter(|&&x| x == 0).count();
        let count_v2 = config.iter().filter(|&&x| x == 1).count();
        if count_v1 > self.size_bound || count_v2 > self.size_bound {
            return false;
        }

        // Compute cut weight
        let mut cut_weight = W::Sum::zero();
        for ((u, v), weight) in self.graph.edges().iter().zip(self.edge_weights.iter()) {
            if config[*u] != config[*v] {
                cut_weight += weight.to_sum();
            }
        }

        // Check cut weight <= K
        cut_weight <= self.cut_bound
    }
}

impl<G, W> SatisfactionProblem for MinimumCutIntoBoundedSets<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_cut_into_bounded_sets_i32",
        instance: Box::new(MinimumCutIntoBoundedSets::new(
            SimpleGraph::new(
                8,
                vec![
                    (0, 1),
                    (0, 2),
                    (1, 2),
                    (1, 3),
                    (2, 4),
                    (3, 5),
                    (3, 6),
                    (4, 5),
                    (4, 6),
                    (5, 7),
                    (6, 7),
                    (5, 6),
                ],
            ),
            vec![2, 3, 1, 4, 2, 1, 3, 2, 1, 2, 3, 1],
            0,
            7,
            5,
            6,
        )),
        // V1={0,1,2,3}, V2={4,5,6,7}: cut edges (2,4)=2,(3,5)=1,(3,6)=3 => 6
        optimal_config: vec![0, 0, 0, 0, 1, 1, 1, 1],
        optimal_value: serde_json::json!(true),
    }]
}

crate::declare_variants! {
    default sat MinimumCutIntoBoundedSets<SimpleGraph, i32> => "2^num_vertices",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_cut_into_bounded_sets.rs"]
mod tests;
