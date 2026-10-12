//! Simple directed graph — validated wrapper around MultiDigraph.

use super::MultiDigraph;
use serde::{Deserialize, Serialize};

/// A directed graph with no self-loops or parallel arcs.
///
/// `(u, v)` and `(v, u)` are distinct arcs. Arc order and JSON match
/// [`MultiDigraph`]; algorithms delegate to the wrapped graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct SimpleDigraph {
    inner: MultiDigraph,
}

impl SimpleDigraph {
    /// Construct a simple digraph, panicking on an invalid arc.
    pub fn new(num_vertices: usize, arcs: Vec<(usize, usize)>) -> Self {
        Self::try_new(num_vertices, arcs).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Construct a simple digraph, returning an error naming an invalid arc.
    pub fn try_new(
        num_vertices: usize,
        arcs: Vec<(usize, usize)>,
    ) -> Result<Self, crate::registry::ConstructionError> {
        Self::from_inner(MultiDigraph::try_new(num_vertices, arcs)?)
    }

    fn from_inner(inner: MultiDigraph) -> Result<Self, crate::registry::ConstructionError> {
        super::graph::validate_simple_connections(inner.arcs(), true)?;
        Ok(Self { inner })
    }

    /// Construct a graph with no arcs.
    pub fn empty(num_vertices: usize) -> Self {
        Self::new(num_vertices, vec![])
    }
    /// Borrow the underlying multigraph without allowing mutation.
    pub fn inner(&self) -> &MultiDigraph {
        &self.inner
    }
    /// Number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.inner.num_vertices()
    }
    /// Number of arcs.
    pub fn num_arcs(&self) -> usize {
        self.inner.num_arcs()
    }
    /// Arcs in their stored order.
    pub fn arcs(&self) -> Vec<(usize, usize)> {
        self.inner.arcs()
    }
    /// Whether the ordered arc `(u, v)` is present.
    pub fn has_arc(&self, u: usize, v: usize) -> bool {
        self.inner.arc_count(u, v) > 0
    }
    /// Outgoing neighbors, with one entry per neighbor.
    pub fn successors(&self, v: usize) -> Vec<usize> {
        self.inner.successors(v)
    }
    /// Whether the graph is acyclic.
    pub fn is_dag(&self) -> bool {
        self.inner.is_dag()
    }
    /// Whether every vertex can reach every other vertex.
    pub fn is_strongly_connected(&self) -> bool {
        self.inner.is_strongly_connected()
    }
}

impl<'de> Deserialize<'de> for SimpleDigraph {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::from_inner(MultiDigraph::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

crate::impl_variant_param!(SimpleDigraph, "graph");

#[cfg(test)]
#[path = "../unit_tests/topology/simple_digraph.rs"]
mod tests;
