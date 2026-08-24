//! Triangular Subgraph — an unweighted unit disk graph on a triangular lattice.
//!
//! This is a public graph type produced by the triangular unit disk mapping reduction.
//! It stores only integer grid positions; edges are computed on-the-fly from geometry.

use super::graph::Graph;
use super::unit_disk_graph::UnitDiskGraph;
use crate::registry::ConstructionError;
use crate::types::i64_to_exact_f64;
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
    positions: Vec<(i64, i64)>,
}

/// Fixed radius for triangular lattice adjacency.
const TRIANGULAR_RADIUS: f64 = 1.1;

impl TriangularSubgraph {
    /// Create a TriangularSubgraph from a list of integer positions.
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

    /// Compute the physical position for a triangular lattice coordinate.
    ///
    /// Uses `offset_even_cols = true` convention:
    /// - `x = row + 0.5` if col is even, else `x = row`
    /// - `y = col * sqrt(3)/2`
    #[allow(unknown_lints, clippy::manual_is_multiple_of)]
    fn physical_position(row: i64, col: i64) -> Result<(f64, f64), ConstructionError> {
        let row = i64_to_exact_f64(row)?;
        let col_f64 = i64_to_exact_f64(col)?;
        let y = col_f64 * (3.0_f64.sqrt() / 2.0);
        let offset = if col % 2 == 0 { 0.5 } else { 0.0 };
        let x = row + offset;
        Ok((x, y))
    }

    fn are_adjacent(p1: (i64, i64), p2: (i64, i64)) -> bool {
        let column_delta = (i128::from(p1.1) - i128::from(p2.1)).abs();
        if column_delta > 1 {
            return false;
        }
        let x1 = 2 * i128::from(p1.0) + i128::from(p1.1.rem_euclid(2) == 0);
        let x2 = 2 * i128::from(p2.0) + i128::from(p2.1.rem_euclid(2) == 0);
        let x_delta = (x1 - x2).abs();
        x_delta <= 2 && x_delta * x_delta + 3 * column_delta * column_delta <= 4
    }

    pub(crate) fn try_to_unit_disk_graph(&self) -> Result<UnitDiskGraph, ConstructionError> {
        let positions = self
            .positions
            .iter()
            .map(|&(row, column)| Self::physical_position(row, column))
            .collect::<Result<Vec<_>, _>>()?;
        let graph = UnitDiskGraph::new(positions, TRIANGULAR_RADIUS)?;
        for first in 0..self.positions.len() {
            for second in (first + 1)..self.positions.len() {
                if Self::are_adjacent(self.positions[first], self.positions[second])
                    != graph.has_edge(first, second)
                {
                    return Err(ConstructionError::Conversion(format!(
                        "triangular-subgraph coordinates at indices {first} and {second} cannot be represented in UnitDiskGraph without changing adjacency"
                    )));
                }
            }
        }
        Ok(graph)
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

impl crate::variant::VariantParam for TriangularSubgraph {
    const CATEGORY: &'static str = "graph";
    const VALUE: &'static str = "TriangularSubgraph";
    const PARENT_VALUE: Option<&'static str> = Some("UnitDiskGraph");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::MAX_EXACT_F64_INTEGER;

    #[test]
    fn adjacency_handles_full_i64_coordinate_range() {
        let graph = TriangularSubgraph::new(vec![(i64::MAX, 0), (i64::MAX, 1), (i64::MIN, 0)]);

        assert!(graph.has_edge(0, 1));
        assert!(!graph.has_edge(0, 2));
    }

    #[test]
    fn integer_adjacency_matches_euclidean_definition() {
        for row_a in -4..=4 {
            for column_a in -4..=4 {
                for row_b in -4..=4 {
                    for column_b in -4..=4 {
                        let a = TriangularSubgraph::physical_position(row_a, column_a).unwrap();
                        let b = TriangularSubgraph::physical_position(row_b, column_b).unwrap();
                        let euclidean = (a.0 - b.0).hypot(a.1 - b.1) < TRIANGULAR_RADIUS;
                        assert_eq!(
                            TriangularSubgraph::are_adjacent((row_a, column_a), (row_b, column_b)),
                            euclidean
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn unit_disk_conversion_rejects_inexact_coordinates() {
        let graph = TriangularSubgraph::new(vec![(MAX_EXACT_F64_INTEGER + 1, 0)]);

        assert!(matches!(
            graph.try_to_unit_disk_graph(),
            Err(ConstructionError::InexactFloatConversion(_))
        ));
    }

    #[test]
    fn unit_disk_conversion_rejects_changed_adjacency() {
        let graph =
            TriangularSubgraph::new(vec![(MAX_EXACT_F64_INTEGER, 0), (MAX_EXACT_F64_INTEGER, 1)]);

        assert!(matches!(
            graph.try_to_unit_disk_graph(),
            Err(ConstructionError::Conversion(_))
        ));
    }
}
