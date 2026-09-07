use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

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
    assert_eq!(problem.dimensions(), vec![2, 2, 2, 2, 2, 2]);

    // Evaluate a valid balanced partition: A={0,1,2}, B={3,4,5}
    // config: [0, 0, 0, 1, 1, 1]
    // Crossing edges: (1,3), (2,3), (2,4) => cut = 3
    let config = vec![false, false, false, true, true, true];
    let result = problem.evaluate(&config).unwrap();
    assert_eq!(result, Min(Some(3)));
}

#[test]
fn test_graphpartitioning_serialization() {
    let problem = issue_example();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: GraphPartitioning<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.graph().num_vertices(), 6);
    assert_eq!(deserialized.graph().num_edges(), 9);

    // Verify evaluation is consistent after round-trip
    let config = vec![false, false, false, true, true, true];
    assert_eq!(
        problem.evaluate(&config).unwrap(),
        deserialized.evaluate(&config).unwrap()
    );
}

#[test]
fn test_graphpartitioning_solver() {
    let problem = issue_example();
    let solver = BruteForce::new();
    let best = solver.solve(&problem).unwrap().unwrap();
    let size = problem.evaluate(&best).unwrap();
    assert_eq!(size, Min(Some(3)));

    // All optimal solutions should have cut = 3
    let all_best = solver.find_all_witnesses(&problem).unwrap();
    assert!(!all_best.is_empty());
    for sol in &all_best {
        assert_eq!(problem.evaluate(sol).unwrap(), Min(Some(3)));
    }
}

#[test]
fn test_graphpartitioning_odd_vertices() {
    // 3 vertices: all configs must be Invalid since n is odd
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = GraphPartitioning::new(graph);

    // Every possible config should be Invalid
    for a in [false, true] {
        for b in [false, true] {
            for c in [false, true] {
                assert_eq!(
                    problem.evaluate(&vec![a, b, c]).unwrap(),
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
    assert_eq!(
        problem.evaluate(&vec![false, false, false, false]).unwrap(),
        Min(None)
    );

    // All ones: 4 ones, not balanced
    assert_eq!(
        problem.evaluate(&vec![true, true, true, true]).unwrap(),
        Min(None)
    );

    // One vertex in partition 1: not balanced
    assert_eq!(
        problem.evaluate(&vec![true, false, false, false]).unwrap(),
        Min(None)
    );

    // Three vertices in partition 1: not balanced
    assert_eq!(
        problem.evaluate(&vec![true, true, true, false]).unwrap(),
        Min(None)
    );

    // Two vertices in partition 1: balanced, should be Valid
    // 4-cycle edges: (0,1),(1,2),(2,3),(0,3). Config [1,1,0,0] cuts (1,2) and (0,3) => cut=2
    assert_eq!(
        problem.evaluate(&vec![true, true, false, false]).unwrap(),
        Min(Some(2))
    );
}

#[test]
fn test_graphpartitioning_rejects_non_binary_configs() {
    let problem = issue_example();

    assert!(crate::registry::DynProblem::evaluate_dyn(
        &problem,
        &serde_json::json!([false, false, true, true, true, 2])
    )
    .is_err());
}

#[test]
fn test_graphpartitioning_parameter_getters() {
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
    let all_best = solver.find_all_witnesses(&problem).unwrap();

    // Minimum bisection of a 4-cycle: cut = 2
    for sol in &all_best {
        assert_eq!(problem.evaluate(sol).unwrap(), Min(Some(2)));
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

    let config = vec![false, false, true, true];
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(0)));
}
