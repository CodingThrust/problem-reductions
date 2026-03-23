use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::{ObjectiveProblem, Problem};
use crate::types::ExtremumSense;

/// Issue example: 6 vertices, edges forming two triangles connected by 3 edges.
/// Optimal partition A={0,1,2}, B={3,4,5}, cut=3.
fn issue_example() -> GraphPartitioning<SimpleGraph> {
    let graph = SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (0, 2),
            (1, 2),
            (1, 3),
            (2, 3),
            (2, 4),
            (3, 4),
            (3, 5),
            (4, 5),
        ],
    );
    GraphPartitioning::new(graph)
}

#[test]
fn test_graphpartitioning_basic() {
    let problem = issue_example();

    // Check dims: 6 binary variables
    assert_eq!(problem.dims(), vec![2, 2, 2, 2, 2, 2]);

    // Evaluate a valid balanced partition: A={0,1,2}, B={3,4,5}
    // config: [0, 0, 0, 1, 1, 1]
    // Crossing edges: (1,3), (2,3), (2,4) => cut = 3
    let config = vec![0, 0, 0, 1, 1, 1];
    let result = problem.evaluate(&config);
    assert_eq!(result, Min(Some(3)));
}

#[test]
fn test_graphpartitioning_direction() {
    let problem = issue_example();
    assert_eq!(problem.direction(), ExtremumSense::Minimize);
}

#[test]
fn test_graphpartitioning_serialization() {
    let problem = issue_example();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: GraphPartitioning<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.graph().num_vertices(), 6);
    assert_eq!(deserialized.graph().num_edges(), 9);

    // Verify evaluation is consistent after round-trip
    let config = vec![0, 0, 0, 1, 1, 1];
    assert_eq!(problem.evaluate(&config), deserialized.evaluate(&config));
}

#[test]
fn test_graphpartitioning_solver() {
    let problem = issue_example();
    let solver = BruteForce::new();
    let best = solver.find_witness(&problem).unwrap();
    let size = problem.evaluate(&best);
    assert_eq!(size, Min(Some(3)));

    // All optimal solutions should have cut = 3
    let all_best = solver.find_all_witnesses(&problem);
    assert!(!all_best.is_empty());
    for sol in &all_best {
        assert_eq!(problem.evaluate(sol), Min(Some(3)));
    }
}

#[test]
fn test_graphpartitioning_odd_vertices() {
    // 3 vertices: all configs must be Invalid since n is odd
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = GraphPartitioning::new(graph);

    // Every possible config should be Invalid
    for a in 0..2 {
        for b in 0..2 {
            for c in 0..2 {
                assert_eq!(
                    problem.evaluate(&[a, b, c]),
                    Min(None),
                    "Expected Invalid for odd n, config [{}, {}, {}]",
                    a,
                    b,
                    c
                );
            }
        }
    }
}

#[test]
fn test_graphpartitioning_unbalanced_invalid() {
    // 4 vertices: only configs with exactly 2 ones are valid
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]);
    let problem = GraphPartitioning::new(graph);

    // All zeros: 0 ones, not balanced
    assert_eq!(problem.evaluate(&[0, 0, 0, 0]), Min(None));

    // All ones: 4 ones, not balanced
    assert_eq!(problem.evaluate(&[1, 1, 1, 1]), Min(None));

    // One vertex in partition 1: not balanced
    assert_eq!(problem.evaluate(&[1, 0, 0, 0]), Min(None));

    // Three vertices in partition 1: not balanced
    assert_eq!(problem.evaluate(&[1, 1, 1, 0]), Min(None));

    // Two vertices in partition 1: balanced, should be Valid
    // 4-cycle edges: (0,1),(1,2),(2,3),(0,3). Config [1,1,0,0] cuts (1,2) and (0,3) => cut=2
    assert_eq!(problem.evaluate(&[1, 1, 0, 0]), Min(Some(2)));
}

#[test]
fn test_graphpartitioning_size_getters() {
    let problem = issue_example();
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 9);
}

#[test]
fn test_graphpartitioning_square_graph() {
    // Square graph: 0-1, 1-2, 2-3, 3-0 (the doctest example)
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]);
    let problem = GraphPartitioning::new(graph);

    let solver = BruteForce::new();
    let all_best = solver.find_all_witnesses(&problem);

    // Minimum bisection of a 4-cycle: cut = 2
    for sol in &all_best {
        assert_eq!(problem.evaluate(sol), Min(Some(2)));
    }
}

#[test]
fn test_graphpartitioning_problem_name() {
    assert_eq!(
        <GraphPartitioning<SimpleGraph> as Problem>::NAME,
        "GraphPartitioning"
    );
}

#[test]
fn test_graphpartitioning_graph_accessor() {
    let problem = issue_example();
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 6);
    assert_eq!(graph.num_edges(), 9);
}

#[test]
fn test_graphpartitioning_empty_graph() {
    // 4 vertices, no edges: any balanced partition has cut = 0
    let graph = SimpleGraph::new(4, vec![]);
    let problem = GraphPartitioning::new(graph);

    let config = vec![0, 0, 1, 1];
    assert_eq!(problem.evaluate(&config), Min(Some(0)));
}
