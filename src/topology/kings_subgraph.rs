//! King's Subgraph — an unweighted unit disk graph on a square grid (king's move connectivity).
//!
//! This is a public graph type produced by the KSG unit disk mapping reduction.
//! It stores only integer grid positions; edges are computed on-the-fly from geometry.

use super::graph::Graph;
use super::unit_disk_graph::UnitDiskGraph;
use crate::registry::ConstructionError;
use crate::types::i64_to_exact_f64;
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
    positions: Vec<(i64, i64)>,
}

/// Fixed radius for king's move connectivity on integer grid.
const KINGS_RADIUS: f64 = 1.5;

impl KingsSubgraph {
    /// Create a KingsSubgraph from a list of integer positions.
    pub fn new(positions: Vec<(i64, i64)>) -> Self {
        Self { positions }
    }

    /// Get the positions of all vertices.
    pub fn positions(&self) -> &[(i64, i64)] {
        &self.positions
    }

    /// Get the number of positions (vertices).
    pub fn num_positions(&self) -> usize {
        self.positions.len()
    }

    fn are_adjacent(p1: (i64, i64), p2: (i64, i64)) -> bool {
        p1.0.abs_diff(p2.0) <= 1 && p1.1.abs_diff(p2.1) <= 1
    }

    pub(crate) fn try_to_unit_disk_graph(&self) -> Result<UnitDiskGraph, ConstructionError> {
        let positions = self
            .positions
            .iter()
            .map(|&(row, column)| {
                let row = i64_to_exact_f64(row)?;
                let column = i64_to_exact_f64(column)?;
                Ok((row, column))
            })
            .collect::<Result<Vec<_>, ConstructionError>>()?;
        let graph = UnitDiskGraph::new(positions, KINGS_RADIUS)?;
        for first in 0..self.positions.len() {
            for second in (first + 1)..self.positions.len() {
                if Self::are_adjacent(self.positions[first], self.positions[second])
                    != graph.has_edge(first, second)
                {
                    return Err(ConstructionError::Conversion(format!(
                        "king's-subgraph coordinates at indices {first} and {second} cannot be represented in UnitDiskGraph without changing adjacency"
                    )));
                }
            }
        }
        Ok(graph)
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
                if Self::are_adjacent(self.positions[i], self.positions[j]) {
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
                if Self::are_adjacent(self.positions[i], self.positions[j]) {
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
        Self::are_adjacent(self.positions[u], self.positions[v])
    }

    fn neighbors(&self, v: usize) -> Vec<usize> {
        if v >= self.positions.len() {
            return Vec::new();
        }
        (0..self.positions.len())
            .filter(|&u| u != v && Self::are_adjacent(self.positions[v], self.positions[u]))
            .collect()
    }
}

impl crate::variant::VariantParam for KingsSubgraph {
    const CATEGORY: &'static str = "graph";
    const VALUE: &'static str = "KingsSubgraph";
    const PARENT_VALUE: Option<&'static str> = Some("UnitDiskGraph");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::MAX_EXACT_F64_INTEGER;

    #[test]
    fn adjacency_handles_full_i64_coordinate_range() {
        let graph = KingsSubgraph::new(vec![
            (i64::MAX, i64::MAX),
            (i64::MAX - 1, i64::MAX - 1),
            (i64::MIN, i64::MIN),
        ]);

        assert!(graph.has_edge(0, 1));
        assert!(!graph.has_edge(0, 2));
    }

    #[test]
    fn integer_adjacency_matches_euclidean_definition() {
        for row_a in -4..=4 {
            for column_a in -4..=4 {
                for row_b in -4..=4 {
                    for column_b in -4..=4 {
                        let dr = (row_a - row_b) as f64;
                        let dc = (column_a - column_b) as f64;
                        let euclidean = dr.hypot(dc) < KINGS_RADIUS;
                        assert_eq!(
                            KingsSubgraph::are_adjacent((row_a, column_a), (row_b, column_b)),
                            euclidean
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn unit_disk_conversion_rejects_inexact_coordinates() {
        let graph = KingsSubgraph::new(vec![(MAX_EXACT_F64_INTEGER + 1, 0)]);

        assert!(matches!(
            graph.try_to_unit_disk_graph(),
            Err(ConstructionError::InexactFloatConversion(_))
        ));
    }
}
