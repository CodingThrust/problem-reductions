use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::BipartiteGraph;
use crate::traits::Problem;

fn issue_instance_1_graph() -> BipartiteGraph {
    BipartiteGraph::new(
        4,
        4,
        vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 3),
            (2, 0),
            (2, 2),
            (2, 3),
            (3, 1),
        ],
    )
}

fn issue_instance_2_graph() -> BipartiteGraph {
    BipartiteGraph::new(
        4,
        4,
        vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
            (3, 0),
            (3, 1),
            (3, 3),
        ],
    )
}

fn issue_instance_2_witness() -> Vec<usize> {
    vec![1, 1, 1, 0, 1, 1, 1, 0]
}

#[test]
fn test_balanced_complete_bipartite_subgraph_creation() {
    let problem = BalancedCompleteBipartiteSubgraph::new(issue_instance_1_graph(), 2);

    assert_eq!(problem.left_size(), 4);
    assert_eq!(problem.right_size(), 4);
    assert_eq!(problem.num_vertices(), 8);
    assert_eq!(problem.num_edges(), 10);
    assert_eq!(problem.k(), 2);
    assert_eq!(problem.dims(), vec![2; 8]);
}

#[test]
fn test_balanced_complete_bipartite_subgraph_evaluation_yes_instance() {
    let problem = BalancedCompleteBipartiteSubgraph::new(issue_instance_1_graph(), 2);

    assert!(problem.evaluate(&[1, 1, 0, 0, 1, 1, 0, 0]));
}

#[test]
fn test_balanced_complete_bipartite_subgraph_evaluation_no_instance() {
    let problem = BalancedCompleteBipartiteSubgraph::new(issue_instance_1_graph(), 3);

    assert!(!problem.evaluate(&[1, 1, 1, 0, 1, 1, 1, 0]));
}

#[test]
fn test_balanced_complete_bipartite_subgraph_invalid_pairing() {
    let problem = BalancedCompleteBipartiteSubgraph::new(issue_instance_1_graph(), 2);

    assert!(!problem.evaluate(&[1, 1, 0, 0, 1, 0, 1, 0]));
}

#[test]
fn test_balanced_complete_bipartite_subgraph_edge_lookup() {
    let problem = BalancedCompleteBipartiteSubgraph::new(issue_instance_1_graph(), 2);

    assert!(problem.has_selected_edge(0, 0));
    assert!(problem.has_selected_edge(1, 3));
    assert!(!problem.has_selected_edge(3, 3));
}

#[test]
fn test_balanced_complete_bipartite_subgraph_rejects_invalid_configs() {
    let problem = BalancedCompleteBipartiteSubgraph::new(issue_instance_1_graph(), 2);

    assert!(!problem.evaluate(&[1, 1, 0, 0, 1, 1, 0]));
    assert!(!problem.evaluate(&[1, 2, 0, 0, 1, 1, 0, 0]));
}

#[test]
fn test_balanced_complete_bipartite_subgraph_solver_yes_instance() {
    let problem = BalancedCompleteBipartiteSubgraph::new(issue_instance_2_graph(), 3);
    let solver = BruteForce::new();

    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));

    let all = solver.find_all_satisfying(&problem);
    assert_eq!(all, vec![issue_instance_2_witness()]);
}

#[test]
fn test_balanced_complete_bipartite_subgraph_solver_no_instance() {
    let problem = BalancedCompleteBipartiteSubgraph::new(issue_instance_1_graph(), 3);
    let solver = BruteForce::new();

    assert!(solver.find_satisfying(&problem).is_none());
}

#[test]
fn test_balanced_complete_bipartite_subgraph_serialization() {
    let problem = BalancedCompleteBipartiteSubgraph::new(issue_instance_2_graph(), 3);
    let witness = issue_instance_2_witness();

    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: BalancedCompleteBipartiteSubgraph = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.left_size(), 4);
    assert_eq!(deserialized.right_size(), 4);
    assert_eq!(deserialized.num_edges(), 12);
    assert_eq!(
        deserialized.graph().left_edges(),
        problem.graph().left_edges()
    );
    assert_eq!(deserialized.k(), 3);
    assert!(deserialized.evaluate(&witness));
}

#[test]
fn test_balanced_complete_bipartite_subgraph_is_valid_solution() {
    let problem = BalancedCompleteBipartiteSubgraph::new(issue_instance_2_graph(), 3);
    let yes_config = issue_instance_2_witness();
    let no_config = vec![1, 1, 0, 1, 1, 1, 0, 0];

    assert!(problem.is_valid_solution(&yes_config));
    assert!(!problem.is_valid_solution(&no_config));
    assert!(!problem.is_valid_solution(&[1, 1, 1]));
}

#[test]
fn test_balanced_complete_bipartite_subgraph_paper_example() {
    let problem = BalancedCompleteBipartiteSubgraph::new(issue_instance_2_graph(), 3);
    let witness = issue_instance_2_witness();
    let solver = BruteForce::new();

    assert!(problem.evaluate(&witness));
    assert_eq!(solver.find_all_satisfying(&problem), vec![witness]);
}
