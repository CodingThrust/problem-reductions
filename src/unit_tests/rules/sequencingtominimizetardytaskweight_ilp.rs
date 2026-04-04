use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;

#[test]
fn test_sequencingtominimizetardytaskweight_to_ilp_closed_loop() {
    let problem =
        SequencingToMinimizeTardyTaskWeight::new(vec![3, 2, 1], vec![4, 2, 3], vec![4, 3, 6]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_optimization_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "SequencingToMinimizeTardyTaskWeight->ILP closed loop",
    );
}

#[test]
fn test_sequencingtominimizetardytaskweight_to_ilp_bf_vs_ilp() {
    let problem = SequencingToMinimizeTardyTaskWeight::new(
        vec![3, 2, 4, 1, 2],
        vec![5, 3, 7, 2, 4],
        vec![6, 4, 10, 2, 8],
    );

    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem).expect("should find a solution");
    let bf_value = problem.evaluate(&bf_witness);

    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(bf_value, ilp_value);
    assert_eq!(ilp_value.0, Some(3));
}

#[test]
fn test_sequencingtominimizetardytaskweight_to_ilp_all_on_time() {
    let problem =
        SequencingToMinimizeTardyTaskWeight::new(vec![1, 1, 1], vec![2, 3, 4], vec![10, 10, 10]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let value = problem.evaluate(&extracted);
    assert!(value.is_valid());
    assert_eq!(value.0, Some(0));
}

#[test]
fn test_sequencingtominimizetardytaskweight_to_ilp_optimal_ordering() {
    // 3 tasks where order matters:
    // t0: length=4, weight=5, deadline=4
    // t1: length=1, weight=1, deadline=5
    // t2: length=2, weight=3, deadline=3
    // Best schedule: [2,0,1] -> t2 completes 2 (ok), t0 completes 6 (tardy wt=5), t1 completes 7 (tardy wt=1)
    // or: [2,1,0] -> t2 completes 2 (ok), t1 completes 3 (ok), t0 completes 7 (tardy wt=5)
    // or: [1,2,0] -> t1 completes 1 (ok), t2 completes 3 (ok), t0 completes 7 (tardy wt=5)
    // or: [0,1,2] -> t0 completes 4 (ok), t1 completes 5 (ok), t2 completes 7 (tardy wt=3) = 3
    // Minimum is 3 (schedule [0,1,2])
    let problem =
        SequencingToMinimizeTardyTaskWeight::new(vec![4, 1, 2], vec![5, 1, 3], vec![4, 5, 3]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem).expect("should have solution");
    let bf_value = problem.evaluate(&bf_witness);

    assert_eq!(ilp_value, bf_value);
}
