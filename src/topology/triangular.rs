//! Triangular lattice graph — a weighted unit disk graph on a triangular grid.
//!
//! This is a newtype wrapper around [`GridGraph`] with triangular geometry,
//! exposed as a distinct graph type for the reduction system.

use super::graph::Graph;
use super::grid_graph::GridGraph;
use serde::{Deserialize, Serialize};

/// A triangular lattice graph.
///
/// Wraps a [`GridGraph<i32>`] that uses triangular lattice geometry.
/// This is a subtype of `UnitDiskGraph` — all triangular lattice graphs
/// are unit disk graphs (and therefore also simple graphs).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Triangular(GridGraph<i32>);

impl Triangular {
    /// Create a new Triangular graph from a GridGraph.
    pub fn new(grid_graph: GridGraph<i32>) -> Self {
        Self(grid_graph)
    }

    /// Get a reference to the inner GridGraph.
    pub fn grid_graph(&self) -> &GridGraph<i32> {
        &self.0
    }

    /// Get the nodes of the graph.
    pub fn nodes(&self) -> &[super::grid_graph::GridNode<i32>] {
        self.0.nodes()
    }
}

impl Graph for Triangular {
    const NAME: &'static str = "Triangular";

    fn num_vertices(&self) -> usize {
        self.0.num_vertices()
    }

    fn num_edges(&self) -> usize {
        self.0.num_edges()
    }

    fn edges(&self) -> Vec<(usize, usize)> {
        Graph::edges(&self.0)
    }

    fn has_edge(&self, u: usize, v: usize) -> bool {
        self.0.has_edge(u, v)
    }

    fn neighbors(&self, v: usize) -> Vec<usize> {
        self.0.neighbors(v)
    }
}
