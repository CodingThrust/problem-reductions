use super::*;
use crate::models::misc::Partition;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn reduce_partition(sizes: &[u64]) -> (Partition, ReductionPartitionToMPS) {
    let source = Partition::new(sizes.to_vec());
    let reduction = ReduceTo::<MultiprocessorScheduling>::reduce_to(&source);
    (source, reduction)
}

fn assert_satisfiability_matches(
    source: &Partition,
    target: &MultiprocessorScheduling,
    expected: bool,
) {
    let solver = BruteForce::new();
    assert_eq!(solver.find_witness(source).is_some(), expected);
    assert_eq!(solver.find_witness(target).is_some(), expected);
}

#[test]
fn test_partition_to_multiprocessorscheduling_closed_loop() {
    // sizes [1, 2, 3, 4], sum=10, target=5
    // partition: {1,4} and {2,3}
    let (source, reduction) = reduce_partition(&[1, 2, 3, 4]);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> MultiprocessorScheduling closed loop",
    );
}

#[test]
fn test_partition_to_multiprocessorscheduling_structure() {
    let (source, reduction) = reduce_partition(&[1, 2, 3, 4]);
    let target = reduction.target_problem();

    assert_eq!(target.lengths(), &[1, 2, 3, 4]);
    assert_eq!(target.num_processors(), 2);
    assert_eq!(target.deadline(), 5); // sum=10, 10/2=5
    assert_eq!(target.num_tasks(), source.num_elements());
}

#[test]
fn test_partition_to_multiprocessorscheduling_odd_sum() {
    // sum = 2+4+5 = 11 (odd), no valid partition exists
    let (source, reduction) = reduce_partition(&[2, 4, 5]);
    let target = reduction.target_problem();

    // deadline = floor(11/2) = 5
    assert_eq!(target.deadline(), 5);
    assert_eq!(target.num_processors(), 2);

    assert_satisfiability_matches(&source, target, false);
}

#[test]
fn test_partition_to_multiprocessorscheduling_equal_elements() {
    // [3, 3, 3, 3], sum=12, target=6
    let (source, reduction) = reduce_partition(&[3, 3, 3, 3]);
    let target = reduction.target_problem();

    assert_eq!(target.deadline(), 6);
    assert_satisfiability_matches(&source, target, true);

    // Round trip should work
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> MPS equal elements",
    );
}

#[test]
fn test_partition_to_multiprocessorscheduling_solution_extraction() {
    let (source, reduction) = reduce_partition(&[1, 2, 3, 4]);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_solutions = solver.find_all_witnesses(target);

    for sol in &target_solutions {
        let extracted = reduction.extract_solution(sol);
        // Solution length should match number of elements
        assert_eq!(extracted.len(), source.num_elements());
        // Extracted solution should satisfy source if target is satisfied
        let target_valid = target.evaluate(sol);
        let source_valid = source.evaluate(&extracted);
        if target_valid.0 {
            assert!(
                source_valid.0,
                "Valid MPS solution should yield valid Partition"
            );
        }
    }
}

#[test]
fn test_partition_to_multiprocessorscheduling_single_element() {
    // Single element: [4], sum=4, target=2
    // Not partitionable since we need 2 subsets summing to 2 each but only have one element of size 4
    let (source, reduction) = reduce_partition(&[4]);
    let target = reduction.target_problem();

    assert_eq!(target.deadline(), 2);
    assert_eq!(target.num_tasks(), 1);
    assert_eq!(target.lengths(), &[4]);

    assert_satisfiability_matches(&source, target, false);
}

#[test]
fn test_partition_to_multiprocessorscheduling_two_elements() {
    // [5, 5], sum=10, target=5
    let (source, reduction) = reduce_partition(&[5, 5]);
    let target = reduction.target_problem();

    assert_eq!(target.deadline(), 5);
    assert_satisfiability_matches(&source, target, true);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> MPS two elements",
    );
}
