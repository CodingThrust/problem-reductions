use super::*;

#[test]
fn test_hypergraph_basic() {
    let hg = HyperGraph::new(4, vec![vec![0, 1, 2], vec![2, 3]]);
    assert_eq!(hg.num_vertices(), 4);
    assert_eq!(hg.num_edges(), 2);
}

#[test]
fn test_hypergraph_empty() {
    let hg = HyperGraph::empty(5);
    assert_eq!(hg.num_vertices(), 5);
    assert_eq!(hg.num_edges(), 0);
}

#[test]
fn test_hypergraph_neighbors() {
    let hg = HyperGraph::new(4, vec![vec![0, 1, 2], vec![2, 3]]);
    let neighbors = hg.neighbors(2);
    assert!(neighbors.contains(&0));
    assert!(neighbors.contains(&1));
    assert!(neighbors.contains(&3));
    assert!(!neighbors.contains(&2)); // Not its own neighbor
}

#[test]
fn test_hypergraph_has_edge() {
    let hg = HyperGraph::new(4, vec![vec![0, 1, 2]]);
    assert!(hg.has_edge(&[0, 1, 2]));
    assert!(hg.has_edge(&[2, 1, 0])); // Order doesn't matter
    assert!(!hg.has_edge(&[0, 1]));
    assert!(!hg.has_edge(&[0, 1, 3]));
}

#[test]
fn test_hypergraph_degree() {
    let hg = HyperGraph::new(4, vec![vec![0, 1, 2], vec![2, 3]]);
    assert_eq!(hg.degree(0), 1);
    assert_eq!(hg.degree(2), 2);
    assert_eq!(hg.degree(3), 1);
}

#[test]
fn test_hypergraph_edges_containing() {
    let hg = HyperGraph::new(4, vec![vec![0, 1, 2], vec![2, 3]]);
    let edges = hg.edges_containing(2);
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_hypergraph_add_edge() {
    let mut hg = HyperGraph::empty(4);
    hg.add_edge(vec![0, 1]);
    hg.add_edge(vec![1, 2, 3]);
    assert_eq!(hg.num_edges(), 2);
}

#[test]
fn test_hypergraph_max_edge_size() {
    let hg = HyperGraph::new(4, vec![vec![0, 1], vec![0, 1, 2, 3]]);
    assert_eq!(hg.max_edge_size(), 4);
}

#[test]
fn test_hypergraph_is_regular_graph() {
    let regular = HyperGraph::new(3, vec![vec![0, 1], vec![1, 2]]);
    assert!(regular.is_regular_graph());

    let not_regular = HyperGraph::new(4, vec![vec![0, 1, 2]]);
    assert!(!not_regular.is_regular_graph());
}

#[test]
fn test_hypergraph_to_graph_edges() {
    let hg = HyperGraph::new(3, vec![vec![0, 1], vec![1, 2]]);
    let edges = hg.to_graph_edges();
    assert!(edges.is_some());
    let edges = edges.unwrap();
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_hypergraph_to_graph_edges_not_regular() {
    // Hypergraph with a hyperedge of size 3 (not a regular graph)
    let hg = HyperGraph::new(4, vec![vec![0, 1, 2]]);
    assert!(hg.to_graph_edges().is_none());
}

#[test]
fn test_hypergraph_get_edge() {
    let hg = HyperGraph::new(4, vec![vec![0, 1, 2], vec![2, 3]]);
    assert_eq!(hg.edge(0), Some(&vec![0, 1, 2]));
    assert_eq!(hg.edge(1), Some(&vec![2, 3]));
    assert_eq!(hg.edge(2), None);
}

#[test]
#[should_panic(expected = "vertex index 5 out of bounds")]
fn test_hypergraph_invalid_vertex() {
    HyperGraph::new(4, vec![vec![0, 5]]);
}

#[test]
#[should_panic(expected = "vertex index 4 out of bounds")]
fn test_hypergraph_add_invalid_edge() {
    let mut hg = HyperGraph::empty(4);
    hg.add_edge(vec![0, 4]);
}
