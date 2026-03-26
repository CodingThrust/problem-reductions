use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Universe {0..5}, 3 triples
    let problem = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);
    let reduction: ReductionX3CToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars, 3);
    assert_eq!(ilp.constraints.len(), 7); // 6 element constraints + 1 cardinality
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_exactcoverby3sets_to_ilp_bf_vs_ilp() {
    let problem = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4], [1, 2, 5]]);
    let reduction: ReductionX3CToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
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
    let problem = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5]]);
    let reduction: ReductionX3CToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solution = vec![1, 1]; // select both triples
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![1, 1]);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_exactcoverby3sets_to_ilp_trivial() {
    let problem = ExactCoverBy3Sets::new(0, vec![]);
    let reduction: ReductionX3CToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars, 0);
    assert_eq!(ilp.constraints.len(), 1); // just the cardinality constraint Σ = 0
}
