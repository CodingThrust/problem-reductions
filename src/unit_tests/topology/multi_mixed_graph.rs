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
    for (arcs, edges, offending) in [
        (vec![(0, 2)], vec![], "arc (0, 2)"),
        (vec![], vec![(2, 0)], "edge (2, 0)"),
    ] {
        let error = serde_json::from_value::<MultiMixedGraph>(serde_json::json!({
            "num_vertices": 2, "arcs": arcs, "edges": edges
        }))
        .unwrap_err();
        assert!(error.to_string().contains(offending), "{error}");
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

#[test]
fn mixed_graph_incidence_counts_each_parallel_record_and_loop() {
    let graph = MultiMixedGraph::new(
        2,
        vec![(0, 0), (0, 1), (0, 1), (1, 0)],
        vec![(0, 0), (0, 1), (1, 0)],
    );
    assert_eq!(graph.out_degree(0), 3);
    assert_eq!(graph.in_degree(0), 2);
    assert_eq!(graph.out_degree(1), 1);
    assert_eq!(graph.in_degree(1), 2);
    // This API counts incident edge records, so a loop contributes one.
    assert_eq!(graph.undirected_degree(0), 3);
    assert_eq!(graph.undirected_degree(1), 2);
}

#[test]
fn mixed_graph_equality_preserves_vertices_direction_and_multiplicity() {
    let graph = MultiMixedGraph::new(2, vec![(0, 1), (0, 1)], vec![(0, 1), (1, 0)]);
    assert_eq!(
        graph,
        MultiMixedGraph::new(2, vec![(0, 1), (0, 1)], vec![(1, 0), (0, 1)])
    );
    assert_ne!(graph, MultiMixedGraph::new(3, graph.arcs(), graph.edges()));
    assert_ne!(
        graph,
        MultiMixedGraph::new(2, vec![(0, 1), (1, 0)], graph.edges())
    );
    assert_ne!(graph, MultiMixedGraph::new(2, vec![(0, 1)], graph.edges()));
    assert_ne!(graph, MultiMixedGraph::new(2, graph.arcs(), vec![(0, 1)]));

    let empty = MultiMixedGraph::empty(0);
    let isolated = MultiMixedGraph::empty(2);
    assert!(empty.is_empty());
    assert!(!isolated.is_empty());
    assert_eq!(isolated.num_vertices(), 2);
    assert_eq!(isolated.num_arcs(), 0);
    assert_eq!(isolated.num_edges(), 0);
    assert_ne!(empty, isolated);
}

#[test]
#[should_panic(expected = "edge (3, 0) references vertex >= num_vertices (3)")]
fn mixed_graph_construction_rejects_an_out_of_range_edge() {
    MultiMixedGraph::new(3, vec![], vec![(3, 0)]);
}
