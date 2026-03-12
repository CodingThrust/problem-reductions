use super::*;

#[test]
fn test_directed_graph_creation() {
    let graph = DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    assert_eq!(graph.num_vertices(), 4);
    assert_eq!(graph.num_arcs(), 3);
}

#[test]
fn test_directed_graph_empty() {
    let graph = DirectedGraph::new(3, vec![]);
    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.num_arcs(), 0);
    assert!(!graph.is_empty());

    let empty = DirectedGraph::new(0, vec![]);
    assert!(empty.is_empty());
}

#[test]
fn test_directed_graph_has_arc() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2)]);
    assert!(graph.has_arc(0, 1));
    assert!(graph.has_arc(1, 2));
    // Direction matters
    assert!(!graph.has_arc(1, 0));
    assert!(!graph.has_arc(2, 1));
    assert!(!graph.has_arc(0, 2));
}

#[test]
fn test_directed_graph_arcs() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let mut arcs = graph.arcs();
    arcs.sort();
    assert_eq!(arcs, vec![(0, 1), (1, 2), (2, 0)]);
}

#[test]
fn test_directed_graph_successors() {
    let graph = DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 3)]);
    let mut succ = graph.successors(0);
    succ.sort();
    assert_eq!(succ, vec![1, 2]);
    assert_eq!(graph.successors(1), vec![3]);
    assert!(graph.successors(3).is_empty());
}

#[test]
fn test_directed_graph_predecessors() {
    let graph = DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 2)]);
    let mut preds = graph.predecessors(2);
    preds.sort();
    assert_eq!(preds, vec![0, 1]);
    assert_eq!(graph.predecessors(0), Vec::<usize>::new());
}

#[test]
fn test_directed_graph_degrees() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    assert_eq!(graph.out_degree(0), 2);
    assert_eq!(graph.out_degree(1), 1);
    assert_eq!(graph.out_degree(2), 0);
    assert_eq!(graph.in_degree(0), 0);
    assert_eq!(graph.in_degree(1), 1);
    assert_eq!(graph.in_degree(2), 2);
}

#[test]
fn test_directed_graph_is_acyclic_subgraph() {
    // Cycle: 0->1->2->0
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    // Keep all arcs -> has cycle
    assert!(!graph.is_acyclic_subgraph(&[true, true, true]));
    // Remove arc 2->0 -> acyclic
    assert!(graph.is_acyclic_subgraph(&[true, true, false]));
    // Remove arc 0->1 -> acyclic
    assert!(graph.is_acyclic_subgraph(&[false, true, true]));
    // Keep no arcs -> trivially acyclic
    assert!(graph.is_acyclic_subgraph(&[false, false, false]));
}

#[test]
fn test_directed_graph_serialization() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let json = serde_json::to_string(&graph).unwrap();
    let deserialized: DirectedGraph = serde_json::from_str(&json).unwrap();
    assert_eq!(graph, deserialized);
}

#[test]
fn test_directed_graph_equality() {
    let g1 = DirectedGraph::new(3, vec![(0, 1), (1, 2)]);
    let g2 = DirectedGraph::new(3, vec![(0, 1), (1, 2)]);
    let g3 = DirectedGraph::new(3, vec![(1, 0), (1, 2)]);
    assert_eq!(g1, g2);
    assert_ne!(g1, g3);
}

#[test]
#[should_panic(expected = "arc (3, 0) references vertex >= num_vertices (3)")]
fn test_directed_graph_invalid_arc() {
    DirectedGraph::new(3, vec![(3, 0)]);
}
