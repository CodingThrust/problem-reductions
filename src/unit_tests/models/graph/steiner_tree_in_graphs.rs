use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;

#[test]
fn test_steiner_tree_creation() {
    // Path graph: 0-1-2-3
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = SteinerTreeInGraphs::new(graph, vec![0, 3], vec![1i32, 2, 3]);
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 3);
    assert_eq!(problem.terminals(), &[0, 3]);
    assert_eq!(problem.dims().len(), 3);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.num_terminals(), 2);
}

#[test]
fn test_steiner_tree_evaluation() {
    // Triangle graph: 0-1, 1-2, 0-2, with terminal {0, 2}
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = SteinerTreeInGraphs::new(graph, vec![0, 2], vec![3i32, 4, 1]);

    // Select edge 0-2 (weight 1): valid, connects terminals directly
    let config_direct = vec![0, 0, 1];
    let result = problem.evaluate(&config_direct);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 1);

    // Select edges 0-1 and 1-2 (weights 3+4=7): valid, connects via vertex 1
    let config_via = vec![1, 1, 0];
    let result = problem.evaluate(&config_via);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 7);

    // Select only edge 0-1: invalid (terminal 2 not reached)
    let config_invalid = vec![1, 0, 0];
    let result = problem.evaluate(&config_invalid);
    assert!(!result.is_valid());

    // Select no edges: invalid
    let config_empty = vec![0, 0, 0];
    let result = problem.evaluate(&config_empty);
    assert!(!result.is_valid());
}

#[test]
fn test_steiner_tree_direction() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = SteinerTreeInGraphs::new(graph, vec![0, 2], vec![1i32; 2]);
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_steiner_tree_solver() {
    // Diamond graph:
    //     1
    //    / \
    //   0   3
    //    \ /
    //     2
    // Edges: 0-1(w=2), 0-2(w=1), 1-3(w=2), 2-3(w=1)
    // Terminals: {0, 3}
    // Optimal path: 0-2-3 with weight 1+1=2
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]);
    let problem = SteinerTreeInGraphs::new(graph, vec![0, 3], vec![2, 1, 2, 1]);

    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).unwrap();
    let value = problem.evaluate(&solution);
    assert!(value.is_valid());
    assert_eq!(value.unwrap(), 2);
    // Should select edges 0-2 and 2-3
    assert_eq!(solution, vec![0, 1, 0, 1]);
}

#[test]
fn test_steiner_tree_with_steiner_vertices() {
    // Star graph: center vertex 1 connected to 0, 2, 3
    // Edges: 0-1(w=1), 1-2(w=1), 1-3(w=1)
    // Terminals: {0, 2, 3}
    // Optimal: use vertex 1 as Steiner vertex, select all 3 edges, weight = 3
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (1, 3)]);
    let problem = SteinerTreeInGraphs::new(graph, vec![0, 2, 3], vec![1i32; 3]);

    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).unwrap();
    let value = problem.evaluate(&solution);
    assert!(value.is_valid());
    assert_eq!(value.unwrap(), 3);
    assert_eq!(solution, vec![1, 1, 1]);
}

#[test]
fn test_steiner_tree_is_valid_solution() {
    // Path graph: 0-1-2
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = SteinerTreeInGraphs::new(graph, vec![0, 2], vec![1i32; 2]);

    // Valid: both edges selected
    assert!(problem.is_valid_solution(&[1, 1]));
    // Invalid: only first edge
    assert!(!problem.is_valid_solution(&[1, 0]));
    // Invalid: only second edge
    assert!(!problem.is_valid_solution(&[0, 1]));
    // Invalid: no edges
    assert!(!problem.is_valid_solution(&[0, 0]));
}

#[test]
fn test_steiner_tree_size_getters() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let problem = SteinerTreeInGraphs::new(graph, vec![0, 2, 4], vec![1i32; 4]);
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 4);
    assert_eq!(problem.num_terminals(), 3);
}

#[test]
fn test_steiner_tree_problem_name() {
    assert_eq!(
        <SteinerTreeInGraphs<SimpleGraph, i32> as Problem>::NAME,
        "SteinerTreeInGraphs"
    );
}

#[test]
fn test_steiner_tree_serialization() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = SteinerTreeInGraphs::new(graph, vec![0, 2], vec![1i32; 2]);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: SteinerTreeInGraphs<SimpleGraph, i32> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.graph().num_vertices(), 3);
    assert_eq!(deserialized.terminals(), &[0, 2]);
    assert_eq!(deserialized.num_edges(), 2);
}

#[test]
fn test_steiner_tree_single_terminal() {
    // Single terminal: any config (including empty) is valid
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = SteinerTreeInGraphs::new(graph, vec![1], vec![1i32; 2]);

    // No edges needed for a single terminal
    let result = problem.evaluate(&[0, 0]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_steiner_tree_all_vertices_terminal() {
    // When all vertices are terminals, it degenerates to spanning tree
    // Path: 0-1-2
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = SteinerTreeInGraphs::new(graph, vec![0, 1, 2], vec![1i32; 2]);

    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).unwrap();
    let value = problem.evaluate(&solution);
    assert!(value.is_valid());
    assert_eq!(value.unwrap(), 2);
}

#[test]
fn test_steiner_tree_edges_accessor() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = SteinerTreeInGraphs::new(graph, vec![0, 2], vec![5i32, 10]);
    let edges = problem.edges();
    assert_eq!(edges.len(), 2);
    assert_eq!(edges[0].2, 5);
    assert_eq!(edges[1].2, 10);
}

#[test]
fn test_steiner_tree_weights_management() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let mut problem = SteinerTreeInGraphs::new(graph, vec![0, 2], vec![1i32; 2]);
    assert!(problem.is_weighted());
    assert_eq!(problem.weights(), vec![1, 1]);

    problem.set_weights(vec![5, 10]);
    assert_eq!(problem.weights(), vec![5, 10]);
}

#[test]
fn test_steiner_tree_example_from_issue() {
    // Example from issue #255:
    // Graph with 8 vertices {0,1,2,3,4,5,6,7} and 12 edges
    // Terminals R = {0, 3, 5, 7}
    let graph = SimpleGraph::new(
        8,
        vec![
            (0, 1), // w=2, idx=0
            (0, 2), // w=3, idx=1
            (1, 2), // w=1, idx=2
            (1, 3), // w=4, idx=3
            (2, 4), // w=2, idx=4
            (3, 4), // w=3, idx=5
            (3, 5), // w=5, idx=6
            (4, 5), // w=1, idx=7
            (4, 6), // w=2, idx=8
            (5, 6), // w=3, idx=9
            (5, 7), // w=4, idx=10
            (6, 7), // w=1, idx=11
        ],
    );
    let weights = vec![2, 3, 1, 4, 2, 3, 5, 1, 2, 3, 4, 1];
    let problem =
        SteinerTreeInGraphs::new(graph, vec![0, 3, 5, 7], weights);

    // Verify the claimed optimal solution from the issue:
    // Edges: {0,1}(2) + {1,2}(1) + {2,4}(2) + {3,4}(3) + {4,5}(1) + {4,6}(2) + {6,7}(1) = 12
    let config = vec![1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1];
    let result = problem.evaluate(&config);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 12);
}
