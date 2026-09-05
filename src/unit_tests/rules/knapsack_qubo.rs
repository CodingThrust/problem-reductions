use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_knapsack_to_qubo_closed_loop() {
    let knapsack = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&knapsack).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    assert_eq!(qubo.num_vars(), 7);

    assert_optimization_round_trip_from_optimization_target(
        &knapsack,
        &reduction,
        "Knapsack->QUBO closed loop",
    );
}

#[test]
fn test_knapsack_to_qubo_single_item() {
    let knapsack = Knapsack::new(vec![1], vec![1], 1);
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&knapsack).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    assert_eq!(qubo.num_vars(), 2);

    let solver = BruteForce::new();
    let best_target = solver.find_all_witnesses(qubo).unwrap();
    let extracted = reduction.extract_solution(&best_target[0]).unwrap();
    assert_eq!(extracted, vec![true]);
}

#[test]
fn test_knapsack_to_qubo_infeasible_rejected() {
    let knapsack = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&knapsack).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let best_target = solver.find_all_witnesses(qubo).unwrap();

    for sol in &best_target {
        let source_sol = reduction.extract_solution(sol).unwrap();
        let eval = knapsack.evaluate(&source_sol).unwrap();
        assert!(
            eval.is_valid(),
            "Optimal QUBO solution maps to infeasible knapsack solution"
        );
    }
}

#[test]
fn test_knapsack_to_qubo_empty() {
    let knapsack = Knapsack::new(vec![1, 2], vec![3, 4], 0);
    let reduction = ReduceTo::<QUBO<i64>>::reduce_to(&knapsack).expect("reduction should succeed");
    let qubo = reduction.target_problem();

    assert_eq!(qubo.num_vars(), 3);

    let solver = BruteForce::new();
    let best_target = solver.find_all_witnesses(qubo).unwrap();
    let extracted = reduction.extract_solution(&best_target[0]).unwrap();
    assert_eq!(extracted, vec![false, false]);
}

#[cfg(feature = "example-db")]
#[test]
fn test_knapsack_to_qubo_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "knapsack_to_qubo")
        .expect("missing canonical Knapsack -> QUBO example spec");
    let example = (spec.build)();

    assert_eq!(example.source.problem, "Knapsack");
    assert_eq!(example.target.problem, "QUBO");
    assert_eq!(example.source.instance["capacity"], 7);
    assert_eq!(example.target.instance["num_vars"], 7);
    assert!(!example.solutions.is_empty());
}
