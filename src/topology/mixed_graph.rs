//! Mixed graph implementation.
//!
//! This module provides [`MixedGraph`], a graph topology with both directed arcs
//! and undirected edges. It is intended for models whose instances genuinely
//! depend on both kinds of adjacency, such as Mixed Chinese Postman.

use serde::{Deserialize, Serialize};

/// A graph with both directed arcs and undirected edges.
///
/// Arcs are ordered pairs `(u, v)`. Undirected edges preserve the input order
/// so higher-level models can use that order as part of their configuration
/// semantics, but edge-membership queries treat them as unordered pairs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedGraph {
    num_vertices: usize,
    arcs: Vec<(usize, usize)>,
    edges: Vec<(usize, usize)>,
}

impl MixedGraph {
    /// Create a new mixed graph.
    ///
    /// # Panics
    ///
    /// Panics if any endpoint references a vertex outside `0..num_vertices`.
    pub fn new(num_vertices: usize, arcs: Vec<(usize, usize)>, edges: Vec<(usize, usize)>) -> Self {
        for &(u, v) in &arcs {
            assert!(
                u < num_vertices && v < num_vertices,
                "arc ({}, {}) references vertex >= num_vertices ({})",
                u,
                v,
                num_vertices
            );
        }

        for &(u, v) in &edges {
            assert!(
                u < num_vertices && v < num_vertices,
                "edge ({}, {}) references vertex >= num_vertices ({})",
                u,
                v,
                num_vertices
            );
        }

        Self {
            num_vertices,
            arcs,
            edges,
        }
    }

    /// Create an empty mixed graph with no arcs or undirected edges.
    pub fn empty(num_vertices: usize) -> Self {
        Self::new(num_vertices, vec![], vec![])
    }

    /// Return the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    /// Return the number of directed arcs.
    pub fn num_arcs(&self) -> usize {
        self.arcs.len()
    }

    /// Return the number of undirected edges.
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    /// Return the directed arcs.
    pub fn arcs(&self) -> Vec<(usize, usize)> {
        self.arcs.clone()
    }

    /// Return the undirected edges.
    pub fn edges(&self) -> Vec<(usize, usize)> {
        self.edges.clone()
    }

    /// Return true when the directed arc `(u, v)` is present.
    pub fn has_arc(&self, u: usize, v: usize) -> bool {
        self.arcs.iter().any(|&(src, dst)| src == u && dst == v)
    }

    /// Return true when the undirected edge `{u, v}` is present.
    pub fn has_edge(&self, u: usize, v: usize) -> bool {
        let edge = normalize_edge(u, v);
        self.edges
            .iter()
            .any(|&(a, b)| normalize_edge(a, b) == edge)
    }

    /// Return the outgoing arc count of vertex `v`.
    pub fn out_degree(&self, v: usize) -> usize {
        self.arcs.iter().filter(|&&(u, _)| u == v).count()
    }

    /// Return the incoming arc count of vertex `v`.
    pub fn in_degree(&self, v: usize) -> usize {
        self.arcs.iter().filter(|&&(_, w)| w == v).count()
    }

    /// Return the undirected-edge count incident to vertex `v`.
    pub fn undirected_degree(&self, v: usize) -> usize {
        self.edges
            .iter()
            .filter(|&&(u, w)| u == v || w == v)
            .count()
    }

    /// Return true if the graph has no vertices.
    pub fn is_empty(&self) -> bool {
        self.num_vertices == 0
    }
}

fn normalize_edge(u: usize, v: usize) -> (usize, usize) {
    if u <= v {
        (u, v)
    } else {
        (v, u)
    }
}

impl PartialEq for MixedGraph {
    fn eq(&self, other: &Self) -> bool {
        if self.num_vertices != other.num_vertices {
            return false;
        }

        let mut self_arcs = self.arcs.clone();
        let mut other_arcs = other.arcs.clone();
        self_arcs.sort();
        other_arcs.sort();
        if self_arcs != other_arcs {
            return false;
        }

        let mut self_edges = self.edges.clone();
        let mut other_edges = other.edges.clone();
        for edge in &mut self_edges {
            *edge = normalize_edge(edge.0, edge.1);
        }
        for edge in &mut other_edges {
            *edge = normalize_edge(edge.0, edge.1);
        }
        self_edges.sort();
        other_edges.sort();
        self_edges == other_edges
    }
}

impl Eq for MixedGraph {}

use crate::impl_variant_param;
impl_variant_param!(MixedGraph, "graph");

#[cfg(test)]
#[path = "../unit_tests/topology/mixed_graph.rs"]
mod tests;
