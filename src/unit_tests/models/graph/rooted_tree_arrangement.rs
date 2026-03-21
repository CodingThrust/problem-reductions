use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn issue_example() -> RootedTreeArrangement<SimpleGraph> {
    let graph = SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 2), (2, 3), (3, 4)]);
    RootedTreeArrangement::new(graph, 7)
}

fn issue_chain_witness() -> Vec<usize> {
    vec![0, 0, 1, 2, 3, 0, 1, 2, 3, 4]
}

#[test]
fn test_rootedtreearrangement_basic_yes_example() {
    let problem = issue_example();
    let config = issue_chain_witness();

    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 5);
    assert_eq!(problem.bound(), 7);
    assert_eq!(problem.dims(), vec![5; 10]);
    assert!(problem.evaluate(&config));
    assert_eq!(problem.total_edge_stretch(&config), Some(6));
}

#[test]
fn test_rootedtreearrangement_rejects_invalid_parent_arrays() {
    let problem = issue_example();

    // Two roots: node 0 and node 1 are both self-parented.
    let multiple_roots = vec![0, 1, 1, 2, 3, 0, 1, 2, 3, 4];
    assert!(!problem.evaluate(&multiple_roots));
    assert_eq!(problem.total_edge_stretch(&multiple_roots), None);

    // Directed cycle between nodes 1 and 2.
    let cycle = vec![0, 2, 1, 2, 3, 0, 1, 2, 3, 4];
    assert!(!problem.evaluate(&cycle));
    assert_eq!(problem.total_edge_stretch(&cycle), None);
}

#[test]
fn test_rootedtreearrangement_rejects_invalid_bijections() {
    let problem = issue_example();

    let duplicate_image = vec![0, 0, 1, 2, 3, 0, 0, 2, 3, 4];
    assert!(!problem.evaluate(&duplicate_image));
    assert_eq!(problem.total_edge_stretch(&duplicate_image), None);

    let out_of_range = vec![0, 0, 1, 2, 3, 0, 1, 2, 3, 5];
    assert!(!problem.evaluate(&out_of_range));
    assert_eq!(problem.total_edge_stretch(&out_of_range), None);

    let wrong_length = vec![0, 0, 1, 2, 3, 0, 1, 2, 3];
    assert!(!problem.evaluate(&wrong_length));
    assert_eq!(problem.total_edge_stretch(&wrong_length), None);
}

#[test]
fn test_rootedtreearrangement_rejects_noncomparable_edges() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 2), (2, 3), (3, 4)]);
    let problem = RootedTreeArrangement::new(graph, 99);

    // Tree: 0 is root, 1 and 2 are siblings, 3 and 4 descend from 2.
    // The graph edge {1,2} is invalid because mapped nodes 1 and 2 are not ancestor-comparable.
    let branching_tree = vec![0, 0, 0, 2, 3, 0, 1, 2, 3, 4];
    assert!(!problem.evaluate(&branching_tree));
    assert_eq!(problem.total_edge_stretch(&branching_tree), None);
}

#[test]
fn test_rootedtreearrangement_enforces_bound() {
    let problem = issue_example();

    // Same chain tree as the YES witness, but the mapping stretches edge {2,3} too far.
    let over_bound = vec![0, 0, 1, 2, 3, 2, 1, 0, 3, 4];
    assert!(!problem.evaluate(&over_bound));
    assert_eq!(problem.total_edge_stretch(&over_bound), Some(8));
}

#[test]
fn test_rootedtreearrangement_solver_and_serialization() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = RootedTreeArrangement::new(graph, 2);

    let solver = BruteForce::new();
    let solution = solver
        .find_satisfying(&problem)
        .expect("expected satisfying solution");
    assert!(problem.evaluate(&solution));

    let json = serde_json::to_string(&problem).unwrap();
    let restored: RootedTreeArrangement<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.num_vertices(), 3);
    assert_eq!(restored.num_edges(), 2);
    assert_eq!(restored.bound(), 2);
    assert_eq!(restored.evaluate(&solution), problem.evaluate(&solution));
}

#[test]
fn test_rootedtreearrangement_problem_name() {
    assert_eq!(
        <RootedTreeArrangement<SimpleGraph> as Problem>::NAME,
        "RootedTreeArrangement"
    );
}
