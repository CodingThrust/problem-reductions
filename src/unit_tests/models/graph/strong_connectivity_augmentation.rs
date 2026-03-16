use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;

fn issue_graph() -> DirectedGraph {
    DirectedGraph::new(
        6,
        vec![
            (0, 1),
            (1, 2),
            (2, 0),
            (3, 4),
            (4, 3),
            (2, 3),
            (4, 5),
            (5, 3),
        ],
    )
}

fn issue_candidate_arcs() -> Vec<(usize, usize, i32)> {
    vec![
        (3, 0, 5),
        (3, 1, 3),
        (3, 2, 4),
        (4, 0, 6),
        (4, 1, 2),
        (4, 2, 7),
        (5, 0, 4),
        (5, 1, 3),
        (5, 2, 1),
        (0, 3, 8),
        (0, 4, 3),
        (0, 5, 2),
        (1, 3, 6),
        (1, 4, 4),
        (1, 5, 5),
        (2, 4, 3),
        (2, 5, 7),
        (1, 0, 2),
    ]
}

fn issue_example_yes() -> StrongConnectivityAugmentation<i32> {
    StrongConnectivityAugmentation::new(issue_graph(), issue_candidate_arcs(), 1)
}

fn yes_config() -> Vec<usize> {
    vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
}

fn issue_example_already_strongly_connected() -> StrongConnectivityAugmentation<i32> {
    StrongConnectivityAugmentation::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
        vec![(0, 2, 5)],
        0,
    )
}

#[test]
fn test_strong_connectivity_augmentation_creation() {
    let problem = issue_example_yes();

    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_arcs(), 8);
    assert_eq!(problem.num_potential_arcs(), 18);
    assert_eq!(problem.candidate_arcs().len(), 18);
    assert_eq!(problem.bound(), &1);
    assert_eq!(problem.dims(), vec![2; 18]);
    assert!(problem.is_weighted());
}

#[test]
fn test_strong_connectivity_augmentation_issue_example_yes() {
    let problem = issue_example_yes();
    let config = yes_config();

    assert!(problem.evaluate(&config));
    assert!(problem.is_valid_solution(&config));
}

#[test]
fn test_strong_connectivity_augmentation_issue_example_no() {
    let problem = issue_example_yes();
    assert!(!problem.evaluate(&[0; 18]));
}

#[test]
fn test_strong_connectivity_augmentation_wrong_length() {
    let problem = issue_example_yes();
    assert!(!problem.evaluate(&[0, 1]));
    assert!(!problem.is_valid_solution(&[0, 1]));
}

#[test]
fn test_strong_connectivity_augmentation_already_strongly_connected() {
    let problem = issue_example_already_strongly_connected();
    assert_eq!(problem.dims(), vec![2]);
    assert!(problem.evaluate(&[0]));
    assert!(!problem.evaluate(&[1]));
}

#[test]
fn test_strong_connectivity_augmentation_serialization() {
    let problem = issue_example_yes();
    let json = serde_json::to_string(&problem).unwrap();
    let restored: StrongConnectivityAugmentation<i32> = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.graph(), problem.graph());
    assert_eq!(restored.candidate_arcs(), problem.candidate_arcs());
    assert_eq!(restored.bound(), problem.bound());
}

#[test]
fn test_strong_connectivity_augmentation_solver() {
    let problem = issue_example_yes();
    let solver = BruteForce::new();

    let satisfying = solver.find_satisfying(&problem).unwrap();
    assert!(problem.evaluate(&satisfying));

    let all_satisfying = solver.find_all_satisfying(&problem);
    assert_eq!(all_satisfying, vec![yes_config()]);
}

#[test]
fn test_strong_connectivity_augmentation_variant() {
    let variant = <StrongConnectivityAugmentation<i32> as Problem>::variant();
    assert_eq!(variant, vec![("weight", "i32")]);
}

#[test]
fn test_strong_connectivity_augmentation_paper_example() {
    let problem = issue_example_yes();
    let config = yes_config();

    assert!(problem.evaluate(&config));
}

#[test]
#[should_panic(expected = "candidate arc (0, 1) already exists in the base graph")]
fn test_strong_connectivity_augmentation_existing_arc_candidate_panics() {
    StrongConnectivityAugmentation::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![(0, 1, 1)],
        1,
    );
}

#[test]
#[should_panic(expected = "duplicate candidate arc (0, 2)")]
fn test_strong_connectivity_augmentation_duplicate_candidate_arc_panics() {
    StrongConnectivityAugmentation::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![(0, 2, 1), (0, 2, 3)],
        3,
    );
}

#[test]
#[should_panic(expected = "candidate arc (0, 3) references vertex >= num_vertices")]
fn test_strong_connectivity_augmentation_out_of_range_candidate_panics() {
    StrongConnectivityAugmentation::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![(0, 3, 1)],
        1,
    );
}
