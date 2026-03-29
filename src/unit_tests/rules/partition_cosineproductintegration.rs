use super::*;
use crate::models::misc::{CosineProductIntegration, Partition};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn reduce_partition(sizes: &[u64]) -> (Partition, ReductionPartitionToCPI) {
    let source = Partition::new(sizes.to_vec());
    let reduction = ReduceTo::<CosineProductIntegration>::reduce_to(&source);
    (source, reduction)
}

fn assert_satisfiability_matches(
    source: &Partition,
    target: &CosineProductIntegration,
    expected: bool,
) {
    let solver = BruteForce::new();
    assert_eq!(solver.find_witness(source).is_some(), expected);
    assert_eq!(solver.find_witness(target).is_some(), expected);
}

#[test]
fn test_partition_to_cosineproductintegration_closed_loop() {
    let (source, reduction) = reduce_partition(&[3, 1, 1, 2, 2, 1]);
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> CosineProductIntegration closed loop",
    );
}

#[test]
fn test_partition_to_cosineproductintegration_structure() {
    let (source, reduction) = reduce_partition(&[2, 3, 5]);
    let target = reduction.target_problem();

    assert_eq!(target.coefficients(), &[2, 3, 5]);
    assert_eq!(target.num_coefficients(), source.num_elements());
}

#[test]
fn test_partition_to_cosineproductintegration_odd_sum() {
    // sum = 2+4+5 = 11 (odd), no valid partition / no balanced sign assignment
    let (source, reduction) = reduce_partition(&[2, 4, 5]);
    let target = reduction.target_problem();
    assert_satisfiability_matches(&source, target, false);
}

#[test]
fn test_partition_to_cosineproductintegration_equal_elements() {
    let (source, reduction) = reduce_partition(&[3, 3, 3, 3]);
    let target = reduction.target_problem();
    assert_satisfiability_matches(&source, target, true);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> CPI equal elements",
    );
}

#[test]
fn test_partition_to_cosineproductintegration_solution_extraction() {
    let (source, reduction) = reduce_partition(&[1, 2, 3, 4]);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_solutions = solver.find_all_witnesses(target);

    for sol in &target_solutions {
        let extracted = reduction.extract_solution(sol);
        assert_eq!(extracted.len(), source.num_elements());
        let target_valid = target.evaluate(sol);
        let source_valid = source.evaluate(&extracted);
        if target_valid.0 {
            assert!(
                source_valid.0,
                "Valid CPI solution should yield valid Partition"
            );
        }
    }
}

#[test]
fn test_partition_to_cosineproductintegration_single_element() {
    let (source, reduction) = reduce_partition(&[4]);
    let target = reduction.target_problem();
    assert_satisfiability_matches(&source, target, false);
}

#[test]
fn test_partition_to_cosineproductintegration_two_elements() {
    let (source, reduction) = reduce_partition(&[5, 5]);
    let target = reduction.target_problem();
    assert_satisfiability_matches(&source, target, true);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> CPI two elements",
    );
}
