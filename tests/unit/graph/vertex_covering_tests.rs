//! Unit tests for the Vertex Covering problem.

use problemreductions::models::graph::{is_vertex_cover, IndependentSet, VertexCovering};
use problemreductions::prelude::*;

#[test]
fn test_vertex_cover_creation() {
    let problem = VertexCovering::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(problem.num_flavors(), 2);
}

#[test]
fn test_vertex_cover_with_weights() {
    let problem = VertexCovering::with_weights(3, vec![(0, 1)], vec![1, 2, 3]);
    assert_eq!(problem.weights(), vec![1, 2, 3]);
    assert!(problem.is_weighted());
}

#[test]
fn test_solution_size_valid() {
    let problem = VertexCovering::<i32>::new(3, vec![(0, 1), (1, 2)]);

    // Valid: select vertex 1 (covers both edges)
    let sol = problem.solution_size(&[0, 1, 0]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 1);

    // Valid: select all vertices
    let sol = problem.solution_size(&[1, 1, 1]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 3);
}

#[test]
fn test_solution_size_invalid() {
    let problem = VertexCovering::<i32>::new(3, vec![(0, 1), (1, 2)]);

    // Invalid: no vertex selected
    let sol = problem.solution_size(&[0, 0, 0]);
    assert!(!sol.is_valid);

    // Invalid: only vertex 0 selected (edge 1-2 not covered)
    let sol = problem.solution_size(&[1, 0, 0]);
    assert!(!sol.is_valid);
}

#[test]
fn test_brute_force_path() {
    // Path graph 0-1-2: minimum vertex cover is {1}
    let problem = VertexCovering::<i32>::new(3, vec![(0, 1), (1, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0, 1, 0]);
}

#[test]
fn test_brute_force_triangle() {
    // Triangle: minimum vertex cover has size 2
    let problem = VertexCovering::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // There are 3 minimum covers of size 2
    assert_eq!(solutions.len(), 3);
    for sol in &solutions {
        assert_eq!(sol.iter().sum::<usize>(), 2);
        assert!(problem.solution_size(sol).is_valid);
    }
}

#[test]
fn test_brute_force_weighted() {
    // Weighted: prefer selecting low-weight vertices
    let problem = VertexCovering::with_weights(3, vec![(0, 1), (1, 2)], vec![100, 1, 100]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    assert_eq!(solutions.len(), 1);
    // Should select vertex 1 (weight 1) instead of 0 and 2 (total 200)
    assert_eq!(solutions[0], vec![0, 1, 0]);
}

#[test]
fn test_is_vertex_cover_function() {
    assert!(is_vertex_cover(3, &[(0, 1), (1, 2)], &[false, true, false]));
    assert!(is_vertex_cover(3, &[(0, 1), (1, 2)], &[true, false, true]));
    assert!(!is_vertex_cover(
        3,
        &[(0, 1), (1, 2)],
        &[true, false, false]
    ));
    assert!(!is_vertex_cover(
        3,
        &[(0, 1), (1, 2)],
        &[false, false, false]
    ));
}

#[test]
fn test_constraints() {
    let problem = VertexCovering::<i32>::new(3, vec![(0, 1), (1, 2)]);
    let constraints = problem.constraints();
    assert_eq!(constraints.len(), 2);
}

#[test]
fn test_energy_mode() {
    let problem = VertexCovering::<i32>::new(3, vec![(0, 1)]);
    assert!(problem.energy_mode().is_minimization());
}

#[test]
fn test_empty_graph() {
    let problem = VertexCovering::<i32>::new(3, vec![]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // No edges means empty cover is valid and optimal
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0, 0, 0]);
}

#[test]
fn test_single_edge() {
    let problem = VertexCovering::<i32>::new(2, vec![(0, 1)]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Either vertex covers the single edge
    assert_eq!(solutions.len(), 2);
}

#[test]
fn test_is_satisfied() {
    let problem = VertexCovering::<i32>::new(3, vec![(0, 1), (1, 2)]);

    assert!(problem.is_satisfied(&[0, 1, 0])); // Valid cover
    assert!(problem.is_satisfied(&[1, 0, 1])); // Valid cover
    assert!(!problem.is_satisfied(&[1, 0, 0])); // Edge 1-2 uncovered
    assert!(!problem.is_satisfied(&[0, 0, 1])); // Edge 0-1 uncovered
}

#[test]
fn test_complement_relationship() {
    // For a graph, if S is an independent set, then V\S is a vertex cover
    let edges = vec![(0, 1), (1, 2), (2, 3)];
    let is_problem = IndependentSet::<i32>::new(4, edges.clone());
    let vc_problem = VertexCovering::<i32>::new(4, edges);

    let solver = BruteForce::new();

    let is_solutions = solver.find_best(&is_problem);
    for is_sol in &is_solutions {
        // Complement should be a valid vertex cover
        let vc_config: Vec<usize> = is_sol.iter().map(|&x| 1 - x).collect();
        assert!(vc_problem.solution_size(&vc_config).is_valid);
    }
}

#[test]
fn test_objectives() {
    let problem = VertexCovering::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
    let objectives = problem.objectives();
    assert_eq!(objectives.len(), 3);
}

#[test]
fn test_set_weights() {
    let mut problem = VertexCovering::<i32>::new(3, vec![(0, 1)]);
    assert!(!problem.is_weighted()); // Initially uniform
    problem.set_weights(vec![1, 2, 3]);
    assert!(problem.is_weighted());
    assert_eq!(problem.weights(), vec![1, 2, 3]);
}

#[test]
fn test_is_weighted_empty() {
    let problem = VertexCovering::<i32>::new(0, vec![]);
    assert!(!problem.is_weighted());
}

#[test]
fn test_is_vertex_cover_wrong_len() {
    // Wrong length should return false
    assert!(!is_vertex_cover(3, &[(0, 1)], &[true, false]));
}
