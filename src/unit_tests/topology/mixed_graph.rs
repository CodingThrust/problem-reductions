use crate::topology::MixedGraph;

#[test]
fn test_mixed_graph_creation_and_counts() {
    let graph = MixedGraph::new(4, vec![(0, 1), (2, 3)], vec![(0, 2), (1, 3)]);

    assert_eq!(graph.num_vertices(), 4);
    assert_eq!(graph.num_arcs(), 2);
    assert_eq!(graph.num_edges(), 2);

    let mut arcs = graph.arcs();
    arcs.sort();
    assert_eq!(arcs, vec![(0, 1), (2, 3)]);

    let mut edges = graph.edges();
    edges.sort();
    assert_eq!(edges, vec![(0, 2), (1, 3)]);
}

#[test]
fn test_mixed_graph_incidence_queries() {
    let graph = MixedGraph::new(4, vec![(0, 1), (2, 1)], vec![(1, 3), (0, 2)]);

    assert!(graph.has_arc(0, 1));
    assert!(!graph.has_arc(1, 0));
    assert!(graph.has_edge(1, 3));
    assert!(graph.has_edge(3, 1));
    assert!(!graph.has_edge(0, 3));

    assert_eq!(graph.out_degree(0), 1);
    assert_eq!(graph.in_degree(1), 2);
    assert_eq!(graph.undirected_degree(1), 1);
    assert_eq!(graph.undirected_degree(0), 1);
}

#[test]
fn test_mixed_graph_has_edge_is_order_insensitive() {
    let graph = MixedGraph::new(3, vec![], vec![(2, 0)]);

    assert!(graph.has_edge(0, 2));
    assert!(graph.has_edge(2, 0));
}

#[test]
fn test_mixed_graph_serialization_roundtrip() {
    let graph = MixedGraph::new(5, vec![(0, 1), (1, 4)], vec![(0, 2), (2, 3), (3, 4)]);

    let json = serde_json::to_string(&graph).unwrap();
    let restored: MixedGraph = serde_json::from_str(&json).unwrap();

    assert_eq!(restored, graph);
}

#[test]
#[should_panic(expected = "references vertex >= num_vertices")]
fn test_mixed_graph_panics_on_out_of_bounds_arc() {
    MixedGraph::new(3, vec![(0, 3)], vec![]);
}
