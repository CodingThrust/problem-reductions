use super::*;

#[test]
fn test_simple_graph_new() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    assert_eq!(graph.num_vertices(), 4);
    assert_eq!(graph.num_edges(), 3);
}

#[test]
fn test_simple_graph_empty() {
    let graph = SimpleGraph::empty(5);
    assert_eq!(graph.num_vertices(), 5);
    assert_eq!(graph.num_edges(), 0);
}

#[test]
fn test_simple_graph_complete() {
    let graph = SimpleGraph::complete(4);
    assert_eq!(graph.num_vertices(), 4);
    assert_eq!(graph.num_edges(), 6); // C(4,2) = 6
}

#[test]
fn test_simple_graph_path() {
    let graph = SimpleGraph::path(5);
    assert_eq!(graph.num_vertices(), 5);
    assert_eq!(graph.num_edges(), 4);
    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(3, 4));
    assert!(!graph.has_edge(0, 4));
}

#[test]
fn test_simple_graph_cycle() {
    let graph = SimpleGraph::cycle(4);
    assert_eq!(graph.num_vertices(), 4);
    assert_eq!(graph.num_edges(), 4);
    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(3, 0)); // Cycle edge
}

#[test]
fn test_simple_graph_star() {
    let graph = SimpleGraph::star(5);
    assert_eq!(graph.num_vertices(), 5);
    assert_eq!(graph.num_edges(), 4);
    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(0, 4));
    assert!(!graph.has_edge(1, 2));
}

#[test]
fn test_simple_graph_grid() {
    let graph = SimpleGraph::grid(2, 3);
    assert_eq!(graph.num_vertices(), 6);
    // 2 rows: 2 horizontal edges per row = 4
    // 3 cols: 1 vertical edge per col = 3
    assert_eq!(graph.num_edges(), 7);
}

#[test]
fn test_simple_graph_has_edge() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(1, 0)); // Undirected
    assert!(graph.has_edge(1, 2));
    assert!(!graph.has_edge(0, 2));
}

#[test]
fn test_simple_graph_neighbors() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]);
    let mut neighbors = graph.neighbors(0);
    neighbors.sort();
    assert_eq!(neighbors, vec![1, 2, 3]);
    assert_eq!(graph.neighbors(1), vec![0]);
}

#[test]
fn test_simple_graph_degree() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]);
    assert_eq!(graph.degree(0), 3);
    assert_eq!(graph.degree(1), 1);
}

#[test]
fn test_simple_graph_is_empty() {
    let empty = SimpleGraph::empty(0);
    assert!(empty.is_empty());

    let non_empty = SimpleGraph::empty(1);
    assert!(!non_empty.is_empty());
}

#[test]
fn test_simple_graph_for_each_edge() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let mut count = 0;
    graph.for_each_edge(|_, _| count += 1);
    assert_eq!(count, 2);
}

#[test]
fn test_simple_graph_eq() {
    let g1 = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let g2 = SimpleGraph::new(3, vec![(1, 2), (0, 1)]); // Different order
    let g3 = SimpleGraph::new(3, vec![(0, 1)]);

    assert_eq!(g1, g2);
    assert_ne!(g1, g3);
}

#[test]
#[should_panic(expected = "edge (0, 5) references vertex >= num_vertices")]
fn test_simple_graph_invalid_edge() {
    SimpleGraph::new(3, vec![(0, 5)]);
}

#[test]
fn test_simple_graph_cycle_small() {
    // Test cycle with fewer than 3 vertices (should fall back to path)
    let graph = SimpleGraph::cycle(2);
    assert_eq!(graph.num_vertices(), 2);
    assert_eq!(graph.num_edges(), 1); // Path: 0-1
    assert!(graph.has_edge(0, 1));
}

#[test]
fn test_simple_graph_eq_different_sizes() {
    // Test PartialEq when graphs have different sizes
    let g1 = SimpleGraph::new(3, vec![(0, 1)]);
    let g2 = SimpleGraph::new(4, vec![(0, 1)]); // Different vertex count
    assert_ne!(g1, g2);
}
