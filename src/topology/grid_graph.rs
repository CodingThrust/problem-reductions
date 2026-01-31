//! Grid Graph implementation.
//!
//! A grid graph is a weighted graph on a 2D integer lattice, where edges are
//! determined by distance (unit disk graph property). Supports both square
//! and triangular lattice geometries.

use super::graph::Graph;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The type of grid lattice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridType {
    /// Square lattice where physical position (row, col) = (row, col).
    Square,
    /// Triangular lattice where:
    /// - y = col * (sqrt(3) / 2)
    /// - x = row + offset, where offset is 0.5 for odd/even columns depending on `offset_even_cols`
    Triangular {
        /// If true, even columns are offset by 0.5; if false, odd columns are offset.
        offset_even_cols: bool,
    },
}

/// A node in a grid graph with integer coordinates and a weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridNode<W> {
    /// Row coordinate (integer).
    pub row: i32,
    /// Column coordinate (integer).
    pub col: i32,
    /// Weight of the node.
    pub weight: W,
}

impl<W> GridNode<W> {
    /// Create a new grid node.
    pub fn new(row: i32, col: i32, weight: W) -> Self {
        Self { row, col, weight }
    }
}

/// A weighted graph on a 2D integer lattice.
///
/// Edges are determined by distance: two nodes are connected if their
/// physical distance is at most the specified radius.
///
/// # Example
///
/// ```
/// use problemreductions::topology::{Graph, GridGraph, GridNode, GridType};
///
/// let nodes = vec![
///     GridNode::new(0, 0, 1),
///     GridNode::new(1, 0, 1),
///     GridNode::new(0, 1, 1),
/// ];
/// let grid = GridGraph::new(GridType::Square, (2, 2), nodes, 1.5);
/// assert_eq!(grid.num_vertices(), 3);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridGraph<W> {
    /// The type of grid lattice.
    grid_type: GridType,
    /// The size of the grid as (rows, cols).
    size: (usize, usize),
    /// The nodes in the graph.
    nodes: Vec<GridNode<W>>,
    /// The radius threshold for edge creation.
    radius: f64,
    /// Precomputed edges as (node_index, node_index) pairs.
    edges: Vec<(usize, usize)>,
}

impl<W: Clone> GridGraph<W> {
    /// Create a new grid graph.
    ///
    /// # Arguments
    ///
    /// * `grid_type` - The type of lattice (Square or Triangular)
    /// * `size` - The size of the grid as (rows, cols)
    /// * `nodes` - The nodes in the graph with their coordinates and weights
    /// * `radius` - Maximum distance for an edge to exist
    pub fn new(
        grid_type: GridType,
        size: (usize, usize),
        nodes: Vec<GridNode<W>>,
        radius: f64,
    ) -> Self {
        let n = nodes.len();
        let mut edges = Vec::new();

        // Compute all edges based on physical distance
        // Use strict < to match Julia's unitdisk_graph which uses: dist² < radius²
        for i in 0..n {
            for j in (i + 1)..n {
                let pos_i = Self::physical_position_static(grid_type, nodes[i].row, nodes[i].col);
                let pos_j = Self::physical_position_static(grid_type, nodes[j].row, nodes[j].col);
                let dist = Self::distance(&pos_i, &pos_j);
                if dist < radius {
                    edges.push((i, j));
                }
            }
        }

        Self {
            grid_type,
            size,
            nodes,
            radius,
            edges,
        }
    }

    /// Get the grid type.
    pub fn grid_type(&self) -> GridType {
        self.grid_type
    }

    /// Get the size of the grid as (rows, cols).
    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    /// Get the radius threshold.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Get the nodes.
    pub fn nodes(&self) -> &[GridNode<W>] {
        &self.nodes
    }

    /// Get a node by index.
    pub fn node(&self, index: usize) -> Option<&GridNode<W>> {
        self.nodes.get(index)
    }

