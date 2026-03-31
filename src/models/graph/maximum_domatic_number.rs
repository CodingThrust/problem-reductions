//! Maximum Domatic Number problem implementation.
//!
//! The Maximum Domatic Number problem asks for the maximum number k such that the
//! vertex set V of a graph G=(V,E) can be partitioned into k disjoint dominating sets.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::Max;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumDomaticNumber",
        display_name: "Maximum Domatic Number",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Find maximum number of disjoint dominating sets partitioning V",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
        ],
    }
}

/// The Maximum Domatic Number problem.
///
/// Given a graph G = (V, E), find the maximum k such that V can be partitioned
/// into k disjoint dominating sets. A dominating set D ⊆ V is a set such that
/// every vertex is either in D or adjacent to a vertex in D.
///
/// The configuration assigns each vertex to a set index (0..n-1). The value is
/// `Max(Some(k))` where k is the number of non-empty sets if all non-empty sets
/// are dominating, or `Max(None)` if any non-empty set fails domination.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaximumDomaticNumber;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Path graph P3: 0-1-2
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
/// let problem = MaximumDomaticNumber::new(graph);
///
/// let solver = BruteForce::new();
/// let witness = solver.find_witness(&problem).unwrap();
/// let value = problem.evaluate(&witness);
/// // Domatic number of P3 is 2
/// assert_eq!(value, problemreductions::types::Max(Some(2)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximumDomaticNumber<G> {
    /// The underlying graph.
    graph: G,
}

impl<G: Graph> MaximumDomaticNumber<G> {
    /// Create a Maximum Domatic Number problem from a graph.
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

    /// Check whether a partition is valid (all non-empty sets are dominating).
    ///
    /// Returns `Some(k)` where k is the number of non-empty dominating sets,
    /// or `None` if any non-empty set fails the domination property.
    fn evaluate_partition(&self, config: &[usize]) -> Option<usize> {
        let n = self.graph.num_vertices();

        // Collect which vertices belong to each set
        let mut sets: Vec<Vec<usize>> = vec![vec![]; n];
        for (v, &set_idx) in config.iter().enumerate() {
            sets[set_idx].push(v);
        }

        // Check each non-empty set is a dominating set
        let mut count = 0;
        for set in &sets {
            if set.is_empty() {
                continue;
            }
            count += 1;

            // Build membership lookup
            let mut in_set = vec![false; n];
            for &v in set {
                in_set[v] = true;
            }

            // Every vertex must be in the set or adjacent to someone in the set
            for v in 0..n {
                if in_set[v] {
                    continue;
                }
                if !self.graph.neighbors(v).iter().any(|&u| in_set[u]) {
                    return None;
                }
            }
        }

        Some(count)
    }
}

impl<G> Problem for MaximumDomaticNumber<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "MaximumDomaticNumber";
    type Value = Max<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.graph.num_vertices();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> Max<usize> {
        match self.evaluate_partition(config) {
            Some(k) => Max(Some(k)),
            None => Max(None),
        }
    }
}

crate::declare_variants! {
    default MaximumDomaticNumber<SimpleGraph> => "2.695^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "maximum_domatic_number_simplegraph",
        instance: Box::new(MaximumDomaticNumber::new(SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 4),
                (2, 5),
                (3, 4),
                (3, 5),
                (4, 5),
            ],
        ))),
        optimal_config: vec![0, 1, 2, 0, 2, 1],
        optimal_value: serde_json::json!(3),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/maximum_domatic_number.rs"]
mod tests;
