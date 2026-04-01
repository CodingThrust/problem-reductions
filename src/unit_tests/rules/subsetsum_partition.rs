use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;

#[test]
fn test_subsetsum_to_partition_closed_loop() {
    // YES instance: sizes=[3,5,7,1,4], target=8
    // Sigma=20, 2T=16, d=4 -> partition_sizes=[3,5,7,1,4,4]
    let source = SubsetSum::new(vec![3u32, 5, 7, 1, 4], 8u32);
    let reduction = ReduceTo::<Partition>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "SubsetSum -> Partition closed loop",
    );
}

#[test]
fn test_subsetsum_to_partition_infeasible() {
    // NO instance: sizes=[3,7,11], target=5
    // Sigma=21, 2T=10, d=11 -> partition_sizes=[3,7,11,11]
    // Total=32, half=16 — no subset sums to 16 among {3,7,11,11}
    let source = SubsetSum::new(vec![3u32, 7, 11], 5u32);
    let reduction = ReduceTo::<Partition>::reduce_to(&source);
    let target = reduction.target_problem();

    // No witness should exist for the target partition
    let witness = BruteForce::new().find_witness(target);
    assert!(witness.is_none(), "NO instance should yield infeasible Partition");

    // Source should also be infeasible
    let source_witness = BruteForce::new().find_witness(&source);
    assert!(source_witness.is_none(), "Source SubsetSum should be infeasible");
}

#[test]
fn test_subsetsum_to_partition_structure() {
    // sizes=[3,5,7,1,4], target=8
    // Sigma=20, d=|20-16|=4 -> partition_sizes=[3,5,7,1,4,4]
    let source = SubsetSum::new(vec![3u32, 5, 7, 1, 4], 8u32);
    let reduction = ReduceTo::<Partition>::reduce_to(&source);
    let target = reduction.target_problem();

    // One extra element (the padding d=4)
    assert_eq!(target.num_elements(), source.num_elements() + 1);
    assert_eq!(target.sizes(), &[3, 5, 7, 1, 4, 4]);
    // Total sum = 20 + 4 = 24, half = 12
    assert_eq!(target.total_sum(), 24);
}

#[test]
fn test_subsetsum_to_partition_d_zero() {
    // When Sigma == 2T, no padding is added.
    // sizes=[2,3,5], target=5 -> Sigma=10, 2T=10, d=0
    // partition_sizes=[2,3,5], total=10, half=5
    let source = SubsetSum::new(vec![2u32, 3, 5], 5u32);
    let reduction = ReduceTo::<Partition>::reduce_to(&source);
    let target = reduction.target_problem();

    // Same number of elements (no padding)
    assert_eq!(target.num_elements(), source.num_elements());
    assert_eq!(target.sizes(), &[2, 3, 5]);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "SubsetSum -> Partition d=0 case",
    );
}

#[test]
fn test_subsetsum_to_partition_sigma_less_than_2t() {
    // Sigma < 2T case: sizes=[1,2,3], target=5
    // Sigma=6, 2T=10, d=4 -> partition_sizes=[1,2,3,4]
    // Total=10, half=5. Subset {1,4} or {2,3} sums to 5.
    // SubsetSum: need subset summing to 5 from {1,2,3} -> {2,3}
    let source = SubsetSum::new(vec![1u32, 2, 3], 5u32);
    let reduction = ReduceTo::<Partition>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_elements(), 4);
    assert_eq!(target.sizes(), &[1, 2, 3, 4]);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "SubsetSum -> Partition sigma < 2T case",
    );
}
