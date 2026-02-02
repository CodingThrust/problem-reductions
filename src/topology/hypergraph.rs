//! Hypergraph implementation.
//!
//! A hypergraph is a generalization of a graph where edges (called hyperedges)
//! can connect any number of vertices, not just two.

use serde::{Deserialize, Serialize};

/// A hypergraph where edges can connect any number of vertices.
///
/// # Example
///
/// ```
/// use problemreductions::topology::HyperGraph;
///
/// // Create a hypergraph with 4 vertices and 2 hyperedges
/// let hg = HyperGraph::new(4, vec![
///     vec![0, 1, 2],  // Edge connecting vertices 0, 1, 2
///     vec![2, 3],     // Edge connecting vertices 2, 3
/// ]);
///
/// assert_eq!(hg.num_vertices(), 4);
/// assert_eq!(hg.num_edges(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperGraph {
    num_vertices: usize,
    edges: Vec<Vec<usize>>,
}

impl HyperGraph {
    /// Create a new hypergraph.
    ///
    /// # Panics
    ///
    /// Panics if any vertex index in an edge is out of bounds.
    pub fn new(num_vertices: usize, edges: Vec<Vec<usize>>) -> Self {
        for edge in &edges {
            for &v in edge {
                assert!(
                    v < num_vertices,
                    "vertex index {} out of bounds (max {})",
                    v,
                    num_vertices - 1
                );
            }
        }
        Self {
            num_vertices,
            edges,
        }
    }

