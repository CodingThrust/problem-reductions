use super::*;
use crate::solvers::BruteForce;
use crate::types::SolutionSize;

#[test]
fn test_maximal_is_creation() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
}

#[test]
fn test_maximal_is_with_weights() {
    let problem = MaximalIS::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![1, 2, 3]);
    assert_eq!(problem.weights(), vec![1, 2, 3]);
    assert!(problem.is_weighted());
}

#[test]
fn test_maximal_is_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximalIS::<SimpleGraph, i32>::from_graph(graph, vec![1, 2, 3]);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.weights(), vec![1, 2, 3]);
}

#[test]
fn test_maximal_is_from_graph_unit_weights() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximalIS::<SimpleGraph, i32>::from_graph_unit_weights(graph);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.weights(), vec![1, 1, 1]);
}

#[test]
fn test_is_independent() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

    assert!(problem.is_independent(&[1, 0, 1]));
    assert!(problem.is_independent(&[0, 1, 0]));
    assert!(!problem.is_independent(&[1, 1, 0]));
}

#[test]
fn test_is_maximal() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

    // {0, 2} is maximal (cannot add 1)
    assert!(problem.is_maximal(&[1, 0, 1]));

    // {1} is maximal (cannot add 0 or 2)
    assert!(problem.is_maximal(&[0, 1, 0]));

    // {0} is not maximal (can add 2)
    assert!(!problem.is_maximal(&[1, 0, 0]));

    // {} is not maximal (can add any vertex)
    assert!(!problem.is_maximal(&[0, 0, 0]));
}

#[test]
fn test_evaluate() {
    use crate::traits::Problem;

    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

    // Maximal: {0, 2}
    assert_eq!(problem.evaluate(&[1, 0, 1]), SolutionSize::Valid(2));

    // Maximal: {1}
    assert_eq!(problem.evaluate(&[0, 1, 0]), SolutionSize::Valid(1));

    // Not maximal: {0} - returns Invalid
    assert_eq!(problem.evaluate(&[1, 0, 0]), SolutionSize::Invalid);
}

#[test]
fn test_brute_force_path() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Largest maximal IS is {0, 2} with size 2
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 0, 1]);
}

#[test]
fn test_brute_force_triangle() {
    use crate::traits::Problem;

    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // All maximal IS have size 1 (any single vertex)
    assert_eq!(solutions.len(), 3);
    for sol in &solutions {
        assert_eq!(sol.iter().sum::<usize>(), 1);
        // Maximal IS should evaluate to Valid(1)
        assert_eq!(problem.evaluate(sol), SolutionSize::Valid(1));
    }
}

#[test]
fn test_is_maximal_independent_set_function() {
    let edges = vec![(0, 1), (1, 2)];

    assert!(is_maximal_independent_set(3, &edges, &[true, false, true]));
    assert!(is_maximal_independent_set(3, &edges, &[false, true, false]));
    assert!(!is_maximal_independent_set(
        3,
        &edges,
        &[true, false, false]
    )); // Can add 2
    assert!(!is_maximal_independent_set(3, &edges, &[true, true, false])); // Not independent
}

#[test]
fn test_direction() {
    use crate::traits::OptimizationProblem;
    use crate::types::Direction;

    let problem = MaximalIS::<SimpleGraph, i32>::new(2, vec![(0, 1)]);
    assert_eq!(problem.direction(), Direction::Maximize);
}

#[test]
fn test_empty_graph() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Only maximal IS is all vertices
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 1, 1]);
}

#[test]
fn test_weights() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    let weights = problem.weights();
    assert_eq!(weights, vec![1, 1, 1]); // Unit weights
}

#[test]
fn test_set_weights() {
    let mut problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    problem.set_weights(vec![1, 2, 3]);
    assert_eq!(problem.weights(), vec![1, 2, 3]);
}

#[test]
fn test_is_weighted() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    assert!(!problem.is_weighted()); // Initially uniform
}

#[test]
fn test_is_weighted_empty() {
    let problem = MaximalIS::<SimpleGraph, i32>::with_weights(0, vec![], vec![]);
    assert!(!problem.is_weighted());
}

#[test]
fn test_is_maximal_independent_set_wrong_len() {
    assert!(!is_maximal_independent_set(3, &[(0, 1)], &[true, false]));
}

#[test]
fn test_graph_ref() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.num_edges(), 2);
}

#[test]
fn test_edges() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let edges = problem.edges();
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_has_edge() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    assert!(problem.has_edge(0, 1));
    assert!(problem.has_edge(1, 0)); // Undirected
    assert!(problem.has_edge(1, 2));
    assert!(!problem.has_edge(0, 2));
}

#[test]
fn test_weights_ref() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    assert_eq!(problem.weights_ref(), &vec![1, 1, 1]);
}

#[test]
fn test_weighted_solution() {
    let problem =
        MaximalIS::<SimpleGraph, i32>::with_weights(3, vec![(0, 1), (1, 2)], vec![10, 100, 10]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Should prefer {1} with weight 100 over {0, 2} with weight 20
    // With LargerSizeIsBetter, {1} with 100 > {0, 2} with 20
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0, 1, 0]);
}

#[test]
fn test_maximal_is_problem() {
    use crate::traits::{OptimizationProblem, Problem};
    use crate::types::Direction;

    // Path graph 0-1-2
    let p = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    assert_eq!(p.dims(), vec![2, 2, 2]);
    // Valid maximal IS: {0, 2} - independent and maximal
    assert_eq!(p.evaluate(&[1, 0, 1]), SolutionSize::Valid(2));
    // Not maximal: {0} alone - vertex 2 could be added
    assert_eq!(p.evaluate(&[1, 0, 0]), SolutionSize::Invalid);
    assert_eq!(p.direction(), Direction::Maximize);
}
