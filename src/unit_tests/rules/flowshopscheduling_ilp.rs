use super::*;
use crate::models::algebraic::ILP;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn zero_duration_jobs_preserve_the_common_machine_order() {
    let source = FlowShopScheduling::new(2, vec![vec![3, 0], vec![1, 10]], 11);
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).unwrap();
    // Job 1 precedes job 0, but both finish on machine 1 at time 11.
    let assignment = vec![0, 4, 11, 1, 11];
    assert!(reduction
        .target_problem()
        .evaluate(&assignment)
        .unwrap()
        .value
        .is_some());
    let decoded = reduction.extract_solution(&assignment).unwrap();
    assert_eq!(decoded, vec![1, 0]);
    assert_eq!(source.evaluate(&decoded).unwrap(), Or(true));

    let no_machines = FlowShopScheduling::new(0, vec![vec![], vec![]], 0);
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&no_machines).unwrap();
    crate::rules::test_helpers::assert_bf_vs_ilp(&no_machines, &reduction);
}

#[test]
fn test_flowshopscheduling_to_ilp_closed_loop() {
    // 2 machines, 3 jobs, deadline 10
    let problem = FlowShopScheduling::new(2, vec![vec![2, 3], vec![3, 2], vec![1, 4]], 10);
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");

    let bf = BruteForce::new();
    let bf_witness = bf
        .solve(&problem)
        .unwrap()
        .expect("feasible instance should have a witness");
    assert_eq!(problem.evaluate(&bf_witness).unwrap(), Or(true));

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(
        problem.evaluate(&extracted).unwrap(),
        Or(true),
        "ILP extracted solution should be a valid schedule"
    );
}

#[test]
fn test_flowshopscheduling_to_ilp_infeasible() {
    // 2 machines, 3 jobs with large processing times, very tight deadline
    let problem = FlowShopScheduling::new(2, vec![vec![5, 5], vec![5, 5], vec![5, 5]], 6);
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_err(),
        "infeasible FSS should produce infeasible ILP"
    );
}

#[test]
fn test_flowshopscheduling_to_ilp_single_job() {
    // 2 machines, 1 job, deadline 10
    let problem = FlowShopScheduling::new(2, vec![vec![3, 4]], 10);
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("single-job ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(problem.evaluate(&extracted).unwrap(), Or(true));
}

#[test]
fn test_flowshopscheduling_to_ilp_bf_vs_ilp() {
    let problem = FlowShopScheduling::new(2, vec![vec![2, 3], vec![3, 2], vec![1, 4]], 10);
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");

    let bf = BruteForce::new();
    let bf_witness = bf.solve(&problem).unwrap().expect("should be feasible");
    assert_eq!(problem.evaluate(&bf_witness).unwrap(), Or(true));

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(problem.evaluate(&extracted).unwrap(), Or(true));
}
