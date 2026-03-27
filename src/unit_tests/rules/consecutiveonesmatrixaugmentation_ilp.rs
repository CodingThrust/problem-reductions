use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_coma_to_ilp_structure() {
    let problem = ConsecutiveOnesMatrixAugmentation::new(
        vec![vec![true, false, true], vec![false, true, true]],
        1,
    );
    let reduction: ReductionCOMAToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // x: 3*3=9, a+l+u+h+f: 5*2*3=30 => 39
    assert_eq!(ilp.num_vars, 39);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_coma_to_ilp_closed_loop() {
    let problem = ConsecutiveOnesMatrixAugmentation::new(
        vec![vec![true, false, true], vec![false, true, true]],
        1,
    );
    let reduction: ReductionCOMAToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // Use ILP solver instead of brute-force on the target (39 binary vars too large)
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));

    // Also verify that brute-force on the source agrees
    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem).expect("should be feasible");
    assert_eq!(problem.evaluate(&bf_witness), Or(true));
}

#[test]
fn test_coma_to_ilp_bf_vs_ilp() {
    let problem = ConsecutiveOnesMatrixAugmentation::new(
        vec![vec![true, false, true], vec![false, true, true]],
        1,
    );
    let reduction: ReductionCOMAToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem).expect("should be feasible");
    assert_eq!(problem.evaluate(&bf_witness), Or(true));

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_coma_to_ilp_trivial() {
    // 1x1 matrix, bound 0 — already consecutive
    let problem = ConsecutiveOnesMatrixAugmentation::new(vec![vec![true]], 0);
    let reduction: ReductionCOMAToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // x: 1, a+l+u+h+f: 5*1=5 => 6
    assert_eq!(ilp.num_vars, 6);
}
