use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, ILPSolver};

#[test]
fn test_qubo_to_ilp_closed_loop() {
    // QUBO: minimize 2*x0 - 3*x1 + x0*x1
    // Q = [[2, 1], [0, -3]]
    // x=0,0 -> 0, x=1,0 -> 2, x=0,1 -> -3, x=1,1 -> 0
    // Optimal: x = [0, 1] with obj = -3
    let qubo = QUBO::from_matrix(vec![vec![2.0, 1.0], vec![0.0, -3.0]]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&qubo);
    assert_optimization_round_trip_from_optimization_target(
        &qubo,
        &reduction,
        "QUBO->ILP closed loop",
    );
}

#[test]
fn test_qubo_to_ilp_bf_vs_ilp() {
    // QUBO: minimize 2*x0 - 3*x1 + x0*x1
    let qubo = QUBO::from_matrix(vec![vec![2.0, 1.0], vec![0.0, -3.0]]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&qubo);

    let bf_solutions = BruteForce::new().find_all_witnesses(&qubo);
    let bf_value = qubo.evaluate(&bf_solutions[0]);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = qubo.evaluate(&extracted);

    assert_eq!(bf_value, ilp_value);
}

#[test]
fn test_qubo_to_ilp_diagonal_only() {
    // No quadratic terms: minimize 3*x0 - 2*x1
    // Optimal: x = [0, 1] with obj = -2
    let qubo = QUBO::from_matrix(vec![vec![3.0, 0.0], vec![0.0, -2.0]]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&qubo);
    let ilp = reduction.target_problem();

    // No auxiliary variables when no off-diagonal terms
    assert_eq!(ilp.num_variables(), 2);
    assert!(ilp.constraints.is_empty());

    let solver = BruteForce::new();
    let best = solver.find_all_witnesses(ilp);
    let extracted = reduction.extract_solution(&best[0]);
    assert_eq!(extracted, vec![0, 1]);
}

#[test]
fn test_qubo_to_ilp_3var() {
    // QUBO: minimize -x0 - x1 - x2 + 4*x0*x1 + 4*x1*x2
    // Penalty on adjacent pairs → optimal is [1, 0, 1]
    let qubo = QUBO::from_matrix(vec![
        vec![-1.0, 4.0, 0.0],
        vec![0.0, -1.0, 4.0],
        vec![0.0, 0.0, -1.0],
    ]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&qubo);
    let ilp = reduction.target_problem();

    // 3 original + 2 auxiliary (for two off-diagonal terms)
    assert_eq!(ilp.num_variables(), 5);
    // 3 constraints per auxiliary = 6
    assert_eq!(ilp.constraints.len(), 6);

    let solver = BruteForce::new();
    let best = solver.find_all_witnesses(ilp);
    let extracted = reduction.extract_solution(&best[0]);
    assert_eq!(extracted, vec![1, 0, 1]);
}
