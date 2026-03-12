use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

fn path_graph_5() -> OptimalLinearArrangement<SimpleGraph> {
    // Path: 0-1-2-3-4
    OptimalLinearArrangement::new(SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]))
}

fn triangle_graph() -> OptimalLinearArrangement<SimpleGraph> {
    // Triangle: 0-1, 1-2, 0-2
    OptimalLinearArrangement::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]))
}

#[test]
fn test_optimal_linear_arrangement_creation() {
    let problem = path_graph_5();
    assert_eq!(problem.graph().num_vertices(), 5);
    assert_eq!(problem.graph().num_edges(), 4);
    assert_eq!(problem.dims().len(), 5);
    // Each variable can take values 0..5
    assert_eq!(problem.dims(), vec![5; 5]);
}

#[test]
fn test_optimal_linear_arrangement_evaluation_valid() {
    let problem = triangle_graph();
    // Permutation [0, 1, 2]: cost = |0-1| + |1-2| + |0-2| = 1 + 1 + 2 = 4
    assert_eq!(problem.evaluate(&[0, 1, 2]), SolutionSize::Valid(4));
    // Permutation [1, 0, 2]: cost = |1-0| + |0-2| + |1-2| = 1 + 2 + 1 = 4
    assert_eq!(problem.evaluate(&[1, 0, 2]), SolutionSize::Valid(4));
}

#[test]
fn test_optimal_linear_arrangement_evaluation_invalid() {
    let problem = triangle_graph();
    // Not a permutation: repeated position
    assert_eq!(problem.evaluate(&[0, 0, 1]), SolutionSize::Invalid);
    // Out of range
    assert_eq!(problem.evaluate(&[0, 1, 5]), SolutionSize::Invalid);
    // Wrong length
    assert_eq!(problem.evaluate(&[0, 1]), SolutionSize::Invalid);
}

#[test]
fn test_optimal_linear_arrangement_direction() {
    let problem = triangle_graph();
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_optimal_linear_arrangement_path_graph() {
    // For a path graph with n vertices, the optimal arrangement has cost n-1
    // (just place vertices in path order)
    let problem = path_graph_5();
    // Identity permutation: 0->0, 1->1, 2->2, 3->3, 4->4
    // Cost = |0-1| + |1-2| + |2-3| + |3-4| = 1+1+1+1 = 4
    assert_eq!(problem.evaluate(&[0, 1, 2, 3, 4]), SolutionSize::Valid(4));
}

#[test]
fn test_optimal_linear_arrangement_solver_triangle() {
    let problem = triangle_graph();
    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert!(!solutions.is_empty());
    // All optimal solutions should have cost 4
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), SolutionSize::Valid(4));
    }
}

#[test]
fn test_optimal_linear_arrangement_solver_path() {
    let problem = OptimalLinearArrangement::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let solver = BruteForce::new();
    let best = solver.find_best(&problem).unwrap();
    // Optimal cost for a path of 4 vertices is 3
    assert_eq!(problem.evaluate(&best), SolutionSize::Valid(3));
}

#[test]
fn test_optimal_linear_arrangement_issue_example() {
    // Instance 1 from issue: 6 vertices, 7 edges
    let problem = OptimalLinearArrangement::new(SimpleGraph::new(
        6,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 3), (2, 5)],
    ));
    // Arrangement f(0)=0, f(1)=1, f(2)=2, f(3)=3, f(4)=4, f(5)=5 (identity, 0-indexed)
    // Cost: |0-1|+|1-2|+|2-3|+|3-4|+|4-5|+|0-3|+|2-5| = 1+1+1+1+1+3+3 = 11
    assert_eq!(
        problem.evaluate(&[0, 1, 2, 3, 4, 5]),
        SolutionSize::Valid(11)
    );
}

#[test]
fn test_optimal_linear_arrangement_serialization() {
    let problem = triangle_graph();
    let json = serde_json::to_value(&problem).unwrap();
    let deserialized: OptimalLinearArrangement<SimpleGraph> = serde_json::from_value(json).unwrap();
    assert_eq!(deserialized.graph().num_vertices(), 3);
    assert_eq!(deserialized.graph().num_edges(), 3);
}

#[test]
fn test_problem_name() {
    assert_eq!(
        <OptimalLinearArrangement<SimpleGraph> as Problem>::NAME,
        "OptimalLinearArrangement"
    );
}

#[test]
fn test_is_valid_solution() {
    let problem = triangle_graph();
    assert!(problem.is_valid_solution(&[0, 1, 2]));
    assert!(problem.is_valid_solution(&[2, 0, 1]));
    assert!(!problem.is_valid_solution(&[0, 0, 1]));
    assert!(!problem.is_valid_solution(&[0, 1]));
    assert!(!problem.is_valid_solution(&[0, 1, 3]));
}

#[test]
fn test_total_edge_length() {
    let problem = triangle_graph();
    assert_eq!(problem.total_edge_length(&[0, 1, 2]), Some(4));
    assert_eq!(problem.total_edge_length(&[0, 0, 1]), None);
}

#[test]
fn test_size_getters() {
    let problem = triangle_graph();
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_edges(), 3);
}

#[test]
fn test_new() {
    let problem = OptimalLinearArrangement::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 3);
}

#[test]
fn test_single_vertex() {
    // Graph with one vertex and no edges
    let problem = OptimalLinearArrangement::new(SimpleGraph::new(1, vec![]));
    assert_eq!(problem.dims(), vec![1]);
    assert_eq!(problem.evaluate(&[0]), SolutionSize::Valid(0));
}

#[test]
fn test_complete_graph_k4() {
    // K4: all 6 edges present
    let problem = OptimalLinearArrangement::new(SimpleGraph::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    ));
    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert!(!solutions.is_empty());
    // For K4, optimal cost = 10 (any linear arrangement of K4 has cost 1+2+3+1+2+1 = 10)
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), SolutionSize::Valid(10));
    }
}
