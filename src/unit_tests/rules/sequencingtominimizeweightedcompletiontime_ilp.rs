use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::misc::SequencingToMinimizeWeightedCompletionTime;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_reduction_creates_expected_ilp_shape() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1], vec![3, 5], vec![]);
    let reduction: ReductionSTMWCTToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // 2 completion variables + 1 pair-order variable.
    assert_eq!(ilp.num_vars(), 3);

    // 2 lower bounds + 2 upper bounds + 1 binary upper bound + 2 disjunctive constraints.
    assert_eq!(ilp.constraints().len(), 7);
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);

    // Objective is w_0 * C_0 + w_1 * C_1.
    assert_eq!(ilp.objective(), vec![(0, 3), (1, 5)]);
}

#[test]
fn test_variable_layout_helpers() {
    let problem =
        SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1, 3], vec![3, 5, 1], vec![(0, 2)]);
    let reduction: ReductionSTMWCTToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");

    assert_eq!(reduction.completion_var(0), 0);
    assert_eq!(reduction.completion_var(2), 2);
    assert_eq!(reduction.order_var(0, 1), 3);
    assert_eq!(reduction.order_var(0, 2), 4);
    assert_eq!(reduction.order_var(1, 2), 5);
}

#[test]
fn test_extract_solution_encodes_schedule_as_lehmer_code() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1], vec![3, 5], vec![]);
    let reduction: ReductionSTMWCTToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");

    // Completion times C0 = 3, C1 = 1 imply schedule [1, 0].
    // y_{0,1} = 0 means task 1 before task 0.
    let extracted = reduction.extract_solution(&vec![3, 1, 0]).unwrap();
    assert_eq!(extracted, vec![1, 0]);
    assert_eq!(problem.evaluate(&extracted).unwrap(), Min(Some(14)));
}

#[test]
fn test_issue_example_closed_loop() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![2, 1, 3, 1, 2],
        vec![3, 5, 1, 4, 2],
        vec![(0, 2), (1, 4)],
    );
    let reduction: ReductionSTMWCTToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let ilp_solution = ILPSolver::new().solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert_eq!(extracted, vec![1, 3, 0, 4, 2]);
    assert_eq!(problem.evaluate(&extracted).unwrap(), Min(Some(46)));
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
        .solve(&problem)
        .unwrap()
        .expect("brute force should find a schedule");
    let brute_force_metric = problem.evaluate(&brute_force_solution).unwrap();

    let reduction: ReductionSTMWCTToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    let ilp_solution = ILPSolver::new().solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let ilp_metric = problem.evaluate(&extracted).unwrap();

    assert_eq!(ilp_metric, brute_force_metric);
}

#[test]
fn test_cyclic_precedence_instance_is_infeasible() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![1, 1],
        vec![1, 1],
        vec![(0, 1), (1, 0)],
    );
    let reduction: ReductionSTMWCTToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert!(
        ILPSolver::new().solve(ilp).is_err(),
        "cyclic precedences should make the ILP infeasible"
    );
}

#[test]
fn test_reduction_rejects_total_processing_time_outside_i64_domain() {
    let problem =
        SequencingToMinimizeWeightedCompletionTime::new(vec![i64::MAX, 1], vec![1, 1], vec![]);
    assert!(matches!(
        ReduceTo::<ILP<i64>>::reduce_to(&problem),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));
}

#[test]
fn test_reduction_preserves_a_weight_outside_exact_f64_integer_range() {
    let problem =
        SequencingToMinimizeWeightedCompletionTime::new(vec![1], vec![(1i64 << 53) + 1], vec![]);
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&problem).unwrap();
    assert_eq!(
        reduction.target_problem().objective(),
        &[(0, (1i64 << 53) + 1)]
    );
}

#[test]
fn test_reduction_preserves_large_weighted_completion_objective() {
    let problem =
        SequencingToMinimizeWeightedCompletionTime::new(vec![1, 1], vec![1 << 52, 1 << 52], vec![]);
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&problem).unwrap();
    assert_eq!(
        reduction.target_problem().objective(),
        &[(0, 1 << 52), (1, 1 << 52)]
    );
}

#[test]
fn test_ilp_pipeline_matches_source_optimum() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![2, 1, 3, 1, 2],
        vec![3, 5, 1, 4, 2],
        vec![(0, 2), (1, 4)],
    );
    let reduction: ReductionSTMWCTToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let source_solution = reduction.extract_solution(&ilp_solution).unwrap();

    assert_eq!(source_solution, vec![1, 3, 0, 4, 2]);
    assert_eq!(problem.evaluate(&source_solution).unwrap(), Min(Some(46)));
}

#[test]
fn test_sequencingtominimizeweightedcompletiontime_to_ilp_bf_vs_ilp() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1], vec![3, 5], vec![]);
    let reduction: ReductionSTMWCTToILP =
        ReduceTo::<ILP<i64>>::reduce_to(&problem).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}
