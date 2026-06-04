use super::*;
use crate::models::misc::{Partition, SumOfSquaresPartition};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Min;

fn reduce_partition(sizes: &[u64]) -> (Partition, ReductionPartitionToSumOfSquaresPartition) {
    let source = Partition::new(sizes.to_vec());
    let reduction = ReduceTo::<SumOfSquaresPartition>::reduce_to(&source);
    (source, reduction)
}

#[test]
fn test_partition_to_sumofsquarespartition_closed_loop() {
    // YES case: sizes [3, 1, 1, 2, 2, 1], S = 10, balanced split sums to 5.
    let (source, reduction) = reduce_partition(&[3, 1, 1, 2, 2, 1]);
    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "Partition -> SumOfSquaresPartition closed loop (YES, balanced)",
    );

    // Even-sum but unbalanced NO case: sizes [1, 1, 1, 5], S = 8 but no subset sums to 4.
    // The optimal SoSP witness is {5}, {1,1,1} -> 25 + 9 = 34 > S^2/2 = 32.
    // Partition::evaluate on that witness must return Or(false).
    let (source_no_even, reduction_no_even) = reduce_partition(&[1, 1, 1, 5]);
    let target_no_even = reduction_no_even.target_problem();
    let solver = BruteForce::new();
    let target_witnesses = solver.find_all_witnesses(target_no_even);
    assert!(!target_witnesses.is_empty());
    for witness in &target_witnesses {
        let extracted = reduction_no_even.extract_solution(witness);
        assert_eq!(extracted.len(), source_no_even.num_elements());
        assert!(
            !source_no_even.evaluate(&extracted).0,
            "even-sum but unbalanced NO Partition: extracted witness {extracted:?} should not satisfy source"
        );
    }
    // Confirm the source is genuinely NO via direct solve.
    let direct_witness = solver.find_witness(&source_no_even);
    assert!(direct_witness.is_none());

    // Odd-sum NO case: sizes [2, 4, 5], S = 11.
    let (source_no_odd, reduction_no_odd) = reduce_partition(&[2, 4, 5]);
    let target_no_odd = reduction_no_odd.target_problem();
    let target_witnesses_odd = solver.find_all_witnesses(target_no_odd);
    assert!(!target_witnesses_odd.is_empty());
    for witness in &target_witnesses_odd {
        let extracted = reduction_no_odd.extract_solution(witness);
        assert!(
            !source_no_odd.evaluate(&extracted).0,
            "odd-sum NO Partition: extracted witness {extracted:?} should not satisfy source"
        );
    }
    assert!(solver.find_witness(&source_no_odd).is_none());
}

#[test]
fn test_partition_to_sumofsquarespartition_structure() {
    let (source, reduction) = reduce_partition(&[3, 1, 1, 2, 2, 1]);
    let target = reduction.target_problem();

    assert_eq!(target.sizes(), &[3i64, 1, 1, 2, 2, 1]);
    assert_eq!(target.num_groups(), 2);
    assert_eq!(target.num_elements(), source.num_elements());
}

#[test]
fn test_partition_to_sumofsquarespartition_optimal_value_yes() {
    // YES case: S = 10, expected optimum S^2/2 = 50.
    let (_source, reduction) = reduce_partition(&[3, 1, 1, 2, 2, 1]);
    let target = reduction.target_problem();
    let solver = BruteForce::new();
    let optimal = solver.solve(target);
    assert_eq!(optimal, Min(Some(50)));
}

#[test]
fn test_partition_to_sumofsquarespartition_optimal_value_no_even() {
    // Even-sum but unbalanced: S = 8. The best 2-split is {5} vs {1,1,1} -> 25+9=34.
    let (_source, reduction) = reduce_partition(&[1, 1, 1, 5]);
    let target = reduction.target_problem();
    let solver = BruteForce::new();
    let optimal = solver.solve(target);
    assert_eq!(optimal, Min(Some(34)));
    // Strictly greater than S^2/2 = 32.
    assert!(optimal.0.unwrap() > 32);
}

#[test]
fn test_partition_to_sumofsquarespartition_singleton_sentinel() {
    // n < 2 sentinel path: a single element cannot be partitioned into two
    // equal-sum subsets. The sentinel target has two unit elements.
    let (source, reduction) = reduce_partition(&[5]);
    let target = reduction.target_problem();

    assert_eq!(target.sizes(), &[1i64, 1]);
    assert_eq!(target.num_groups(), 2);
    assert_eq!(target.num_elements(), 2);

    let solver = BruteForce::new();
    let target_witnesses = solver.find_all_witnesses(target);
    assert!(!target_witnesses.is_empty());

    for witness in &target_witnesses {
        let extracted = reduction.extract_solution(witness);
        assert_eq!(extracted.len(), source.num_elements());
        assert_eq!(extracted, vec![0]);
        assert!(
            !source.evaluate(&extracted).0,
            "singleton Partition: extracted witness must yield Or(false)"
        );
    }

    // Direct solve confirms the source is NO.
    assert!(solver.find_witness(&source).is_none());
}

#[test]
fn test_partition_to_sumofsquarespartition_solution_extraction_identity() {
    // For the canonical YES case, extracted solutions must be a subset of
    // direct-solver source witnesses (i.e. genuinely balanced partitions).
    let (source, reduction) = reduce_partition(&[3, 1, 1, 2, 2, 1]);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_witnesses = solver.find_all_witnesses(target);
    let source_witnesses: std::collections::HashSet<Vec<usize>> =
        solver.find_all_witnesses(&source).into_iter().collect();

    for witness in &target_witnesses {
        let extracted = reduction.extract_solution(witness);
        assert_eq!(extracted, *witness);
        assert!(
            source_witnesses.contains(&extracted),
            "extracted witness {extracted:?} must be a valid Partition solution"
        );
    }
}
