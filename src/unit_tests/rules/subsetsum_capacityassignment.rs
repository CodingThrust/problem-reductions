use super::*;
use crate::models::misc::{CapacityAssignment, SubsetSum};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_subsetsum_to_capacityassignment_closed_loop() {
    // YES instance: {3, 7, 1, 8, 2, 4}, target 11 → subset {3, 8} sums to 11
    let source = SubsetSum::new(vec![3u32, 7, 1, 8, 2, 4], 11u32);
    let reduction = ReduceTo::<CapacityAssignment>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "SubsetSum -> CapacityAssignment closed loop",
    );
}

#[test]
fn test_subsetsum_to_capacityassignment_structure() {
    let source = SubsetSum::new(vec![3u32, 7, 1, 8, 4, 12], 15u32);
    let reduction = ReduceTo::<CapacityAssignment>::reduce_to(&source);
    let target = reduction.target_problem();

    // 6 elements → 6 links, 2 capacities
    assert_eq!(target.num_links(), 6);
    assert_eq!(target.num_capacities(), 2);
    assert_eq!(target.capacities(), &[1, 2]);

    // Check cost/delay for first link (a_0 = 3):
    // cost(c_0, low) = 0, cost(c_0, high) = 3
    assert_eq!(target.cost()[0], vec![0, 3]);
    // delay(c_0, low) = 3, delay(c_0, high) = 0
    assert_eq!(target.delay()[0], vec![3, 0]);

    // Delay budget = S - B = 35 - 15 = 20
    assert_eq!(target.delay_budget(), 20);
}

#[test]
fn test_subsetsum_to_capacityassignment_no_instance() {
    // NO instance: {1, 5, 11, 6}, target 4 → no subset sums to 4
    let source = SubsetSum::new(vec![1u32, 5, 11, 6], 4u32);
    let reduction = ReduceTo::<CapacityAssignment>::reduce_to(&source);
    let target = reduction.target_problem();

    // S = 23, B = 4, delay_budget = 19
    assert_eq!(target.delay_budget(), 19);

    // The optimal CapacityAssignment cost should be > 4 (since no subset sums to 4)
    let best = BruteForce::new()
        .find_witness(target)
        .expect("CapacityAssignment should have a feasible solution");
    let extracted = reduction.extract_solution(&best);
    // The extracted config should NOT satisfy SubsetSum
    assert!(!source.evaluate(&extracted));
}

#[test]
fn test_subsetsum_to_capacityassignment_small() {
    // Two elements: {3, 3}, target 3 → subset {3} sums to 3
    let source = SubsetSum::new(vec![3u32, 3], 3u32);
    let reduction = ReduceTo::<CapacityAssignment>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "SubsetSum [3,3] target 3 -> CapacityAssignment",
    );
}

#[test]
fn test_subsetsum_to_capacityassignment_monotonicity() {
    // Verify cost non-decreasing and delay non-increasing for all links
    let source = SubsetSum::new(vec![5u32, 10, 15], 20u32);
    let reduction = ReduceTo::<CapacityAssignment>::reduce_to(&source);
    let target = reduction.target_problem();

    for (link, cost_row) in target.cost().iter().enumerate() {
        assert!(
            cost_row.windows(2).all(|w| w[0] <= w[1]),
            "cost row {link} must be non-decreasing"
        );
    }
    for (link, delay_row) in target.delay().iter().enumerate() {
        assert!(
            delay_row.windows(2).all(|w| w[0] >= w[1]),
            "delay row {link} must be non-increasing"
        );
    }
}
