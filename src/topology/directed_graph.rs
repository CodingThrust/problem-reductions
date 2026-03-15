//! Directed graph implementation.
//!
//! This module provides [`DirectedGraph`], a directed graph wrapping petgraph's
//! `DiGraph`. It is used for problems that require directed input, such as
//! [`MinimumFeedbackVertexSet`] and [`MinimumFeedbackArcSet`].
//!
//! Unlike [`SimpleGraph`], `DirectedGraph` does **not** implement the [`Graph`]
//! trait (which is specific to undirected graphs). Arcs are ordered pairs `(u, v)`
//! representing a directed edge from `u` to `v`.
//!
//! [`SimpleGraph`]: crate::topology::SimpleGraph
//! [`Graph`]: crate::topology::Graph
//! [`MinimumFeedbackVertexSet`]: crate::models::graph::MinimumFeedbackVertexSet
//! [`MinimumFeedbackArcSet`]: crate::models::graph::MinimumFeedbackArcSet

use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};

/// A simple unweighted directed graph.
///
/// Arcs are represented as ordered pairs `(u, v)` meaning there is an arc
/// from vertex `u` to vertex `v`. Self-loops are permitted.
///
/// # Example
///
/// ```
/// use problemreductions::topology::DirectedGraph;
///
/// let g = DirectedGraph::new(3, vec![(0, 1), (1, 2)]);
/// assert_eq!(g.num_vertices(), 3);
/// assert_eq!(g.num_arcs(), 2);
/// assert!(g.is_dag());
///
/// let cyclic = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
/// assert!(!cyclic.is_dag());
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
    /// * `arcs` - List of arcs as `(source, target)` pairs
    ///
    /// # Panics
    ///
    /// Panics if any arc references a vertex index >= `num_vertices`.
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

    /// Creates an empty directed graph with the given number of vertices and no arcs.
    pub fn empty(num_vertices: usize) -> Self {
        Self::new(num_vertices, vec![])
    }

    /// Returns the number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.inner.node_count()
    }

    /// Returns the number of arcs in the graph.
    pub fn num_arcs(&self) -> usize {
        self.inner.edge_count()
    }

    /// Returns all arcs as `(source, target)` pairs.
    pub fn arcs(&self) -> Vec<(usize, usize)> {
        self.inner
            .edge_references()
            .map(|e| (e.source().index(), e.target().index()))
            .collect()
    }

    /// Returns `true` if there is an arc from `u` to `v`.
    pub fn has_arc(&self, u: usize, v: usize) -> bool {
        self.inner
            .find_edge(NodeIndex::new(u), NodeIndex::new(v))
            .is_some()
    }

    /// Returns the outgoing neighbors (successors) of vertex `v`.
    ///
    /// These are all vertices `w` such that there is an arc `v → w`.
    pub fn successors(&self, v: usize) -> Vec<usize> {
        self.inner
            .neighbors(NodeIndex::new(v))
            .map(|n| n.index())
            .collect()
    }

    /// Returns the incoming neighbors (predecessors) of vertex `v`.
    ///
    /// These are all vertices `u` such that there is an arc `u → v`.
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

    /// Returns `true` if the graph is a directed acyclic graph (DAG).
    ///
    /// Uses petgraph's topological sort to detect cycles: if a topological
    /// ordering exists, the graph is acyclic.
    pub fn is_dag(&self) -> bool {
        toposort(&self.inner, None).is_ok()
    }

    /// Check if the subgraph induced by keeping only the given arcs is acyclic (a DAG).
    ///
    /// `kept_arcs` is a boolean slice of length `num_arcs()`, where `true` means the arc is kept.
    ///
    /// # Panics
    ///
    /// Panics if `kept_arcs.len() != self.num_arcs()`.
    pub fn is_acyclic_subgraph(&self, kept_arcs: &[bool]) -> bool {
        assert_eq!(
            kept_arcs.len(),
            self.num_arcs(),
            "kept_arcs slice length must equal num_arcs"
        );
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

    /// Returns the induced subgraph on vertices where `keep[v] == true`.
    ///
    /// Vertex indices are remapped to be contiguous starting from 0. An arc
    /// `(u, v)` is included only if both `u` and `v` are kept. The new index
    /// of a kept vertex is its rank among the kept vertices in increasing order.
    ///
    /// # Panics
    ///
    /// Panics if `keep.len() != self.num_vertices()`.
    pub fn induced_subgraph(&self, keep: &[bool]) -> Self {
        assert_eq!(
            keep.len(),
            self.num_vertices(),
            "keep slice length must equal num_vertices"
        );

        // Build old index -> new index mapping
        let mut new_index = vec![usize::MAX; self.num_vertices()];
        let mut count = 0;
        for (v, &kept) in keep.iter().enumerate() {
            if kept {
                new_index[v] = count;
                count += 1;
            }
        }

        let new_arcs: Vec<(usize, usize)> = self
            .arcs()
            .into_iter()
            .filter(|&(u, v)| keep[u] && keep[v])
            .map(|(u, v)| (new_index[u], new_index[v]))
            .collect();

        Self::new(count, new_arcs)
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
