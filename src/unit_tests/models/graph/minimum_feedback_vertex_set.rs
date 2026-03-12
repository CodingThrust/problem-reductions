use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;

#[test]
fn test_feedback_vertex_set_creation() {
    let problem = MinimumFeedbackVertexSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32; 4],
    );
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 3);
    assert_eq!(problem.dims().len(), 4);
}

#[test]
fn test_feedback_vertex_set_with_weights() {
    let problem = MinimumFeedbackVertexSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1, 2, 3]);
    assert_eq!(problem.weights().to_vec(), vec![1, 2, 3]);
    assert!(problem.is_weighted());
}

#[test]
fn test_feedback_vertex_set_tree() {
    // A tree has no cycles, so empty set is a valid FVS
    // Path graph: 0-1-2-3
    let problem = MinimumFeedbackVertexSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32; 4],
    );
    // Empty set is valid FVS for a tree
    assert!(problem.is_valid_solution(&[0, 0, 0, 0]));

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    // Optimal FVS of a tree is empty
    assert!(solutions.contains(&vec![0, 0, 0, 0]));
}

#[test]
fn test_feedback_vertex_set_triangle() {
    // Triangle: 0-1-2-0 has one cycle, need to remove 1 vertex
    let problem = MinimumFeedbackVertexSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32; 3],
    );

    // Removing any single vertex breaks the cycle
    assert!(problem.is_valid_solution(&[1, 0, 0]));
    assert!(problem.is_valid_solution(&[0, 1, 0]));
    assert!(problem.is_valid_solution(&[0, 0, 1]));

    // Empty set is not valid (triangle has a cycle)
    assert!(!problem.is_valid_solution(&[0, 0, 0]));

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    // Minimum FVS has size 1
    assert!(solutions.iter().all(|s| s.iter().sum::<usize>() == 1));
    assert_eq!(solutions.len(), 3); // Any of the 3 vertices works
}

#[test]
fn test_feedback_vertex_set_two_cycles() {
    // Two triangles sharing an edge: 0-1-2-0 and 0-1-3-0
    // Edges: (0,1), (1,2), (0,2), (1,3), (0,3)
    let problem = MinimumFeedbackVertexSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (0, 2), (1, 3), (0, 3)]),
        vec![1i32; 4],
    );

    // Removing vertex 0 or 1 breaks both cycles (they share the edge 0-1)
    assert!(problem.is_valid_solution(&[1, 0, 0, 0]));
    assert!(problem.is_valid_solution(&[0, 1, 0, 0]));

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    // Minimum FVS should have size 1
    assert!(solutions.iter().all(|s| s.iter().sum::<usize>() == 1));
}

#[test]
fn test_feedback_vertex_set_k4() {
    // Complete graph K4: every pair of 3 vertices forms a cycle
    let problem = MinimumFeedbackVertexSet::new(SimpleGraph::complete(4), vec![1i32; 4]);

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    // K4 has 6 edges, 4 vertices. A forest on 4 vertices has at most 3 edges.
    // Need to remove vertices until remaining graph is a forest.
    // Removing 2 vertices leaves K2 (1 edge, which is a tree). Size = 2.
    assert!(solutions.iter().all(|s| s.iter().sum::<usize>() == 2));
}

#[test]
fn test_feedback_vertex_set_empty_graph() {
    // No edges = no cycles
    let problem = MinimumFeedbackVertexSet::new(SimpleGraph::empty(3), vec![1i32; 3]);
    assert!(problem.is_valid_solution(&[0, 0, 0]));

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert!(solutions.contains(&vec![0, 0, 0]));
}

#[test]
fn test_feedback_vertex_set_cycle_graph() {
    // Cycle on 5 vertices: 0-1-2-3-4-0
    let problem = MinimumFeedbackVertexSet::new(SimpleGraph::cycle(5), vec![1i32; 5]);

    // Removing any single vertex breaks the only cycle
    assert!(problem.is_valid_solution(&[1, 0, 0, 0, 0]));
    assert!(!problem.is_valid_solution(&[0, 0, 0, 0, 0]));

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert!(solutions.iter().all(|s| s.iter().sum::<usize>() == 1));
    assert_eq!(solutions.len(), 5);
}

