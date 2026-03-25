use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;

#[test]
fn test_bmf_to_ilp_structure() {
    // 2x2 identity matrix, rank 1
    let problem = BMF::new(vec![vec![true, false], vec![false, true]], 1);
    let reduction: ReductionBMFToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // b: 2*1=2, c: 1*2=2, p: 2*1*2=4, w: 2*2=4, e: 2*2=4 => 16
    assert_eq!(ilp.num_vars, 16);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_bmf_to_ilp_closed_loop() {
    // 2x2 identity, rank 2 — exact factorization exists
    let problem = BMF::new(vec![vec![true, false], vec![false, true]], 2);
    let reduction: ReductionBMFToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    assert_optimization_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "BMF->ILP closed loop",
    );
}

#[test]
fn test_bmf_to_ilp_bf_vs_ilp() {
    let problem = BMF::new(vec![vec![true, true], vec![true, false]], 1);
    let reduction: ReductionBMFToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem).expect("should have a witness");
    let bf_value = problem.evaluate(&bf_witness);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(bf_value, ilp_value);
}

#[test]
fn test_bmf_to_ilp_trivial() {
    // 1x1 matrix, rank 1
    let problem = BMF::new(vec![vec![true]], 1);
    let reduction: ReductionBMFToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // b: 1, c: 1, p: 1, w: 1, e: 1 => 5
    assert_eq!(ilp.num_vars, 5);
}
