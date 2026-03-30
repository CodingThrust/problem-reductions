use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::One;

// ===== Unit-length variant =====

#[test]
fn test_minimumtardinesssequencing_to_ilp_closed_loop() {
    let problem = MinimumTardinessSequencing::<One>::new(3, vec![2, 3, 1], vec![(0, 2)]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_optimization_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "MinimumTardinessSequencing->ILP closed loop",
    );
}

#[test]
fn test_minimumtardinesssequencing_to_ilp_bf_vs_ilp() {
    let problem = MinimumTardinessSequencing::<One>::new(4, vec![2, 3, 1, 4], vec![(0, 2)]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_witnesses(&problem);
    let bf_value = problem.evaluate(&bf_solutions[0]);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(bf_value, ilp_value);
    assert!(ilp_value.is_valid());
}

#[test]
fn test_minimumtardinesssequencing_to_ilp_no_precedences() {
    let problem = MinimumTardinessSequencing::<One>::new(3, vec![1, 2, 3], vec![]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_minimumtardinesssequencing_to_ilp_all_tight() {
    let problem = MinimumTardinessSequencing::<One>::new(3, vec![1, 1, 1], vec![]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let value = problem.evaluate(&extracted);
    assert!(value.is_valid());
    assert_eq!(value.0, Some(2));
}

// ===== Arbitrary-length variant =====

#[test]
fn test_minimumtardinesssequencing_weighted_to_ilp_closed_loop() {
    let problem =
        MinimumTardinessSequencing::<i32>::with_lengths(vec![2, 1, 3], vec![3, 4, 5], vec![(0, 2)]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_optimization_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "MinimumTardinessSequencing<i32>->ILP closed loop",
    );
}

#[test]
fn test_minimumtardinesssequencing_weighted_to_ilp_vs_brute_force() {
    let problem = MinimumTardinessSequencing::<i32>::with_lengths(
        vec![3, 2, 2, 1, 2],
        vec![4, 3, 8, 3, 6],
        vec![(0, 2), (1, 3)],
    );

    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem).expect("should have solution");
    let bf_value = problem.evaluate(&bf_witness);

    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(bf_value, ilp_value);
    assert_eq!(ilp_value.0, Some(2));
}
