use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::MixedGraph;
use crate::traits::Problem;

fn yes_instance() -> MixedChinesePostman<i32> {
    MixedChinesePostman::new(
        MixedGraph::new(
            5,
            vec![(0, 1), (1, 2), (2, 3), (3, 0)],
            vec![(0, 2), (1, 3), (0, 4), (4, 2)],
        ),
        vec![2, 3, 1, 4],
        vec![2, 3, 1, 2],
        24,
    )
}

fn no_instance() -> MixedChinesePostman<i32> {
    MixedChinesePostman::new(
        MixedGraph::new(
            6,
            vec![(0, 1), (1, 0), (2, 3)],
            vec![(0, 2), (1, 3), (3, 4), (4, 5), (5, 2)],
        ),
        vec![1, 1, 1],
        vec![1, 1, 5, 5, 5],
        10,
    )
}

#[test]
fn test_mixed_chinese_postman_creation_and_accessors() {
    let problem = yes_instance();

    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_arcs(), 4);
    assert_eq!(problem.num_edges(), 4);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2]);
    assert_eq!(problem.arc_weights(), &[2, 3, 1, 4]);
    assert_eq!(problem.edge_weights(), &[2, 3, 1, 2]);
    assert_eq!(*problem.bound(), 24);
}

#[test]
fn test_mixed_chinese_postman_evaluate_yes_issue_example() {
    let problem = yes_instance();

    // Reverse (0,2) and (1,3), keep (0,4) and (4,2) forward.
    assert!(problem.evaluate(&[1, 1, 0, 0]));
}

#[test]
fn test_mixed_chinese_postman_evaluate_no_issue_example() {
    let problem = no_instance();

    assert!(!problem.evaluate(&[0, 0, 0, 0, 0]));
}

#[test]
fn test_mixed_chinese_postman_single_edge_walk() {
    // V={0,1}, A=∅, E={{0,1}}, weight=1, B=2.
    // Walk 0→1→0 is valid: traverses the edge at least once, total cost 2 ≤ 2.
    let problem =
        MixedChinesePostman::new(MixedGraph::new(2, vec![], vec![(0, 1)]), vec![], vec![1], 2);

    assert!(problem.evaluate(&[0]));
    assert!(problem.evaluate(&[1]));

    let solver = BruteForce::new();
    assert!(solver.find_satisfying(&problem).is_some());
}

#[test]
fn test_mixed_chinese_postman_rejects_disconnected_graph() {
    // Two disconnected components {0,1} and {2,3}: no closed walk can cover all edges.
    let problem = MixedChinesePostman::new(
        MixedGraph::new(4, vec![], vec![(0, 1), (2, 3)]),
        vec![],
        vec![1, 1],
        100,
    );

    assert!(!problem.evaluate(&[0, 0]));
    assert!(!problem.evaluate(&[0, 1]));
    assert!(!problem.evaluate(&[1, 0]));
    assert!(!problem.evaluate(&[1, 1]));
}

#[test]
fn test_mixed_chinese_postman_rejects_wrong_config_length() {
    let problem = yes_instance();

    assert!(!problem.evaluate(&[]));
    assert!(!problem.evaluate(&[1, 1, 0]));
    assert!(!problem.evaluate(&[1, 1, 0, 0, 1]));
}

#[test]
fn test_mixed_chinese_postman_solver_finds_satisfying_orientation() {
    let problem = yes_instance();
    let solver = BruteForce::new();

    let solution = solver
        .find_satisfying(&problem)
        .expect("expected a satisfying orientation");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_mixed_chinese_postman_solver_reports_unsat_issue_example() {
    let problem = no_instance();
    let solver = BruteForce::new();

    assert!(solver.find_satisfying(&problem).is_none());
}

#[test]
fn test_mixed_chinese_postman_serialization_roundtrip() {
    let problem = yes_instance();

    let json = serde_json::to_string(&problem).unwrap();
    let restored: MixedChinesePostman<i32> = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.num_vertices(), 5);
    assert_eq!(restored.num_arcs(), 4);
    assert_eq!(restored.num_edges(), 4);
    assert_eq!(restored.arc_weights(), &[2, 3, 1, 4]);
    assert_eq!(restored.edge_weights(), &[2, 3, 1, 2]);
    assert_eq!(*restored.bound(), 24);
}

#[test]
fn test_mixed_chinese_postman_problem_name() {
    assert_eq!(
        <MixedChinesePostman<i32> as Problem>::NAME,
        "MixedChinesePostman"
    );
}
