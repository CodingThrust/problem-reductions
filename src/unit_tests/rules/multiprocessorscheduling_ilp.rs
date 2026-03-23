use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    // 3 tasks, 2 processors, deadline 5
    let problem = MultiprocessorScheduling::new(vec![2, 3, 2], 2, 5);
    let reduction: ReductionMSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // num_vars = 3 tasks * 2 processors = 6
    assert_eq!(ilp.num_vars, 6, "Should have 6 variables (3 tasks * 2 processors)");

    // num_constraints = 3 assignment + 2 load = 5
    assert_eq!(ilp.constraints.len(), 5, "Should have 5 constraints (3 assignment + 2 load)");
    assert_eq!(ilp.sense, ObjectiveSense::Minimize, "Should minimize (feasibility)");
}

#[test]
fn test_multiprocessorscheduling_to_ilp_bf_vs_ilp() {
    // 4 tasks [2, 2, 2, 2], 2 processors, deadline 4 → feasible (2+2 per proc)
    let problem = MultiprocessorScheduling::new(vec![2, 2, 2, 2], 2, 4);
    let reduction: ReductionMSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_witness = bf.find_witness(&problem).expect("BF should find a solution");
    assert_eq!(problem.evaluate(&bf_witness), Or(true));

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(
        problem.evaluate(&extracted),
        Or(true),
        "Extracted ILP solution should be valid"
    );
}

#[test]
fn test_solution_extraction() {
    // 3 tasks, 2 processors
    let problem = MultiprocessorScheduling::new(vec![1, 2, 3], 2, 5);
    let reduction: ReductionMSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // Manually set: task 0 → proc 0, task 1 → proc 1, task 2 → proc 0
    // Variables: x_{0,0}=1, x_{0,1}=0, x_{1,0}=0, x_{1,1}=1, x_{2,0}=1, x_{2,1}=0
    let ilp_solution = vec![1, 0, 0, 1, 1, 0];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 1, 0]);
    // loads: proc 0 = 1+3=4 ≤ 5, proc 1 = 2 ≤ 5
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_multiprocessorscheduling_to_ilp_trivial() {
    // Single task on single processor
    let problem = MultiprocessorScheduling::new(vec![5], 1, 5);
    let reduction: ReductionMSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // num_vars = 1 task * 1 processor = 1
    assert_eq!(ilp.num_vars, 1);
    // num_constraints = 1 assignment + 1 load = 2
    assert_eq!(ilp.constraints.len(), 2);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}
