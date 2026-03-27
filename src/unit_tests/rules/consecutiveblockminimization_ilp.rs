use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_cbm_to_ilp_structure() {
    let problem = ConsecutiveBlockMinimization::new(
        vec![vec![true, false, true], vec![false, true, true]],
        2,
    );
    let reduction: ReductionCBMToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // x: 3*3=9, a: 2*3=6, b: 2*3=6 => 21
    assert_eq!(ilp.num_vars, 21);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_cbm_to_ilp_closed_loop() {
    let problem = ConsecutiveBlockMinimization::new(
        vec![vec![true, false, true], vec![false, true, true]],
        2,
    );
    let reduction: ReductionCBMToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    assert_satisfaction_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "ConsecutiveBlockMinimization->ILP closed loop",
    );
}

#[test]
fn test_cbm_to_ilp_bf_vs_ilp() {
    let problem = ConsecutiveBlockMinimization::new(
        vec![vec![true, false, true], vec![false, true, true]],
        2,
    );
    let reduction: ReductionCBMToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

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
fn test_cbm_to_ilp_trivial() {
    // 1x1 matrix, bound 1
    let problem = ConsecutiveBlockMinimization::new(vec![vec![true]], 1);
    let reduction: ReductionCBMToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // x: 1, a: 1, b: 1 => 3
    assert_eq!(ilp.num_vars, 3);
}
