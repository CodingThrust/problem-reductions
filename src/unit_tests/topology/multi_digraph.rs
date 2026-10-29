use super::*;

#[test]
fn test_directed_graph_new() {
    let g = MultiDigraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    assert_eq!(g.num_vertices(), 4);
    assert_eq!(g.num_arcs(), 3);
}

#[test]
fn test_directed_graph_empty() {
    let g = MultiDigraph::empty(5);
    assert_eq!(g.num_vertices(), 5);
    assert_eq!(g.num_arcs(), 0);
    assert!(!g.is_empty());

    let empty = MultiDigraph::new(0, vec![]);
    assert!(empty.is_empty());
}

#[test]
fn test_directed_graph_arcs() {
    let g = MultiDigraph::new(3, vec![(0, 1), (2, 0)]);
    let mut arcs = g.arcs();
    arcs.sort();
    assert_eq!(arcs, vec![(0, 1), (2, 0)]);
}

#[test]
fn test_multi_digraph_arc_count() {
    let g = MultiDigraph::new(3, vec![(0, 1), (1, 2)]);
    assert_eq!(g.arc_count(0, 1), 1);
    assert_eq!(g.arc_count(1, 2), 1);
    assert_eq!(g.arc_count(1, 0), 0); // Directed: reverse not present
    assert_eq!(g.arc_count(0, 2), 0);
}

#[test]
fn test_directed_graph_successors() {
    // 0 → 1, 0 → 2, 1 → 2
    let g = MultiDigraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let mut succ0 = g.successors(0);
    succ0.sort();
    assert_eq!(succ0, vec![1, 2]);
    let mut succ1 = g.successors(1);
    succ1.sort();
    assert_eq!(succ1, vec![2]);
    assert_eq!(g.successors(2), Vec::<usize>::new());
}

#[test]
fn test_directed_graph_predecessors() {
    // 0 → 1, 0 → 2, 1 → 2
    let g = MultiDigraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    assert_eq!(g.predecessors(0), Vec::<usize>::new());
    let mut pred2 = g.predecessors(2);
    pred2.sort();
    assert_eq!(pred2, vec![0, 1]);
    assert_eq!(g.predecessors(1), vec![0]);
}

#[test]
fn test_directed_graph_degrees() {
    let graph = MultiDigraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    assert_eq!(graph.out_degree(0), 2);
    assert_eq!(graph.out_degree(1), 1);
    assert_eq!(graph.out_degree(2), 0);
    assert_eq!(graph.in_degree(0), 0);
    assert_eq!(graph.in_degree(1), 1);
    assert_eq!(graph.in_degree(2), 2);
}

#[test]
fn test_directed_graph_is_dag_true() {
    // Simple path: 0 → 1 → 2
    let g = MultiDigraph::new(3, vec![(0, 1), (1, 2)]);
    assert!(g.is_dag());
}

#[test]
fn test_directed_graph_is_dag_false() {
    // Cycle: 0 → 1 → 2 → 0
    let g = MultiDigraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    assert!(!g.is_dag());
}

#[test]
fn test_directed_graph_is_dag_empty() {
    let g = MultiDigraph::empty(4);
    assert!(g.is_dag());
}

#[test]
fn test_directed_graph_is_dag_self_loop() {
    // Self-loop is a cycle
    let g = MultiDigraph::new(2, vec![(0, 0)]);
    assert!(!g.is_dag());
}

#[test]
fn test_is_strongly_connected_cycle() {
    let g = MultiDigraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    assert!(g.is_strongly_connected());
}

#[test]
fn test_is_strongly_connected_path() {
    let g = MultiDigraph::new(3, vec![(0, 1), (1, 2)]);
    assert!(!g.is_strongly_connected());
}

#[test]
fn test_is_strongly_connected_single_vertex() {
    let g = MultiDigraph::new(1, vec![]);
    assert!(g.is_strongly_connected());
}

#[test]
fn test_is_strongly_connected_empty() {
    let g = MultiDigraph::empty(0);
    assert!(g.is_strongly_connected());
}

