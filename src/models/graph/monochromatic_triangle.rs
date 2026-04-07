//! Monochromatic Triangle problem implementation.
//!
//! Given a graph G = (V, E), determine whether the edges of G can be 2-colored
//! (each edge assigned color 0 or 1) so that no triangle is monochromatic,
//! i.e., no three mutually adjacent vertices have all three connecting edges
//! the same color.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::variant::VariantParam;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MonochromaticTriangle",
        display_name: "Monochromatic Triangle",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "2-color edges so that no triangle is monochromatic",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
        ],
    }
}

/// The Monochromatic Triangle problem.
///
/// Given a graph G = (V, E), determine whether the edges of G can be 2-colored
/// so that no triangle (three mutually adjacent vertices) has all three edges
/// the same color.
///
/// Each configuration entry corresponds to an edge (in the order returned by
/// `graph.edges()`), with value 0 or 1 representing the two colors.
///
/// # Type Parameters
///
/// * `G` - Graph type (e.g., SimpleGraph)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MonochromaticTriangle;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // K4: complete graph on 4 vertices
/// let graph = SimpleGraph::new(4, vec![(0,1),(0,2),(0,3),(1,2),(1,3),(2,3)]);
/// let problem = MonochromaticTriangle::new(graph);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct MonochromaticTriangle<G> {
    /// The underlying graph.
    graph: G,
    /// Precomputed list of triangles, each stored as three edge indices.
    triangles: Vec<[usize; 3]>,
    /// Ordered edge list (mirrors `graph.edges()` order).
    edge_list: Vec<(usize, usize)>,
}

impl<G: Graph> MonochromaticTriangle<G> {
    /// Create a new Monochromatic Triangle instance.
    pub fn new(graph: G) -> Self {
        let edge_list = graph.edges();
        // Build edge-to-index mapping: (min(u,v), max(u,v)) -> index
        let mut edge_index: HashMap<(usize, usize), usize> = HashMap::new();
        for (idx, &(u, v)) in edge_list.iter().enumerate() {
            let key = if u < v { (u, v) } else { (v, u) };
            edge_index.insert(key, idx);
        }

        // Find all triangles: for each triple (u, v, w) with u < v < w,
        // check if all three edges exist.
        let n = graph.num_vertices();
        let mut triangles = Vec::new();
        for u in 0..n {
            for v in (u + 1)..n {
                if !graph.has_edge(u, v) {
                    continue;
                }
                for w in (v + 1)..n {
                    if graph.has_edge(u, w) && graph.has_edge(v, w) {
                        let e_uv = edge_index[&(u, v)];
                        let e_uw = edge_index[&(u, w)];
                        let e_vw = edge_index[&(v, w)];
                        triangles.push([e_uv, e_uw, e_vw]);
                    }
                }
            }
        }

        Self {
            graph,
            triangles,
            edge_list,
        }
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

    /// Get the precomputed list of triangles (as edge-index triples).
    pub fn triangles(&self) -> &[[usize; 3]] {
        &self.triangles
    }

    /// Get the number of triangles in the graph.
    pub fn num_triangles(&self) -> usize {
        self.triangles.len()
    }

    /// Get the ordered edge list.
    pub fn edge_list(&self) -> &[(usize, usize)] {
        &self.edge_list
    }
}

impl<G> Problem for MonochromaticTriangle<G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "MonochromaticTriangle";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.edge_list.len()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or({
            if config.len() != self.edge_list.len() {
                return crate::types::Or(false);
            }

            // Check each triangle: if all three edges have the same color,
            // the coloring is invalid.
            for tri in &self.triangles {
                let c0 = config[tri[0]];
                let c1 = config[tri[1]];
                let c2 = config[tri[2]];
                if c0 == c1 && c1 == c2 {
                    return crate::types::Or(false);
                }
            }

            true
        })
    }
}

crate::declare_variants! {
    default MonochromaticTriangle<SimpleGraph> => "2^num_edges",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // K4: 4 vertices, 6 edges, has a valid 2-coloring avoiding monochromatic triangles.
    // Edges in order: (0,1),(0,2),(0,3),(1,2),(1,3),(2,3)
    // Config [0,0,1,1,0,1]:
    //   Triangle (0,1,2): edges 0,1,3 -> colors 0,0,1 -> not monochromatic
    //   Triangle (0,1,3): edges 0,2,4 -> colors 0,1,0 -> not monochromatic
    //   Triangle (0,2,3): edges 1,2,5 -> colors 0,1,1 -> not monochromatic
    //   Triangle (1,2,3): edges 3,4,5 -> colors 1,0,1 -> not monochromatic
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "monochromatic_triangle_simplegraph",
        instance: Box::new(MonochromaticTriangle::new(SimpleGraph::new(
            4,
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
        ))),
        optimal_config: vec![0, 0, 1, 1, 0, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/monochromatic_triangle.rs"]
mod tests;
