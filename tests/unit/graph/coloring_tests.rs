//! Unit tests for the Graph Coloring problem.

use problemreductions::models::graph::{is_valid_coloring, Coloring};
use problemreductions::prelude::*;

#[test]
fn test_coloring_creation() {
    let problem = Coloring::new(4, 3, vec![(0, 1), (1, 2), (2, 3)]);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.num_colors(), 3);
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(problem.num_flavors(), 3);
}

#[test]
fn test_solution_size_valid() {
    let problem = Coloring::new(3, 3, vec![(0, 1), (1, 2)]);

    // Valid: different colors on adjacent vertices
    let sol = problem.solution_size(&[0, 1, 0]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 0);

    let sol = problem.solution_size(&[0, 1, 2]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 0);
}

#[test]
fn test_solution_size_invalid() {
    let problem = Coloring::new(3, 3, vec![(0, 1), (1, 2)]);

    // Invalid: adjacent vertices have same color
    let sol = problem.solution_size(&[0, 0, 1]);
    assert!(!sol.is_valid);
    assert_eq!(sol.size, 1); // 1 conflict

    let sol = problem.solution_size(&[0, 0, 0]);
    assert!(!sol.is_valid);
    assert_eq!(sol.size, 2); // 2 conflicts
}

#[test]
fn test_brute_force_path() {
    // Path graph can be 2-colored
    let problem = Coloring::new(4, 2, vec![(0, 1), (1, 2), (2, 3)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // All solutions should be valid (0 conflicts)
    for sol in &solutions {
        assert!(problem.solution_size(sol).is_valid);
    }
}

#[test]
fn test_brute_force_triangle() {
    // Triangle needs 3 colors
    let problem = Coloring::new(3, 3, vec![(0, 1), (1, 2), (0, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    for sol in &solutions {
        assert!(problem.solution_size(sol).is_valid);
        // All three vertices have different colors
        assert_ne!(sol[0], sol[1]);
        assert_ne!(sol[1], sol[2]);
        assert_ne!(sol[0], sol[2]);
    }
}

#[test]
fn test_triangle_2_colors() {
    // Triangle cannot be 2-colored
    let problem = Coloring::new(3, 2, vec![(0, 1), (1, 2), (0, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Best we can do is 1 conflict
    for sol in &solutions {
        assert!(!problem.solution_size(sol).is_valid);
        assert_eq!(problem.solution_size(sol).size, 1);
    }
}

#[test]
fn test_constraints() {
    let problem = Coloring::new(3, 2, vec![(0, 1), (1, 2)]);
    let constraints = problem.constraints();
    assert_eq!(constraints.len(), 2); // One per edge
}

#[test]
fn test_energy_mode() {
    let problem = Coloring::new(2, 2, vec![(0, 1)]);
    assert!(problem.energy_mode().is_minimization());
}

#[test]
fn test_is_valid_coloring_function() {
    let edges = vec![(0, 1), (1, 2)];

    assert!(is_valid_coloring(3, &edges, &[0, 1, 0], 2));
    assert!(is_valid_coloring(3, &edges, &[0, 1, 2], 3));
    assert!(!is_valid_coloring(3, &edges, &[0, 0, 1], 2)); // 0-1 conflict
    assert!(!is_valid_coloring(3, &edges, &[0, 1, 1], 2)); // 1-2 conflict
    assert!(!is_valid_coloring(3, &edges, &[0, 1], 2)); // Wrong length
    assert!(!is_valid_coloring(3, &edges, &[0, 2, 0], 2)); // Color out of range
}

#[test]
fn test_empty_graph() {
    let problem = Coloring::new(3, 1, vec![]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Any coloring is valid when there are no edges
    assert!(problem.solution_size(&solutions[0]).is_valid);
}

#[test]
fn test_complete_graph_k4() {
    // K4 needs 4 colors
    let problem = Coloring::new(4, 4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    for sol in &solutions {
        assert!(problem.solution_size(sol).is_valid);
    }
}

#[test]
fn test_is_satisfied() {
    let problem = Coloring::new(3, 3, vec![(0, 1), (1, 2)]);

    assert!(problem.is_satisfied(&[0, 1, 0]));
    assert!(problem.is_satisfied(&[0, 1, 2]));
    assert!(!problem.is_satisfied(&[0, 0, 1]));
}

#[test]
fn test_problem_size() {
    let problem = Coloring::new(5, 3, vec![(0, 1), (1, 2)]);
    let size = problem.problem_size();
    assert_eq!(size.get("num_vertices"), Some(5));
    assert_eq!(size.get("num_edges"), Some(2));
    assert_eq!(size.get("num_colors"), Some(3));
}

#[test]
fn test_csp_methods() {
    let problem = Coloring::new(3, 2, vec![(0, 1)]);

    // Coloring has no objectives (pure CSP)
    let objectives = problem.objectives();
    assert!(objectives.is_empty());

    // Coloring has no weights
    let weights: Vec<i32> = problem.weights();
    assert!(weights.is_empty());

    // is_weighted should return false
    assert!(!problem.is_weighted());
}

#[test]
fn test_set_weights() {
    let mut problem = Coloring::new(3, 2, vec![(0, 1)]);
    // set_weights does nothing for Coloring
    problem.set_weights(vec![1, 2, 3]);
    assert!(!problem.is_weighted());
}
