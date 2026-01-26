//! Graph to grid mapping functions.

use super::copyline::{create_copylines, mis_overhead_copyline, CopyLine};
use super::grid::MappingGrid;
use crate::topology::{GridGraph, GridNode, GridType};
use serde::{Deserialize, Serialize};

const DEFAULT_SPACING: usize = 4;
const DEFAULT_PADDING: usize = 2;
const SQUARE_UNIT_RADIUS: f64 = 1.5;

/// Result of mapping a graph to a grid graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingResult {
    /// The resulting grid graph.
    pub grid_graph: GridGraph<i32>,
    /// Copy lines used in the mapping.
    pub lines: Vec<CopyLine>,
    /// Padding used.
    pub padding: usize,
    /// Spacing used.
    pub spacing: usize,
    /// MIS overhead from the mapping.
    pub mis_overhead: i32,
}

impl MappingResult {
    /// Map a configuration back from grid to original graph.
    pub fn map_config_back(&self, grid_config: &[usize]) -> Vec<usize> {
        let mut result = vec![0; self.lines.len()];

        for line in &self.lines {
            let locs = line.locations(self.padding, self.spacing);
            let mut count = 0;

            for &(row, col, _weight) in locs.iter() {
                // Find the node index at this location
                if let Some(node_idx) = self.find_node_at(row, col) {
                    if grid_config.get(node_idx).copied().unwrap_or(0) > 0 {
                        count += 1;
                    }
                }
            }

            // The original vertex is in the IS if count exceeds half the line length
            result[line.vertex] = if count > locs.len() / 2 { 1 } else { 0 };
        }

        result
    }

    fn find_node_at(&self, row: usize, col: usize) -> Option<usize> {
        self.grid_graph
            .nodes()
            .iter()
            .position(|n| n.row as usize == row && n.col as usize == col)
    }
}

/// Internal function that creates both the mapping grid and copylines.
fn embed_graph_internal(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> Option<(MappingGrid, Vec<CopyLine>)> {
    if num_vertices == 0 {
        return None;
    }

    let spacing = DEFAULT_SPACING;
    let padding = DEFAULT_PADDING;

    let copylines = create_copylines(num_vertices, edges, vertex_order);

    // Calculate grid dimensions
    let max_hslot = copylines.iter().map(|l| l.hslot).max().unwrap_or(1);
    let max_vslot = copylines.iter().map(|l| l.vslot).max().unwrap_or(1);
    let max_hstop = copylines.iter().map(|l| l.hstop).max().unwrap_or(1);
    let max_vstop = copylines.iter().map(|l| l.vstop).max().unwrap_or(1);

    let rows = max_hslot.max(max_vstop) * spacing + 2 + 2 * padding;
    let cols = max_vslot.max(max_hstop) * spacing + 2 + 2 * padding;

    let mut grid = MappingGrid::with_padding(rows, cols, spacing, padding);

    // Add copy line nodes
    for line in &copylines {
        for (row, col, weight) in line.locations(padding, spacing) {
            grid.add_node(row, col, weight as i32);
        }
    }

    // Mark edge connections
    for &(u, v) in edges {
        let u_idx = vertex_order
            .iter()
            .position(|&x| x == u)
            .expect("Edge vertex u not found in vertex_order");
        let v_idx = vertex_order
            .iter()
            .position(|&x| x == v)
            .expect("Edge vertex v not found in vertex_order");
        let u_line = &copylines[u_idx];
        let v_line = &copylines[v_idx];

        let (row, col) = grid.cross_at(u_line.vslot, v_line.vslot, u_line.hslot.min(v_line.hslot));

        // Mark connected cells
        if col > 0 {
            grid.connect(row, col - 1);
        }
        if row > 0 && grid.is_occupied(row - 1, col) {
            grid.connect(row - 1, col);
        } else if row + 1 < grid.size().0 && grid.is_occupied(row + 1, col) {
            grid.connect(row + 1, col);
        }
    }

    Some((grid, copylines))
}

/// Embed a graph into a mapping grid.
///
/// # Panics
///
/// Panics if any edge vertex is not found in `vertex_order`.
pub fn embed_graph(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> Option<MappingGrid> {
    embed_graph_internal(num_vertices, edges, vertex_order).map(|(grid, _)| grid)
}

/// Map a graph to a grid graph.
pub fn map_graph(num_vertices: usize, edges: &[(usize, usize)]) -> MappingResult {
    // Use simple ordering: 0, 1, 2, ...
    let vertex_order: Vec<usize> = (0..num_vertices).collect();
    map_graph_with_order(num_vertices, edges, &vertex_order)
}

/// Map a graph with a specific vertex ordering.
///
/// # Panics
///
/// Panics if `num_vertices == 0`.
pub fn map_graph_with_order(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertex_order: &[usize],
) -> MappingResult {
    let spacing = DEFAULT_SPACING;
    let padding = DEFAULT_PADDING;

    let (grid, copylines) = embed_graph_internal(num_vertices, edges, vertex_order)
        .expect("Failed to embed graph: num_vertices must be > 0");

    // Calculate MIS overhead
    let mis_overhead: i32 = copylines
        .iter()
        .map(|line| mis_overhead_copyline(line, spacing) as i32)
        .sum();

    // Convert to GridGraph
    let nodes: Vec<GridNode<i32>> = grid
        .occupied_coords()
        .into_iter()
        .filter_map(|(row, col)| {
            grid.get(row, col).map(|cell| {
                GridNode::new(row as i32, col as i32, cell.weight())
            })
        })
        .filter(|n| n.weight > 0)
        .collect();

    let grid_graph = GridGraph::new(GridType::Square, grid.size(), nodes, SQUARE_UNIT_RADIUS);

    MappingResult {
        grid_graph,
        lines: copylines,
        padding,
        spacing,
        mis_overhead,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::Graph;

    #[test]
    fn test_embed_graph_path() {
        // Path graph: 0-1-2
        let edges = vec![(0, 1), (1, 2)];
        let result = embed_graph(3, &edges, &[0, 1, 2]);

        assert!(result.is_some());
        let grid = result.unwrap();
        assert!(!grid.occupied_coords().is_empty());
    }

    #[test]
    fn test_map_graph_triangle() {
        // Triangle graph
        let edges = vec![(0, 1), (1, 2), (0, 2)];
        let result = map_graph(3, &edges);

        assert!(result.grid_graph.num_vertices() > 0);
        assert!(result.mis_overhead >= 0);
    }

    #[test]
    fn test_mapping_result_config_back() {
        let edges = vec![(0, 1)];
        let result = map_graph(2, &edges);

        // Create a dummy config
        let config: Vec<usize> = vec![0; result.grid_graph.num_vertices()];
        let original = result.map_config_back(&config);

        assert_eq!(original.len(), 2);
    }
}
