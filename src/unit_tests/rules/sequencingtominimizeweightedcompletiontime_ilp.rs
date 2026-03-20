use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::misc::SequencingToMinimizeWeightedCompletionTime;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::traits::Problem;
use crate::types::SolutionSize;

#[test]
fn test_reduction_creates_expected_ilp_shape() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1], vec![3, 5], vec![]);
    let reduction: ReductionSTMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 2 completion variables + 1 pair-order variable.
    assert_eq!(ilp.num_vars, 3);

    // 2 lower bounds + 2 upper bounds + 1 binary upper bound + 2 disjunctive constraints.
    assert_eq!(ilp.constraints.len(), 7);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);

    // Objective is w_0 * C_0 + w_1 * C_1.
    assert_eq!(ilp.objective, vec![(0, 3.0), (1, 5.0)]);
}

#[test]
fn test_variable_layout_helpers() {
    let problem =
        SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1, 3], vec![3, 5, 1], vec![(0, 2)]);
    let reduction: ReductionSTMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    assert_eq!(reduction.completion_var(0), 0);
    assert_eq!(reduction.completion_var(2), 2);
    assert_eq!(reduction.order_var(0, 1), 3);
    assert_eq!(reduction.order_var(0, 2), 4);
    assert_eq!(reduction.order_var(1, 2), 5);
}

#[test]
fn test_extract_solution_encodes_schedule_as_lehmer_code() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1], vec![3, 5], vec![]);
    let reduction: ReductionSTMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    // Completion times C0 = 3, C1 = 1 imply schedule [1, 0].
    // y_{0,1} = 0 means task 1 before task 0.
    let extracted = reduction.extract_solution(&[3, 1, 0]);
    assert_eq!(extracted, vec![1, 0]);
    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(14));
}

#[test]
fn test_issue_example_closed_loop() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![2, 1, 3, 1, 2],
        vec![3, 5, 1, 4, 2],
        vec![(0, 2), (1, 4)],
    );
    let reduction: ReductionSTMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solution = ILPSolver::new().solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted, vec![1, 2, 0, 1, 0]);
    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(46));
}

#[test]
fn test_ilp_matches_bruteforce_optimum() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![2, 1, 3, 1, 2],
        vec![3, 5, 1, 4, 2],
        vec![(0, 2), (1, 4)],
    );

    let brute_force = BruteForce::new();
    let brute_force_solution = brute_force
        .find_best(&problem)
        .expect("brute force should find a schedule");
    let brute_force_metric = problem.evaluate(&brute_force_solution);

    let reduction: ReductionSTMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solution = ILPSolver::new().solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_metric = problem.evaluate(&extracted);

    assert_eq!(ilp_metric, brute_force_metric);
}

#[test]
fn test_cyclic_precedence_instance_is_infeasible() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![1, 1],
        vec![1, 1],
        vec![(0, 1), (1, 0)],
    );
    let reduction: ReductionSTMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert!(
        ILPSolver::new().solve(ilp).is_none(),
        "cyclic precedences should make the ILP infeasible"
    );
}

#[test]
#[should_panic(expected = "task lengths must fit in ILP<i32> variable bounds")]
fn test_reduction_panics_when_a_task_length_exceeds_i32_domain() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![(i32::MAX as u64) + 1],
        vec![1],
        vec![],
    );
    let _: ReductionSTMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
}

#[test]
#[should_panic(expected = "total processing time must fit in ILP<i32> variable bounds")]
fn test_reduction_panics_when_total_processing_time_exceeds_i32_domain() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![i32::MAX as u64, 1],
        vec![1, 1],
        vec![],
    );
    let _: ReductionSTMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
}

#[test]
#[should_panic(expected = "weighted completion objective must fit exactly in f64")]
fn test_reduction_panics_when_a_weight_exceeds_exact_f64_integer_range() {
    let problem =
        SequencingToMinimizeWeightedCompletionTime::new(vec![1], vec![(1u64 << 53) + 1], vec![]);
    let _: ReductionSTMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
}

#[test]
#[should_panic(expected = "weighted completion objective must fit exactly in f64")]
fn test_reduction_panics_when_weighted_completion_objective_exceeds_exact_f64_range() {
    let problem =
        SequencingToMinimizeWeightedCompletionTime::new(vec![1, 1], vec![1 << 52, 1 << 52], vec![]);
    let _: ReductionSTMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
}

#[test]
fn test_solve_reduced_matches_source_optimum() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![2, 1, 3, 1, 2],
        vec![3, 5, 1, 4, 2],
        vec![(0, 2), (1, 4)],
    );
    let reduction: ReductionSTMWCTToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let source_solution = reduction.extract_solution(&ilp_solution);

    assert_eq!(source_solution, vec![1, 2, 0, 1, 0]);
    assert_eq!(problem.evaluate(&source_solution), SolutionSize::Valid(46));
}
