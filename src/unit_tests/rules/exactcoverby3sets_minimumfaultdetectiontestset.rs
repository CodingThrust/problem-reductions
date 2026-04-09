use super::*;
use crate::models::misc::MinimumFaultDetectionTestSet;
use crate::models::set::ExactCoverBy3Sets;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

fn issue_yes_instance() -> ExactCoverBy3Sets {
    ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]])
}

fn no_cover_instance() -> ExactCoverBy3Sets {
    ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [0, 3, 4], [0, 4, 5]])
}

#[test]
fn test_exactcoverby3sets_to_minimumfaultdetectiontestset_closed_loop() {
    let source = issue_yes_instance();
    let reduction = ReduceTo::<MinimumFaultDetectionTestSet>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "ExactCoverBy3Sets -> MinimumFaultDetectionTestSet closed loop",
    );

    let best = BruteForce::new()
        .find_witness(target)
        .expect("expected an optimal target witness");
    assert_eq!(target.evaluate(&best), Min(Some(2)));
}

#[test]
fn test_exactcoverby3sets_to_minimumfaultdetectiontestset_structure() {
    let source = issue_yes_instance();
    let reduction = ReduceTo::<MinimumFaultDetectionTestSet>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 10);
    assert_eq!(target.num_arcs(), 15);
    assert_eq!(target.inputs(), &[0, 1, 2]);
    assert_eq!(target.outputs(), &[9]);

    assert_eq!(
        target.arcs(),
        &[
            (0, 3),
            (0, 4),
            (0, 5),
            (1, 6),
            (1, 7),
            (1, 8),
            (2, 3),
            (2, 6),
            (2, 7),
            (3, 9),
            (4, 9),
            (5, 9),
            (6, 9),
            (7, 9),
            (8, 9),
        ]
    );
}

#[test]
fn test_exactcoverby3sets_to_minimumfaultdetectiontestset_no_instance_gap() {
    let source = no_cover_instance();
    let reduction = ReduceTo::<MinimumFaultDetectionTestSet>::reduce_to(&source);
    let target = reduction.target_problem();

    let best = BruteForce::new()
        .find_witness(target)
        .expect("expected an optimal target witness");
    assert_eq!(target.evaluate(&best), Min(Some(3)));

    let extracted = reduction.extract_solution(&best);
    assert!(!source.evaluate(&extracted));
}

#[test]
fn test_exactcoverby3sets_to_minimumfaultdetectiontestset_extract_solution_identity() {
    let source = issue_yes_instance();
    let reduction = ReduceTo::<MinimumFaultDetectionTestSet>::reduce_to(&source);

    assert_eq!(reduction.extract_solution(&[1, 1, 0]), vec![1, 1, 0]);
    assert!(source.evaluate(&[1, 1, 0]).0);
}
