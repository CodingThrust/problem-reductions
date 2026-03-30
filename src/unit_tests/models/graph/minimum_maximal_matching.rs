use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_minimum_maximal_matching_creation() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = MinimumMaximalMatching::new(graph);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.num_variables(), 3);
}

#[test]
fn test_minimum_maximal_matching_evaluate_valid() {
    // Path P4: edges (0,1),(1,2),(2,3)
    // config [0,1,0]: select edge (1,2). Is it maximal?
    // Edge (0,1): shares vertex 1 with (1,2) ✓ blocked
    // Edge (2,3): shares vertex 2 with (1,2) ✓ blocked
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = MinimumMaximalMatching::new(graph);
    assert_eq!(problem.evaluate(&[0, 1, 0]), Min(Some(1)));
}

#[test]
fn test_minimum_maximal_matching_evaluate_not_maximal() {
    // Path P4: edges (0,1),(1,2),(2,3)
    // config [1,0,0]: select only (0,1). Edge (2,3) is not blocked — not maximal.
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = MinimumMaximalMatching::new(graph);
    assert_eq!(problem.evaluate(&[1, 0, 0]), Min(None));
}

#[test]
fn test_minimum_maximal_matching_evaluate_not_matching() {
    // Triangle: edges (0,1),(1,2),(0,2)
    // config [1,1,0]: select (0,1) and (1,2) — vertex 1 shared → not a matching.
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = MinimumMaximalMatching::new(graph);
    assert_eq!(problem.evaluate(&[1, 1, 0]), Min(None));
}

#[test]
fn test_minimum_maximal_matching_evaluate_wrong_length() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumMaximalMatching::new(graph);
    // Provide config of wrong length
    assert_eq!(problem.evaluate(&[1]), Min(None));
}

#[test]
fn test_minimum_maximal_matching_empty_graph() {
    // No edges: empty config is a valid (vacuously maximal) matching of size 0.
    let graph = SimpleGraph::new(3, vec![]);
    let problem = MinimumMaximalMatching::new(graph);
    assert_eq!(problem.evaluate(&[]), Min(Some(0)));
}

#[test]
fn test_minimum_maximal_matching_path_p6_solver() {
    // Path P6: 6 vertices, 5 edges.
    // Optimal minimum maximal matching has size 2, e.g. {(1,2),(3,4)}.
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)]);
    let problem = MinimumMaximalMatching::new(graph);
    let solver = BruteForce::new();
    let best = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&best), Min(Some(2)));
}

#[test]
fn test_minimum_maximal_matching_canonical_example() {
    // Canonical example: P6 with config [0,1,0,1,0] → edges (1,2) and (3,4).
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)]);
    let problem = MinimumMaximalMatching::new(graph);
    let config = vec![0, 1, 0, 1, 0];
    assert_eq!(problem.evaluate(&config), Min(Some(2)));
}

#[test]
fn test_minimum_maximal_matching_triangle() {
    // Triangle: any single edge is a maximal matching (both remaining edges are blocked).
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = MinimumMaximalMatching::new(graph);
    let solver = BruteForce::new();
    let best = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&best), Min(Some(1)));
}

#[test]
fn test_minimum_maximal_matching_star() {
    // Star K_{1,3}: center 0 connected to 1,2,3.
    // Any single edge from center is a maximal matching.
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]);
    let problem = MinimumMaximalMatching::new(graph);
    let solver = BruteForce::new();
    let best = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&best), Min(Some(1)));
}

#[test]
fn test_minimum_maximal_matching_serialization() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = MinimumMaximalMatching::new(graph);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinimumMaximalMatching<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vertices(), 4);
    assert_eq!(deserialized.num_edges(), 3);
}
