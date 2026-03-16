use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_subgraph_isomorphism_creation() {
    let host = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let pattern = SimpleGraph::new(2, vec![(0, 1)]);
    let problem = SubgraphIsomorphism::new(host, pattern);
    assert_eq!(problem.num_host_vertices(), 4);
    assert_eq!(problem.num_host_edges(), 3);
    assert_eq!(problem.num_pattern_vertices(), 2);
    assert_eq!(problem.num_pattern_edges(), 1);
    // dims: 2 pattern vertices, each can map to 4 host vertices
    assert_eq!(problem.dims(), vec![4, 4]);
}

#[test]
fn test_subgraph_isomorphism_evaluation_valid() {
    // Host: triangle 0-1-2
    let host = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    // Pattern: single edge
    let pattern = SimpleGraph::new(2, vec![(0, 1)]);
    let problem = SubgraphIsomorphism::new(host, pattern);

    // Valid mapping: pattern vertex 0->host 0, pattern vertex 1->host 1
    assert!(problem.evaluate(&[0, 1]));
    // Valid: 0->1, 1->2
    assert!(problem.evaluate(&[1, 2]));
    // Valid: 0->0, 1->2
    assert!(problem.evaluate(&[0, 2]));
}

#[test]
fn test_subgraph_isomorphism_evaluation_invalid() {
    // Host: path 0-1-2 (no edge 0-2)
    let host = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    // Pattern: single edge
    let pattern = SimpleGraph::new(2, vec![(0, 1)]);
    let problem = SubgraphIsomorphism::new(host, pattern);

    // Invalid: non-injective (both map to same host vertex)
    assert!(!problem.evaluate(&[0, 0]));
    // Invalid: no edge between host vertices 0 and 2
    assert!(!problem.evaluate(&[0, 2]));
    // Valid: 0->0, 1->1 (edge 0-1 exists)
    assert!(problem.evaluate(&[0, 1]));
}

#[test]
fn test_subgraph_isomorphism_triangle_in_k4() {
    // Host: K4 (complete graph on 4 vertices)
    let host = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    // Pattern: triangle K3
    let pattern = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let problem = SubgraphIsomorphism::new(host, pattern);

    // Any injective mapping into K4 should work for K3
    assert!(problem.evaluate(&[0, 1, 2]));
    assert!(problem.evaluate(&[1, 2, 3]));
    assert!(problem.evaluate(&[0, 2, 3]));

    // Non-injective should fail
    assert!(!problem.evaluate(&[0, 0, 1]));
}

#[test]
fn test_subgraph_isomorphism_no_solution() {
    // Host: path 0-1-2 (no triangles)
    let host = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    // Pattern: triangle K3
    let pattern = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let problem = SubgraphIsomorphism::new(host, pattern);

    // No possible mapping should work
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_none());
}

#[test]
fn test_subgraph_isomorphism_solver() {
    // Host: K4
    let host = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    // Pattern: triangle
    let pattern = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let problem = SubgraphIsomorphism::new(host, pattern);

    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());

    let sol = solution.unwrap();
    assert!(problem.evaluate(&sol));
}

#[test]
fn test_subgraph_isomorphism_all_satisfying() {
    // Host: triangle 0-1-2
    let host = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    // Pattern: single edge
    let pattern = SimpleGraph::new(2, vec![(0, 1)]);
    let problem = SubgraphIsomorphism::new(host, pattern);

    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    // 3 edges in host, each can be mapped in 2 directions = 6 solutions
    assert_eq!(solutions.len(), 6);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_subgraph_isomorphism_serialization() {
    let host = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let pattern = SimpleGraph::new(2, vec![(0, 1)]);
    let problem = SubgraphIsomorphism::new(host, pattern);

    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: SubgraphIsomorphism = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.num_host_vertices(), 3);
    assert_eq!(deserialized.num_pattern_vertices(), 2);
    assert_eq!(deserialized.num_host_edges(), 2);
    assert_eq!(deserialized.num_pattern_edges(), 1);
}

#[test]
fn test_subgraph_isomorphism_problem_name() {
    assert_eq!(
        <SubgraphIsomorphism as Problem>::NAME,
        "SubgraphIsomorphism"
    );
}

#[test]
fn test_subgraph_isomorphism_is_valid_solution() {
    let host = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let pattern = SimpleGraph::new(2, vec![(0, 1)]);
    let problem = SubgraphIsomorphism::new(host, pattern);

    assert!(problem.is_valid_solution(&[0, 1]));
    assert!(!problem.is_valid_solution(&[0, 0]));
}

#[test]
fn test_subgraph_isomorphism_empty_pattern() {
    // Pattern with no edges — any injective mapping is valid
    let host = SimpleGraph::new(3, vec![(0, 1)]);
    let pattern = SimpleGraph::new(2, vec![]);
    let problem = SubgraphIsomorphism::new(host, pattern);

    // Any two distinct host vertices work
    assert!(problem.evaluate(&[0, 1]));
    assert!(problem.evaluate(&[1, 2]));
    assert!(problem.evaluate(&[0, 2]));
    // Non-injective fails
    assert!(!problem.evaluate(&[0, 0]));
}

#[test]
fn test_subgraph_isomorphism_issue_example() {
    // Example from issue #218
    // Host: K4 on {0,1,2,3} plus triangle {4,5,6} connected via 3-4
    let host = SimpleGraph::new(
        7,
        vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 2),
            (1, 3),
            (2, 3),
            (3, 4),
            (4, 5),
            (4, 6),
            (5, 6),
        ],
    );
    // Pattern: K4
    let pattern = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let problem = SubgraphIsomorphism::new(host, pattern);

    // The mapping from the issue: a->0, b->1, c->2, d->3
    assert!(problem.evaluate(&[0, 1, 2, 3]));

    // Verify solver can find a solution
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));
}

#[test]
fn test_subgraph_isomorphism_size_getters() {
    let host = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let pattern = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = SubgraphIsomorphism::new(host, pattern);
    assert_eq!(problem.num_host_vertices(), 5);
    assert_eq!(problem.num_host_edges(), 4);
    assert_eq!(problem.num_pattern_vertices(), 3);
    assert_eq!(problem.num_pattern_edges(), 2);
}
