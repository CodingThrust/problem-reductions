use super::*;

#[test]
fn test_layout_empty() {
    let layout = Layout::empty(5);
    assert_eq!(layout.vertices.len(), 0);
    assert_eq!(layout.vsep(), 0);
    assert_eq!(layout.disconnected.len(), 5);
    assert_eq!(layout.neighbors.len(), 0);
}

#[test]
fn test_layout_new() {
    // Path graph: 0-1-2
    let edges = vec![(0, 1), (1, 2)];
    let layout = Layout::new(3, &edges, vec![0, 1, 2]);
    assert_eq!(layout.vertices, vec![0, 1, 2]);
    assert_eq!(layout.vsep(), 1); // Path has pathwidth 1
}

#[test]
fn test_vsep_and_neighbors_path() {
    // Path: 0-1-2
    let edges = vec![(0, 1), (1, 2)];
    let (vsep, _) = vsep_and_neighbors(3, &edges, &[0, 1, 2]);
    assert_eq!(vsep, 1);
}

#[test]
fn test_vsep_and_neighbors_star() {
    // Star: 0 connected to 1, 2, 3
    let edges = vec![(0, 1), (0, 2), (0, 3)];
    // Order: 0, 1, 2, 3 - after adding 0, all others become neighbors
    let (vsep, _) = vsep_and_neighbors(4, &edges, &[0, 1, 2, 3]);
    assert_eq!(vsep, 3); // After adding 0, neighbors = {1, 2, 3}
}

#[test]
fn test_extend() {
    // Path: 0-1-2
    let edges = vec![(0, 1), (1, 2)];
    let layout = Layout::empty(3);
    let layout = extend(3, &edges, &layout, 0);
    assert_eq!(layout.vertices, vec![0]);
    assert!(layout.neighbors.contains(&1));
    assert!(layout.disconnected.contains(&2));
}

#[test]
fn test_greedy_decompose_path() {
    // Path: 0-1-2
    let edges = vec![(0, 1), (1, 2)];
    let layout = greedy_decompose(3, &edges);
    assert_eq!(layout.vertices.len(), 3);
    assert_eq!(layout.vsep(), 1);
}

#[test]
fn test_greedy_decompose_triangle() {
    // Triangle: 0-1, 1-2, 0-2
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let layout = greedy_decompose(3, &edges);
    assert_eq!(layout.vertices.len(), 3);
    assert_eq!(layout.vsep(), 2); // Triangle has pathwidth 2
}

#[test]
fn test_greedy_decompose_k4() {
    // Complete graph K4
    let edges = vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    let layout = greedy_decompose(4, &edges);
    assert_eq!(layout.vertices.len(), 4);
    assert_eq!(layout.vsep(), 3); // K4 has pathwidth 3
}

#[test]
fn test_branch_and_bound_path() {
    // Path: 0-1-2
    let edges = vec![(0, 1), (1, 2)];
    let layout = branch_and_bound(3, &edges);
    assert_eq!(layout.vertices.len(), 3);
    assert_eq!(layout.vsep(), 1);
}

#[test]
fn test_branch_and_bound_triangle() {
    // Triangle
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let layout = branch_and_bound(3, &edges);
    assert_eq!(layout.vertices.len(), 3);
    assert_eq!(layout.vsep(), 2);
}

#[test]
fn test_pathwidth_greedy() {
    let edges = vec![(0, 1), (1, 2)];
    let layout = pathwidth(3, &edges, PathDecompositionMethod::greedy());
    assert_eq!(layout.vertices.len(), 3);
    assert_eq!(layout.vsep(), 1);
}

#[test]
fn test_pathwidth_minhthi() {
    let edges = vec![(0, 1), (1, 2)];
    let layout = pathwidth(3, &edges, PathDecompositionMethod::MinhThiTrick);
    assert_eq!(layout.vertices.len(), 3);
    assert_eq!(layout.vsep(), 1);
}

#[test]
fn test_vertex_order_from_layout() {
    let layout = Layout {
        vertices: vec![0, 1, 2],
        vsep: 1,
        neighbors: vec![],
        disconnected: vec![],
    };
    let order = vertex_order_from_layout(&layout);
    // Returns vertices in same order as layout (matching Julia's behavior)
    assert_eq!(order, vec![0, 1, 2]);
}

#[test]
fn test_petersen_graph_pathwidth() {
    // Petersen graph edges
    let edges = vec![
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 4),
        (4, 0), // outer pentagon
        (5, 7),
        (7, 9),
        (9, 6),
        (6, 8),
        (8, 5), // inner star
        (0, 5),
        (1, 6),
        (2, 7),
        (3, 8),
        (4, 9), // connections
    ];

    let layout = pathwidth(10, &edges, PathDecompositionMethod::MinhThiTrick);
    assert_eq!(layout.vertices.len(), 10);
    // Petersen graph has pathwidth 5
    assert_eq!(layout.vsep(), 5);
}

#[test]
fn test_cycle_graph_pathwidth() {
    // Cycle C5: 0-1-2-3-4-0
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)];
    let layout = pathwidth(5, &edges, PathDecompositionMethod::MinhThiTrick);
    assert_eq!(layout.vertices.len(), 5);
    // Cycle has pathwidth 2
    assert_eq!(layout.vsep(), 2);
}

#[test]
fn test_disconnected_graph() {
    // Two disconnected edges: 0-1, 2-3
    let edges = vec![(0, 1), (2, 3)];
    let layout = pathwidth(4, &edges, PathDecompositionMethod::MinhThiTrick);
    assert_eq!(layout.vertices.len(), 4);
    // Pathwidth is 1 (each component has pathwidth 1)
    assert_eq!(layout.vsep(), 1);
}

#[test]
fn test_empty_graph() {
    // No edges
    let edges: Vec<(usize, usize)> = vec![];
    let layout = pathwidth(5, &edges, PathDecompositionMethod::MinhThiTrick);
    assert_eq!(layout.vertices.len(), 5);
    assert_eq!(layout.vsep(), 0); // No edges means pathwidth 0
}
