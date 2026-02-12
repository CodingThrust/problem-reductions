use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

#[test]
fn test_independent_set_creation() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.dims().len(), 4);
}

#[test]
fn test_independent_set_with_weights() {
    let problem =
        MaximumIndependentSet::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![1, 2, 3]);
    assert_eq!(problem.weights(), vec![1, 2, 3]);
    assert!(problem.is_weighted());
}

#[test]
fn test_independent_set_unweighted() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    assert!(!problem.is_weighted());
}

#[test]
fn test_has_edge() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    assert!(problem.has_edge(0, 1));
    assert!(problem.has_edge(1, 0)); // Undirected
    assert!(problem.has_edge(1, 2));
    assert!(!problem.has_edge(0, 2));
}

#[test]
fn test_evaluate_valid() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (2, 3)]);

    // Valid: select 0 and 2 (not adjacent)
    assert_eq!(problem.evaluate(&[1, 0, 1, 0]), SolutionSize::Valid(2));

    // Valid: select 1 and 3 (not adjacent)
    assert_eq!(problem.evaluate(&[0, 1, 0, 1]), SolutionSize::Valid(2));
}

#[test]
fn test_evaluate_invalid() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (2, 3)]);

    // Invalid: 0 and 1 are adjacent -> returns Invalid
    assert_eq!(problem.evaluate(&[1, 1, 0, 0]), SolutionSize::Invalid);

    // Invalid: 2 and 3 are adjacent -> returns Invalid
    assert_eq!(problem.evaluate(&[0, 0, 1, 1]), SolutionSize::Invalid);
}

#[test]
fn test_evaluate_empty() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    assert_eq!(problem.evaluate(&[0, 0, 0]), SolutionSize::Valid(0));
}

#[test]
fn test_weighted_evaluate() {
    let problem =
        MaximumIndependentSet::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![10, 20, 30]);

    // Select vertex 2 (weight 30)
    assert_eq!(problem.evaluate(&[0, 0, 1]), SolutionSize::Valid(30));

    // Select vertices 0 and 2 (weights 10 + 30 = 40)
    assert_eq!(problem.evaluate(&[1, 0, 1]), SolutionSize::Valid(40));
}

#[test]
fn test_brute_force_triangle() {
    // Triangle graph: maximum IS has size 1
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // All solutions should have exactly 1 vertex selected
    assert_eq!(solutions.len(), 3); // Three equivalent solutions
    for sol in &solutions {
        assert_eq!(sol.iter().sum::<usize>(), 1);
    }
}

#[test]
fn test_brute_force_path() {
    // Path graph 0-1-2-3: maximum IS = {0,2} or {1,3} or {0,3}
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Maximum size is 2
    for sol in &solutions {
        let size: usize = sol.iter().sum();
        assert_eq!(size, 2);
        // Verify it's valid (evaluate returns Valid)
        let eval = problem.evaluate(sol);
        assert!(eval.is_valid());
    }
}

#[test]
fn test_brute_force_weighted() {
    // Graph with weights: vertex 1 has high weight but is connected to both 0 and 2
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::with_weights(
        3,
        vec![(0, 1), (1, 2)],
        vec![1, 100, 1],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    assert_eq!(solutions.len(), 1);
    // Should select vertex 1 (weight 100) over vertices 0+2 (weight 2)
    assert_eq!(solutions[0], vec![0, 1, 0]);
}

#[test]
fn test_brute_force_weighted_f64() {
    let problem = MaximumIndependentSet::<SimpleGraph, f64>::with_weights(
        3,
        vec![(0, 1), (1, 2)],
        vec![0.5, 2.0, 0.75],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    assert_eq!(solutions, vec![vec![0, 1, 0]]);
    assert_eq!(problem.evaluate(&solutions[0]), SolutionSize::Valid(2.0));
}

#[test]
fn test_is_independent_set_function() {
    assert!(is_independent_set(3, &[(0, 1)], &[true, false, true]));
    assert!(is_independent_set(3, &[(0, 1)], &[false, true, true]));
    assert!(!is_independent_set(3, &[(0, 1)], &[true, true, false]));
    assert!(is_independent_set(
        3,
        &[(0, 1), (1, 2)],
        &[true, false, true]
    ));
    assert!(!is_independent_set(
        3,
        &[(0, 1), (1, 2)],
        &[false, true, true]
    ));
}

#[test]
fn test_direction() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    assert_eq!(problem.direction(), Direction::Maximize);
}

#[test]
fn test_edges() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (2, 3)]);
    let edges = problem.edges();
    assert_eq!(edges.len(), 2);
    assert!(edges.contains(&(0, 1)) || edges.contains(&(1, 0)));
    assert!(edges.contains(&(2, 3)) || edges.contains(&(3, 2)));
}

#[test]
fn test_set_weights() {
    let mut problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    problem.set_weights(vec![5, 10, 15]);
    assert_eq!(problem.weights(), vec![5, 10, 15]);
}

#[test]
fn test_empty_graph() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    assert_eq!(solutions.len(), 1);
    // All vertices can be selected
    assert_eq!(solutions[0], vec![1, 1, 1]);
}

#[test]
fn test_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem =
        MaximumIndependentSet::<SimpleGraph, i32>::from_graph(graph.clone(), vec![1, 2, 3]);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.weights(), vec![1, 2, 3]);
}

#[test]
fn test_from_graph_unit_weights() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::from_graph_unit_weights(graph);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.weights(), vec![1, 1, 1]);
}

#[test]
fn test_graph_accessor() {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.num_edges(), 1);
}

#[test]
fn test_weights_ref() {
    let problem =
        MaximumIndependentSet::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
    assert_eq!(problem.weights_ref(), &vec![5, 10, 15]);
}

#[test]
fn test_mis_problem_trait() {
    // Triangle graph with explicit weights
    let p = MaximumIndependentSet::<SimpleGraph, i32>::with_weights(
        3,
        vec![(0, 1), (1, 2), (0, 2)],
        vec![1, 1, 1],
    );
    assert_eq!(p.dims(), vec![2, 2, 2]);
    // Valid IS: select vertex 0 only
    assert_eq!(p.evaluate(&[1, 0, 0]), SolutionSize::Valid(1));
    // Invalid IS: select adjacent 0,1 -> should return Invalid
    assert_eq!(p.evaluate(&[1, 1, 0]), SolutionSize::Invalid);
    assert_eq!(p.direction(), Direction::Maximize);
}

#[test]
fn test_mis_unweighted() {
    // Unweighted MIS uses i32 weight type with unit weights
    let p = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    assert_eq!(p.dims(), vec![2, 2, 2]);
    assert_eq!(p.evaluate(&[1, 0, 0]), SolutionSize::Valid(1));
    assert_eq!(p.evaluate(&[0, 0, 0]), SolutionSize::Valid(0));
}

#[test]
fn test_problem_name() {
    assert_eq!(
        <MaximumIndependentSet<SimpleGraph, i32> as Problem>::NAME,
        "MaximumIndependentSet"
    );
}
