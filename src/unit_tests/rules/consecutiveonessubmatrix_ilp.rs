use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_cos_to_ilp_structure() {
    // Tucker matrix (3x4), K=3
    let problem = ConsecutiveOnesSubmatrix::new(
        vec![
            vec![true, true, false, true],
            vec![true, false, true, true],
            vec![false, true, true, false],
        ],
        3,
    );
    let reduction: ReductionCOSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // s: 4, x: 4*3=12, a+l+u+h+f: 5*3*3=45 => 61
    assert_eq!(ilp.num_vars, 61);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_cos_to_ilp_closed_loop() {
    // Tucker matrix (3x4), K=3
    let problem = ConsecutiveOnesSubmatrix::new(
        vec![
            vec![true, true, false, true],
            vec![true, false, true, true],
            vec![false, true, true, false],
        ],
        3,
    );
    let reduction: ReductionCOSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // Use ILP solver (61 binary vars too large for brute force on target)
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));

    // Verify brute-force on source agrees
    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem).expect("should be feasible");
    assert_eq!(problem.evaluate(&bf_witness), Or(true));
}

#[test]
fn test_cos_to_ilp_bf_vs_ilp() {
    let problem = ConsecutiveOnesSubmatrix::new(
        vec![
            vec![true, true, false, true],
            vec![true, false, true, true],
            vec![false, true, true, false],
        ],
        3,
    );
    let reduction: ReductionCOSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

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
fn test_cos_to_ilp_trivial() {
    // 2x2 identity, K=2
    let problem = ConsecutiveOnesSubmatrix::new(vec![vec![true, false], vec![false, true]], 2);
    let reduction: ReductionCOSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}
