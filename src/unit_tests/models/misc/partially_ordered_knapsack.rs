use super::*;
use crate::solvers::BruteForce;
use crate::traits::{ObjectiveProblem, Problem};
use crate::types::ExtremumSense;

/// Helper: create the example instance from the issue.
/// Items: a=0, b=1, c=2, d=3, e=4, f=5
/// Precedences: a<c (0<2), a<d (0<3), b<e (1<4), d<f (3<5), e<f (4<5)
/// Sizes:  [2, 3, 4, 1, 2, 3]
/// Values: [3, 2, 5, 4, 3, 8]
/// Capacity: 11
fn example_instance() -> PartiallyOrderedKnapsack {
    PartiallyOrderedKnapsack::new(
        vec![2, 3, 4, 1, 2, 3],
        vec![3, 2, 5, 4, 3, 8],
        vec![(0, 2), (0, 3), (1, 4), (3, 5), (4, 5)],
        11,
    )
}

#[test]
fn test_partially_ordered_knapsack_basic() {
    let problem = example_instance();
    assert_eq!(problem.num_items(), 6);
    assert_eq!(problem.weights(), &[2, 3, 4, 1, 2, 3]);
    assert_eq!(problem.values(), &[3, 2, 5, 4, 3, 8]);
    assert_eq!(
        problem.precedences(),
        &[(0, 2), (0, 3), (1, 4), (3, 5), (4, 5)]
    );
    assert_eq!(problem.capacity(), 11);
    assert_eq!(problem.dims(), vec![2; 6]);
    assert_eq!(problem.direction(), ExtremumSense::Maximize);
    assert_eq!(
        <PartiallyOrderedKnapsack as Problem>::NAME,
        "PartiallyOrderedKnapsack"
    );
    assert_eq!(<PartiallyOrderedKnapsack as Problem>::variant(), vec![]);
}

#[test]
fn test_partially_ordered_knapsack_evaluate_valid() {
    let problem = example_instance();
    // U' = {a, b, d, e, f} = indices {0, 1, 3, 4, 5}
    // Total size: 2+3+1+2+3 = 11 <= 11
    // Total value: 3+2+4+3+8 = 20
    assert_eq!(problem.evaluate(&[1, 1, 0, 1, 1, 1]), Max(Some(20)));
}

#[test]
fn test_partially_ordered_knapsack_evaluate_precedence_violation() {
    let problem = example_instance();
    // U' = {d, f} = indices {3, 5} — f requires e and b (transitively), d requires a
    // Not downward-closed: d selected but a (predecessor of d) not selected
    assert_eq!(problem.evaluate(&[0, 0, 0, 1, 0, 1]), Max(None));
}

#[test]
fn test_partially_ordered_knapsack_evaluate_transitive_precedence_violation() {
    let problem = example_instance();
    // U' = {d, e, f} = indices {3, 4, 5}
    // f requires d (ok) and e (ok), but d requires a (0) which is not selected
    // Also e requires b (1) which is not selected
    assert_eq!(problem.evaluate(&[0, 0, 0, 1, 1, 1]), Max(None));
}

#[test]
fn test_partially_ordered_knapsack_evaluate_overweight() {
    let problem = example_instance();
    // U' = {a, b, c, d, e, f} = all items
    // Total size: 2+3+4+1+2+3 = 15 > 11
    assert_eq!(problem.evaluate(&[1, 1, 1, 1, 1, 1]), Max(None));
}

#[test]
fn test_partially_ordered_knapsack_evaluate_empty() {
    let problem = example_instance();
    assert_eq!(problem.evaluate(&[0, 0, 0, 0, 0, 0]), Max(Some(0)));
}

#[test]
fn test_partially_ordered_knapsack_evaluate_single_root() {
    let problem = example_instance();
    // Just item a (no predecessors)
    assert_eq!(problem.evaluate(&[1, 0, 0, 0, 0, 0]), Max(Some(3)));
}

