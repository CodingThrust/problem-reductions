use super::*;
use crate::models::misc::Partition;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_partition_to_multiprocessorscheduling_closed_loop() {
    // sizes [1, 2, 3, 4], sum=10, target=5
    // partition: {1,4} and {2,3}
    let source = Partition::new(vec![1, 2, 3, 4]);
    let reduction = ReduceTo::<MultiprocessorScheduling>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> MultiprocessorScheduling closed loop",
    );
}

#[test]
fn test_partition_to_multiprocessorscheduling_structure() {
    let source = Partition::new(vec![1, 2, 3, 4]);
    let reduction = ReduceTo::<MultiprocessorScheduling>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.lengths(), &[1, 2, 3, 4]);
    assert_eq!(target.num_processors(), 2);
    assert_eq!(target.deadline(), 5); // sum=10, 10/2=5
    assert_eq!(target.num_tasks(), source.num_elements());
}

#[test]
fn test_partition_to_multiprocessorscheduling_odd_sum() {
    // sum = 2+4+5 = 11 (odd), no valid partition exists
    let source = Partition::new(vec![2, 4, 5]);
    let reduction = ReduceTo::<MultiprocessorScheduling>::reduce_to(&source);
    let target = reduction.target_problem();

    // deadline = floor(11/2) = 5
    assert_eq!(target.deadline(), 5);
    assert_eq!(target.num_processors(), 2);

    // Source is not satisfiable (odd sum)
    let solver = BruteForce::new();
    let source_solutions = solver.find_all_witnesses(&source);
    assert!(source_solutions.is_empty());

    // Target should also be infeasible: total=11, floor(11/2)=5,
    // so one processor must get at least ceil(11/2)=6 > 5
    let target_solutions = solver.find_all_witnesses(target);
    assert!(target_solutions.is_empty());
}

#[test]
fn test_partition_to_multiprocessorscheduling_equal_elements() {
    // [3, 3, 3, 3], sum=12, target=6
    let source = Partition::new(vec![3, 3, 3, 3]);
    let reduction = ReduceTo::<MultiprocessorScheduling>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.deadline(), 6);

    // Both should be satisfiable (e.g., {3,3} and {3,3})
    let solver = BruteForce::new();
    let source_solutions = solver.find_all_witnesses(&source);
    let target_solutions = solver.find_all_witnesses(target);

    assert!(!source_solutions.is_empty());
    assert!(!target_solutions.is_empty());

    // Round trip should work
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> MPS equal elements",
    );
}

#[test]
fn test_partition_to_multiprocessorscheduling_solution_extraction() {
    let source = Partition::new(vec![1, 2, 3, 4]);
    let reduction = ReduceTo::<MultiprocessorScheduling>::reduce_to(&source);
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
    let source = Partition::new(vec![4]);
    let reduction = ReduceTo::<MultiprocessorScheduling>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.deadline(), 2);
    assert_eq!(target.num_tasks(), 1);
    assert_eq!(target.lengths(), &[4]);

    // Source: not satisfiable (single element, total_sum=4, need 2 per subset but only have 4)
    let solver = BruteForce::new();
    let source_solutions = solver.find_all_witnesses(&source);
    assert!(source_solutions.is_empty());

    // Target: task of length 4 > deadline 2, so infeasible
    let target_solutions = solver.find_all_witnesses(target);
    assert!(target_solutions.is_empty());
}

#[test]
fn test_partition_to_multiprocessorscheduling_two_elements() {
    // [5, 5], sum=10, target=5
    let source = Partition::new(vec![5, 5]);
    let reduction = ReduceTo::<MultiprocessorScheduling>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.deadline(), 5);

    let solver = BruteForce::new();
    let source_solutions = solver.find_all_witnesses(&source);
    let target_solutions = solver.find_all_witnesses(target);

    // Both should be satisfiable: one element per subset/processor
    assert!(!source_solutions.is_empty());
    assert!(!target_solutions.is_empty());

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> MPS two elements",
    );
}
