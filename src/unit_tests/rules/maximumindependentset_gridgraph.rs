use super::*;
use crate::models::graph::MaximumIndependentSet;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;

#[test]
fn test_mis_simple_to_grid_closed_loop() {
    // Triangle graph: 3 vertices, 3 edges
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let result =
        ReduceTo::<MaximumIndependentSet<GridGraph<i32>, i32>>::reduce_to(&problem);
    let target = result.target_problem();

    // The grid graph should have more vertices than the original
    assert!(target.num_vertices() > 3);

    // Find best solution on the grid graph using brute force
    let solver = BruteForce::new();
    let grid_solutions = solver.find_all_best(target);
    assert!(!grid_solutions.is_empty());

    // Map solution back
    let original_solution = result.extract_solution(&grid_solutions[0]);
    assert_eq!(original_solution.len(), 3);

    // For a triangle, MIS size is 1
    let size: usize = original_solution.iter().sum();
    assert_eq!(size, 1, "Max IS in triangle should be 1");
}

#[test]
fn test_mis_simple_to_grid_path_graph() {
    // Path graph: 0-1-2
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let result =
        ReduceTo::<MaximumIndependentSet<GridGraph<i32>, i32>>::reduce_to(&problem);
    let target = result.target_problem();

    let solver = BruteForce::new();
    let grid_solutions = solver.find_all_best(target);
    assert!(!grid_solutions.is_empty());

    let original_solution = result.extract_solution(&grid_solutions[0]);

    // Path of 3 vertices has MIS size 2 (vertices 0 and 2)
    let size: usize = original_solution.iter().sum();
    assert_eq!(size, 2, "Max IS in path should be 2");
}
