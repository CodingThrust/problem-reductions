use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_knapsack_to_qubo_closed_loop() {
    let knapsack = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&knapsack);
    let qubo = reduction.target_problem();

    assert_eq!(qubo.num_vars(), 7);

    let solver = BruteForce::new();
    let best_source = solver.find_all_best(&knapsack);
    let best_target = solver.find_all_best(qubo);

    let extracted: std::collections::HashSet<Vec<usize>> = best_target
        .iter()
        .map(|t| reduction.extract_solution(t))
        .collect();
    let source_set: std::collections::HashSet<Vec<usize>> = best_source.into_iter().collect();

    assert!(extracted.is_subset(&source_set));
    assert!(!extracted.is_empty());
}

#[test]
fn test_knapsack_to_qubo_single_item() {
    let knapsack = Knapsack::new(vec![1], vec![1], 1);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&knapsack);
    let qubo = reduction.target_problem();

    assert_eq!(qubo.num_vars(), 2);

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(qubo);
    let extracted = reduction.extract_solution(&best_target[0]);
    assert_eq!(extracted, vec![1]);
}

#[test]
fn test_knapsack_to_qubo_infeasible_rejected() {
    let knapsack = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&knapsack);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(qubo);

    for sol in &best_target {
        let source_sol = reduction.extract_solution(sol);
        let eval = knapsack.evaluate(&source_sol);
        assert!(
            eval.is_valid(),
            "Optimal QUBO solution maps to infeasible knapsack solution"
        );
    }
}

#[test]
fn test_knapsack_to_qubo_empty() {
    let knapsack = Knapsack::new(vec![1, 2], vec![3, 4], 0);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&knapsack);
    let qubo = reduction.target_problem();

    assert_eq!(qubo.num_vars(), 3);

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(qubo);
    let extracted = reduction.extract_solution(&best_target[0]);
    assert_eq!(extracted, vec![0, 0]);
}
