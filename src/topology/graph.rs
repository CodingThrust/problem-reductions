//! Graph trait and SimpleGraph implementation.
//!
//! This module provides a `Graph` trait that abstracts over different graph
//! representations, following Julia's Graphs.jl `AbstractGraph` pattern.
//!
//! Supported graph types:
//! - [`SimpleGraph`]: Standard unweighted graph (wrapper around petgraph)
//! - [`UnitDiskGraph`]: Vertices with 2D positions, edges based on distance
//! - [`HyperGraph`]: Edges can connect any number of vertices (via adapter)

use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};

/// Trait for graph types, following Julia's Graphs.jl AbstractGraph pattern.
///
/// This trait abstracts over different graph representations, allowing
/// problems to be generic over the underlying graph type.
///
/// # Example
///
/// ```rust,ignore
/// use problemreductions::topology::{Graph, SimpleGraph};
///
/// fn count_triangles<G: Graph>(graph: &G) -> usize {
///     let mut count = 0;
///     for u in 0..graph.num_vertices() {
///         for v in graph.neighbors(u) {
///             if v > u {
///                 for w in graph.neighbors(v) {
///                     if w > v && graph.has_edge(u, w) {
///                         count += 1;
///                     }
///                 }
///             }
///         }
///     }
///     count
/// }
/// ```
pub trait Graph: Clone + Send + Sync + 'static {
    /// The name of the graph type (e.g., "SimpleGraph", "GridGraph").
    const NAME: &'static str;

    /// Returns the number of vertices in the graph.
    fn num_vertices(&self) -> usize;

    /// Returns the number of edges in the graph.
    fn num_edges(&self) -> usize;

    /// Returns all edges as a list of (u, v) pairs.
    ///
    /// For undirected graphs, each edge appears once with u < v.
    fn edges(&self) -> Vec<(usize, usize)>;

    /// Checks if an edge exists between vertices u and v.
    fn has_edge(&self, u: usize, v: usize) -> bool;

    /// Returns all neighbors of vertex v.
    fn neighbors(&self, v: usize) -> Vec<usize>;

    /// Returns the degree of vertex v (number of neighbors).
    fn degree(&self, v: usize) -> usize {
        self.neighbors(v).len()
    }

    /// Returns true if the graph has no vertices.
    fn is_empty(&self) -> bool {
        self.num_vertices() == 0
    }

    /// Iterates over all edges, calling a closure for each.
    ///
    /// This can be more efficient than `edges()` when you don't need to collect.
    fn for_each_edge<F>(&self, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        for (u, v) in self.edges() {
            f(u, v);
        }
    }
}

/// A simple unweighted undirected graph.
///
/// This is the default graph type for most problems. It wraps petgraph's
/// `UnGraph` and implements the `Graph` trait.
///
/// # Example
///
/// ```
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::topology::Graph;
///
/// let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
/// assert_eq!(graph.num_vertices(), 4);
/// assert_eq!(graph.num_edges(), 3);
/// assert!(graph.has_edge(0, 1));
/// assert!(!graph.has_edge(0, 2));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleGraph {
    inner: UnGraph<(), ()>,
}

impl SimpleGraph {
    /// Creates a new graph with the given vertices and edges.
    ///
    /// # Arguments
    ///
    /// * `num_vertices` - Number of vertices in the graph
    /// * `edges` - List of edges as (u, v) pairs
    ///
    /// # Panics
    ///
    /// Panics if any edge references a vertex index >= num_vertices.
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self {
        let mut inner = UnGraph::new_undirected();
        for _ in 0..num_vertices {
            inner.add_node(());
        }
        for (u, v) in edges {
            assert!(
                u < num_vertices && v < num_vertices,
                "edge ({}, {}) references vertex >= num_vertices ({})",
                u,
                v,
                num_vertices
            );
            inner.add_edge(NodeIndex::new(u), NodeIndex::new(v), ());
        }
        Self { inner }
    }

    /// Creates an empty graph with the given number of vertices.
    pub fn empty(num_vertices: usize) -> Self {
        Self::new(num_vertices, vec![])
    }

    /// Creates a complete graph (all vertices connected).
    pub fn complete(num_vertices: usize) -> Self {
        let mut edges = Vec::new();
        for i in 0..num_vertices {
            for j in (i + 1)..num_vertices {
                edges.push((i, j));
            }
        }
        Self::new(num_vertices, edges)
    }

    /// Creates a path graph (0-1-2-...-n).
    pub fn path(num_vertices: usize) -> Self {
        let edges: Vec<_> = (0..num_vertices.saturating_sub(1))
            .map(|i| (i, i + 1))
            .collect();
        Self::new(num_vertices, edges)
    }

    /// Creates a cycle graph (0-1-2-...-n-0).
    pub fn cycle(num_vertices: usize) -> Self {
        if num_vertices < 3 {
            return Self::path(num_vertices);
        }
        let mut edges: Vec<_> = (0..num_vertices - 1).map(|i| (i, i + 1)).collect();
        edges.push((num_vertices - 1, 0));
        Self::new(num_vertices, edges)
    }

    /// Creates a star graph (vertex 0 connected to all others).
    pub fn star(num_vertices: usize) -> Self {
        let edges: Vec<_> = (1..num_vertices).map(|i| (0, i)).collect();
        Self::new(num_vertices, edges)
    }

    /// Creates a grid graph with the given dimensions.
    ///
    /// Vertices are numbered row by row: vertex `r * cols + c` is at row `r`, column `c`.
    pub fn grid(rows: usize, cols: usize) -> Self {
        let num_vertices = rows * cols;
        let mut edges = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                let v = r * cols + c;
                // Right neighbor
                if c + 1 < cols {
                    edges.push((v, v + 1));
                }
                // Down neighbor
                if r + 1 < rows {
                    edges.push((v, v + cols));
                }
            }
        }

        Self::new(num_vertices, edges)
    }
}

impl Graph for SimpleGraph {
    const NAME: &'static str = "SimpleGraph";

    fn num_vertices(&self) -> usize {
        self.inner.node_count()
    }

    fn num_edges(&self) -> usize {
        self.inner.edge_count()
    }

    fn edges(&self) -> Vec<(usize, usize)> {
        self.inner
            .edge_references()
            .map(|e| (e.source().index(), e.target().index()))
            .collect()
    }

    fn has_edge(&self, u: usize, v: usize) -> bool {
        self.inner
            .find_edge(NodeIndex::new(u), NodeIndex::new(v))
            .is_some()
    }

    fn neighbors(&self, v: usize) -> Vec<usize> {
        self.inner
            .neighbors(NodeIndex::new(v))
            .map(|n| n.index())
            .collect()
    }
}

impl PartialEq for SimpleGraph {
    fn eq(&self, other: &Self) -> bool {
        if self.num_vertices() != other.num_vertices() {
            return false;
        }
        if self.num_edges() != other.num_edges() {
            return false;
        }
        // Check all edges exist in both
        let mut self_edges: Vec<_> = self.edges();
        let mut other_edges: Vec<_> = other.edges();
        // Normalize edge order
        for e in &mut self_edges {
            if e.0 > e.1 {
                *e = (e.1, e.0);
            }
        }
        for e in &mut other_edges {
            if e.0 > e.1 {
                *e = (e.1, e.0);
            }
        }
        self_edges.sort();
        other_edges.sort();
        self_edges == other_edges
    }
}

impl Eq for SimpleGraph {}

#[cfg(test)]
#[path = "../tests_unit/topology/graph.rs"]
mod tests;
