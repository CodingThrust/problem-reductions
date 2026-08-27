use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
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
    assert_eq!(problem.dimensions(), vec![5; 10]);
    assert!(problem.evaluate(&config).unwrap());
    assert_eq!(problem.total_edge_stretch(&config).unwrap(), Some(6));
}

#[test]
fn test_rootedtreearrangement_rejects_invalid_parent_arrays() {
    let problem = issue_example();

    // Two roots: node 0 and node 1 are both self-parented.
    let multiple_roots = vec![0, 1, 1, 2, 3, 0, 1, 2, 3, 4];
    assert!(!problem.evaluate(&multiple_roots).unwrap());
    assert_eq!(problem.total_edge_stretch(&multiple_roots).unwrap(), None);

    // Directed cycle between nodes 1 and 2.
    let cycle = vec![0, 2, 1, 2, 3, 0, 1, 2, 3, 4];
    assert!(!problem.evaluate(&cycle).unwrap());
    assert_eq!(problem.total_edge_stretch(&cycle).unwrap(), None);
}

#[test]
fn test_rootedtreearrangement_rejects_invalid_bijections() {
    let problem = issue_example();

    let duplicate_image = vec![0, 0, 1, 2, 3, 0, 0, 2, 3, 4];
    assert!(!problem.evaluate(&duplicate_image).unwrap());
    assert_eq!(problem.total_edge_stretch(&duplicate_image).unwrap(), None);

    let out_of_range = vec![0, 0, 1, 2, 3, 0, 1, 2, 3, 5];
    assert!(!problem.evaluate(&out_of_range).unwrap());
    assert_eq!(problem.total_edge_stretch(&out_of_range).unwrap(), None);

    let wrong_length = vec![0, 0, 1, 2, 3, 0, 1, 2, 3];
    assert!(matches!(
        problem.evaluate(&wrong_length),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert_eq!(problem.total_edge_stretch(&wrong_length).unwrap(), None);
}

#[test]
fn test_rootedtreearrangement_rejects_noncomparable_edges() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 2), (2, 3), (3, 4)]);
    let problem = RootedTreeArrangement::new(graph, 99);

    // Tree: 0 is root, 1 and 2 are siblings, 3 and 4 descend from 2.
    // The graph edge {1,2} is invalid because mapped nodes 1 and 2 are not ancestor-comparable.
    let branching_tree = vec![0, 0, 0, 2, 3, 0, 1, 2, 3, 4];
    assert!(!problem.evaluate(&branching_tree).unwrap());
    assert_eq!(problem.total_edge_stretch(&branching_tree).unwrap(), None);
}

#[test]
fn test_rootedtreearrangement_enforces_bound() {
    let problem = issue_example();

    // Same chain tree as the YES witness, but the mapping stretches edge {2,3} too far.
    let over_bound = vec![0, 0, 1, 2, 3, 2, 1, 0, 3, 4];
    assert!(!problem.evaluate(&over_bound).unwrap());
    assert_eq!(problem.total_edge_stretch(&over_bound).unwrap(), Some(8));
}

#[test]
fn test_rootedtreearrangement_solver_and_serialization() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = RootedTreeArrangement::new(graph, 2);

    let solver = BruteForce::new();
    let solution = solver
        .solve(&problem)
        .unwrap()
        .expect("expected satisfying solution");
    assert!(problem.evaluate(&solution).unwrap());

    let json = serde_json::to_string(&problem).unwrap();
    let restored: RootedTreeArrangement<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.num_vertices(), 3);
    assert_eq!(restored.num_edges(), 2);
    assert_eq!(restored.bound(), 2);
    assert_eq!(
        restored.evaluate(&solution).unwrap(),
        problem.evaluate(&solution).unwrap()
    );
}

#[test]
fn test_rootedtreearrangement_problem_name() {
    assert_eq!(
        <RootedTreeArrangement<SimpleGraph> as Problem>::NAME,
        "RootedTreeArrangement"
    );
}