#[test]
fn test_is_feedback_vertex_set_function() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);

    // Removing vertex 0 breaks the triangle cycle
    assert!(is_feedback_vertex_set(&graph, &[true, false, false]));
    // Removing vertex 1 breaks the cycle
    assert!(is_feedback_vertex_set(&graph, &[false, true, false]));
    // Removing no vertices: cycle remains
    assert!(!is_feedback_vertex_set(&graph, &[false, false, false]));
    // Removing all vertices: trivially valid
    assert!(is_feedback_vertex_set(&graph, &[true, true, true]));
}

#[test]
fn test_direction() {
    let problem = MinimumFeedbackVertexSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_problem_name() {
    assert_eq!(
        <MinimumFeedbackVertexSet<SimpleGraph, i32> as Problem>::NAME,
        "MinimumFeedbackVertexSet"
    );
}

#[test]
fn test_graph_accessor() {
    let problem = MinimumFeedbackVertexSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.num_edges(), 1);
}

#[test]
fn test_weights() {
    let problem = MinimumFeedbackVertexSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![5, 10, 15]);
    assert_eq!(problem.weights(), &[5, 10, 15]);
}

#[test]
fn test_is_valid_solution() {
    // Triangle: 0-1-2-0
    let problem = MinimumFeedbackVertexSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32; 3],
    );
    // Valid: {1} breaks the triangle
    assert!(problem.is_valid_solution(&[0, 1, 0]));
    // Invalid: empty set leaves the triangle
    assert!(!problem.is_valid_solution(&[0, 0, 0]));
}

#[test]
fn test_size_getters() {
    let problem = MinimumFeedbackVertexSet::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32; 4],
    );
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
}

#[test]
fn test_evaluate_valid() {
    let problem = MinimumFeedbackVertexSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32; 3],
    );
    // Remove vertex 0 (weight 1)
    let result = problem.evaluate(&[1, 0, 0]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn test_evaluate_invalid() {
    let problem = MinimumFeedbackVertexSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32; 3],
    );
    // Remove nothing: triangle cycle remains
    let result = problem.evaluate(&[0, 0, 0]);
    assert!(!result.is_valid());
}

#[test]
fn test_evaluate_weighted() {
    let problem = MinimumFeedbackVertexSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![5, 10, 15],
    );
    // Remove vertex 0 (weight 5)
    let result = problem.evaluate(&[1, 0, 0]);
    assert_eq!(result.unwrap(), 5);
    // Remove vertex 2 (weight 15)
    let result = problem.evaluate(&[0, 0, 1]);
    assert_eq!(result.unwrap(), 15);

    // Optimal is to remove lightest vertex
    let solver = BruteForce::new();
    let best = solver.find_all_best(&problem);
    assert!(best.contains(&vec![1, 0, 0])); // Remove vertex 0 (weight 5)
}

#[test]
fn test_feedback_vertex_set_disconnected() {
    // Two disconnected triangles: {0,1,2} and {3,4,5}
    let problem = MinimumFeedbackVertexSet::new(
        SimpleGraph::new(6, vec![(0, 1), (1, 2), (0, 2), (3, 4), (4, 5), (3, 5)]),
        vec![1i32; 6],
    );

    // Must break both cycles — need at least 2 vertices
    assert!(!problem.is_valid_solution(&[1, 0, 0, 0, 0, 0]));
    assert!(problem.is_valid_solution(&[1, 0, 0, 1, 0, 0]));

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert!(solutions.iter().all(|s| s.iter().sum::<usize>() == 2));
}

#[test]
#[should_panic(expected = "selected length must match num_vertices")]
fn test_is_feedback_vertex_set_wrong_len() {
    is_feedback_vertex_set(&SimpleGraph::new(3, vec![(0, 1)]), &[true, false]);
}

#[test]
fn test_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumFeedbackVertexSet::new(graph.clone(), vec![1, 2, 3]);
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.weights().to_vec(), vec![1, 2, 3]);
}

#[test]
fn test_edges() {
    let problem =
        MinimumFeedbackVertexSet::new(SimpleGraph::new(4, vec![(0, 1), (2, 3)]), vec![1i32; 4]);
    let edges = problem.graph().edges();
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_has_edge() {
    let problem =
        MinimumFeedbackVertexSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
    assert!(problem.graph().has_edge(0, 1));
    assert!(problem.graph().has_edge(1, 0)); // Undirected
    assert!(problem.graph().has_edge(1, 2));
    assert!(!problem.graph().has_edge(0, 2));
}
