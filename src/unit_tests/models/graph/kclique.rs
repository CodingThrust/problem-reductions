use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn issue_graph() -> SimpleGraph {
    SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)])
}

fn issue_witness() -> Vec<usize> {
    vec![0, 0, 1, 1, 1]
}

#[test]
fn test_kclique_creation() {
    let problem = KClique::new(issue_graph(), 3);

    assert_eq!(problem.graph().num_vertices(), 5);
    assert_eq!(problem.graph().num_edges(), 6);
    assert_eq!(problem.k(), 3);
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 6);
    assert_eq!(problem.dims(), vec![2; 5]);
}

#[test]
fn test_kclique_evaluate_yes_instance() {
    let problem = KClique::new(issue_graph(), 3);

    assert!(problem.evaluate(&issue_witness()));
    assert!(problem.is_valid_solution(&issue_witness()));
}

#[test]
fn test_kclique_evaluate_rejects_non_clique() {
    let problem = KClique::new(issue_graph(), 3);

    assert!(!problem.evaluate(&[1, 0, 1, 1, 0]));
    assert!(!problem.is_valid_solution(&[1, 0, 1, 1, 0]));
}

#[test]
fn test_kclique_evaluate_rejects_too_small_clique() {
    let problem = KClique::new(issue_graph(), 3);

    assert!(!problem.evaluate(&[1, 0, 1, 0, 0]));
    assert!(!problem.evaluate(&[0, 0, 1, 1, 0]));
}

#[test]
fn test_kclique_solver_finds_unique_witness() {
    let problem = KClique::new(issue_graph(), 3);
    let solver = BruteForce::new();

    assert_eq!(solver.find_satisfying(&problem), Some(issue_witness()));
    assert_eq!(solver.find_all_satisfying(&problem), vec![issue_witness()]);
}

#[test]
fn test_kclique_serialization_round_trip() {
    let problem = KClique::new(issue_graph(), 3);
    let json = serde_json::to_string(&problem).unwrap();
    let restored: KClique<SimpleGraph> = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.graph().edges(), problem.graph().edges());
    assert_eq!(restored.k(), 3);
    assert!(restored.evaluate(&issue_witness()));
}

#[test]
fn test_kclique_paper_example() {
    let problem = KClique::new(issue_graph(), 3);
    let solver = BruteForce::new();

    assert!(problem.evaluate(&issue_witness()));
    assert_eq!(solver.find_all_satisfying(&problem), vec![issue_witness()]);
}
