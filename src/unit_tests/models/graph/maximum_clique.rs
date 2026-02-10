use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_clique_creation() {
    let problem = MaximumClique::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(problem.num_flavors(), 2);
}

#[test]
fn test_clique_with_weights() {
    let problem =
        MaximumClique::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![1, 2, 3]);
    assert_eq!(problem.weights(), vec![1, 2, 3]);
    assert!(problem.is_weighted());
}

#[test]
fn test_clique_unweighted() {
    let problem = MaximumClique::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    assert!(!problem.is_weighted());
}

#[test]
fn test_has_edge() {
    let problem = MaximumClique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    assert!(problem.has_edge(0, 1));
    assert!(problem.has_edge(1, 0)); // Undirected
    assert!(problem.has_edge(1, 2));
    assert!(!problem.has_edge(0, 2));
}

#[test]
fn test_solution_size_valid() {
    // Complete graph K3 (triangle)
    let problem = MaximumClique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);

    // Valid: all three form a clique
    let sol = problem.solution_size(&[1, 1, 1]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 3);

    // Valid: any pair
    let sol = problem.solution_size(&[1, 1, 0]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 2);
}

#[test]
fn test_solution_size_invalid() {
    // Path graph: 0-1-2 (no edge between 0 and 2)
    let problem = MaximumClique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

    // Invalid: 0 and 2 are not adjacent
    let sol = problem.solution_size(&[1, 0, 1]);
    assert!(!sol.is_valid);
    assert_eq!(sol.size, 2);

    // Invalid: all three selected but not a clique
    let sol = problem.solution_size(&[1, 1, 1]);
    assert!(!sol.is_valid);
}

#[test]
fn test_solution_size_empty() {
    let problem = MaximumClique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let sol = problem.solution_size(&[0, 0, 0]);
    assert!(sol.is_valid); // Empty set is a valid clique
    assert_eq!(sol.size, 0);
}

#[test]
fn test_weighted_solution() {
    let problem =
        MaximumClique::<SimpleGraph, i32>::with_weights(3, vec![(0, 1), (1, 2), (0, 2)], vec![10, 20, 30]);

    // Select vertex 2 (weight 30)
    let sol = problem.solution_size(&[0, 0, 1]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 30);

    // Select all three (weights 10 + 20 + 30 = 60)
    let sol = problem.solution_size(&[1, 1, 1]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 60);
}

#[test]
fn test_constraints() {
    // Path graph: 0-1-2 (non-edge between 0 and 2)
    let problem = MaximumClique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let constraints = problem.constraints();
    assert_eq!(constraints.len(), 1); // One constraint for non-edge (0, 2)
}

#[test]
fn test_objectives() {
    let problem =
        MaximumClique::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
    let objectives = problem.objectives();
    assert_eq!(objectives.len(), 3); // One per vertex
}

#[test]
fn test_brute_force_triangle() {
    // Triangle graph (K3): max clique is all 3 vertices
    let problem =
        MaximumClique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 1, 1]);
}

#[test]
fn test_brute_force_path() {
    // Path graph 0-1-2: max clique is any adjacent pair
    let problem = MaximumClique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Maximum size is 2
    for sol in &solutions {
        let size: usize = sol.iter().sum();
        assert_eq!(size, 2);
        // Verify it's valid
        let sol_result = problem.solution_size(sol);
        assert!(sol_result.is_valid);
    }
}

#[test]
fn test_brute_force_weighted() {
    // Path with weights: vertex 1 has high weight
    let problem =
        MaximumClique::<SimpleGraph, i32>::with_weights(3, vec![(0, 1), (1, 2)], vec![1, 100, 1]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Should select {0, 1} (weight 101) or {1, 2} (weight 101)
    assert!(solutions.len() == 2);
    for sol in &solutions {
        assert!(problem.solution_size(sol).is_valid);
        assert_eq!(problem.solution_size(sol).size, 101);
    }
}

#[test]
fn test_is_clique_function() {
    // Triangle
    assert!(is_clique(3, &[(0, 1), (1, 2), (0, 2)], &[true, true, true]));
    assert!(is_clique(3, &[(0, 1), (1, 2), (0, 2)], &[true, true, false]));

    // Path - not all pairs adjacent
    assert!(!is_clique(3, &[(0, 1), (1, 2)], &[true, false, true]));
    assert!(is_clique(3, &[(0, 1), (1, 2)], &[true, true, false])); // Adjacent pair
}

#[test]
fn test_problem_size() {
    let problem = MaximumClique::<SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2), (2, 3)]);
    let size = problem.problem_size();
    assert_eq!(size.get("num_vertices"), Some(5));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_energy_mode() {
    let problem = MaximumClique::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    assert!(problem.energy_mode().is_maximization());
}

#[test]
fn test_edges() {
    let problem = MaximumClique::<SimpleGraph, i32>::new(4, vec![(0, 1), (2, 3)]);
    let edges = problem.edges();
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_set_weights() {
    let mut problem = MaximumClique::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    problem.set_weights(vec![5, 10, 15]);
    assert_eq!(problem.weights(), vec![5, 10, 15]);
}

#[test]
fn test_empty_graph() {
    // No edges means any single vertex is a max clique
    let problem = MaximumClique::<SimpleGraph, i32>::new(3, vec![]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    assert_eq!(solutions.len(), 3);
    // Each solution should have exactly one vertex selected
    for sol in &solutions {
        assert_eq!(sol.iter().sum::<usize>(), 1);
    }
}

#[test]
fn test_is_satisfied() {
    let problem = MaximumClique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

    assert!(problem.is_satisfied(&[1, 1, 0])); // Valid clique
    assert!(problem.is_satisfied(&[0, 1, 1])); // Valid clique
    assert!(!problem.is_satisfied(&[1, 0, 1])); // Invalid: 0-2 not adjacent
}

#[test]
fn test_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumClique::<SimpleGraph, i32>::from_graph(graph.clone(), vec![1, 2, 3]);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.weights(), vec![1, 2, 3]);
}

#[test]
fn test_from_graph_unit_weights() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumClique::<SimpleGraph, i32>::from_graph_unit_weights(graph);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.weights(), vec![1, 1, 1]);
}

#[test]
fn test_graph_accessor() {
    let problem = MaximumClique::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.num_edges(), 1);
}

#[test]
fn test_variant() {
    let variant = MaximumClique::<SimpleGraph, i32>::variant();
    assert_eq!(variant.len(), 2);
    assert_eq!(variant[0], ("graph", "SimpleGraph"));
    assert_eq!(variant[1], ("weight", "i32"));
}

#[test]
fn test_weights_ref() {
    let problem =
        MaximumClique::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
    assert_eq!(problem.weights_ref(), &vec![5, 10, 15]);
}

#[test]
fn test_is_clique_wrong_len() {
    // Wrong length should return false
    assert!(!is_clique(3, &[(0, 1)], &[true, false]));
}

#[test]
fn test_complete_graph() {
    // K4 - complete graph with 4 vertices
    let problem = MaximumClique::<SimpleGraph, i32>::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 1, 1, 1]); // All vertices form a clique
}
