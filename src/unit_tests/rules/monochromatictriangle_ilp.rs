use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::graph::MonochromaticTriangle;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::ILPSolver;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn k4_instance() -> MonochromaticTriangle<SimpleGraph> {
    MonochromaticTriangle::new(SimpleGraph::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    ))
}

#[test]
fn test_monochromatic_triangle_to_ilp_structure() {
    let problem = k4_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 6);
    assert_eq!(ilp.constraints.len(), 8);
    assert_eq!(ilp.objective, vec![]);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_monochromatic_triangle_to_ilp_constraint_pairs_on_single_triangle() {
    let problem = MonochromaticTriangle::new(SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]));
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 3);
    assert_eq!(ilp.constraints.len(), 2);
    assert_eq!(ilp.constraints[0].rhs, 1.0);
    assert_eq!(ilp.constraints[1].rhs, 2.0);
    assert_eq!(ilp.constraints[0].terms.len(), 3);
    assert_eq!(ilp.constraints[1].terms.len(), 3);
}

#[test]
fn test_monochromatic_triangle_to_ilp_closed_loop() {
    let problem = k4_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("K4 should admit a monochromatic-triangle-free 2-edge-coloring");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted, ilp_solution);
    assert!(problem.evaluate(&extracted));
}

#[test]
fn test_monochromatic_triangle_to_ilp_infeasible_k6() {
    let mut edges = Vec::new();
    for u in 0..6 {
        for v in (u + 1)..6 {
            edges.push((u, v));
        }
    }
    let problem = MonochromaticTriangle::new(SimpleGraph::new(6, edges));
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "K6 should be infeasible by R(3,3)=6"
    );
}

#[test]
fn test_monochromatic_triangle_to_ilp_extract_solution_identity() {
    let problem = k4_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let coloring = vec![0, 0, 1, 1, 0, 1];

    let extracted = reduction.extract_solution(&coloring);

    assert_eq!(extracted, coloring);
    assert!(problem.evaluate(&extracted));
}
