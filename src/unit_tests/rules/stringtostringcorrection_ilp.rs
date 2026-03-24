use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    // source = [0,1], target = [1], bound = 1 (delete position 0)
    let problem = StringToStringCorrection::new(2, vec![0, 1], vec![1], 1);
    let reduction: ReductionSTSCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // n=2, K=1: (K+1)*n*n + (K+1)*n + K*n + K*(n-1) + K = 2*4 + 2*2 + 1*2 + 1*1 + 1 = 16
    assert_eq!(ilp.num_vars(), 16);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_stringtostringcorrection_to_ilp_bf_vs_ilp() {
    // source=[0,1], target=[1], bound=1 (delete position 0)
    let problem = StringToStringCorrection::new(2, vec![0, 1], vec![1], 1);

    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem);
    assert!(bf_witness.is_some(), "BF should find a solution");

    let reduction: ReductionSTSCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_solution_extraction_delete() {
    // source=[0,1], target=[1], bound=1 => delete at position 0
    let problem = StringToStringCorrection::new(2, vec![0, 1], vec![1], 1);
    let reduction: ReductionSTSCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted.len(), 1);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_stringtostringcorrection_to_ilp_infeasible() {
    // source=[0], target=[0,1]: m > n, so model rejects before any search
    // The ILP is trivially infeasible (0 vars, unsatisfiable constraint)
    let problem = StringToStringCorrection::new(2, vec![0], vec![0, 1], 1);

    // Verify the source problem is actually infeasible
    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem);
    assert!(bf_witness.is_none(), "source should be infeasible");
}

#[test]
fn test_stringtostringcorrection_to_ilp_swap() {
    // source=[1,0], target=[0,1], bound=1 => swap at position 0
    let problem = StringToStringCorrection::new(2, vec![1, 0], vec![0, 1], 1);

    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem);
    assert!(bf_witness.is_some(), "BF should find a solution");

    let reduction: ReductionSTSCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}
