//! Triangular Subgraph — an unweighted unit disk graph on a triangular lattice.
//!
//! This is a public graph type produced by the triangular unit disk mapping reduction.
//! It stores only integer grid positions; edges are computed on-the-fly from geometry.

use super::graph::Graph;
use super::unit_disk_graph::UnitDiskGraph;
use serde::{Deserialize, Serialize};

/// A Triangular Subgraph — an unweighted unit disk graph on a triangular lattice.
///
/// Vertices occupy positions on a triangular grid with edges determined by distance.
/// This is a subtype of [`UnitDiskGraph`] in the variant hierarchy.
///
/// Physical position for integer coordinates `(row, col)`:
/// - `x = row + 0.5` if col is even, else `x = row`
/// - `y = col * sqrt(3)/2`
///
/// Edges are computed on-the-fly: two positions are connected if their
/// physical Euclidean distance is strictly less than 1.1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriangularSubgraph {
    /// Integer grid positions (row, col) for each vertex.
    positions: Vec<(i32, i32)>,
}

/// Fixed radius for triangular lattice adjacency.
const TRIANGULAR_RADIUS: f64 = 1.1;

impl TriangularSubgraph {
    /// Create a TriangularSubgraph from a list of integer positions.
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

    /// Compute the physical position for a triangular lattice coordinate.
    ///
    /// Uses `offset_even_cols = true` convention:
    /// - `x = row + 0.5` if col is even, else `x = row`
    /// - `y = col * sqrt(3)/2`
    #[allow(unknown_lints, clippy::manual_is_multiple_of)]
    fn physical_position(row: i32, col: i32) -> (f64, f64) {
        let y = col as f64 * (3.0_f64.sqrt() / 2.0);
        let offset = if col % 2 == 0 { 0.5 } else { 0.0 };
        let x = row as f64 + offset;
        (x, y)
    }

    /// Compute Euclidean distance between two physical positions.
    fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
        let dx = p1.0 - p2.0;
        let dy = p1.1 - p2.1;
        (dx * dx + dy * dy).sqrt()
    }
}

impl Graph for TriangularSubgraph {
    const NAME: &'static str = "TriangularSubgraph";

    fn num_vertices(&self) -> usize {
        self.positions.len()
    }

    fn num_edges(&self) -> usize {
        let n = self.positions.len();
        let mut count = 0;
        for i in 0..n {
            let pi = Self::physical_position(self.positions[i].0, self.positions[i].1);
            for j in (i + 1)..n {
                let pj = Self::physical_position(self.positions[j].0, self.positions[j].1);
                if Self::distance(pi, pj) < TRIANGULAR_RADIUS {
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
            let pi = Self::physical_position(self.positions[i].0, self.positions[i].1);
            for j in (i + 1)..n {
                let pj = Self::physical_position(self.positions[j].0, self.positions[j].1);
                if Self::distance(pi, pj) < TRIANGULAR_RADIUS {
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
        let pu = Self::physical_position(self.positions[u].0, self.positions[u].1);
        let pv = Self::physical_position(self.positions[v].0, self.positions[v].1);
        Self::distance(pu, pv) < TRIANGULAR_RADIUS
    }

    fn neighbors(&self, v: usize) -> Vec<usize> {
        if v >= self.positions.len() {
            return Vec::new();
        }
        let pv = Self::physical_position(self.positions[v].0, self.positions[v].1);
        (0..self.positions.len())
            .filter(|&u| {
                u != v && {
                    let pu = Self::physical_position(self.positions[u].0, self.positions[u].1);
                    Self::distance(pv, pu) < TRIANGULAR_RADIUS
                }
            })
            .collect()
    }
}

impl crate::variant::VariantParam for TriangularSubgraph {
    const CATEGORY: &'static str = "graph";
    const VALUE: &'static str = "TriangularSubgraph";
    const PARENT_VALUE: Option<&'static str> = Some("UnitDiskGraph");
}
impl crate::variant::CastToParent for TriangularSubgraph {
    type Parent = UnitDiskGraph;
    fn cast_to_parent(&self) -> UnitDiskGraph {
        let positions: Vec<(f64, f64)> = self
            .positions
            .iter()
            .map(|&(r, c)| Self::physical_position(r, c))
            .collect();
        UnitDiskGraph::new(positions, TRIANGULAR_RADIUS)
    }
}
inventory::submit! {
    crate::variant::VariantTypeEntry {
        category: "graph",
        value: "TriangularSubgraph",
        parent: Some("UnitDiskGraph"),
    }
}
