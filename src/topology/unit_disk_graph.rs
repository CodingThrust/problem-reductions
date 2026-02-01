//! Unit Disk Graph implementation.
//!
//! A unit disk graph (UDG) is a graph where vertices have positions in 2D space,
//! and two vertices are connected if their distance is at most a threshold (radius).

use super::graph::Graph;
use serde::{Deserialize, Serialize};

/// A unit disk graph with vertices at 2D positions.
///
/// Two vertices are connected by an edge if their Euclidean distance
/// is at most the specified radius.
///
/// # Example
///
/// ```
/// use problemreductions::topology::UnitDiskGraph;
///
/// // Create a UDG with 3 vertices at positions (0,0), (1,0), (3,0)
/// // with unit radius (distance <= 1.0 creates an edge)
/// let udg = UnitDiskGraph::new(
///     vec![(0.0, 0.0), (1.0, 0.0), (3.0, 0.0)],
///     1.0,
/// );
///
/// // Vertices 0 and 1 are connected (distance = 1.0)
/// // Vertex 2 is isolated (distance > 1.0 from both)
/// assert!(udg.has_edge(0, 1));
/// assert!(!udg.has_edge(0, 2));
/// assert!(!udg.has_edge(1, 2));
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnitDiskGraph {
    /// Positions of vertices as (x, y) coordinates.
    positions: Vec<(f64, f64)>,
    /// Radius threshold for edge creation.
    radius: f64,
    /// Precomputed edges.
    edges: Vec<(usize, usize)>,
}

impl UnitDiskGraph {
    /// Create a new unit disk graph.
    ///
    /// # Arguments
    ///
    /// * `positions` - 2D coordinates for each vertex
    /// * `radius` - Maximum distance for an edge to exist
    pub fn new(positions: Vec<(f64, f64)>, radius: f64) -> Self {
        let n = positions.len();
        let mut edges = Vec::new();

        // Compute all edges based on distance
        for i in 0..n {
            for j in (i + 1)..n {
                if Self::distance(&positions[i], &positions[j]) <= radius {
                    edges.push((i, j));
                }
            }
        }

        Self {
            positions,
            radius,
            edges,
        }
    }

    /// Create a unit disk graph with radius 1.0.
    pub fn unit(positions: Vec<(f64, f64)>) -> Self {
        Self::new(positions, 1.0)
    }

    /// Compute Euclidean distance between two points.
    fn distance(p1: &(f64, f64), p2: &(f64, f64)) -> f64 {
        let dx = p1.0 - p2.0;
        let dy = p1.1 - p2.1;
        (dx * dx + dy * dy).sqrt()
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.positions.len()
    }

    /// Get the number of edges.
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    /// Get the radius threshold.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Get the position of a vertex.
    pub fn position(&self, v: usize) -> Option<(f64, f64)> {
        self.positions.get(v).copied()
    }

    /// Get all positions.
    pub fn positions(&self) -> &[(f64, f64)] {
        &self.positions
    }

    /// Get all edges.
    pub fn edges(&self) -> &[(usize, usize)] {
        &self.edges
    }

    /// Check if an edge exists between two vertices.
    pub fn has_edge(&self, u: usize, v: usize) -> bool {
        let (u, v) = if u < v { (u, v) } else { (v, u) };
        self.edges.contains(&(u, v))
    }

    /// Get the distance between two vertices.
    pub fn vertex_distance(&self, u: usize, v: usize) -> Option<f64> {
        match (self.positions.get(u), self.positions.get(v)) {
            (Some(p1), Some(p2)) => Some(Self::distance(p1, p2)),
            _ => None,
        }
    }