    /// Create an empty hypergraph with no edges.
    pub fn empty(num_vertices: usize) -> Self {
        Self {
            num_vertices,
            edges: Vec::new(),
        }
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    /// Get the number of hyperedges.
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    /// Get all hyperedges.
    pub fn edges(&self) -> &[Vec<usize>] {
        &self.edges
    }

    /// Get a specific edge by index.
    pub fn edge(&self, index: usize) -> Option<&Vec<usize>> {
        self.edges.get(index)
    }

    /// Check if a hyperedge exists (order-independent).
    pub fn has_edge(&self, edge: &[usize]) -> bool {
        let mut sorted = edge.to_vec();
        sorted.sort();
        self.edges.iter().any(|e| {
            let mut e_sorted = e.clone();
            e_sorted.sort();
            e_sorted == sorted
        })
    }

    /// Get all vertices adjacent to vertex v (share a hyperedge with v).
    pub fn neighbors(&self, v: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        for edge in &self.edges {
            if edge.contains(&v) {
                for &u in edge {
                    if u != v && !neighbors.contains(&u) {
                        neighbors.push(u);
                    }
                }
            }
        }
        neighbors
    }

    /// Get the degree of a vertex (number of hyperedges containing it).
    pub fn degree(&self, v: usize) -> usize {
        self.edges.iter().filter(|edge| edge.contains(&v)).count()
    }

    /// Get all edges containing a specific vertex.
    pub fn edges_containing(&self, v: usize) -> Vec<&Vec<usize>> {
        self.edges.iter().filter(|edge| edge.contains(&v)).collect()
    }

    /// Add a new hyperedge.
    ///
    /// # Panics
    ///
    /// Panics if any vertex index is out of bounds.
    pub fn add_edge(&mut self, edge: Vec<usize>) {
        for &v in &edge {
            assert!(v < self.num_vertices, "vertex index {} out of bounds", v);
        }
        self.edges.push(edge);
    }

    /// Get the maximum edge size (maximum number of vertices in any hyperedge).
    pub fn max_edge_size(&self) -> usize {
        self.edges.iter().map(|e| e.len()).max().unwrap_or(0)
    }

    /// Check if this is a regular graph (all edges have size 2).
    pub fn is_regular_graph(&self) -> bool {
        self.edges.iter().all(|e| e.len() == 2)
    }

    /// Convert to a regular graph if possible (all edges size 2).
    /// Returns None if any edge has size != 2.
    pub fn to_graph_edges(&self) -> Option<Vec<(usize, usize)>> {
        if !self.is_regular_graph() {
            return None;
        }
        Some(self.edges.iter().map(|e| (e[0], e[1])).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypergraph_basic() {
        let hg = HyperGraph::new(4, vec![vec![0, 1, 2], vec![2, 3]]);
        assert_eq!(hg.num_vertices(), 4);
        assert_eq!(hg.num_edges(), 2);
    }

    #[test]
    fn test_hypergraph_empty() {
        let hg = HyperGraph::empty(5);
        assert_eq!(hg.num_vertices(), 5);
        assert_eq!(hg.num_edges(), 0);
    }

    #[test]
    fn test_hypergraph_neighbors() {
        let hg = HyperGraph::new(4, vec![vec![0, 1, 2], vec![2, 3]]);
        let neighbors = hg.neighbors(2);
        assert!(neighbors.contains(&0));
        assert!(neighbors.contains(&1));
        assert!(neighbors.contains(&3));
        assert!(!neighbors.contains(&2)); // Not its own neighbor
    }

    #[test]
    fn test_hypergraph_has_edge() {
        let hg = HyperGraph::new(4, vec![vec![0, 1, 2]]);
        assert!(hg.has_edge(&[0, 1, 2]));
        assert!(hg.has_edge(&[2, 1, 0])); // Order doesn't matter
        assert!(!hg.has_edge(&[0, 1]));
        assert!(!hg.has_edge(&[0, 1, 3]));
    }

    #[test]
    fn test_hypergraph_degree() {
        let hg = HyperGraph::new(4, vec![vec![0, 1, 2], vec![2, 3]]);
        assert_eq!(hg.degree(0), 1);
        assert_eq!(hg.degree(2), 2);
        assert_eq!(hg.degree(3), 1);
    }

    #[test]
    fn test_hypergraph_edges_containing() {
        let hg = HyperGraph::new(4, vec![vec![0, 1, 2], vec![2, 3]]);
        let edges = hg.edges_containing(2);
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_hypergraph_add_edge() {
        let mut hg = HyperGraph::empty(4);
        hg.add_edge(vec![0, 1]);
        hg.add_edge(vec![1, 2, 3]);
        assert_eq!(hg.num_edges(), 2);
    }

    #[test]
    fn test_hypergraph_max_edge_size() {
        let hg = HyperGraph::new(4, vec![vec![0, 1], vec![0, 1, 2, 3]]);
        assert_eq!(hg.max_edge_size(), 4);
    }

    #[test]
    fn test_hypergraph_is_regular_graph() {
        let regular = HyperGraph::new(3, vec![vec![0, 1], vec![1, 2]]);
        assert!(regular.is_regular_graph());

        let not_regular = HyperGraph::new(4, vec![vec![0, 1, 2]]);
        assert!(!not_regular.is_regular_graph());
    }

    #[test]
    fn test_hypergraph_to_graph_edges() {
        let hg = HyperGraph::new(3, vec![vec![0, 1], vec![1, 2]]);
        let edges = hg.to_graph_edges();
        assert!(edges.is_some());
        let edges = edges.unwrap();
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_hypergraph_to_graph_edges_not_regular() {
        // Hypergraph with a hyperedge of size 3 (not a regular graph)
        let hg = HyperGraph::new(4, vec![vec![0, 1, 2]]);
        assert!(hg.to_graph_edges().is_none());
    }

    #[test]
    fn test_hypergraph_get_edge() {
        let hg = HyperGraph::new(4, vec![vec![0, 1, 2], vec![2, 3]]);
        assert_eq!(hg.edge(0), Some(&vec![0, 1, 2]));
        assert_eq!(hg.edge(1), Some(&vec![2, 3]));
        assert_eq!(hg.edge(2), None);
    }

    #[test]
    #[should_panic(expected = "vertex index 5 out of bounds")]
    fn test_hypergraph_invalid_vertex() {
        HyperGraph::new(4, vec![vec![0, 5]]);
    }

    #[test]
    #[should_panic(expected = "vertex index 4 out of bounds")]
    fn test_hypergraph_add_invalid_edge() {
        let mut hg = HyperGraph::empty(4);
        hg.add_edge(vec![0, 4]);
    }
}
