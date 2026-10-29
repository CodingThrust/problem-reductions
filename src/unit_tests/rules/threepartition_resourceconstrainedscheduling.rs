use super::*;
use crate::models::misc::{ResourceConstrainedScheduling, ThreePartition};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn reduce_three_partition(
    sizes: &[i64],
    bound: i64,
) -> (ThreePartition, ReductionThreePartitionToRCS) {
    let source = ThreePartition::new(sizes.to_vec(), bound);
    let reduction = ReduceTo::<ResourceConstrainedScheduling>::reduce_to(&source)
        .expect("reduction should succeed");
    (source, reduction)
}

fn assert_satisfiability_matches(
    source: &ThreePartition,
    target: &ResourceConstrainedScheduling,
    expected: bool,
) {
    let solver = BruteForce::new();
    assert_eq!(solver.solve(source).unwrap().is_some(), expected);
    assert_eq!(solver.solve(target).unwrap().is_some(), expected);
}

#[test]
fn test_threepartition_to_resourceconstrainedscheduling_closed_loop() {
    // sizes [4, 5, 6, 4, 6, 5], B=15, m=2
    // partition: {4,5,6} and {4,6,5}
    let (source, reduction) = reduce_three_partition(&[4, 5, 6, 4, 6, 5], 15);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "ThreePartition -> ResourceConstrainedScheduling closed loop",
    );
}

#[test]
fn test_threepartition_to_resourceconstrainedscheduling_structure() {
    let (source, reduction) = reduce_three_partition(&[4, 5, 6, 4, 6, 5], 15);
    let target = reduction.target_problem();

    assert_eq!(target.num_tasks(), 6);
    assert_eq!(target.num_tasks(), source.num_elements());
    assert_eq!(target.num_processors(), 3);
    assert_eq!(target.num_resources(), 1);
    assert_eq!(target.resource_bounds(), &[15]);
    assert_eq!(target.deadline(), 2); // m = 6/3 = 2

    for (i, req) in target.resource_requirements().iter().enumerate() {
        assert_eq!(req.len(), 1);
        assert_eq!(req[0], source.sizes()[i]);
    }
}

#[test]
fn test_threepartition_to_resourceconstrainedscheduling_solution_extraction() {
    let (source, reduction) = reduce_three_partition(&[4, 5, 6, 4, 6, 5], 15);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_solutions = solver.find_all_witnesses(target).unwrap();

    for sol in &target_solutions {
        let extracted = reduction.extract_solution(sol).unwrap();
        assert_eq!(extracted.len(), source.num_elements());
        let target_valid = target.evaluate(sol).unwrap();
        let source_valid = source.evaluate(&extracted).unwrap();
        if target_valid.0 {
            assert!(
                source_valid.0,
                "Valid RCS solution should yield valid ThreePartition"
            );
        }
    }
}

#[test]
fn test_threepartition_to_resourceconstrainedscheduling_single_triple() {
    // m=1: sizes [4, 5, 6], B=15
    let (source, reduction) = reduce_three_partition(&[4, 5, 6], 15);
    let target = reduction.target_problem();

    assert_eq!(target.num_tasks(), 3);
    assert_eq!(target.deadline(), 1); // m=1
    assert_eq!(target.num_processors(), 3);

    assert_satisfiability_matches(&source, target, true);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "ThreePartition -> RCS single triple",
    );
}

#[test]
fn test_threepartition_to_resourceconstrainedscheduling_two_triples() {
    // Two {6,7,8} triples meet B=21, with each size strictly between B/4 and B/2.
    let (source, reduction) = reduce_three_partition(&[6, 7, 8, 6, 7, 8], 21);
    let target = reduction.target_problem();

    assert_satisfiability_matches(&source, target, true);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "ThreePartition -> RCS two triples",
    );
}
