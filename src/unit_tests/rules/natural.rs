use crate::models::graph::MaximumIndependentSet;
use crate::rules::{ReduceTo, ReductionResult};
use crate::topology::{SimpleGraph, Triangular};

#[test]
fn test_mis_triangular_to_simple_natural() {
    // Create MIS on SimpleGraph, reduce to Triangular, then reduce back via natural edge
    let source = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

    // SimpleGraph → Triangular (explicit reduction via unit disk mapping)
    let to_tri = ReduceTo::<MaximumIndependentSet<Triangular, i32>>::reduce_to(&source);
    let tri_problem = to_tri.target_problem();

    // Triangular → SimpleGraph (natural edge: graph subtype relaxation)
    let to_simple = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(tri_problem);
    let simple_problem = to_simple.target_problem();

    // The relaxed problem should have the same number of vertices
    assert_eq!(simple_problem.num_vertices(), tri_problem.num_vertices());
    assert_eq!(simple_problem.num_edges(), tri_problem.num_edges());

    // Identity solution mapping: a solution on SimpleGraph maps back unchanged
    let config = vec![0; simple_problem.num_vertices()];
    let back = to_simple.extract_solution(&config);
    assert_eq!(back, config);
}
