use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

fn issue_problem() -> StackerCrane {
    StackerCrane::new(
        6,
        vec![(0, 4), (2, 5), (5, 1), (3, 0), (4, 3)],
        vec![(0, 1), (1, 2), (2, 3), (3, 5), (4, 5), (0, 3), (1, 5)],
        vec![3, 4, 2, 5, 3],
        vec![2, 1, 3, 2, 1, 4, 3],
    )
}

fn small_problem() -> StackerCrane {
    StackerCrane::new(3, vec![(0, 1), (1, 2)], vec![(0, 2)], vec![1, 1], vec![1])
}

#[test]
fn test_stacker_crane_creation_and_metadata() {
    let problem = issue_problem();

    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_arcs(), 5);
    assert_eq!(problem.num_edges(), 7);
    assert_eq!(problem.dims(), vec![5; 5]);
    assert_eq!(<StackerCrane as Problem>::NAME, "StackerCrane");
    assert!(<StackerCrane as Problem>::variant().is_empty());
}

#[test]
fn test_stacker_crane_rejects_non_permutations_and_wrong_lengths() {
    let problem = issue_problem();

    assert_eq!(problem.evaluate(&[0, 2, 1, 4, 4]), Min(None));
    assert_eq!(problem.evaluate(&[0, 2, 1, 4, 5]), Min(None));
    assert_eq!(problem.evaluate(&[0, 2, 1, 4]), Min(None));
    assert_eq!(problem.evaluate(&[0, 2, 1, 4, 3, 0]), Min(None));
}

#[test]
fn test_stacker_crane_issue_witness_value() {
    let problem = issue_problem();
    assert_eq!(problem.evaluate(&[0, 2, 1, 4, 3]), Min(Some(20)));
}

#[test]
fn test_stacker_crane_paper_example() {
    let problem = issue_problem();
    let witness = vec![0, 2, 1, 4, 3];

    assert_eq!(problem.closed_walk_length(&witness), Some(20));
    assert_eq!(problem.evaluate(&witness), Min(Some(20)));

    let solver = BruteForce::new();
    let optimal = solver
        .find_witness(&problem)
        .expect("should have a witness");
    let optimal_value = problem.evaluate(&optimal);
    assert_eq!(optimal_value, Min(Some(20)));
}

#[test]
fn test_stacker_crane_small_solver_instance() {
    let problem = small_problem();
    let solver = BruteForce::new();

    let optimal = solver
        .find_witness(&problem)
        .expect("small instance should have a witness");
    let mut sorted = optimal.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, vec![0, 1]);
    assert!(problem.evaluate(&optimal).0.is_some());
}

#[test]
fn test_stacker_crane_serialization_round_trip() {
    let problem = issue_problem();
    let json = serde_json::to_string(&problem).unwrap();
    let round_trip: StackerCrane = serde_json::from_str(&json).unwrap();

    assert_eq!(round_trip.num_vertices(), 6);
    assert_eq!(round_trip.num_arcs(), 5);
    assert_eq!(round_trip.num_edges(), 7);
    assert_eq!(round_trip.evaluate(&[0, 2, 1, 4, 3]), Min(Some(20)));
}

#[test]
fn test_stacker_crane_try_new_validation_errors() {
    // Mismatched arc_lengths length
    assert!(StackerCrane::try_new(3, vec![(0, 1)], vec![], vec![1, 2], vec![]).is_err());

    // Mismatched edge_lengths length
    assert!(StackerCrane::try_new(3, vec![], vec![(0, 1)], vec![], vec![]).is_err());

    // Arc endpoint out of range
    assert!(StackerCrane::try_new(2, vec![(0, 5)], vec![], vec![1], vec![]).is_err());

    // Edge endpoint out of range
    assert!(StackerCrane::try_new(2, vec![], vec![(0, 5)], vec![], vec![1]).is_err());

    // Negative arc length
    assert!(StackerCrane::try_new(3, vec![(0, 1)], vec![], vec![-1], vec![]).is_err());

    // Negative edge length
    assert!(StackerCrane::try_new(3, vec![], vec![(0, 1)], vec![], vec![-1]).is_err());
}

#[test]
fn test_stacker_crane_unreachable_connector() {
    // Two disconnected components: arc 0->1 and arc 2->3 with no connecting edges.
    let problem = StackerCrane::new(4, vec![(0, 1), (2, 3)], vec![], vec![1, 1], vec![]);

    // No permutation can find a connector path from vertex 1 to vertex 2 (or 3 to 0).
    assert_eq!(problem.closed_walk_length(&[0, 1]), None);
    assert_eq!(problem.closed_walk_length(&[1, 0]), None);
    assert_eq!(problem.evaluate(&[0, 1]), Min(None));
    assert_eq!(problem.evaluate(&[1, 0]), Min(None));
}

#[test]
fn test_stacker_crane_deserialization_rejects_invalid() {
    let bad_json =
        r#"{"num_vertices":2,"arcs":[[0,5]],"edges":[],"arc_lengths":[1],"edge_lengths":[]}"#;
    assert!(serde_json::from_str::<StackerCrane>(bad_json).is_err());
}

#[test]
fn test_stacker_crane_is_available_in_prelude() {
    let problem = crate::prelude::StackerCrane::new(
        3,
        vec![(0, 1), (1, 2)],
        vec![(0, 2)],
        vec![1, 1],
        vec![1],
    );

    assert_eq!(problem.num_arcs(), 2);
}
