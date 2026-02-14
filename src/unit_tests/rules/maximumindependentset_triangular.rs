use super::*;
use crate::models::graph::MaximumIndependentSet;
use crate::topology::{SimpleGraph, Triangular};

#[test]
fn test_mis_simple_to_triangular_closed_loop() {
    // Path graph: 0-1-2
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let result = ReduceTo::<MaximumIndependentSet<Triangular, i32>>::reduce_to(&problem);
    let target = result.target_problem();

    // The triangular graph should have more vertices than the original
    assert!(target.num_vertices() > 3);

    // Map a trivial zero solution back to verify dimensions
    let zero_config = vec![0; target.num_vertices()];
    let original_solution = result.extract_solution(&zero_config);
    assert_eq!(original_solution.len(), 3);
}
