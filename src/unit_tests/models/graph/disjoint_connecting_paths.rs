use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn issue_yes_problem() -> DisjointConnectingPaths<SimpleGraph> {
    DisjointConnectingPaths::new(
        SimpleGraph::new(
            6,
            vec![(0, 1), (1, 3), (0, 2), (1, 4), (2, 4), (3, 5), (4, 5)],
        ),
        vec![(0, 3), (2, 5)],
    )
}

fn issue_yes_config() -> Vec<usize> {
    vec![1, 0, 1, 0, 1, 0, 1]
}

fn issue_no_problem() -> DisjointConnectingPaths<SimpleGraph> {
    DisjointConnectingPaths::new(
        SimpleGraph::new(6, vec![(0, 2), (1, 2), (2, 3), (3, 4), (3, 5)]),
        vec![(0, 4), (1, 5)],
    )
}

#[test]
fn test_disjoint_connecting_paths_creation() {
    let problem = issue_yes_problem();
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 7);
    assert_eq!(problem.num_pairs(), 2);
    assert_eq!(problem.terminal_pairs(), &[(0, 3), (2, 5)]);
    assert_eq!(problem.dims(), vec![2; 7]);
    assert_eq!(
        problem.ordered_edges(),
        vec![(0, 1), (0, 2), (1, 3), (1, 4), (2, 4), (3, 5), (4, 5)]
    );
}

#[test]
#[should_panic(expected = "terminal_pairs must contain at least one pair")]
fn test_disjoint_connecting_paths_rejects_empty_pairs() {
    let _ = DisjointConnectingPaths::new(SimpleGraph::new(2, vec![(0, 1)]), vec![]);
}

#[test]
#[should_panic(expected = "terminal vertices must be pairwise disjoint across pairs")]
fn test_disjoint_connecting_paths_rejects_overlapping_terminals() {
    let _ = DisjointConnectingPaths::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![(0, 2), (2, 3)],
    );
}

#[test]
fn test_disjoint_connecting_paths_yes_instance() {
    let problem = issue_yes_problem();
    assert!(problem.evaluate(&issue_yes_config()));
}

#[test]
fn test_disjoint_connecting_paths_no_instance() {
    let problem = issue_no_problem();
    let solver = BruteForce::new();
    assert!(solver.find_satisfying(&problem).is_none());
}

#[test]
fn test_disjoint_connecting_paths_rejects_wrong_length_config() {
    let problem = issue_yes_problem();
    assert!(!problem.evaluate(&[1, 0, 1]));
}

#[test]
fn test_disjoint_connecting_paths_rejects_non_binary_entries() {
    let problem = issue_yes_problem();
    let mut config = issue_yes_config();
    config[3] = 2;
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_disjoint_connecting_paths_rejects_branching_subgraph() {
    let problem = issue_yes_problem();
    assert!(!problem.evaluate(&[1, 0, 1, 1, 1, 0, 1]));
}

#[test]
fn test_disjoint_connecting_paths_serialization() {
    let problem = issue_yes_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let round_trip: DisjointConnectingPaths<SimpleGraph> = serde_json::from_value(json).unwrap();
    assert_eq!(round_trip.num_vertices(), 6);
    assert_eq!(round_trip.num_edges(), 7);
    assert_eq!(round_trip.terminal_pairs(), &[(0, 3), (2, 5)]);
}

#[test]
fn test_disjoint_connecting_paths_paper_example() {
    let problem = issue_yes_problem();
    let config = issue_yes_config();
    assert!(problem.evaluate(&config));

    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));
}
