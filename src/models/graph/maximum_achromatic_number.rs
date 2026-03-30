//! Maximum Achromatic Number problem implementation.
//!
//! Given a graph G = (V, E), find a proper coloring that uses the maximum
//! number of colors such that the coloring is also complete: for every pair
//! of distinct colors, there exists an edge connecting a vertex of one color
//! to a vertex of the other.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::Max;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumAchromaticNumber",
        display_name: "Maximum Achromatic Number",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Find a complete proper coloring maximizing the number of colors",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
        ],
    }
}

/// The Maximum Achromatic Number problem.
///
/// Given a graph G = (V, E), find a proper coloring of the vertices using the
/// maximum number of colors such that the coloring is *complete*: for every
/// pair of distinct colors used, there exists at least one edge between a
/// vertex of one color and a vertex of the other.
///
/// Variables: one per vertex, each selecting a color class (0..n-1).
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaximumAchromaticNumber;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // C6: achromatic number is 3
/// let graph = SimpleGraph::new(6, vec![(0,1),(1,2),(2,3),(3,4),(4,5),(5,0)]);
/// let problem = MaximumAchromaticNumber::new(graph);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem).unwrap();
/// let value = problem.evaluate(&solution);
/// assert_eq!(value, problemreductions::types::Max(Some(3)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximumAchromaticNumber<G> {
    /// The underlying graph.
    graph: G,
}

impl<G: Graph> MaximumAchromaticNumber<G> {
    /// Create a MaximumAchromaticNumber problem from a graph.
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

    /// Check whether a configuration is a proper coloring.
    ///
    /// A proper coloring assigns colors to vertices such that no two adjacent
    /// vertices share the same color.
    pub fn is_proper_coloring(&self, config: &[usize]) -> bool {
        for (u, v) in self.graph.edges() {
            if config[u] == config[v] {
                return false;
            }
        }
        true
    }

    /// Check whether a proper coloring is complete.
    ///
    /// A coloring is complete if for every pair of distinct colors used,
    /// there exists an edge between a vertex of one color and a vertex
    /// of the other.
    pub fn is_complete_coloring(&self, config: &[usize]) -> bool {
        let used_colors: HashSet<usize> = config.iter().copied().collect();
        let colors: Vec<usize> = used_colors.into_iter().collect();

        for i in 0..colors.len() {
            for j in (i + 1)..colors.len() {
                let c1 = colors[i];
                let c2 = colors[j];
                let has_edge = self.graph.edges().iter().any(|&(u, v)| {
                    (config[u] == c1 && config[v] == c2) || (config[u] == c2 && config[v] == c1)
                });
                if !has_edge {
                    return false;
                }
            }
        }
        true
    }
}

impl<G> Problem for MaximumAchromaticNumber<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "MaximumAchromaticNumber";
    type Value = Max<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.graph.num_vertices(); self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> Max<usize> {
        if config.len() != self.graph.num_vertices() {
            return Max(None);
        }
        if self.graph.num_vertices() == 0 {
            return Max(Some(0));
        }
        if !self.is_proper_coloring(config) {
            return Max(None);
        }
        if !self.is_complete_coloring(config) {
            return Max(None);
        }
        let distinct_colors: HashSet<usize> = config.iter().copied().collect();
        Max(Some(distinct_colors.len()))
    }
}

crate::declare_variants! {
    default MaximumAchromaticNumber<SimpleGraph> => "num_vertices^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // C6: 6-cycle, achromatic number = 3
    // Coloring [0, 1, 2, 0, 1, 2] uses 3 colors and is both proper and complete.
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "maximum_achromatic_number_simplegraph",
        instance: Box::new(MaximumAchromaticNumber::new(SimpleGraph::new(
            6,
            vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0)],
        ))),
        optimal_config: vec![0, 1, 2, 0, 1, 2],
        optimal_value: serde_json::json!(3),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/maximum_achromatic_number.rs"]
mod tests;
