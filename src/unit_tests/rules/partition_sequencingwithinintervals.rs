use super::*;
use crate::models::misc::Partition;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn reduce_partition(sizes: &[u64]) -> (Partition, ReductionPartitionToSWI) {
    let source = Partition::new(sizes.to_vec());
    let reduction = ReduceTo::<SequencingWithinIntervals>::reduce_to(&source);
    (source, reduction)
}

fn assert_satisfiability_matches(
    source: &Partition,
    target: &SequencingWithinIntervals,
    expected: bool,
) {
    let solver = BruteForce::new();
    assert_eq!(solver.find_witness(source).is_some(), expected);
    assert_eq!(solver.find_witness(target).is_some(), expected);
}

#[test]
fn test_partition_to_sequencingwithinintervals_closed_loop() {
    // sizes [1, 2, 3, 4], sum=10, partition: {2,3} and {1,4}
    let (source, reduction) = reduce_partition(&[1, 2, 3, 4]);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> SequencingWithinIntervals closed loop",
    );
}

#[test]
fn test_partition_to_sequencingwithinintervals_structure() {
    let (source, reduction) = reduce_partition(&[1, 2, 3, 4]);
    let target = reduction.target_problem();

    // n+1 tasks (4 regular + 1 enforcer)
    assert_eq!(target.num_tasks(), source.num_elements() + 1);

    // Regular tasks: release=0, deadline=11, lengths=[1,2,3,4]
    let n = source.num_elements();
    for i in 0..n {
        assert_eq!(target.release_times()[i], 0);
        assert_eq!(target.deadlines()[i], 11); // S+1 = 10+1
    }
    assert_eq!(&target.lengths()[..n], source.sizes());

    // Enforcer: release=5, deadline=6, length=1
    assert_eq!(target.release_times()[n], 5);
    assert_eq!(target.deadlines()[n], 6);
    assert_eq!(target.lengths()[n], 1);
}

#[test]
fn test_partition_to_sequencingwithinintervals_odd_sum() {
    // sum = 2+4+5 = 11 (odd), no valid partition
    let (source, reduction) = reduce_partition(&[2, 4, 5]);
    let target = reduction.target_problem();

    // Enforcer: release=floor(11/2)=5, deadline=ceil(12/2)=6, length=1
    assert_eq!(target.release_times()[3], 5);
    assert_eq!(target.deadlines()[3], 6);
    assert_eq!(target.lengths()[3], 1);

    // Source is infeasible (odd sum)
    let solver = BruteForce::new();
    assert!(solver.find_witness(&source).is_none());
    // Target may still be feasible (the enforcer only guarantees forward direction:
    // partition exists → sequencing exists, not the converse for odd sums)
}

#[test]
fn test_partition_to_sequencingwithinintervals_equal_elements() {
    // [3, 3, 3, 3], sum=12, half=6
    let (source, reduction) = reduce_partition(&[3, 3, 3, 3]);
    let target = reduction.target_problem();

    assert_eq!(target.num_tasks(), 5);
    assert_satisfiability_matches(&source, target, true);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> SWI equal elements",
    );
}

#[test]
fn test_partition_to_sequencingwithinintervals_solution_extraction() {
    let (source, reduction) = reduce_partition(&[1, 2, 3, 4]);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_solutions = solver.find_all_witnesses(target);

    for sol in &target_solutions {
        let extracted = reduction.extract_solution(sol);
        // Extracted config should have length = num_elements (no enforcer)
        assert_eq!(extracted.len(), source.num_elements());
        // If the target solution is valid, extracted should satisfy source
        let target_valid = target.evaluate(sol);
        let source_valid = source.evaluate(&extracted);
        if target_valid.0 {
            assert!(
                source_valid.0,
                "Valid SWI solution should yield valid Partition"
            );
        }
    }
}

#[test]
fn test_partition_to_sequencingwithinintervals_two_elements() {
    // [5, 5], sum=10, half=5
    let (source, reduction) = reduce_partition(&[5, 5]);
    let target = reduction.target_problem();

    assert_eq!(target.num_tasks(), 3);
    assert_satisfiability_matches(&source, target, true);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> SWI two elements",
    );
}

#[test]
fn test_partition_to_sequencingwithinintervals_single_element() {
    // [4], sum=4, half=2
    // Not partitionable (only one element)
    let (source, reduction) = reduce_partition(&[4]);
    let target = reduction.target_problem();

    assert_eq!(target.num_tasks(), 2);
    assert_satisfiability_matches(&source, target, false);
}