    /// Get the weight of a node by index.
    pub fn weight(&self, index: usize) -> Option<&W> {
        self.nodes.get(index).map(|n| &n.weight)
    }

    /// Compute the physical position of a grid coordinate.
    ///
    /// For Square: (row, col) -> (row, col)
    /// For Triangular:
    ///   - y = col * (sqrt(3) / 2)
    ///   - x = row + offset, where offset is 0.5 for odd/even columns
    pub fn physical_position(&self, row: i32, col: i32) -> (f64, f64) {
        Self::physical_position_static(self.grid_type, row, col)
    }

    /// Static version of physical_position for use during construction.
    fn physical_position_static(grid_type: GridType, row: i32, col: i32) -> (f64, f64) {
        match grid_type {
            GridType::Square => (row as f64, col as f64),
            GridType::Triangular { offset_even_cols } => {
                let y = col as f64 * (3.0_f64.sqrt() / 2.0);
                let offset = if offset_even_cols {
                    if col % 2 == 0 {
                        0.5
                    } else {
                        0.0
                    }
                } else if col % 2 != 0 {
                    0.5
                } else {
                    0.0
                };
                let x = row as f64 + offset;
                (x, y)
            }
        }
    }

    /// Compute Euclidean distance between two points.
    fn distance(p1: &(f64, f64), p2: &(f64, f64)) -> f64 {
        let dx = p1.0 - p2.0;
        let dy = p1.1 - p2.1;
        (dx * dx + dy * dy).sqrt()
    }

    /// Get all edges as a slice.
    pub fn edges(&self) -> &[(usize, usize)] {
        &self.edges
    }

    /// Get the physical position of a node by index.
    pub fn node_position(&self, index: usize) -> Option<(f64, f64)> {
        self.nodes
            .get(index)
            .map(|n| self.physical_position(n.row, n.col))
    }
}

