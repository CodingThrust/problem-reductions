use super::*;
use crate::models::misc::Partition;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Max;

#[test]
fn test_partition_to_knapsack_closed_loop() {
    let source = Partition::new(vec![3, 1, 1, 2, 2, 1]).unwrap();
    let reduction = ReduceTo::<Knapsack>::reduce_to(&source).expect("reduction should succeed");

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "Partition -> Knapsack closed loop",
    );
}

#[test]
fn test_partition_to_knapsack_structure() {
    let source = Partition::new(vec![3, 1, 1, 2, 2, 1]).unwrap();
    let reduction = ReduceTo::<Knapsack>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.weights(), &[3, 1, 1, 2, 2, 1]);
    assert_eq!(target.values(), &[3, 1, 1, 2, 2, 1]);
    assert_eq!(target.capacity(), 5);
    assert_eq!(target.num_items(), source.num_elements());
}

#[test]
fn test_partition_to_knapsack_odd_total_is_not_satisfying() {
    let source = Partition::new(vec![2, 4, 5]).unwrap();
    let reduction = ReduceTo::<Knapsack>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();
    let best = BruteForce::new()
        .solve(target)
        .unwrap()
        .expect("Knapsack target should always have an optimal solution");

    assert_eq!(target.evaluate(&best).unwrap(), Max(Some(5)));

    let extracted = reduction.extract_solution(&best).unwrap();
    assert!(!source.evaluate(&extracted).unwrap());
}
