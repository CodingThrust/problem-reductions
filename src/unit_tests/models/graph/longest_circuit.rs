use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn issue_problem_with_bound(bound: i32) -> LongestCircuit<SimpleGraph, i32> {
    LongestCircuit::new(
        SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 5),
                (5, 0),
                (0, 3),
                (1, 4),
                (2, 5),
                (3, 5),
            ],
        ),
        vec![3, 2, 4, 1, 5, 2, 3, 2, 1, 2],
        bound,
    )
}

fn issue_problem() -> LongestCircuit<SimpleGraph, i32> {
    issue_problem_with_bound(17)
}

#[test]
fn test_longest_circuit_creation() {
    let problem = issue_problem();
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 10);
    assert_eq!(problem.edge_lengths(), &[3, 2, 4, 1, 5, 2, 3, 2, 1, 2]);
    assert_eq!(problem.bound(), &17);
    assert_eq!(problem.dims(), vec![2; 10]);
    assert!(problem.is_weighted());
}

#[test]
fn test_longest_circuit_evaluate_valid_and_invalid() {
    let problem = issue_problem();

    assert!(problem.evaluate(&[1, 1, 1, 1, 1, 1, 0, 0, 0, 0]));
    assert!(!problem.evaluate(&[1, 1, 1, 0, 0, 0, 0, 0, 0, 0]));
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0, 0, 1, 1, 1, 0]));
}

#[test]
fn test_longest_circuit_rejects_disconnected_cycles() {
    let problem = LongestCircuit::new(
        SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)]),
        vec![1, 1, 1, 1, 1, 1],
        3,
    );
    assert!(!problem.evaluate(&[1, 1, 1, 1, 1, 1]));
}

#[test]
fn test_longest_circuit_rejects_non_binary_and_below_bound_configs() {
    let problem = issue_problem();
    assert!(!problem.is_valid_solution(&[1, 1, 1, 1, 1, 1, 0, 0, 0, 2]));

    let tighter_problem = issue_problem_with_bound(18);
    assert!(!tighter_problem.evaluate(&[1, 1, 1, 1, 1, 1, 0, 0, 0, 0]));
}

#[test]
fn test_longest_circuit_bruteforce_yes_and_no() {
    let yes_problem = issue_problem();
    let solver = BruteForce::new();
    assert!(solver.find_witness(&yes_problem).is_some());

    let no_problem = LongestCircuit::new(
        SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 5),
                (5, 0),
                (0, 3),
                (1, 4),
                (2, 5),
                (3, 5),
            ],
        ),
        vec![3, 2, 4, 1, 5, 2, 3, 2, 1, 2],
        19,
    );
    assert!(solver.find_witness(&no_problem).is_none());
}

#[test]
fn test_longest_circuit_serialization() {
    let problem = issue_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: LongestCircuit<SimpleGraph, i32> = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_vertices(), problem.num_vertices());
    assert_eq!(restored.num_edges(), problem.num_edges());
    assert_eq!(restored.edge_lengths(), problem.edge_lengths());
    assert_eq!(restored.bound(), problem.bound());
}

#[test]
fn test_longest_circuit_paper_example() {
    let problem = issue_problem();
    let config = vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0];
    assert!(problem.evaluate(&config));

    let all = BruteForce::new().find_all_witnesses(&problem);
    assert!(all.contains(&config));
}

#[test]
#[should_panic(expected = "All edge lengths must be positive (> 0)")]
fn test_longest_circuit_rejects_non_positive_edge_lengths() {
    LongestCircuit::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
        vec![1, 0, 1],
        3,
    );
}

#[test]
#[should_panic(expected = "bound must be positive (> 0)")]
fn test_longest_circuit_rejects_non_positive_bound() {
    LongestCircuit::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
        vec![1, 1, 1],
        0,
    );
}

#[test]
#[should_panic(expected = "All edge lengths must be positive (> 0)")]
fn test_longest_circuit_set_lengths_rejects_non_positive_values() {
    let mut problem = LongestCircuit::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
        vec![1, 1, 1],
        3,
    );
    problem.set_lengths(vec![1, -2, 1]);
}
