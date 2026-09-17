use super::*;
use crate::models::misc::{Partition, SumOfSquaresPartition};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::solvers::SolveOutcome;
use crate::traits::EvaluationError::InvalidConfiguration;
use crate::traits::Problem;
use crate::types::Min;

fn reduce_partition(sizes: &[i64]) -> (Partition, ReductionPartitionToSumOfSquaresPartition) {
    let source = Partition::new(sizes.to_vec()).unwrap();
    let reduction =
        ReduceTo::<SumOfSquaresPartition>::reduce_to(&source).expect("reduction should succeed");
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
    let target_witnesses = solver.find_all_witnesses(target_no_even).unwrap();
    assert!(!target_witnesses.is_empty());
    for witness in &target_witnesses {
        let recovered = reduction_no_even
            .recover_result(
                &source_no_even,
                SolveOutcome::optimal(target_no_even, witness.clone()).unwrap(),
            )
            .unwrap();
        assert_eq!(recovered, SolveOutcome::Infeasible);
    }
    // Confirm the source is genuinely NO via direct solve.
    let direct_witness = solver.solve(&source_no_even).unwrap();
    assert!(direct_witness.is_none());

    // Odd-sum NO case: sizes [2, 4, 5], S = 11.
    let (source_no_odd, reduction_no_odd) = reduce_partition(&[2, 4, 5]);
    let target_no_odd = reduction_no_odd.target_problem();
    let target_witnesses_odd = solver.find_all_witnesses(target_no_odd).unwrap();
    assert!(!target_witnesses_odd.is_empty());
    for witness in &target_witnesses_odd {
        let recovered = reduction_no_odd
            .recover_result(
                &source_no_odd,
                SolveOutcome::optimal(target_no_odd, witness.clone()).unwrap(),
            )
            .unwrap();
        assert_eq!(recovered, SolveOutcome::Infeasible);
    }
    assert!(solver.solve(&source_no_odd).unwrap().is_none());
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
    let optimal_solution = solver.solve(target).unwrap().unwrap();
    let optimal = target.evaluate(&optimal_solution).unwrap();
    assert_eq!(optimal, Min(Some(50)));
}

#[test]
fn test_partition_to_sumofsquarespartition_optimal_value_no_even() {
    // Even-sum but unbalanced: S = 8. The best 2-split is {5} vs {1,1,1} -> 25+9=34.
    let (_source, reduction) = reduce_partition(&[1, 1, 1, 5]);
    let target = reduction.target_problem();
    let solver = BruteForce::new();
    let optimal_solution = solver.solve(target).unwrap().unwrap();
    let optimal = target.evaluate(&optimal_solution).unwrap();
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
    let target_witnesses = solver.find_all_witnesses(target).unwrap();
    assert!(!target_witnesses.is_empty());

    for witness in &target_witnesses {
        let mapped = reduction.map_solution(witness).unwrap();
        assert_eq!(mapped.len(), source.num_elements());
        assert_eq!(
            mapped,
            witness[..source.num_elements()]
                .iter()
                .map(|&value| value != 0)
                .collect::<Vec<_>>()
        );
        assert!(!source.evaluate(&mapped).unwrap().0);
        assert_eq!(
            reduction
                .recover_result(
                    &source,
                    SolveOutcome::optimal(target, witness.clone()).unwrap()
                )
                .unwrap(),
            SolveOutcome::Infeasible
        );
    }

    // Direct solve confirms the source is NO.
    assert!(solver.solve(&source).unwrap().is_none());
}

#[test]
fn test_partition_to_sumofsquarespartition_solution_extraction_identity() {
    // For the canonical YES case, extracted solutions must be a subset of
    // direct-solver source witnesses (i.e. genuinely balanced partitions).
    let (source, reduction) = reduce_partition(&[3, 1, 1, 2, 2, 1]);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_witnesses = solver.find_all_witnesses(target).unwrap();
    let source_witnesses: std::collections::HashSet<Vec<bool>> = solver
        .find_all_witnesses(&source)
        .unwrap()
        .into_iter()
        .collect();

    for witness in &target_witnesses {
        let extracted = reduction
            .recover_result(
                &source,
                SolveOutcome::optimal(reduction.target_problem(), (witness).clone()).unwrap(),
            )
            .unwrap()
            .into_solution()
            .expect("qualifying target result must recover a source solution");
        assert_eq!(
            extracted,
            witness.iter().map(|&value| value != 0).collect::<Vec<_>>()
        );
        assert!(
            source_witnesses.contains(&extracted),
            "extracted witness {extracted:?} must be a valid Partition solution"
        );
    }

    assert!(matches!(
        ReductionResult::target_problem(&reduction).evaluate(&vec![0]),
        Err(InvalidConfiguration(_))
    ));
}

#[test]
fn test_partition_to_sumofsquarespartition_feasible_target_incumbents() {
    // sizes [3, 1, 1, 2, 2, 1], S = 10.
    let (source, reduction) = reduce_partition(&[3, 1, 1, 2, 2, 1]);
    let target = reduction.target_problem();

    // {3, 2} | {1, 1, 2, 1}: 5^2 + 5^2 = 50 = S^2 / 2, already a balanced partition.
    let balanced = vec![0, 1, 1, 0, 1, 1];
    assert_eq!(target.evaluate(&balanced).unwrap(), Min(Some(50)));
    assert_eq!(
        reduction.recover_result(&source, SolveOutcome::feasible(target, balanced).unwrap()),
        Ok(SolveOutcome::Feasible {
            solution: vec![false, true, true, false, true, true],
            evaluation: crate::types::Or(true),
        })
    );

    // {3, 1, 1, 2} | {2, 1}: 7^2 + 3^2 = 58 > 50 proves nothing about the source.
    let unbalanced = vec![0, 0, 0, 0, 1, 1];
    assert_eq!(target.evaluate(&unbalanced).unwrap(), Min(Some(58)));
    assert_eq!(
        reduction.recover_result(&source, SolveOutcome::feasible(target, unbalanced).unwrap()),
        Err(crate::rules::ExtractionError::InsufficientSolutionQuality)
    );
}
