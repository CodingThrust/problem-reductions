use super::*;
use crate::models::graph::MaximumIndependentSet;
use crate::solvers::BruteForce;
use crate::topology::{Graph, KingsSubgraph, SimpleGraph};
use crate::types::One;

#[test]
fn test_mis_simple_one_to_kings_one_closed_loop() {
    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![One; 3],
    );
    let result = ReduceTo::<MaximumIndependentSet<KingsSubgraph, One>>::reduce_to(&problem);
    let target = result.target_problem();
    assert!(target.graph().num_vertices() > 3);

    let solver = BruteForce::new();
    let grid_solutions = solver.find_all_best(target);
    assert!(!grid_solutions.is_empty());

    let original_solution = result.extract_solution(&grid_solutions[0]);
    assert_eq!(original_solution.len(), 3);
    let size: usize = original_solution.iter().sum();
    assert_eq!(size, 1, "Max IS in triangle should be 1");
}

#[test]
fn test_mis_simple_one_to_kings_weighted_closed_loop() {
    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![One; 3],
    );
    let result = ReduceTo::<MaximumIndependentSet<KingsSubgraph, i32>>::reduce_to(&problem);
    let target = result.target_problem();
    assert!(target.graph().num_vertices() > 3);

    let solver = BruteForce::new();
    let grid_solutions = solver.find_all_best(target);
    assert!(!grid_solutions.is_empty());

    let original_solution = result.extract_solution(&grid_solutions[0]);
    assert_eq!(original_solution.len(), 3);
    let size: usize = original_solution.iter().sum();
    assert_eq!(size, 1, "Max IS in triangle should be 1");
}
