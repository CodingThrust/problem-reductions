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

#[test]
fn test_grid_graph_display() {
    let nodes = vec![GridNode::new(0, 0, 1), GridNode::new(1, 0, 2)];
    let grid = GridGraph::new(GridType::Square, (2, 2), nodes, 2.0);

    // Test Display trait
    let display_str = format!("{}", grid);
    assert!(!display_str.is_empty());
}

#[test]
fn test_grid_graph_format_empty() {
    let nodes: Vec<GridNode<i32>> = vec![];
    let grid = GridGraph::new(GridType::Square, (0, 0), nodes, 1.0);

    // Empty grid should return "(empty grid graph)"
    let formatted = grid.format_with_config(None, false);
    assert_eq!(formatted, "(empty grid graph)");
}

#[test]
fn test_grid_graph_format_with_config() {
    let nodes = vec![GridNode::new(0, 0, 1), GridNode::new(1, 0, 1)];
    let grid = GridGraph::new(GridType::Square, (2, 2), nodes, 2.0);

    // Test format with config
    let formatted = grid.format_with_config(Some(&[1, 0]), false);
    assert!(!formatted.is_empty());
}
