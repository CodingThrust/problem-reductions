//! King's Subgraph — an unweighted unit disk graph on a square grid (king's move connectivity).
//!
//! This is a public graph type produced by the KSG unit disk mapping reduction.
//! It stores only integer grid positions; edges are computed on-the-fly from geometry.

use super::graph::Graph;
use super::unit_disk_graph::UnitDiskGraph;
use serde::{Deserialize, Serialize};

/// A King's Subgraph — an unweighted unit disk graph on a square lattice.
///
/// Vertices occupy integer grid positions with edges determined by distance
/// (king's move connectivity: adjacent horizontally, vertically, or diagonally).
/// This is a subtype of [`UnitDiskGraph`] in the variant hierarchy.
///
/// Edges are computed on-the-fly: two positions are connected if their
/// Euclidean distance is strictly less than 1.5.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KingsSubgraph {
    /// Integer grid positions (row, col) for each vertex.
    positions: Vec<(i32, i32)>,
}

/// Fixed radius for king's move connectivity on integer grid.
const KINGS_RADIUS: f64 = 1.5;

impl KingsSubgraph {
    /// Create a KingsSubgraph from a list of integer positions.
    pub fn new(positions: Vec<(i32, i32)>) -> Self {
        Self { positions }
    }

    /// Get the positions of all vertices.
    pub fn positions(&self) -> &[(i32, i32)] {
        &self.positions
    }

    /// Get the number of positions (vertices).
    pub fn num_positions(&self) -> usize {
        self.positions.len()
    }

    /// Compute Euclidean distance between two integer positions.
    fn distance(p1: (i32, i32), p2: (i32, i32)) -> f64 {
        let dx = (p1.0 - p2.0) as f64;
        let dy = (p1.1 - p2.1) as f64;
        (dx * dx + dy * dy).sqrt()
    }
}

impl Graph for KingsSubgraph {
    const NAME: &'static str = "KingsSubgraph";

    fn num_vertices(&self) -> usize {
        self.positions.len()
    }

    fn num_edges(&self) -> usize {
        let n = self.positions.len();
        let mut count = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                if Self::distance(self.positions[i], self.positions[j]) < KINGS_RADIUS {
                    count += 1;
                }
            }
        }
        count
    }

    fn edges(&self) -> Vec<(usize, usize)> {
        let n = self.positions.len();
        let mut edges = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                if Self::distance(self.positions[i], self.positions[j]) < KINGS_RADIUS {
                    edges.push((i, j));
                }
            }
        }
        edges
    }

    fn has_edge(&self, u: usize, v: usize) -> bool {
        if u >= self.positions.len() || v >= self.positions.len() || u == v {
            return false;
        }
        Self::distance(self.positions[u], self.positions[v]) < KINGS_RADIUS
    }

    fn neighbors(&self, v: usize) -> Vec<usize> {
        if v >= self.positions.len() {
            return Vec::new();
        }
        (0..self.positions.len())
            .filter(|&u| u != v && Self::distance(self.positions[v], self.positions[u]) < KINGS_RADIUS)
            .collect()
    }
}

impl crate::variant::VariantParam for KingsSubgraph {
    const CATEGORY: &'static str = "graph";
    const VALUE: &'static str = "KingsSubgraph";
    const PARENT_VALUE: Option<&'static str> = Some("UnitDiskGraph");
}
impl crate::variant::CastToParent for KingsSubgraph {
    type Parent = UnitDiskGraph;
    fn cast_to_parent(&self) -> UnitDiskGraph {
        let positions: Vec<(f64, f64)> = self
            .positions
            .iter()
            .map(|&(r, c)| (r as f64, c as f64))
            .collect();
        UnitDiskGraph::new(positions, KINGS_RADIUS)
    }
}
inventory::submit! {
    crate::variant::VariantTypeEntry {
        category: "graph",
        value: "KingsSubgraph",
        parent: Some("UnitDiskGraph"),
    }
}
