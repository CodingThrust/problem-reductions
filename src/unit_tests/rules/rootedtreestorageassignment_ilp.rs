use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    let problem = RootedTreeStorageAssignment::new(3, vec![vec![0, 1], vec![1, 2]], 1);
    let reduction: ReductionRTSAToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // n=3, r=2 (both subsets have size 2)
    let n = 3;
    let r = 2;
    let expected = n * n * n + 2 * n * n + n + r * (n * n + 2 * n + 3);
    assert_eq!(ilp.num_vars(), expected);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_rootedtreestorageassignment_to_ilp_bf_vs_ilp() {
    let problem = RootedTreeStorageAssignment::new(3, vec![vec![0, 1], vec![1, 2]], 1);

    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem);
    let bf_value = bf_witness
        .as_ref()
        .map(|w| problem.evaluate(w))
        .unwrap_or(Or(false));

    let reduction: ReductionRTSAToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_result = ilp_solver.solve(reduction.target_problem());

    match ilp_result {
        Some(ilp_solution) => {
            let extracted = reduction.extract_solution(&ilp_solution);
            let ilp_value = problem.evaluate(&extracted);
            assert!(ilp_value.0, "ILP solution should be feasible");
            assert!(bf_value.0, "BF should also find feasible solution");
        }
        None => {
            assert!(!bf_value.0, "both should agree on infeasibility");
        }
    }
}

#[test]
fn test_rootedtreestorageassignment_to_ilp_infeasible() {
    // With bound 0, every 2-element subset must appear as a parent-child pair.
    // The crossing pairs below force vertex 3 to have two distinct parents.
    let problem = RootedTreeStorageAssignment::new(
        4,
        vec![vec![0, 1], vec![2, 3], vec![0, 2], vec![1, 3]],
        0,
    );

    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem);

    let reduction: ReductionRTSAToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_result = ilp_solver.solve(reduction.target_problem());
    assert!(bf_witness.is_none(), "source should be infeasible");
    assert!(
        ilp_result.is_none(),
        "reduced ILP should also be infeasible"
    );
}

#[test]
fn test_solution_extraction() {
    let problem = RootedTreeStorageAssignment::new(3, vec![vec![0, 1, 2]], 0);
    let reduction: ReductionRTSAToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted.len(), 3);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}
