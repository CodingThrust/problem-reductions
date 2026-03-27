use super::is_feedback_vertex_set;
use crate::models::graph::MinimumFeedbackVertexSet;
use crate::solvers::BruteForce;
use crate::topology::DirectedGraph;
use crate::traits::Problem;

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
    let best = solver.find_witness(&problem);
    assert!(best.is_some(), "Expected a solution to exist");
    let best_config = best.unwrap();
    let best_result = problem.evaluate(&best_config);
    assert!(best_result.is_valid());
    assert_eq!(best_result.unwrap(), 3, "Expected optimal FVS size 3");

    let all_best = BruteForce::new().find_all_witnesses(&problem);
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
fn test_minimum_feedback_vertex_set_accessors() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let mut problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 3]);

    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_arcs(), 3);
    assert!(problem.is_weighted());

    // set_weights
    problem.set_weights(vec![2, 3, 4]);
    assert_eq!(problem.weights(), &[2, 3, 4]);
}

#[test]
fn test_minimum_feedback_vertex_set_is_valid_solution() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 3]);

    // Valid FVS: remove vertex 0
    assert!(problem.is_valid_solution(&[1, 0, 0]));
    // Invalid: no vertices removed, cycle remains
    assert!(!problem.is_valid_solution(&[0, 0, 0]));
    // Wrong length returns false
    assert!(!problem.is_valid_solution(&[1, 0]));
}

#[test]
fn test_minimum_feedback_vertex_set_evaluate_wrong_length() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 3]);

    // Wrong length config returns Invalid
    assert!(!problem.evaluate(&[1, 0]).is_valid());
}

#[test]
fn test_minimum_feedback_vertex_set_variant() {
    let v = <MinimumFeedbackVertexSet<i32> as Problem>::variant();
    assert_eq!(v, vec![("weight", "i32")]);
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

#[test]
fn test_minimum_feedback_vertex_set_paper_example() {
    // Paper: 5 vertices, 7 arcs, two overlapping cycles:
    // C_1 = v_0→v_1→v_2→v_0, C_2 = v_0→v_3→v_4→v_1
    // 7th arc: (4,2) — removing v_0 leaves DAG with topo order (v_3, v_4, v_1, v_2)
    // FVS = {v_0}, weight = 1
    let graph = DirectedGraph::new(
        5,
        vec![(0, 1), (1, 2), (2, 0), (0, 3), (3, 4), (4, 1), (4, 2)],
    );
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 5]);

    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_arcs(), 7);

    // {v_0} is a valid FVS with weight 1
    let config = vec![1, 0, 0, 0, 0];
    let result = problem.evaluate(&config);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 1);

    // Removing v_1 alone leaves cycle v_0→v_3→v_4→...→v_2→v_0 (through arc (2,0))
    let config_v1 = vec![0, 1, 0, 0, 0];
    assert!(!problem.evaluate(&config_v1).is_valid());

    // Verify optimal FVS weight is 1
    let solver = BruteForce::new();
    let best = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&best).unwrap(), 1);
}
