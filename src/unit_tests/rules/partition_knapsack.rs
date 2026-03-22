use super::*;
use crate::models::misc::Partition;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::SolutionSize;

#[test]
fn test_partition_to_knapsack_closed_loop() {
    let source = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    let reduction = ReduceTo::<Knapsack>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "Partition -> Knapsack closed loop",
    );
}

#[test]
fn test_partition_to_knapsack_structure() {
    let source = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    let reduction = ReduceTo::<Knapsack>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.weights(), &[3, 1, 1, 2, 2, 1]);
    assert_eq!(target.values(), &[3, 1, 1, 2, 2, 1]);
    assert_eq!(target.capacity(), 5);
    assert_eq!(target.num_items(), source.num_elements());
}

#[test]
fn test_partition_to_knapsack_odd_total_is_not_satisfying() {
    let source = Partition::new(vec![2, 4, 5]);
    let reduction = ReduceTo::<Knapsack>::reduce_to(&source);
    let target = reduction.target_problem();
    let best = BruteForce::new()
        .find_best(target)
        .expect("Knapsack target should always have an optimal solution");

    assert_eq!(target.evaluate(&best), SolutionSize::Valid(5));

    let extracted = reduction.extract_solution(&best);
    assert!(!source.evaluate(&extracted));
}

#[test]
#[should_panic(
    expected = "Partition -> Knapsack requires all sizes and total_sum / 2 to fit in i64"
)]
fn test_partition_to_knapsack_panics_on_large_coefficients() {
    let source = Partition::new(vec![(i64::MAX as u64) + 1]);
    let _ = ReduceTo::<Knapsack>::reduce_to(&source);
}