#[test]
fn test_partially_ordered_knapsack_evaluate_valid_chain() {
    let problem = example_instance();
    // U' = {a, d} = indices {0, 3}
    // a has no predecessors, d's predecessor a is selected: downward-closed
    // Total size: 2+1 = 3 <= 11, Total value: 3+4 = 7
    assert_eq!(problem.evaluate(&[1, 0, 0, 1, 0, 0]), Max(Some(7)));
}

#[test]
fn test_partially_ordered_knapsack_evaluate_wrong_config_length() {
    let problem = example_instance();
    assert_eq!(problem.evaluate(&[1, 0]), Max(None));
    assert_eq!(problem.evaluate(&[1, 0, 0, 0, 0, 0, 0]), Max(None));
}

#[test]
fn test_partially_ordered_knapsack_evaluate_invalid_variable_value() {
    let problem = example_instance();
    assert_eq!(problem.evaluate(&[2, 0, 0, 0, 0, 0]), Max(None));
}

#[test]
fn test_partially_ordered_knapsack_brute_force() {
    let problem = example_instance();
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    let metric = problem.evaluate(&solution);
    // The optimal should be {a, b, d, e, f} with value 20
    assert_eq!(metric, Max(Some(20)));
}

#[test]
fn test_partially_ordered_knapsack_empty_instance() {
    let problem = PartiallyOrderedKnapsack::new(vec![], vec![], vec![], 10);
    assert_eq!(problem.num_items(), 0);
    assert_eq!(problem.num_precedences(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert_eq!(problem.evaluate(&[]), Max(Some(0)));
}

#[test]
fn test_partially_ordered_knapsack_no_precedences() {
    // Without precedences, behaves like standard knapsack
    let problem = PartiallyOrderedKnapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], vec![], 7);
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    let metric = problem.evaluate(&solution);
    // Same as standard knapsack: items 0 and 3 give weight 7, value 10
    assert_eq!(metric, Max(Some(10)));
}

#[test]
fn test_partially_ordered_knapsack_zero_capacity() {
    let problem = PartiallyOrderedKnapsack::new(vec![1, 2], vec![10, 20], vec![(0, 1)], 0);
    assert_eq!(problem.evaluate(&[0, 0]), Max(Some(0)));
    assert_eq!(problem.evaluate(&[1, 0]), Max(None));
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Max(Some(0)));
}

#[test]
fn test_partially_ordered_knapsack_serialization() {
    let problem = example_instance();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: PartiallyOrderedKnapsack = serde_json::from_value(json).unwrap();
    assert_eq!(restored.weights(), problem.weights());
    assert_eq!(restored.values(), problem.values());
    assert_eq!(restored.precedences(), problem.precedences());
    assert_eq!(restored.capacity(), problem.capacity());
}

#[test]
#[should_panic(expected = "weights and values must have the same length")]
fn test_partially_ordered_knapsack_mismatched_lengths() {
    PartiallyOrderedKnapsack::new(vec![1, 2], vec![3], vec![], 5);
}

#[test]
#[should_panic(expected = "precedence index 5 out of bounds")]
fn test_partially_ordered_knapsack_invalid_precedence() {
    PartiallyOrderedKnapsack::new(vec![1, 2], vec![3, 4], vec![(0, 5)], 5);
}

#[test]
#[should_panic(expected = "precedences contain a cycle")]
fn test_partially_ordered_knapsack_cycle() {
    PartiallyOrderedKnapsack::new(
        vec![1, 2, 3],
        vec![1, 2, 3],
        vec![(0, 1), (1, 2), (2, 0)],
        10,
    );
}

#[test]
#[should_panic(expected = "capacity must be non-negative")]
fn test_partially_ordered_knapsack_negative_capacity() {
    PartiallyOrderedKnapsack::new(vec![1, 2], vec![3, 4], vec![], -1);
}

#[test]
#[should_panic(expected = "weight[1] must be non-negative")]
fn test_partially_ordered_knapsack_negative_weight() {
    PartiallyOrderedKnapsack::new(vec![1, -2], vec![3, 4], vec![], 5);
}

#[test]
#[should_panic(expected = "value[0] must be non-negative")]
fn test_partially_ordered_knapsack_negative_value() {
    PartiallyOrderedKnapsack::new(vec![1, 2], vec![-3, 4], vec![], 5);
}
