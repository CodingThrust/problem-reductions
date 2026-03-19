use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn issue_example() -> GeneralizedHex<SimpleGraph> {
    GeneralizedHex::new(
        SimpleGraph::new(
            8,
            vec![
                (0, 1),
                (0, 2),
                (1, 3),
                (1, 4),
                (2, 3),
                (2, 5),
                (3, 6),
                (4, 6),
                (5, 6),
                (6, 7),
            ],
        ),
        0,
        7,
    )
}

fn winning_example() -> GeneralizedHex<SimpleGraph> {
    GeneralizedHex::new(
        SimpleGraph::new(
            6,
            vec![(0, 1), (0, 2), (0, 3), (1, 4), (2, 4), (3, 4), (4, 5)],
        ),
        0,
        5,
    )
}

#[test]
fn test_generalized_hex_creation_and_getters() {
    let problem = winning_example();
    assert_eq!(problem.source(), 0);
    assert_eq!(problem.target(), 5);
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 7);
    assert_eq!(problem.num_playable_vertices(), 4);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert_eq!(problem.graph().num_vertices(), 6);
}

#[test]
fn test_generalized_hex_forced_win_on_bottleneck_example() {
    let problem = winning_example();
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_generalized_hex_detects_losing_position() {
    let problem = GeneralizedHex::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]), 0, 3);
    assert!(!problem.evaluate(&[]));
}

#[test]
fn test_generalized_hex_solver_returns_empty_config_for_win() {
    let problem = winning_example();
    let solver = BruteForce::new();
    assert_eq!(solver.find_satisfying(&problem), Some(vec![]));
    assert_eq!(
        solver.find_all_satisfying(&problem),
        Vec::<Vec<usize>>::from([vec![]])
    );
}

#[test]
fn test_generalized_hex_problem_name() {
    assert_eq!(
        <GeneralizedHex<SimpleGraph> as Problem>::NAME,
        "GeneralizedHex"
    );
}

#[test]
fn test_generalized_hex_serialization_round_trip() {
    let problem = winning_example();
    let json = serde_json::to_string(&problem).unwrap();
    let decoded: GeneralizedHex<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.source(), 0);
    assert_eq!(decoded.target(), 5);
    assert!(decoded.evaluate(&[]));
}

#[test]
fn test_generalized_hex_issue_example_is_losing_under_optimal_play() {
    let problem = issue_example();
    assert!(!problem.evaluate(&[]));
}

#[test]
fn test_generalized_hex_paper_example() {
    let problem = winning_example();
    assert!(problem.evaluate(&[]));
    assert_eq!(BruteForce::new().find_satisfying(&problem), Some(vec![]));
}

#[test]
#[should_panic(expected = "source and target must be distinct")]
fn test_generalized_hex_rejects_identical_terminals() {
    let _ = GeneralizedHex::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), 1, 1);
}
