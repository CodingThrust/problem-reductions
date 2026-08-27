use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_smc_to_ilp_structure() {
    let problem = SparseMatrixCompression::new(
        vec![
            vec![true, false, false, true],
            vec![false, true, false, false],
            vec![false, false, true, false],
            vec![true, false, false, false],
        ],
        2,
    );
    let reduction: ReductionSMCToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    // x: 4*2 = 8
    assert_eq!(ilp.num_vars(), 8);
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);
}

#[test]
fn test_smc_to_ilp_closed_loop() {
    let problem = SparseMatrixCompression::new(
        vec![
            vec![true, false, false, true],
            vec![false, true, false, false],
            vec![false, false, true, false],
            vec![true, false, false, false],
        ],
        2,
    );
    let reduction: ReductionSMCToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_smc_to_ilp_bf_vs_ilp() {
    let problem = SparseMatrixCompression::new(
        vec![
            vec![true, false, false, true],
            vec![false, true, false, false],
            vec![false, false, true, false],
            vec![true, false, false, false],
        ],
        2,
    );
    let reduction: ReductionSMCToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    let bf = BruteForce::new();
    let bf_witness = bf.solve(&problem).unwrap().expect("should be feasible");
    assert_eq!(problem.evaluate(&bf_witness).unwrap(), Or(true));

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(problem.evaluate(&extracted).unwrap(), Or(true));
}

#[test]
fn test_smc_to_ilp_trivial() {
    // Single row, K=1
    let problem = SparseMatrixCompression::new(vec![vec![true, false]], 1);
    let reduction: ReductionSMCToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    // x: 1*1 = 1
    assert_eq!(ilp.num_vars(), 1);
}
