use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;

#[test]
fn test_knapsack_basic() {
    let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    assert_eq!(problem.num_items(), 4);
    assert_eq!(problem.weights(), &[2, 3, 4, 5]);
    assert_eq!(problem.values(), &[3, 4, 5, 7]);
    assert_eq!(problem.capacity(), 7);
    assert_eq!(problem.dims(), vec![2; 4]);
    assert_eq!(problem.direction(), Direction::Maximize);
    assert_eq!(<Knapsack as Problem>::NAME, "Knapsack");
    assert_eq!(<Knapsack as Problem>::variant(), vec![]);
}

#[test]
fn test_knapsack_evaluate_optimal() {
    let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    assert_eq!(problem.evaluate(&[1, 0, 0, 1]), SolutionSize::Valid(10));
}

#[test]
fn test_knapsack_evaluate_feasible() {
    let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    assert_eq!(problem.evaluate(&[1, 1, 0, 0]), SolutionSize::Valid(7));
}

#[test]
fn test_knapsack_evaluate_overweight() {
    let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    assert_eq!(problem.evaluate(&[0, 0, 1, 1]), SolutionSize::Invalid);
}

#[test]
fn test_knapsack_evaluate_empty() {
    let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    assert_eq!(problem.evaluate(&[0, 0, 0, 0]), SolutionSize::Valid(0));
}

#[test]
fn test_knapsack_evaluate_all_selected() {
    let problem = Knapsack::new(vec![1, 1, 1], vec![10, 20, 30], 5);
    assert_eq!(problem.evaluate(&[1, 1, 1]), SolutionSize::Valid(60));
}

#[test]
fn test_knapsack_evaluate_wrong_config_length() {
    let problem = Knapsack::new(vec![2, 3], vec![3, 4], 5);
    assert_eq!(problem.evaluate(&[1]), SolutionSize::Invalid);
    assert_eq!(problem.evaluate(&[1, 0, 0]), SolutionSize::Invalid);
}

#[test]
fn test_knapsack_evaluate_invalid_variable_value() {
    let problem = Knapsack::new(vec![2, 3], vec![3, 4], 5);
    assert_eq!(problem.evaluate(&[2, 0]), SolutionSize::Invalid);
}

#[test]
fn test_knapsack_empty_instance() {
    let problem = Knapsack::new(vec![], vec![], 10);
    assert_eq!(problem.num_items(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert_eq!(problem.evaluate(&[]), SolutionSize::Valid(0));
}

#[test]
fn test_knapsack_brute_force() {
    let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert_eq!(metric, SolutionSize::Valid(10));
}

#[test]
fn test_knapsack_serialization() {
    let problem = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: Knapsack = serde_json::from_value(json).unwrap();
    assert_eq!(restored.weights(), problem.weights());
    assert_eq!(restored.values(), problem.values());
    assert_eq!(restored.capacity(), problem.capacity());
}

#[test]
fn test_knapsack_zero_capacity() {
    // Capacity 0: only empty set is feasible
    let problem = Knapsack::new(vec![1, 2], vec![10, 20], 0);
    assert_eq!(problem.evaluate(&[0, 0]), SolutionSize::Valid(0));
    assert_eq!(problem.evaluate(&[1, 0]), SolutionSize::Invalid);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), SolutionSize::Valid(0));
}

#[test]
fn test_knapsack_single_item() {
    // Single item that fits
    let problem = Knapsack::new(vec![3], vec![5], 3);
    assert_eq!(problem.evaluate(&[1]), SolutionSize::Valid(5));
    assert_eq!(problem.evaluate(&[0]), SolutionSize::Valid(0));
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), SolutionSize::Valid(5));
}

#[test]
fn test_knapsack_greedy_not_optimal() {
    // Classic case where greedy by value/weight ratio is suboptimal:
    // Item 0: w=6, v=7, ratio=1.17 (greedy picks this first, then nothing else fits)
    // Item 1: w=5, v=5, ratio=1.00
    // Item 2: w=5, v=5, ratio=1.00
    // Capacity=10. Greedy: {0} value=7. Optimal: {1,2} value=10.
    let problem = Knapsack::new(vec![6, 5, 5], vec![7, 5, 5], 10);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), SolutionSize::Valid(10));
}

#[test]
#[should_panic(expected = "weights and values must have the same length")]
fn test_knapsack_mismatched_lengths() {
    Knapsack::new(vec![1, 2], vec![3], 5);
}
