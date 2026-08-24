use crate::models::set::{ThreeDimensionalMatching, ThreeMatroidIntersection};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionGraph, ReductionResult};
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn feasible_problem() -> ThreeDimensionalMatching {
    ThreeDimensionalMatching::new(
        3,
        vec![(0, 0, 0), (1, 1, 1), (2, 2, 2), (0, 1, 2), (1, 2, 0)],
    )
}

#[test]
fn test_threedimensionalmatching_to_threematroidintersection_structure() {
    let source = ThreeDimensionalMatching::new(2, vec![(0, 0, 0), (0, 1, 1), (1, 0, 1), (1, 1, 0)]);
    let reduction =
        ReduceTo::<ThreeMatroidIntersection>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.ground_set_size(), source.num_triples());
    assert_eq!(target.bound(), source.universe_size());
    assert_eq!(
        target.partitions(),
        &[
            vec![vec![0, 1], vec![2, 3]],
            vec![vec![0, 2], vec![1, 3]],
            vec![vec![0, 3], vec![1, 2]],
        ]
    );
    assert_eq!(target.num_groups(), 6);
}

#[test]
fn test_threedimensionalmatching_to_threematroidintersection_closed_loop() {
    let source = feasible_problem();
    let reduction =
        ReduceTo::<ThreeMatroidIntersection>::reduce_to(&source).expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "ThreeDimensionalMatching -> ThreeMatroidIntersection closed loop",
    );
}

#[test]
fn test_threedimensionalmatching_to_threematroidintersection_issue_no_instance() {
    let source = ThreeDimensionalMatching::new(2, vec![(0, 0, 0), (0, 1, 1), (1, 0, 1), (1, 1, 0)]);
    let reduction =
        ReduceTo::<ThreeMatroidIntersection>::reduce_to(&source).expect("reduction should succeed");

    assert!(
        BruteForce::new().find_witness(&source).unwrap().is_none(),
        "issue example should have no perfect matching"
    );
    assert!(
        BruteForce::new()
            .find_witness(reduction.target_problem())
            .unwrap()
            .is_none(),
        "reduced 3-matroid intersection instance should be infeasible"
    );
}

#[test]
fn test_threedimensionalmatching_to_threematroidintersection_missing_coordinate_creates_empty_group(
) {
    let source = ThreeDimensionalMatching::new(2, vec![(0, 0, 0), (1, 0, 1)]);
    let reduction =
        ReduceTo::<ThreeMatroidIntersection>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.partitions()[1], vec![vec![0, 1], vec![]]);
    assert!(
        BruteForce::new().find_witness(target).unwrap().is_none(),
        "an empty coordinate group makes size-q independence impossible"
    );
}

#[test]
fn test_threedimensionalmatching_to_threematroidintersection_direct_path_exists() {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&ThreeDimensionalMatching::variant());
    let dst = ReductionGraph::variant_to_map(&ThreeMatroidIntersection::variant());

    let path = graph
        .find_all_paths(
            "ThreeDimensionalMatching",
            &src,
            "ThreeMatroidIntersection",
            &dst,
        )
        .into_iter()
        .find(|path| path.type_names() == ["ThreeDimensionalMatching", "ThreeMatroidIntersection"])
        .expect("reduction graph should contain the direct 3DM -> 3MI edge");

    assert_eq!(
        path.type_names(),
        vec!["ThreeDimensionalMatching", "ThreeMatroidIntersection"]
    );
}
