use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_resourceconstrainedscheduling_to_ilp_closed_loop() {
    let problem = ResourceConstrainedScheduling::new(
        3,
        vec![20],
        vec![vec![6], vec![7], vec![7], vec![6], vec![8], vec![6]],
        2,
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_satisfaction_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "ResourceConstrainedScheduling->ILP closed loop",
    );
}

#[test]
fn test_resourceconstrainedscheduling_to_ilp_bf_vs_ilp() {
    let problem = ResourceConstrainedScheduling::new(
        3,
        vec![20],
        vec![vec![6], vec![7], vec![7], vec![6], vec![8], vec![6]],
        2,
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let bf_witness = BruteForce::new()
        .find_witness(&problem)
        .expect("should be feasible");
    assert_eq!(problem.evaluate(&bf_witness), Or(true));

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_resourceconstrainedscheduling_to_ilp_infeasible() {
    // 3 tasks, 1 processor, 1 resource with bound 5, deadline 1
    // Each task requires 6 resource units — can't fit any two in same slot
    let problem =
        ResourceConstrainedScheduling::new(1, vec![5], vec![vec![6], vec![6], vec![6]], 1);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "infeasible RCS should produce infeasible ILP"
    );
}

#[test]
fn test_resourceconstrainedscheduling_to_ilp_structure() {
    let problem =
        ResourceConstrainedScheduling::new(2, vec![10], vec![vec![3], vec![4], vec![5]], 2);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // n=3 tasks, D=2 deadline → 6 variables
    assert_eq!(ilp.num_vars, 6);
    // 3 one-hot + 2 capacity + 1*2 resource = 7
    assert_eq!(ilp.constraints.len(), 7);
}