    /// Get all neighbors of a vertex.
    pub fn neighbors(&self, v: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|&(u1, u2)| {
                if u1 == v {
                    Some(u2)
                } else if u2 == v {
                    Some(u1)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get the degree of a vertex.
    pub fn degree(&self, v: usize) -> usize {
        self.neighbors(v).len()
    }

    /// Get the bounding box of all positions.
    pub fn bounding_box(&self) -> Option<((f64, f64), (f64, f64))> {
        if self.positions.is_empty() {
            return None;
        }

        let min_x = self
            .positions
            .iter()
            .map(|p| p.0)
            .fold(f64::INFINITY, f64::min);
        let max_x = self
            .positions
            .iter()
            .map(|p| p.0)
            .fold(f64::NEG_INFINITY, f64::max);
        let min_y = self
            .positions
            .iter()
            .map(|p| p.1)
            .fold(f64::INFINITY, f64::min);
        let max_y = self
            .positions
            .iter()
            .map(|p| p.1)
            .fold(f64::NEG_INFINITY, f64::max);

        Some(((min_x, min_y), (max_x, max_y)))
    }

    /// Create a unit disk graph on a regular grid.
    ///
    /// # Arguments
    ///
    /// * `rows` - Number of rows
    /// * `cols` - Number of columns
    /// * `spacing` - Distance between adjacent grid points
    /// * `radius` - Edge creation threshold
    pub fn grid(rows: usize, cols: usize, spacing: f64, radius: f64) -> Self {
        let mut positions = Vec::with_capacity(rows * cols);
        for r in 0..rows {
            for c in 0..cols {
                positions.push((c as f64 * spacing, r as f64 * spacing));
            }
        }
        Self::new(positions, radius)
    }
}

impl Graph for UnitDiskGraph {
    fn num_vertices(&self) -> usize {
        self.positions.len()
    }

    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn edges(&self) -> Vec<(usize, usize)> {
        self.edges.clone()
    }

    fn has_edge(&self, u: usize, v: usize) -> bool {
        let (u, v) = if u < v { (u, v) } else { (v, u) };
        self.edges.contains(&(u, v))
    }

    fn neighbors(&self, v: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|&(u1, u2)| {
                if u1 == v {
                    Some(u2)
                } else if u2 == v {
                    Some(u1)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_udg_basic() {
        let udg = UnitDiskGraph::new(vec![(0.0, 0.0), (1.0, 0.0), (3.0, 0.0)], 1.0);
        assert_eq!(udg.num_vertices(), 3);
        assert_eq!(udg.num_edges(), 1); // Only 0-1 are within distance 1
    }

    #[test]
    fn test_udg_unit() {
        let udg = UnitDiskGraph::unit(vec![(0.0, 0.0), (0.5, 0.5)]);
        assert_eq!(udg.radius(), 1.0);
        // Distance is sqrt(0.5^2 + 0.5^2) ≈ 0.707 < 1, so connected
        assert_eq!(udg.num_edges(), 1);
    }

    #[test]
    fn test_udg_has_edge() {
        let udg = UnitDiskGraph::new(vec![(0.0, 0.0), (1.0, 0.0), (3.0, 0.0)], 1.0);
        assert!(udg.has_edge(0, 1));
        assert!(udg.has_edge(1, 0)); // Symmetric
        assert!(!udg.has_edge(0, 2));
        assert!(!udg.has_edge(1, 2));
    }

    #[test]
    fn test_udg_neighbors() {
        let udg = UnitDiskGraph::new(vec![(0.0, 0.0), (1.0, 0.0), (0.5, 0.5)], 1.0);
        let neighbors = udg.neighbors(0);
        // 0 is within 1.0 of both 1 and 2
        assert!(neighbors.contains(&1));
        assert!(neighbors.contains(&2));
    }

    #[test]
    fn test_udg_degree() {
        let udg = UnitDiskGraph::new(vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (5.0, 5.0)], 1.5);
        // Vertex 0 is connected to 1 and 2
        assert_eq!(udg.degree(0), 2);
        // Vertex 3 is isolated
        assert_eq!(udg.degree(3), 0);
    }

    #[test]
    fn test_udg_vertex_distance() {
        let udg = UnitDiskGraph::new(vec![(0.0, 0.0), (3.0, 4.0)], 10.0);
        let dist = udg.vertex_distance(0, 1);
        assert_eq!(dist, Some(5.0)); // 3-4-5 triangle
    }

    #[test]
    fn test_udg_position() {
        let udg = UnitDiskGraph::new(vec![(1.0, 2.0), (3.0, 4.0)], 1.0);
        assert_eq!(udg.position(0), Some((1.0, 2.0)));
        assert_eq!(udg.position(1), Some((3.0, 4.0)));
        assert_eq!(udg.position(2), None);
    }

    #[test]
    fn test_udg_bounding_box() {
        let udg = UnitDiskGraph::new(vec![(1.0, 2.0), (3.0, 4.0), (-1.0, 0.0)], 1.0);
        let bbox = udg.bounding_box();
        assert!(bbox.is_some());
        let ((min_x, min_y), (max_x, max_y)) = bbox.unwrap();
        assert_eq!(min_x, -1.0);
        assert_eq!(max_x, 3.0);
        assert_eq!(min_y, 0.0);
        assert_eq!(max_y, 4.0);
    }

    #[test]
    fn test_udg_empty_bounding_box() {
        let udg = UnitDiskGraph::new(vec![], 1.0);
        assert!(udg.bounding_box().is_none());
    }

    #[test]
    fn test_udg_grid() {
        let udg = UnitDiskGraph::grid(2, 3, 1.0, 1.0);
        assert_eq!(udg.num_vertices(), 6);
        // Grid with spacing 1.0 and radius 1.0: only horizontal/vertical neighbors connected
        // Row 0: 0-1, 1-2
        // Row 1: 3-4, 4-5
        // Vertical: 0-3, 1-4, 2-5
        assert_eq!(udg.num_edges(), 7);
    }

    #[test]
    fn test_udg_grid_diagonal() {
        // With radius > sqrt(2), diagonals are also connected
        let udg = UnitDiskGraph::grid(2, 2, 1.0, 1.5);
        assert_eq!(udg.num_vertices(), 4);
        // All pairs are connected (4 edges: 0-1, 0-2, 0-3, 1-2, 1-3, 2-3)
        // Actually: 0-1 (1.0), 0-2 (1.0), 1-3 (1.0), 2-3 (1.0), 0-3 (sqrt(2)≈1.41), 1-2 (sqrt(2)≈1.41)
        assert_eq!(udg.num_edges(), 6);
    }

    #[test]
    fn test_udg_edges_list() {
        let udg = UnitDiskGraph::new(vec![(0.0, 0.0), (1.0, 0.0)], 1.0);
        let edges = udg.edges();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0], (0, 1));
    }

    #[test]
    fn test_udg_positions() {
        let udg = UnitDiskGraph::new(vec![(1.0, 2.0), (3.0, 4.0)], 1.0);
        let positions = udg.positions();
        assert_eq!(positions.len(), 2);
        assert_eq!(positions[0], (1.0, 2.0));
        assert_eq!(positions[1], (3.0, 4.0));
    }

    #[test]
    fn test_udg_vertex_distance_invalid() {
        let udg = UnitDiskGraph::new(vec![(0.0, 0.0), (1.0, 0.0)], 1.0);
        assert_eq!(udg.vertex_distance(0, 5), None);
        assert_eq!(udg.vertex_distance(5, 0), None);
        assert_eq!(udg.vertex_distance(5, 6), None);
    }

    #[test]
    fn test_udg_graph_trait() {
        // Test the Graph trait implementation
        let udg = UnitDiskGraph::new(vec![(0.0, 0.0), (1.0, 0.0), (0.5, 0.5)], 1.0);
        // Use Graph trait methods
        assert_eq!(Graph::num_vertices(&udg), 3);
        assert!(Graph::num_edges(&udg) > 0);
        assert!(Graph::has_edge(&udg, 0, 1));
        let edges = Graph::edges(&udg);
        assert!(!edges.is_empty());
        let neighbors = Graph::neighbors(&udg, 0);
        assert!(neighbors.contains(&1));
    }
}
