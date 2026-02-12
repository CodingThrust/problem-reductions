use super::*;
use crate::solvers::{BruteForce, Solver};

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
fn test_solution_size() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

    // Maximal: {0, 2}
    let sol = problem.solution_size(&[1, 0, 1]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 2);

    // Maximal: {1}
    let sol = problem.solution_size(&[0, 1, 0]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 1);

    // Not maximal: {0}
    let sol = problem.solution_size(&[1, 0, 0]);
    assert!(!sol.is_valid);
}

#[test]
fn test_brute_force_path() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Largest maximal IS is {0, 2} with size 2
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 0, 1]);
}

#[test]
fn test_brute_force_triangle() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // All maximal IS have size 1 (any single vertex)
    assert_eq!(solutions.len(), 3);
    for sol in &solutions {
        assert_eq!(sol.iter().sum::<usize>(), 1);
        assert!(problem.solution_size(sol).is_valid);
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
fn test_energy_mode() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(2, vec![(0, 1)]);
    assert!(problem.energy_mode().is_maximization());
}

#[test]
fn test_empty_graph() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Only maximal IS is all vertices
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 1, 1]);
}

#[test]
fn test_constraints() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    let constraints = problem.constraints();
    // 1 edge constraint + 3 maximality constraints
    assert_eq!(constraints.len(), 4);
}

#[test]
fn test_is_satisfied() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

    assert!(problem.is_satisfied(&[1, 0, 1])); // Maximal
    assert!(problem.is_satisfied(&[0, 1, 0])); // Maximal
                                               // Note: is_satisfied checks constraints, which may be more complex
}

#[test]
fn test_objectives() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    let objectives = problem.objectives();
    assert_eq!(objectives.len(), 3); // One per vertex
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
fn test_problem_size() {
    let problem = MaximalIS::<SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2), (2, 3)]);
    let size = problem.problem_size();
    assert_eq!(size.get("num_vertices"), Some(5));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_variant() {
    let variant = MaximalIS::<SimpleGraph, i32>::variant();
    assert_eq!(variant.len(), 2);
    assert_eq!(variant[0], ("graph", "SimpleGraph"));
    assert_eq!(variant[1], ("weight", "i32"));
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

    let solutions = solver.find_best(&problem);
    // Should prefer {1} with weight 100 over {0, 2} with weight 20
    // But {0, 2} is also maximal... maximization prefers larger size
    // Actually {0, 2} has size 20 and {1} has size 100
    // With LargerSizeIsBetter, {1} with 100 > {0, 2} with 20
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0, 1, 0]);
}

#[test]
fn test_maximal_is_problem_v2() {
    use crate::traits::{OptimizationProblemV2, ProblemV2};
    use crate::types::Direction;

    // Path graph 0-1-2
    let p = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    assert_eq!(p.dims(), vec![2, 2, 2]);
    // Valid maximal IS: {0, 2} - independent and maximal
    assert_eq!(p.evaluate(&[1, 0, 1]), 2);
    // Not maximal: {0} alone - vertex 2 could be added
    assert_eq!(p.evaluate(&[1, 0, 0]), i32::MIN);
    assert_eq!(p.direction(), Direction::Maximize);
}