#[test]
fn test_directed_graph_is_acyclic_subgraph() {
    // Cycle: 0->1->2->0
    let graph = MultiDigraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
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
fn test_directed_graph_induced_subgraph_basic() {
    // 0 → 1 → 2 → 0 (cycle), keep vertices 0 and 1 (drop 2)
    let g = MultiDigraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let subg = g.induced_subgraph(&[true, true, false]);
    // After dropping vertex 2: vertices 0 and 1 remain, arc (0→1) remains
    // Vertex remapping: 0→0, 1→1
    assert_eq!(subg.num_vertices(), 2);
    assert_eq!(subg.num_arcs(), 1);
    assert_eq!(subg.arc_count(0, 1), 1);
    // Cycle is broken
    assert!(subg.is_dag());
}

#[test]
fn test_directed_graph_induced_subgraph_remapping() {
    // Vertices 0, 1, 2, 3; keep 1 and 3 only
    // Arcs: 1 → 3
    let g = MultiDigraph::new(4, vec![(0, 1), (1, 3), (2, 0)]);
    let subg = g.induced_subgraph(&[false, true, false, true]);
    // Vertex 1 → new index 0, vertex 3 → new index 1
    assert_eq!(subg.num_vertices(), 2);
    assert_eq!(subg.num_arcs(), 1);
    assert_eq!(subg.arc_count(0, 1), 1); // was 1 → 3
}

#[test]
fn test_directed_graph_induced_subgraph_no_cross_arcs() {
    // Keep a subset that has no arcs between kept vertices
    let g = MultiDigraph::new(3, vec![(0, 2), (1, 2)]);
    // Keep 0 and 1 only — neither arc (0→2) nor (1→2) is kept (2 dropped)
    let subg = g.induced_subgraph(&[true, true, false]);
    assert_eq!(subg.num_vertices(), 2);
    assert_eq!(subg.num_arcs(), 0);
}

#[test]
fn test_directed_graph_eq_same_order() {
    let g1 = MultiDigraph::new(3, vec![(0, 1), (1, 2)]);
    let g2 = MultiDigraph::new(3, vec![(0, 1), (1, 2)]);
    assert_eq!(g1, g2);
}

#[test]
fn test_directed_graph_eq_different_arc_order() {
    // Same arcs, provided in different order
    let g1 = MultiDigraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let g2 = MultiDigraph::new(3, vec![(2, 0), (0, 1), (1, 2)]);
    assert_eq!(g1, g2);
}

#[test]
fn test_directed_graph_ne_different_arcs() {
    let g1 = MultiDigraph::new(3, vec![(0, 1)]);
    let g2 = MultiDigraph::new(3, vec![(1, 0)]); // Reversed direction
    assert_ne!(g1, g2);
}

#[test]
fn test_directed_graph_ne_different_vertices() {
    let g1 = MultiDigraph::new(3, vec![(0, 1)]);
    let g2 = MultiDigraph::new(4, vec![(0, 1)]);
    assert_ne!(g1, g2);
}

#[test]
fn test_directed_graph_serialization() {
    let g = MultiDigraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]);
    let json = serde_json::to_string(&g).expect("serialization failed");
    let restored: MultiDigraph = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(g, restored);
}

#[test]
fn test_directed_graph_json_roundtrip() {
    let g = MultiDigraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let json = serde_json::to_value(&g).unwrap();
    assert_eq!(json["num_vertices"], 4);
    let arcs: Vec<(usize, usize)> = serde_json::from_value(json["arcs"].clone()).unwrap();
    assert_eq!(arcs.len(), 3);
    let roundtrip: MultiDigraph = serde_json::from_value(json).unwrap();
    assert_eq!(g, roundtrip);
}

#[test]
fn test_directed_graph_json_format() {
    let g = MultiDigraph::new(3, vec![(0, 1), (1, 2)]);
    let json_str = serde_json::to_string(&g).unwrap();
    assert!(!json_str.contains("edge_property"));
    assert!(!json_str.contains("node_holes"));
    assert!(json_str.contains("num_vertices"));
    assert!(json_str.contains("arcs"));
}

#[test]
#[should_panic(expected = "arc (0, 5) references vertex >= num_vertices")]
fn test_directed_graph_invalid_arc() {
    MultiDigraph::new(3, vec![(0, 5)]);
}

#[test]
fn deserialize_rejects_out_of_range_arcs() {
    assert!(serde_json::from_value::<MultiDigraph>(serde_json::json!({
        "num_vertices": 2, "arcs": [[2, 0]]
    }))
    .is_err());
}

#[test]
fn multi_digraph_preserves_arc_multiplicity_and_order() {
    let arcs = vec![(0, 0), (0, 1), (0, 1), (1, 0)];
    let graph = crate::topology::MultiDigraph::new(2, arcs.clone());
    assert_eq!(graph.arcs(), arcs);
    assert_eq!(graph.num_arcs(), 4);
    assert_eq!(graph.arc_count(0, 1), 2);
    assert_eq!(graph.out_degree(0), 3);
    let restored: crate::topology::MultiDigraph =
        serde_json::from_value(serde_json::to_value(&graph).unwrap()).unwrap();
    assert_eq!(restored.arcs(), arcs);
}
