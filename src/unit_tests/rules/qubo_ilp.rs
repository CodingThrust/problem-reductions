use super::*;
use crate::solvers::BruteForce;
use std::collections::HashSet;

#[test]
fn test_qubo_to_ilp_closed_loop() {
    // QUBO: minimize 2*x0 - 3*x1 + x0*x1
    // Q = [[2, 1], [0, -3]]
    // x=0,0 -> 0, x=1,0 -> 2, x=0,1 -> -3, x=1,1 -> 0
    // Optimal: x = [0, 1] with obj = -3
    let qubo = QUBO::from_matrix(vec![vec![2.0, 1.0], vec![0.0, -3.0]]);
    let reduction = ReduceTo::<ILP>::reduce_to(&qubo);
    let ilp = reduction.target_problem();

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(ilp);
    let best_source: HashSet<_> = solver.find_all_best(&qubo).into_iter().collect();

    let extracted: HashSet<_> = best_target
        .iter()
        .map(|t| reduction.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
}

#[test]
fn test_qubo_to_ilp_diagonal_only() {
    // No quadratic terms: minimize 3*x0 - 2*x1
    // Optimal: x = [0, 1] with obj = -2
    let qubo = QUBO::from_matrix(vec![vec![3.0, 0.0], vec![0.0, -2.0]]);
    let reduction = ReduceTo::<ILP>::reduce_to(&qubo);
    let ilp = reduction.target_problem();

    // No auxiliary variables when no off-diagonal terms
    assert_eq!(ilp.num_variables(), 2);
    assert!(ilp.constraints.is_empty());

    let solver = BruteForce::new();
    let best = solver.find_all_best(ilp);
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
    let reduction = ReduceTo::<ILP>::reduce_to(&qubo);
    let ilp = reduction.target_problem();

    // 3 original + 2 auxiliary (for two off-diagonal terms)
    assert_eq!(ilp.num_variables(), 5);
    // 3 constraints per auxiliary = 6
    assert_eq!(ilp.constraints.len(), 6);

    let solver = BruteForce::new();
    let best = solver.find_all_best(ilp);
    let extracted = reduction.extract_solution(&best[0]);
    assert_eq!(extracted, vec![1, 0, 1]);
}
