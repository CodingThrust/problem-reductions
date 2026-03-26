use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    let problem = RectilinearPictureCompression::new(vec![vec![true, true], vec![true, false]], 2);
    let reduction: ReductionRPCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // Number of vars = number of maximal rectangles (precomputed)
    assert!(ilp.num_vars > 0);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_rectilinearpicturecompression_to_ilp_bf_vs_ilp() {
    let problem = RectilinearPictureCompression::new(vec![vec![true, true], vec![true, true]], 1);
    let reduction: ReductionRPCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_witness = bf.find_witness(&problem).expect("should be feasible");
    assert_eq!(problem.evaluate(&bf_witness), Or(true));

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_solution_extraction() {
    let problem = RectilinearPictureCompression::new(vec![vec![true, true], vec![true, true]], 2);
    let reduction: ReductionRPCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_rectilinearpicturecompression_to_ilp_trivial() {
    // All-zero matrix: no 1-cells, trivially feasible
    let problem =
        RectilinearPictureCompression::new(vec![vec![false, false], vec![false, false]], 0);
    let reduction: ReductionRPCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars, 0); // no maximal rects
}
