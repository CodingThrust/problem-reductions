use crate::topology::{BipartiteGraph, Graph};

#[test]
fn test_bipartite_graph_basic() {
    // K_{2,3}: left={0,1}, right={0,1,2}, all edges
    let edges = vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)];
    let g = BipartiteGraph::new(2, 3, edges);
    assert_eq!(g.num_vertices(), 5);
    assert_eq!(g.num_edges(), 6);
    assert_eq!(g.left_size(), 2);
    assert_eq!(g.right_size(), 3);
}

#[test]
fn test_bipartite_graph_edges_unified() {
    let g = BipartiteGraph::new(1, 2, vec![(0, 0), (0, 1)]);
    let edges = g.edges();
    assert!(edges.contains(&(0, 1)));
    assert!(edges.contains(&(0, 2)));
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_bipartite_graph_has_edge() {
    let g = BipartiteGraph::new(2, 2, vec![(0, 0), (1, 1)]);
    assert!(g.has_edge(0, 2));
    assert!(g.has_edge(1, 3));
    assert!(!g.has_edge(0, 1));
    assert!(!g.has_edge(0, 3));
}

#[test]
fn test_bipartite_graph_neighbors() {
    let g = BipartiteGraph::new(2, 2, vec![(0, 0), (0, 1), (1, 1)]);
    let mut n0 = g.neighbors(0);
    n0.sort();
    assert_eq!(n0, vec![2, 3]);
    let mut n3 = g.neighbors(3);
    n3.sort();
    assert_eq!(n3, vec![0, 1]);
}

#[test]
fn test_bipartite_graph_left_edges() {
    let edges = vec![(0, 0), (1, 1)];
    let g = BipartiteGraph::new(2, 2, edges.clone());
    assert_eq!(g.left_edges(), &edges);
}

#[test]
#[should_panic]
fn test_bipartite_graph_invalid_left_index() {
    BipartiteGraph::new(2, 2, vec![(2, 0)]);
}

#[test]
#[should_panic]
fn test_bipartite_graph_invalid_right_index() {
    BipartiteGraph::new(2, 2, vec![(0, 2)]);
}

#[test]
fn deserialize_checks_partition_endpoints_and_total_size() {
    for edges in [vec![(1, 0)], vec![(0, 1)]] {
        assert!(serde_json::from_value::<BipartiteGraph>(serde_json::json!({
            "left_size": 1, "right_size": 1, "edges": edges
        }))
        .is_err());
    }
    assert!(serde_json::from_value::<BipartiteGraph>(serde_json::json!({
        "left_size": usize::MAX, "right_size": 1, "edges": []
    }))
    .is_err());
    let graph: BipartiteGraph = serde_json::from_value(serde_json::json!({
        "left_size": 1, "right_size": 1, "edges": [[0, 0]]
    }))
    .unwrap();
    assert_eq!(graph.edges(), vec![(0, 1)]);
}
