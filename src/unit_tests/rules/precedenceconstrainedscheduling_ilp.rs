use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;

fn feasible_instance() -> PrecedenceConstrainedScheduling {
    // 3 tasks, 2 processors, deadline 2, precedence: task 0 must complete before task 2
    PrecedenceConstrainedScheduling::new(3, 2, 2, vec![(0, 2)])
}

fn infeasible_instance() -> PrecedenceConstrainedScheduling {
    // 3 tasks, 1 processor, deadline 2: impossible to fit all 3 tasks in 2 slots with 1 proc each
    PrecedenceConstrainedScheduling::new(3, 1, 2, vec![])
}

#[test]
fn test_precedenceconstrainedscheduling_to_ilp_structure() {
    let problem = feasible_instance();
    let reduction: ReductionPCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // n=3 tasks, d=2 deadline → 6 variables
    assert_eq!(ilp.num_vars, 6);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert!(ilp.objective.is_empty());

    // n one-hot constraints + d capacity constraints + 1 precedence = 3 + 2 + 1 = 6
    assert_eq!(ilp.constraints.len(), 6);
}

#[test]
fn test_precedenceconstrainedscheduling_to_ilp_closed_loop() {
    let problem = feasible_instance();
    let bf = BruteForce::new();
    let bf_solution = bf.find_witness(&problem).expect("feasible instance should have a witness");
    assert!(problem.evaluate(&bf_solution).0, "brute force solution should be valid");

    let reduction: ReductionPCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible for feasible instance");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(
        problem.evaluate(&extracted).0,
        "ILP extracted solution should be a valid schedule"
    );
}

#[test]
fn test_precedenceconstrainedscheduling_to_ilp_infeasible() {
    let problem = infeasible_instance();
    let reduction: ReductionPCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "infeasible scheduling instance should produce infeasible ILP"
    );
}

#[test]
fn test_precedenceconstrainedscheduling_to_ilp_extract_solution() {
    let problem = feasible_instance();
    let reduction: ReductionPCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // Manually: task 0 at slot 0, task 1 at slot 0, task 2 at slot 1
    // x_{0,0}=1, x_{0,1}=0, x_{1,0}=1, x_{1,1}=0, x_{2,0}=0, x_{2,1}=1
    let ilp_solution = vec![1, 0, 1, 0, 0, 1];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 0, 1]);
    assert!(problem.evaluate(&extracted).0, "manually constructed solution should be valid");
}
