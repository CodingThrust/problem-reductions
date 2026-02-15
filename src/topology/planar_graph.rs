//! Planar graph — validated wrapper around SimpleGraph.

use super::graph::{Graph, SimpleGraph};
use serde::{Deserialize, Serialize};

/// Planar graph — validated wrapper around SimpleGraph.
///
/// Construction validates the necessary planarity condition: |E| <= 3|V| - 6 for |V| >= 3.
/// This is a necessary but not sufficient condition.
///
/// # Example
///
/// ```
/// use problemreductions::topology::{PlanarGraph, Graph};
///
/// // K4 is planar: 4 vertices, 6 edges, 6 <= 3*4 - 6 = 6
/// let edges = vec![(0,1),(0,2),(0,3),(1,2),(1,3),(2,3)];
/// let g = PlanarGraph::new(4, edges);
/// assert_eq!(g.num_vertices(), 4);
/// assert_eq!(g.num_edges(), 6);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanarGraph {
    inner: SimpleGraph,
}

impl PlanarGraph {
    /// Create a new planar graph.
    ///
    /// # Panics
    /// Panics if the graph violates the necessary planarity condition |E| <= 3|V| - 6.
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self {
        let inner = SimpleGraph::new(num_vertices, edges);
        if num_vertices >= 3 {
            let max_edges = 3 * num_vertices - 6;
            assert!(
                inner.num_edges() <= max_edges,
                "graph has {} edges but a planar graph on {} vertices can have at most {} edges",
                inner.num_edges(),
                num_vertices,
                max_edges
            );
        }
        Self { inner }
    }

    /// Get a reference to the underlying SimpleGraph.
    pub fn inner(&self) -> &SimpleGraph {
        &self.inner
    }
}

impl Graph for PlanarGraph {
    const NAME: &'static str = "PlanarGraph";

    fn num_vertices(&self) -> usize {
        self.inner.num_vertices()
    }

    fn num_edges(&self) -> usize {
        self.inner.num_edges()
    }

    fn edges(&self) -> Vec<(usize, usize)> {
        self.inner.edges()
    }

    fn has_edge(&self, u: usize, v: usize) -> bool {
        self.inner.has_edge(u, v)
    }

    fn neighbors(&self, v: usize) -> Vec<usize> {
        self.inner.neighbors(v)
    }
}

use crate::impl_variant_param;
impl_variant_param!(PlanarGraph, "graph", parent: SimpleGraph,
    cast: |g| g.inner.clone());

#[cfg(test)]
#[path = "../unit_tests/topology/planar_graph.rs"]
mod tests;
