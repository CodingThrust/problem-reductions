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
