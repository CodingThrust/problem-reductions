use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;

#[test]
fn test_partition_to_subsetsum_closed_loop() {
    let source = Partition::new(vec![3, 1, 1, 2, 2, 1]).unwrap();
    let reduction = ReduceTo::<SubsetSum>::reduce_to(&source).expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> SubsetSum closed loop",
    );
}

#[test]
fn test_partition_to_subsetsum_structure() {
    let source = Partition::new(vec![3, 1, 1, 2, 2, 1]).unwrap();
    let reduction = ReduceTo::<SubsetSum>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    // Same number of elements
    assert_eq!(target.num_elements(), source.num_elements());
    // Target is S/2 = 10/2 = 5
    assert_eq!(*target.target(), num_bigint::BigUint::from(5u32));
    // Sizes are preserved
    let expected_sizes: Vec<num_bigint::BigUint> = vec![3u32, 1, 1, 2, 2, 1]
        .into_iter()
        .map(num_bigint::BigUint::from)
        .collect();
    assert_eq!(target.sizes(), &expected_sizes);
}

#[test]
fn test_partition_to_subsetsum_odd_total() {
    // Odd total sum: 2 + 4 + 5 = 11, no balanced partition possible
    let source = Partition::new(vec![2, 4, 5]).unwrap();
    let reduction = ReduceTo::<SubsetSum>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    // Trivially infeasible: empty sizes, target = 1
    assert_eq!(target.num_elements(), 0);
    assert_eq!(*target.target(), num_bigint::BigUint::from(1u32));

    // No witness should exist for the target
    let witness = BruteForce::new().find_witness(target).unwrap();
    assert!(witness.is_none());

    let error = reduction.extract_solution(&[]).unwrap_err();
    assert_eq!(
        error.to_string(),
        "expected 3 subset-selection values, got 0"
    );
}

#[test]
fn test_partition_to_subsetsum_equal_elements() {
    // All equal: [2, 2, 2, 2], total = 8, target = 4
    let source = Partition::new(vec![2, 2, 2, 2]).unwrap();
    let reduction = ReduceTo::<SubsetSum>::reduce_to(&source).expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> SubsetSum equal elements",
    );
}

#[test]
fn test_partition_to_subsetsum_rejects_wrong_solution_length() {
    let source = Partition::new(vec![1, 1, 2, 2]).unwrap();
    let reduction = ReduceTo::<SubsetSum>::reduce_to(&source).expect("reduction should succeed");

    assert!(reduction.extract_solution(&[0, 1, 0]).is_err());
}
