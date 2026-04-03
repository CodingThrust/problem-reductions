use crate::models::graph::DegreeConstrainedSpanningTree;
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;

fn edge_config(graph: &SimpleGraph, selected_edges: &[(usize, usize)]) -> Vec<usize> {
    graph
        .edges()
        .into_iter()
        .map(|(u, v)| {
            usize::from(
                selected_edges
                    .iter()
                    .any(|&(a, b)| (a == u && b == v) || (a == v && b == u)),
            )
        })
        .collect()
}

#[test]
fn test_degree_constrained_spanning_tree_creation() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 2)]);
    let problem = DegreeConstrainedSpanningTree::new(graph.clone(), 2);

    assert_eq!(problem.graph(), &graph);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 4);
    assert_eq!(problem.max_degree(), 2);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2]);
    assert_eq!(
        <DegreeConstrainedSpanningTree<SimpleGraph> as Problem>::NAME,
        "DegreeConstrainedSpanningTree"
    );
}

#[test]
fn test_degree_constrained_spanning_tree_evaluate_accepts_path_tree() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 2)]);
    let problem = DegreeConstrainedSpanningTree::new(graph.clone(), 2);
    let config = edge_config(&graph, &[(0, 1), (1, 2), (2, 3)]);

    assert!(problem.evaluate(&config));
    assert!(problem.is_valid_solution(&config));
}

#[test]
fn test_degree_constrained_spanning_tree_evaluate_rejects_wrong_edge_count() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 2)]);
    let problem = DegreeConstrainedSpanningTree::new(graph.clone(), 2);
    let config = edge_config(&graph, &[(0, 1), (2, 3)]);

    assert!(!problem.evaluate(&config));
}

#[test]
fn test_degree_constrained_spanning_tree_evaluate_rejects_degree_bound_violation() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]);
    let problem = DegreeConstrainedSpanningTree::new(graph.clone(), 2);
    let config = edge_config(&graph, &[(0, 1), (0, 2), (0, 3)]);

    assert!(!problem.evaluate(&config));
}

#[test]
fn test_degree_constrained_spanning_tree_solver_and_serialization() {
    let problem = DegreeConstrainedSpanningTree::new(SimpleGraph::path(4), 2);
    let solver = BruteForce::new();

    let witness = solver.find_witness(&problem).expect("expected a witness");
    assert!(problem.evaluate(&witness));

    let json = serde_json::to_value(&problem).unwrap();
    let restored: DegreeConstrainedSpanningTree<SimpleGraph> =
        serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_vertices(), 4);
    assert_eq!(restored.num_edges(), 3);
    assert_eq!(restored.max_degree(), 2);
    assert!(restored.evaluate(&witness));
}
