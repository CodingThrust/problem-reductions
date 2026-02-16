use super::*;
use crate::models::graph::MaximumIndependentSet;
use crate::solvers::BruteForce;
use crate::topology::{Graph, KingsSubgraph, SimpleGraph, UnitDiskGraph};

#[test]
fn test_mis_simple_to_grid_closed_loop() {
    // Triangle graph: 3 vertices, 3 edges
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]), vec![1i32; 3]);
    let result = ReduceTo::<MaximumIndependentSet<KingsSubgraph, i32>>::reduce_to(&problem);
    let target = result.target_problem();

    // The grid graph should have more vertices than the original
    assert!(target.graph().num_vertices() > 3);

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
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
    let result = ReduceTo::<MaximumIndependentSet<KingsSubgraph, i32>>::reduce_to(&problem);
    let target = result.target_problem();

    let solver = BruteForce::new();
    let grid_solutions = solver.find_all_best(target);
    assert!(!grid_solutions.is_empty());

    let original_solution = result.extract_solution(&grid_solutions[0]);

    // Path of 3 vertices has MIS size 2 (vertices 0 and 2)
    let size: usize = original_solution.iter().sum();
    assert_eq!(size, 2, "Max IS in path should be 2");
}

#[test]
fn test_mis_unitdisk_to_grid_closed_loop() {
    // Create a UnitDiskGraph: 3 points where 0-1 are close, 2 is far
    let udg = UnitDiskGraph::new(vec![(0.0, 0.0), (0.5, 0.0), (3.0, 0.0)], 1.0);
    // Only edge is 0-1 (distance 0.5 <= 1.0), vertex 2 is isolated
    assert_eq!(udg.num_edges(), 1);

    let problem = MaximumIndependentSet::new(udg, vec![1i32, 1, 1]);
    let result = ReduceTo::<MaximumIndependentSet<KingsSubgraph, i32>>::reduce_to(&problem);
    let target = result.target_problem();

    assert!(target.graph().num_vertices() >= 3);

    let solver = BruteForce::new();
    let grid_solutions = solver.find_all_best(target);
    assert!(!grid_solutions.is_empty());

    let original_solution = result.extract_solution(&grid_solutions[0]);
    assert_eq!(original_solution.len(), 3);

    // MIS should be size 2 (one from {0,1} + vertex 2)
    let size: usize = original_solution.iter().sum();
    assert_eq!(size, 2, "Max IS should be 2");
}
