use crate::topology::MultiMixedGraph;

#[test]
fn test_mixed_graph_creation_and_counts() {
    let graph = MultiMixedGraph::new(4, vec![(0, 1), (2, 3)], vec![(0, 2), (1, 3)]);

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
    let graph = MultiMixedGraph::new(4, vec![(0, 1), (2, 1)], vec![(1, 3), (0, 2)]);

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
    let graph = MultiMixedGraph::new(3, vec![], vec![(2, 0)]);

    assert!(graph.has_edge(0, 2));
    assert!(graph.has_edge(2, 0));
}

#[test]
fn test_mixed_graph_serialization_roundtrip() {
    let graph = MultiMixedGraph::new(5, vec![(0, 1), (1, 4)], vec![(0, 2), (2, 3), (3, 4)]);

    let json = serde_json::to_string(&graph).unwrap();
    let restored: MultiMixedGraph = serde_json::from_str(&json).unwrap();

    assert_eq!(restored, graph);
}

#[test]
#[should_panic(expected = "references vertex >= num_vertices")]
fn test_mixed_graph_panics_on_out_of_bounds_arc() {
    MultiMixedGraph::new(3, vec![(0, 3)], vec![]);
}

#[test]
fn deserialize_checks_both_arc_and_edge_endpoints() {
    for (arcs, edges) in [(vec![(0, 2)], vec![]), (vec![], vec![(2, 0)])] {
        assert!(
            serde_json::from_value::<MultiMixedGraph>(serde_json::json!({
                "num_vertices": 2, "arcs": arcs, "edges": edges
            }))
            .is_err()
        );
    }
}

#[test]
fn multi_mixed_graph_preserves_loops_and_parallel_records() {
    let graph = crate::topology::MultiMixedGraph::new(
        2,
        vec![(0, 0), (0, 1), (0, 1)],
        vec![(0, 0), (0, 1), (1, 0)],
    );
    assert_eq!(graph.num_arcs(), 3);
    assert_eq!(graph.num_edges(), 3);
    assert_eq!(graph.arcs(), vec![(0, 0), (0, 1), (0, 1)]);
    assert_eq!(graph.edges(), vec![(0, 0), (0, 1), (1, 0)]);
    let restored: crate::topology::MultiMixedGraph =
        serde_json::from_value(serde_json::to_value(&graph).unwrap()).unwrap();
    assert_eq!(restored.arcs(), graph.arcs());
    assert_eq!(restored.edges(), graph.edges());
}
