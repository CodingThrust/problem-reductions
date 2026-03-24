use super::*;
use crate::models::algebraic::ILP;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_flowshopscheduling_to_ilp_closed_loop() {
    // 2 machines, 3 jobs, deadline 10
    let problem = FlowShopScheduling::new(2, vec![vec![2, 3], vec![3, 2], vec![1, 4]], 10);
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    let bf = BruteForce::new();
    let bf_witness = bf
        .find_witness(&problem)
        .expect("feasible instance should have a witness");
    assert_eq!(problem.evaluate(&bf_witness), Or(true));

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(
        problem.evaluate(&extracted),
        Or(true),
        "ILP extracted solution should be a valid schedule"
    );
}

#[test]
fn test_flowshopscheduling_to_ilp_infeasible() {
    // 2 machines, 3 jobs with large processing times, very tight deadline
    let problem = FlowShopScheduling::new(2, vec![vec![5, 5], vec![5, 5], vec![5, 5]], 6);
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "infeasible FSS should produce infeasible ILP"
    );
}

#[test]
fn test_flowshopscheduling_to_ilp_single_job() {
    // 2 machines, 1 job, deadline 10
    let problem = FlowShopScheduling::new(2, vec![vec![3, 4]], 10);
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("single-job ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_flowshopscheduling_to_ilp_bf_vs_ilp() {
    let problem = FlowShopScheduling::new(2, vec![vec![2, 3], vec![3, 2], vec![1, 4]], 10);
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem).expect("should be feasible");
    assert_eq!(problem.evaluate(&bf_witness), Or(true));

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}