impl<W: Clone + Send + Sync> Graph for GridGraph<W> {
    fn num_vertices(&self) -> usize {
        self.nodes.len()
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

impl<W: Clone + fmt::Display> GridGraph<W> {
    /// Format the grid graph as a string matching Julia's UnitDiskMapping format.
    ///
    /// Characters (matching Julia exactly):
    /// - `⋅` = empty cell
    /// - `●` = node (or selected node when config provided)
    /// - `○` = unselected node (when config provided)
    /// - Each cell is followed by a space
    ///
    /// When show_weight is true, displays the weight as a number for single digits.
    pub fn format_with_config(&self, config: Option<&[usize]>, show_weight: bool) -> String {
        use std::collections::HashMap;

        if self.nodes.is_empty() {
            return String::from("(empty grid graph)");
        }

        // Find grid bounds (use full size, not min/max of nodes)
        let (rows, cols) = self.size;

        // Build position to node index map
        let mut pos_to_idx: HashMap<(i32, i32), usize> = HashMap::new();
        for (idx, node) in self.nodes.iter().enumerate() {
            pos_to_idx.insert((node.row, node.col), idx);
        }

        let mut lines = Vec::new();

        for r in 0..rows as i32 {
            let mut line = String::new();
            for c in 0..cols as i32 {
                let s = if let Some(&idx) = pos_to_idx.get(&(r, c)) {
                    if let Some(cfg) = config {
                        if cfg.get(idx).copied().unwrap_or(0) > 0 {
                            "●".to_string() // Selected node
                        } else {
                            "○".to_string() // Unselected node
                        }
                    } else if show_weight {
                        Self::weight_str(&self.nodes[idx].weight)
                    } else {
                        "●".to_string()
                    }
                } else {
                    "⋅".to_string()
                };
                line.push_str(&s);
                line.push(' ');
            }
            // Remove trailing space
            line.pop();
            lines.push(line);
        }

        lines.join("\n")
    }

    /// Get a string representation of a weight.
    fn weight_str(weight: &W) -> String {
        let s = format!("{}", weight);
        if s.len() == 1 {
            s
        } else {
            "●".to_string()
        }
    }

    /// Print a configuration on this grid graph.
    ///
    /// This is equivalent to Julia's `print_config(res, c)`.
    pub fn print_config(&self, config: &[usize]) {
        print!("{}", self.format_with_config(Some(config), false));
    }
}

impl<W: Clone + fmt::Display> fmt::Display for GridGraph<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_with_config(None, true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_graph_square_basic() {
        let nodes = vec![
            GridNode::new(0, 0, 1),
            GridNode::new(1, 0, 1),
            GridNode::new(0, 1, 1),
        ];
        // With radius 1.1: (0,0)-(1,0) dist=1.0 < 1.1, (0,0)-(0,1) dist=1.0 < 1.1, (1,0)-(0,1) dist=sqrt(2)>1.1
        // Using dist < radius (strict), so edges at exactly 1.0 are included with radius 1.1
        let grid = GridGraph::new(GridType::Square, (2, 2), nodes, 1.1);
        assert_eq!(grid.num_vertices(), 3);
        // Only nodes at (0,0)-(1,0) and (0,0)-(0,1) are within radius 1.1
        assert_eq!(grid.edges().len(), 2);
    }

    #[test]
    fn test_grid_graph_triangular_basic() {
        let nodes = vec![
            GridNode::new(0, 0, 1),
            GridNode::new(1, 0, 1),
            GridNode::new(0, 1, 1),
        ];
        let grid = GridGraph::new(
            GridType::Triangular {
                offset_even_cols: false,
            },
            (2, 2),
            nodes,
            1.1,
        );
        assert_eq!(grid.num_vertices(), 3);
    }

    #[test]
    fn test_grid_node_new() {
        let node: GridNode<i32> = GridNode::new(5, 10, 42);
        assert_eq!(node.row, 5);
        assert_eq!(node.col, 10);
        assert_eq!(node.weight, 42);
    }

    #[test]
    fn test_grid_graph_square_physical_position() {
        let nodes = vec![GridNode::new(3, 4, 1)];
        let grid = GridGraph::new(GridType::Square, (10, 10), nodes, 1.0);
        let pos = grid.physical_position(3, 4);
        assert_eq!(pos, (3.0, 4.0));
    }

    #[test]
    fn test_grid_graph_triangular_physical_position() {
        let nodes = vec![GridNode::new(0, 0, 1)];
        let grid = GridGraph::new(
            GridType::Triangular {
                offset_even_cols: false,
            },
            (10, 10),
            nodes,
            1.0,
        );

        // Col 0 (even), offset_even_cols = false -> no offset
        let pos0 = grid.physical_position(0, 0);
        assert!((pos0.0 - 0.0).abs() < 1e-10);
        assert!((pos0.1 - 0.0).abs() < 1e-10);

        // Col 1 (odd), offset_even_cols = false -> offset 0.5
        let pos1 = grid.physical_position(0, 1);
        assert!((pos1.0 - 0.5).abs() < 1e-10);
        assert!((pos1.1 - (3.0_f64.sqrt() / 2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_grid_graph_triangular_offset_even() {
        let nodes = vec![GridNode::new(0, 0, 1)];
        let grid = GridGraph::new(
            GridType::Triangular {
                offset_even_cols: true,
            },
            (10, 10),
            nodes,
            1.0,
        );

        // Col 0 (even), offset_even_cols = true -> offset 0.5
        let pos0 = grid.physical_position(0, 0);
        assert!((pos0.0 - 0.5).abs() < 1e-10);

        // Col 1 (odd), offset_even_cols = true -> no offset
        let pos1 = grid.physical_position(0, 1);
        assert!((pos1.0 - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_grid_graph_edges_within_radius() {
        // Square grid: place nodes at (0,0), (1,0), (2,0)
        // Distance (0,0)-(1,0) = 1.0
        // Distance (0,0)-(2,0) = 2.0
        // Distance (1,0)-(2,0) = 1.0
        let nodes = vec![
            GridNode::new(0, 0, 1),
            GridNode::new(1, 0, 1),
            GridNode::new(2, 0, 1),
        ];
        // Use radius 1.1 since edges are created for dist < radius (strict)
        // With radius 1.0, no edges at exact distance 1.0
        // With radius 1.1, edges at distance 1.0 are included
        let grid = GridGraph::new(GridType::Square, (3, 1), nodes, 1.1);

        // Only edges within radius 1.1: (0,1) and (1,2) with dist=1.0
        assert_eq!(grid.num_edges(), 2);
        assert!(grid.has_edge(0, 1));
        assert!(grid.has_edge(1, 2));
        assert!(!grid.has_edge(0, 2)); // dist=2.0 >= 1.1
    }

    #[test]
    fn test_grid_graph_neighbors() {
        let nodes = vec![
            GridNode::new(0, 0, 1),
            GridNode::new(1, 0, 1),
            GridNode::new(0, 1, 1),
        ];
        let grid = GridGraph::new(GridType::Square, (2, 2), nodes, 1.5);

        let neighbors_0 = grid.neighbors(0);
        assert_eq!(neighbors_0.len(), 2);
        assert!(neighbors_0.contains(&1));
        assert!(neighbors_0.contains(&2));
    }

    #[test]
    fn test_grid_graph_accessors() {
        let nodes = vec![GridNode::new(0, 0, 10), GridNode::new(1, 0, 20)];
        let grid = GridGraph::new(GridType::Square, (5, 5), nodes, 2.0);

        assert_eq!(grid.grid_type(), GridType::Square);
        assert_eq!(grid.size(), (5, 5));
        assert_eq!(grid.radius(), 2.0);
        assert_eq!(grid.nodes().len(), 2);
        assert_eq!(grid.node(0).map(|n| n.weight), Some(10));
        assert_eq!(grid.weight(1), Some(&20));
        assert_eq!(grid.weight(5), None);
    }

    #[test]
    fn test_grid_graph_node_position() {
        let nodes = vec![GridNode::new(2, 3, 1)];
        let grid = GridGraph::new(GridType::Square, (10, 10), nodes, 1.0);

        let pos = grid.node_position(0);
        assert_eq!(pos, Some((2.0, 3.0)));
        assert_eq!(grid.node_position(1), None);
    }

    #[test]
    fn test_grid_graph_has_edge_symmetric() {
        let nodes = vec![GridNode::new(0, 0, 1), GridNode::new(1, 0, 1)];
        let grid = GridGraph::new(GridType::Square, (2, 1), nodes, 1.5);

        assert!(grid.has_edge(0, 1));
        assert!(grid.has_edge(1, 0)); // Symmetric
    }

    #[test]
    fn test_grid_graph_empty() {
        let nodes: Vec<GridNode<i32>> = vec![];
        let grid = GridGraph::new(GridType::Square, (0, 0), nodes, 1.0);

        assert_eq!(grid.num_vertices(), 0);
        assert_eq!(grid.num_edges(), 0);
        assert!(grid.is_empty());
    }

    #[test]
    fn test_grid_graph_graph_trait() {
        let nodes = vec![
            GridNode::new(0, 0, 1),
            GridNode::new(1, 0, 1),
            GridNode::new(0, 1, 1),
        ];
        // With radius 1.1: 2 edges at dist=1.0 (not including diagonal at sqrt(2)>1.1)
        // Using dist < radius (strict), so edges at exactly 1.0 are included with radius 1.1
        let grid = GridGraph::new(GridType::Square, (2, 2), nodes, 1.1);

        // Test Graph trait methods
        assert_eq!(Graph::num_vertices(&grid), 3);
        assert_eq!(Graph::num_edges(&grid), 2);
        assert_eq!(grid.degree(0), 2);
        assert_eq!(grid.degree(1), 1);
        assert_eq!(grid.degree(2), 1);
    }
}
