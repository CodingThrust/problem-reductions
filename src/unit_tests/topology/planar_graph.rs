use crate::topology::{Graph, PlanarGraph};

#[test]
fn test_planar_graph_basic() {
    // K4 is planar: 4 vertices, 6 edges, 6 <= 3*4 - 6 = 6
    let edges = vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    let g = PlanarGraph::new(4, edges).unwrap();
    assert_eq!(g.num_vertices(), 4);
    assert_eq!(g.num_edges(), 6);
}

#[test]
fn test_planar_graph_delegates_to_inner() {
    let g = PlanarGraph::new(3, vec![(0, 1), (1, 2)]).unwrap();
    assert!(g.has_edge(0, 1));
    assert!(!g.has_edge(0, 2));
    let mut n1 = g.neighbors(1);
    n1.sort();
    assert_eq!(n1, vec![0, 2]);
}

#[test]
fn test_planar_graph_rejects_k5() {
    // K5 has 10 edges, but 3*5 - 6 = 9. Fails necessary condition.
    let mut edges = Vec::new();
    for i in 0..5 {
        for j in (i + 1)..5 {
            edges.push((i, j));
        }
    }
    assert!(PlanarGraph::new(5, edges).is_err());
}

#[test]
fn test_planar_graph_empty() {
    let g = PlanarGraph::new(3, vec![]).unwrap();
    assert_eq!(g.num_vertices(), 3);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_planar_graph_tree() {
    let g = PlanarGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]).unwrap();
    assert_eq!(g.num_edges(), 3);
}

#[test]
fn deserialize_checks_the_edge_bound() {
    let edges: Vec<_> = (0..5)
        .flat_map(|u| ((u + 1)..5).map(move |v| (u, v)))
        .collect();
    assert!(serde_json::from_value::<PlanarGraph>(serde_json::json!({
        "inner": {"num_vertices": 5, "edges": edges}
    }))
    .is_err());
    let graph: PlanarGraph = serde_json::from_value(serde_json::json!({
        "inner": {"num_vertices": 2, "edges": [[0, 1]]}
    }))
    .unwrap();
    assert_eq!(graph.num_edges(), 1);
}
