use super::is_feedback_vertex_set;
use crate::models::graph::MinimumFeedbackVertexSet;
use crate::solvers::{BruteForce, Solver};
use crate::topology::DirectedGraph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;

/// Build the 9-vertex, 15-arc example from the issue.
///
/// Three triangles: 0→1→2→0, 3→4→5→3, 6→7→8→6
/// Cross arcs set 1: 1→3, 4→6, 7→0
/// Cross arcs set 2: 2→5, 5→8, 8→2
fn example_graph() -> DirectedGraph {
    DirectedGraph::new(
        9,
        vec![
            // Triangles
            (0, 1),
            (1, 2),
            (2, 0),
            (3, 4),
            (4, 5),
            (5, 3),
            (6, 7),
            (7, 8),
            (8, 6),
            // Cross set 1
            (1, 3),
            (4, 6),
            (7, 0),
            // Cross set 2
            (2, 5),
            (5, 8),
            (8, 2),
        ],
    )
}

#[test]
fn test_minimum_feedback_vertex_set_basic() {
    let graph = example_graph();
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 9]);

    // dims should be [2; 9]
    assert_eq!(problem.dims(), vec![2usize; 9]);

    // Valid FVS: {0, 3, 8} → config = [1,0,0,1,0,0,0,0,1]
    let config_valid = vec![1, 0, 0, 1, 0, 0, 0, 0, 1];
    let result = problem.evaluate(&config_valid);
    assert!(result.is_valid(), "Expected {{0,3,8}} to be a valid FVS");
    assert_eq!(result.unwrap(), 3, "Expected FVS size 3");

    // Invalid subset {1, 4, 7}: leaves cycle 2→5→8→2
    let config_invalid = vec![0, 1, 0, 0, 1, 0, 0, 1, 0];
    let result2 = problem.evaluate(&config_invalid);
    assert!(
        !result2.is_valid(),
        "Expected {{1,4,7}} to be an invalid FVS (cycle 2→5→8→2 remains)"
    );
}

#[test]
fn test_minimum_feedback_vertex_set_direction() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 3]);
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_minimum_feedback_vertex_set_serialization() {
    let graph = example_graph();
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 9]);

    let json = serde_json::to_string(&problem).expect("serialization failed");
    let deserialized: MinimumFeedbackVertexSet<i32> =
        serde_json::from_str(&json).expect("deserialization failed");

    assert_eq!(deserialized.graph().num_vertices(), 9);
    assert_eq!(deserialized.graph().num_arcs(), 15);
    assert_eq!(deserialized.weights(), problem.weights());
}

#[test]
fn test_minimum_feedback_vertex_set_solver() {
    let graph = example_graph();
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 9]);

    let solver = BruteForce::new();
    let best = solver.find_best(&problem);
    assert!(best.is_some(), "Expected a solution to exist");
    let best_config = best.unwrap();
    let best_result = problem.evaluate(&best_config);
    assert!(best_result.is_valid());
    assert_eq!(best_result.unwrap(), 3, "Expected optimal FVS size 3");

    let all_best = BruteForce::new().find_all_best(&problem);
    assert_eq!(all_best.len(), 18, "Expected 18 optimal FVS solutions");
}

#[test]
fn test_minimum_feedback_vertex_set_dag() {
    // A DAG: 0 → 1 → 2
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 3]);

    // Empty set (all zeros) is a valid FVS — graph is already a DAG
    let config_empty = vec![0, 0, 0];
    let result = problem.evaluate(&config_empty);
    assert!(result.is_valid(), "Empty FVS should be valid for a DAG");
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_minimum_feedback_vertex_set_all_selected() {
    // Selecting all vertices always yields a valid (but suboptimal) FVS
    let graph = example_graph();
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 9]);

    let config_all = vec![1usize; 9];
    let result = problem.evaluate(&config_all);
    assert!(result.is_valid(), "Selecting all vertices should be valid");
    assert_eq!(result.unwrap(), 9);
}

#[test]
fn test_is_feedback_vertex_set_helper() {
    let graph = example_graph();

    // {0, 3, 8} is a valid FVS
    let selected = [true, false, false, true, false, false, false, false, true];
    assert!(is_feedback_vertex_set(&graph, &selected));

    // {1, 4, 7} is NOT a valid FVS (cycle 2→5→8→2 remains)
    let not_fvs = [false, true, false, false, true, false, false, true, false];
    assert!(!is_feedback_vertex_set(&graph, &not_fvs));

    // Empty set is not a valid FVS for the cyclic graph
    let empty = [false; 9];
    assert!(!is_feedback_vertex_set(&graph, &empty));
}
