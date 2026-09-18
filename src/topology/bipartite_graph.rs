//! Bipartite graph with explicit left/right partitions.

use super::graph::Graph;
use serde::{Deserialize, Serialize};

/// Bipartite graph with explicit left/right partitions.
///
/// Vertices are split into left (indices `0..left_size`) and right (`0..right_size`).
/// Edges connect left vertices to right vertices using bipartite-local coordinates.
/// The [`Graph`] trait maps to a unified vertex space where right vertices are offset
/// by `left_size`.
///
/// # Example
///
/// ```
/// use problemreductions::topology::{BipartiteGraph, Graph};
///
/// // K_{2,2}: complete bipartite graph
/// let g = BipartiteGraph::new(2, 2, vec![(0, 0), (0, 1), (1, 0), (1, 1)]);
/// assert_eq!(g.num_vertices(), 4);
/// assert_eq!(g.num_edges(), 4);
/// assert!(g.has_edge(0, 2)); // left 0 -> right 0 (unified index 2)
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "BipartiteGraphData")]
pub struct BipartiteGraph {
    left_size: usize,
    right_size: usize,
    /// Edges in bipartite-local coordinates: (left_index, right_index).
    edges: Vec<(usize, usize)>,
}

#[derive(Deserialize)]
struct BipartiteGraphData {
    left_size: usize,
    right_size: usize,
    edges: Vec<(usize, usize)>,
}

impl TryFrom<BipartiteGraphData> for BipartiteGraph {
    type Error = crate::registry::ConstructionError;
    fn try_from(data: BipartiteGraphData) -> Result<Self, Self::Error> {
        Self::try_new(data.left_size, data.right_size, data.edges)
    }
}

impl BipartiteGraph {
    /// Create a new bipartite graph.
    ///
    /// # Arguments
    ///
    /// * `left_size` - Number of vertices in the left partition
    /// * `right_size` - Number of vertices in the right partition
    /// * `edges` - Edges as `(left_index, right_index)` pairs in bipartite-local coordinates
    ///
    /// # Panics
    ///
    /// Panics if any edge references an out-of-bounds left or right vertex index,
    /// or if the combined vertex count overflows `usize`.
    pub fn new(left_size: usize, right_size: usize, edges: Vec<(usize, usize)>) -> Self {
        Self::try_new(left_size, right_size, edges).unwrap_or_else(|error| panic!("{error}"))
    }

    fn try_new(
        left_size: usize,
        right_size: usize,
        edges: Vec<(usize, usize)>,
    ) -> Result<Self, crate::registry::ConstructionError> {
        left_size
            .checked_add(right_size)
            .ok_or("bipartite vertex count overflows usize")?;
        for &(u, v) in &edges {
            if u >= left_size {
                return Err(
                    format!("left vertex {} out of bounds (left_size={})", u, left_size).into(),
                );
            }
            if v >= right_size {
                return Err(format!(
                    "right vertex {} out of bounds (right_size={})",
                    v, right_size
                )
                .into());
            }
        }
        Ok(Self {
            left_size,
            right_size,
            edges,
        })
    }

    /// Returns the number of vertices in the left partition.
    pub fn left_size(&self) -> usize {
        self.left_size
    }

    /// Returns the number of vertices in the right partition.
    pub fn right_size(&self) -> usize {
        self.right_size
    }

    /// Returns the edges in bipartite-local coordinates.
    pub fn left_edges(&self) -> &[(usize, usize)] {
        &self.edges
    }
}

impl Graph for BipartiteGraph {
    const NAME: &'static str = "BipartiteGraph";

    fn num_vertices(&self) -> usize {
        self.left_size + self.right_size
    }

    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn edges(&self) -> Vec<(usize, usize)> {
        self.edges
            .iter()
            .map(|&(u, v)| {
                let a = u;
                let b = self.left_size + v;
                if a < b {
                    (a, b)
                } else {
                    (b, a)
                }
            })
            .collect()
    }

    fn has_edge(&self, u: usize, v: usize) -> bool {
        let (u, v) = if u < v { (u, v) } else { (v, u) };
        // u must be a left vertex and v must be a right vertex (in unified space)
        if u >= self.left_size || v < self.left_size {
            return false;
        }
        let local_v = v - self.left_size;
        self.edges.contains(&(u, local_v))
    }

    fn neighbors(&self, v: usize) -> Vec<usize> {
        if v < self.left_size {
            // Left vertex: find all right neighbors
            self.edges
                .iter()
                .filter(|(u, _)| *u == v)
                .map(|(_, rv)| self.left_size + rv)
                .collect()
        } else {
            // Right vertex: find all left neighbors
            let local_v = v - self.left_size;
            self.edges
                .iter()
                .filter(|(_, rv)| *rv == local_v)
                .map(|(u, _)| *u)
                .collect()
        }
    }
}

use crate::impl_variant_param;
impl_variant_param!(BipartiteGraph, "graph");

#[cfg(test)]
#[path = "../unit_tests/topology/bipartite_graph.rs"]
mod tests;
