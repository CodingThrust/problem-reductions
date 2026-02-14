use crate::models::graph::MaximumIndependentSet;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::ILPSolver;
use crate::topology::{SimpleGraph, Triangular};
use crate::traits::Problem;

#[test]
fn test_mis_triangular_to_simple_closed_loop() {
    // Petersen graph: 10 vertices, 15 edges, max IS = 4
    let source = MaximumIndependentSet::<SimpleGraph, i32>::new(
        10,
        vec![
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 0), // outer cycle
            (5, 7), (7, 9), (9, 6), (6, 8), (8, 5), // inner pentagram
            (0, 5), (1, 6), (2, 7), (3, 8), (4, 9), // spokes
        ],
    );

    // SimpleGraph → Triangular (unit disk mapping)
    let to_tri = ReduceTo::<MaximumIndependentSet<Triangular, i32>>::reduce_to(&source);
    let tri_problem = to_tri.target_problem();

    // Triangular → SimpleGraph (natural edge: graph subtype relaxation)
    let to_simple = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(tri_problem);
    let simple_problem = to_simple.target_problem();

    // Graph structure is preserved by identity cast
    assert_eq!(simple_problem.num_vertices(), tri_problem.num_vertices());
    assert_eq!(simple_problem.num_edges(), tri_problem.num_edges());

    // Solve with ILP on the relaxed SimpleGraph problem
    let solver = ILPSolver::new();
    let solution = solver.solve_reduced(simple_problem).expect("ILP should find a solution");

    // Identity mapping: solution is unchanged
    let extracted = to_simple.extract_solution(&solution);
    assert_eq!(extracted, solution);

    // Extracted solution is valid on the Triangular problem
    let metric = tri_problem.evaluate(&extracted);
    assert!(metric.is_valid());

    // Map back through the full chain to the original Petersen graph
    let original_solution = to_tri.extract_solution(&extracted);
    let original_metric = source.evaluate(&original_solution);
    assert!(original_metric.is_valid());
    // Petersen graph max IS = 4
    assert_eq!(original_solution.iter().sum::<usize>(), 4);
}
