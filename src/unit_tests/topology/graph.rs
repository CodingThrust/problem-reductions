use super::*;

#[test]
fn test_simple_graph_new() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]).unwrap();
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
    let graph = SimpleGraph::grid(2, 3).unwrap();
    assert_eq!(graph.num_vertices(), 6);
    // 2 rows: 2 horizontal edges per row = 4
    // 3 cols: 1 vertical edge per col = 3
    assert_eq!(graph.num_edges(), 7);
}

#[test]
fn test_simple_graph_has_edge() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]).unwrap();
    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(1, 0)); // Undirected
    assert!(graph.has_edge(1, 2));
    assert!(!graph.has_edge(0, 2));
}

#[test]
fn test_simple_graph_neighbors() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]).unwrap();
    let mut neighbors = graph.neighbors(0);
    neighbors.sort();
    assert_eq!(neighbors, vec![1, 2, 3]);
    assert_eq!(graph.neighbors(1), vec![0]);
}

#[test]
fn test_simple_graph_degree() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]).unwrap();
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
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]).unwrap();
    let mut count = 0;
    graph.for_each_edge(|_, _| count += 1);
    assert_eq!(count, 2);
}

#[test]
fn test_simple_graph_eq() {
    let g1 = SimpleGraph::new(3, vec![(0, 1), (1, 2)]).unwrap();
    let g2 = SimpleGraph::new(3, vec![(1, 2), (0, 1)]).unwrap(); // Different order
    let g3 = SimpleGraph::new(3, vec![(0, 1)]).unwrap();

    assert_eq!(g1, g2);
    assert_ne!(g1, g3);
}

#[test]
fn test_simple_graph_invalid_edge() {
    assert!(SimpleGraph::new(3, vec![(0, 5)]).is_err());
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
    let g1 = SimpleGraph::new(3, vec![(0, 1)]).unwrap();
    let g2 = SimpleGraph::new(4, vec![(0, 1)]).unwrap(); // Different vertex count
    assert_ne!(g1, g2);
}

#[test]
fn test_simplegraph_json_roundtrip() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]).unwrap();
    let json = serde_json::to_value(&graph).unwrap();
    assert_eq!(json["num_vertices"], 4);
    let edges: Vec<(usize, usize)> = serde_json::from_value(json["edges"].clone()).unwrap();
    assert_eq!(edges.len(), 3);
    let roundtrip: SimpleGraph = serde_json::from_value(json).unwrap();
    assert_eq!(graph, roundtrip);
}

#[test]
fn test_simplegraph_json_format() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]).unwrap();
    let json_str = serde_json::to_string(&graph).unwrap();
    assert!(!json_str.contains("edge_property"));
    assert!(!json_str.contains("node_holes"));
    assert!(json_str.contains("num_vertices"));
}

#[test]
fn deserialize_rejects_out_of_range_edges() {
    assert!(serde_json::from_value::<SimpleGraph>(serde_json::json!({
        "num_vertices": 2, "edges": [[0, 2]]
    }))
    .is_err());
    assert!(SimpleGraph::grid(usize::MAX, 2).is_err());
}
