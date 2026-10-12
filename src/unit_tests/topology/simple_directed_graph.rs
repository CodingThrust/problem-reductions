use crate::topology::SimpleDirectedGraph;

#[test]
fn simple_directed_graph_rejects_loops_and_parallel_arcs() {
    for (arcs, offending) in [
        (vec![(0, 0)], "arc (0, 0)"),
        (vec![(0, 1), (0, 1)], "arc (0, 1)"),
        (vec![(0, 3)], "arc (0, 3)"),
    ] {
        let error = SimpleDirectedGraph::try_new(3, arcs.clone()).unwrap_err();
        assert!(matches!(
            error,
            crate::registry::ConstructionError::Conversion(_)
        ));
        assert!(error.to_string().contains(offending), "{error}");
        let error = serde_json::from_value::<SimpleDirectedGraph>(serde_json::json!({
            "num_vertices": 3, "arcs": arcs,
        }))
        .unwrap_err();
        assert!(error.to_string().contains(offending), "{error}");
        assert!(std::panic::catch_unwind(|| SimpleDirectedGraph::new(3, arcs)).is_err());
    }
}

#[test]
fn simple_directed_graph_preserves_direction_and_delegated_algorithms() {
    let graph = SimpleDirectedGraph::new(2, vec![(0, 1), (1, 0)]);
    assert_eq!(graph.num_vertices(), 2);
    assert_eq!(graph.num_arcs(), 2);
    assert_eq!(graph.arcs(), vec![(0, 1), (1, 0)]);
    assert!(graph.has_arc(0, 1));
    assert!(graph.has_arc(1, 0));
    assert!(!graph.has_arc(0, 0));
    assert_eq!(graph.successors(0), vec![1]);
    assert!(graph.is_strongly_connected());
    assert!(!graph.is_dag());
    let json = serde_json::json!({"num_vertices": 2, "arcs": [[0,1],[1,0]]});
    assert_eq!(serde_json::to_value(&graph).unwrap(), json);
    assert_eq!(
        serde_json::from_value::<SimpleDirectedGraph>(json).unwrap(),
        graph
    );

    let empty = SimpleDirectedGraph::empty(3);
    assert!(empty.is_dag());
    assert!(!empty.is_strongly_connected());
    assert_eq!(empty.inner().num_arcs(), 0);
}
