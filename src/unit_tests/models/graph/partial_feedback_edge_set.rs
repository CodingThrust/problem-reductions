use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;

fn issue_graph() -> SimpleGraph {
    SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (1, 2),
            (2, 0),
            (2, 3),
            (3, 4),
            (4, 2),
            (3, 5),
            (5, 4),
            (0, 3),
        ],
    )
}

fn yes_instance() -> PartialFeedbackEdgeSet<SimpleGraph> {
    PartialFeedbackEdgeSet::new(issue_graph(), 3, 4)
}

fn no_instance() -> PartialFeedbackEdgeSet<SimpleGraph> {
    PartialFeedbackEdgeSet::new(issue_graph(), 2, 4)
}

fn normalize_edge(u: usize, v: usize) -> (usize, usize) {
    if u <= v {
        (u, v)
    } else {
        (v, u)
    }
}

fn select_edges<G: Graph>(graph: &G, selected_edges: &[(usize, usize)]) -> Vec<usize> {
    let chosen: std::collections::BTreeSet<_> = selected_edges
        .iter()
        .copied()
        .map(|(u, v)| normalize_edge(u, v))
        .collect();
    graph
        .edges()
        .into_iter()
        .map(|(u, v)| usize::from(chosen.contains(&normalize_edge(u, v))))
        .collect()
}

#[test]
fn test_partial_feedback_edge_set_creation() {
    let problem = yes_instance();
    assert_eq!(problem.graph().num_vertices(), 6);
    assert_eq!(problem.graph().num_edges(), 9);
    assert_eq!(problem.budget(), 3);
    assert_eq!(problem.max_cycle_length(), 4);
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 9);
    assert_eq!(problem.num_variables(), 9);
    assert_eq!(problem.dims(), vec![2; 9]);
}

#[test]
fn test_partial_feedback_edge_set_accepts_correct_issue_solution() {
    let problem = yes_instance();
    let config = select_edges(problem.graph(), &[(0, 2), (2, 3), (3, 4)]);
    assert!(problem.evaluate(&config));
    assert!(problem.is_valid_solution(&config));
}

#[test]
fn test_partial_feedback_edge_set_rejects_under_budget_instance() {
    let problem = no_instance();
    let config = select_edges(problem.graph(), &[(0, 2), (2, 3), (3, 4)]);
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_partial_feedback_edge_set_rejects_missing_cycle_hit() {
    let problem = yes_instance();
    let config = select_edges(problem.graph(), &[(0, 2), (3, 4), (3, 5)]);
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_partial_feedback_edge_set_rejects_wrong_length_config() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[0, 1, 0]));
}

#[test]
fn test_partial_feedback_edge_set_rejects_non_binary_entries() {
    let problem = yes_instance();
    let mut config = select_edges(problem.graph(), &[(0, 2), (2, 3), (3, 4)]);
    config[0] = 2;
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_partial_feedback_edge_set_solver_yes_and_no_instances() {
    let solver = BruteForce::new();

    let yes_problem = yes_instance();
    let solution = solver.find_satisfying(&yes_problem).unwrap();
    assert!(yes_problem.evaluate(&solution));

    let no_problem = no_instance();
    assert!(solver.find_satisfying(&no_problem).is_none());
}

#[test]
fn test_partial_feedback_edge_set_paper_example() {
    let problem = yes_instance();
    let config = select_edges(problem.graph(), &[(0, 2), (2, 3), (3, 4)]);
    assert!(problem.evaluate(&config));

    let satisfying = BruteForce::new().find_all_satisfying(&problem);
    assert_eq!(satisfying.len(), 5);
    assert!(satisfying.iter().any(|candidate| candidate == &config));
}

#[test]
fn test_partial_feedback_edge_set_serialization() {
    let problem = yes_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let round_trip: PartialFeedbackEdgeSet<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(round_trip.num_vertices(), 6);
    assert_eq!(round_trip.num_edges(), 9);
    assert_eq!(round_trip.budget(), 3);
    assert_eq!(round_trip.max_cycle_length(), 4);
}
