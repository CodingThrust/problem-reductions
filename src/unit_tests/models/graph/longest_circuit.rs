use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Max;

fn issue_problem() -> LongestCircuit<SimpleGraph, i32> {
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
    )
}

#[test]
fn test_longest_circuit_creation() {
    let problem = issue_problem();
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 10);
    assert_eq!(problem.edge_lengths(), &[3, 2, 4, 1, 5, 2, 3, 2, 1, 2]);
    assert_eq!(problem.dims(), vec![2; 10]);
    assert!(problem.is_weighted());
}

#[test]
fn test_longest_circuit_evaluate_valid_and_invalid() {
    let problem = issue_problem();

    // Outer hexagon: 3+2+4+1+5+2 = 17
    assert_eq!(
        problem.evaluate(&[1, 1, 1, 1, 1, 1, 0, 0, 0, 0]),
        Max(Some(17))
    );
    // Not a valid circuit (only 3 edges, not forming a cycle)
    assert_eq!(problem.evaluate(&[1, 1, 1, 0, 0, 0, 0, 0, 0, 0]), Max(None));
    // Chord edges only — not a valid circuit
    assert_eq!(problem.evaluate(&[0, 0, 0, 0, 0, 0, 1, 1, 1, 0]), Max(None));
}

#[test]
fn test_longest_circuit_rejects_disconnected_cycles() {
    let problem = LongestCircuit::new(
        SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)]),
        vec![1, 1, 1, 1, 1, 1],
    );
    assert_eq!(problem.evaluate(&[1, 1, 1, 1, 1, 1]), Max(None));
}

#[test]
fn test_longest_circuit_rejects_non_binary() {
    let problem = issue_problem();
    assert!(!problem.is_valid_solution(&[1, 1, 1, 1, 1, 1, 0, 0, 0, 2]));
}

#[test]
fn test_longest_circuit_bruteforce() {
    let problem = issue_problem();
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem);
    assert!(witness.is_some());

    // The optimal circuit has value 18 (circuit 0-1-4-5-2-3-0)
    let value = solver.solve(&problem);
    assert_eq!(value, Max(Some(18)));
}

#[test]
fn test_longest_circuit_serialization() {
    let problem = issue_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: LongestCircuit<SimpleGraph, i32> = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_vertices(), problem.num_vertices());
    assert_eq!(restored.num_edges(), problem.num_edges());
    assert_eq!(restored.edge_lengths(), problem.edge_lengths());
}

#[test]
fn test_longest_circuit_paper_example() {
    let problem = issue_problem();
    // Optimal circuit: 0-1-4-5-2-3-0 with total length 18
    let config = vec![1, 0, 1, 0, 1, 0, 1, 1, 1, 0];
    assert_eq!(problem.evaluate(&config), Max(Some(18)));

    let all = BruteForce::new().find_all_witnesses(&problem);
    assert!(all.contains(&config));
}

#[test]
#[should_panic(expected = "All edge lengths must be positive (> 0)")]
fn test_longest_circuit_rejects_non_positive_edge_lengths() {
    LongestCircuit::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
        vec![1, 0, 1],
    );
}

#[test]
#[should_panic(expected = "All edge lengths must be positive (> 0)")]
fn test_longest_circuit_set_lengths_rejects_non_positive_values() {
    let mut problem = LongestCircuit::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
        vec![1, 1, 1],
    );
    problem.set_lengths(vec![1, -2, 1]);
}
