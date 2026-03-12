//! Directed graph type for problems on digraphs.
//!
//! [`DirectedGraph`] wraps petgraph's `DiGraph` and provides a directed-graph API
//! with arcs (directed edges), successors, and predecessors.

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};

/// A simple directed graph (digraph).
///
/// Wraps petgraph's `DiGraph` and exposes a directed-edge API.
/// Unlike [`SimpleGraph`](super::SimpleGraph) which is undirected,
/// `DirectedGraph` distinguishes between arc (u, v) and arc (v, u).
///
/// # Example
///
/// ```
/// use problemreductions::topology::DirectedGraph;
///
/// let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
/// assert_eq!(graph.num_vertices(), 3);
/// assert_eq!(graph.num_arcs(), 3);
/// assert!(graph.has_arc(0, 1));
/// assert!(!graph.has_arc(1, 0));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectedGraph {
    inner: DiGraph<(), ()>,
}

impl DirectedGraph {
    /// Creates a new directed graph with the given vertices and arcs.
    ///
    /// # Arguments
    ///
    /// * `num_vertices` - Number of vertices in the graph
    /// * `arcs` - List of directed arcs as (source, target) pairs
    ///
    /// # Panics
    ///
    /// Panics if any arc references a vertex index >= num_vertices.
    pub fn new(num_vertices: usize, arcs: Vec<(usize, usize)>) -> Self {
        let mut inner = DiGraph::new();
        for _ in 0..num_vertices {
            inner.add_node(());
        }
        for (u, v) in arcs {
            assert!(
                u < num_vertices && v < num_vertices,
                "arc ({}, {}) references vertex >= num_vertices ({})",
                u,
                v,
                num_vertices
            );
            inner.add_edge(NodeIndex::new(u), NodeIndex::new(v), ());
        }
        Self { inner }
    }

    /// Returns the number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.inner.node_count()
    }

    /// Returns the number of arcs (directed edges) in the graph.
    pub fn num_arcs(&self) -> usize {
        self.inner.edge_count()
    }

    /// Returns all arcs as a list of (source, target) pairs.
    pub fn arcs(&self) -> Vec<(usize, usize)> {
        self.inner
            .edge_references()
            .map(|e| (e.source().index(), e.target().index()))
            .collect()
    }

    /// Checks if a directed arc exists from `u` to `v`.
    pub fn has_arc(&self, u: usize, v: usize) -> bool {
        self.inner
            .find_edge(NodeIndex::new(u), NodeIndex::new(v))
            .is_some()
    }

    /// Returns all successors of vertex `v` (vertices reachable by a single arc from `v`).
    pub fn successors(&self, v: usize) -> Vec<usize> {
        self.inner
            .neighbors_directed(NodeIndex::new(v), petgraph::Direction::Outgoing)
            .map(|n| n.index())
            .collect()
    }

    /// Returns all predecessors of vertex `v` (vertices with an arc to `v`).
    pub fn predecessors(&self, v: usize) -> Vec<usize> {
        self.inner
            .neighbors_directed(NodeIndex::new(v), petgraph::Direction::Incoming)
            .map(|n| n.index())
            .collect()
    }

    /// Returns the out-degree of vertex `v`.
    pub fn out_degree(&self, v: usize) -> usize {
        self.successors(v).len()
    }

    /// Returns the in-degree of vertex `v`.
    pub fn in_degree(&self, v: usize) -> usize {
        self.predecessors(v).len()
    }

    /// Returns true if the graph has no vertices.
    pub fn is_empty(&self) -> bool {
        self.num_vertices() == 0
    }

    /// Check if the subgraph induced by keeping only the given arcs is acyclic (a DAG).
    ///
    /// `kept_arcs` is a boolean slice of length `num_arcs()`, where `true` means the arc is kept.
    pub fn is_acyclic_subgraph(&self, kept_arcs: &[bool]) -> bool {
        let n = self.num_vertices();
        let arcs = self.arcs();

        // Build adjacency list for the subgraph
        let mut adj = vec![vec![]; n];
        let mut in_degree = vec![0usize; n];
        for (i, &(u, v)) in arcs.iter().enumerate() {
            if kept_arcs[i] {
                adj[u].push(v);
                in_degree[v] += 1;
            }
        }

        // Kahn's algorithm (topological sort)
        let mut queue: Vec<usize> = (0..n).filter(|&v| in_degree[v] == 0).collect();
        let mut visited = 0;
        while let Some(u) = queue.pop() {
            visited += 1;
            for &v in &adj[u] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push(v);
                }
            }
        }
        visited == n
    }
}

impl PartialEq for DirectedGraph {
    fn eq(&self, other: &Self) -> bool {
        if self.num_vertices() != other.num_vertices() {
            return false;
        }
        if self.num_arcs() != other.num_arcs() {
            return false;
        }
        let mut self_arcs = self.arcs();
        let mut other_arcs = other.arcs();
        self_arcs.sort();
        other_arcs.sort();
        self_arcs == other_arcs
    }
}

impl Eq for DirectedGraph {}

use crate::impl_variant_param;
impl_variant_param!(DirectedGraph, "graph");

#[cfg(test)]
#[path = "../unit_tests/topology/directed_graph.rs"]
mod tests;
