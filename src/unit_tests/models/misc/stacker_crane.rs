use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

fn issue_problem(bound: i32) -> StackerCrane {
    StackerCrane::new(
        6,
        vec![(0, 4), (2, 5), (5, 1), (3, 0), (4, 3)],
        vec![(0, 1), (1, 2), (2, 3), (3, 5), (4, 5), (0, 3), (1, 5)],
        vec![3, 4, 2, 5, 3],
        vec![2, 1, 3, 2, 1, 4, 3],
        bound,
    )
}

fn small_problem() -> StackerCrane {
    StackerCrane::new(
        3,
        vec![(0, 1), (1, 2)],
        vec![(0, 2)],
        vec![1, 1],
        vec![1],
        3,
    )
}

#[test]
fn test_stacker_crane_creation_and_metadata() {
    let problem = issue_problem(20);

    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_arcs(), 5);
    assert_eq!(problem.num_edges(), 7);
    assert_eq!(problem.bound(), 20);
    assert_eq!(problem.dims(), vec![5; 5]);
    assert_eq!(<StackerCrane as Problem>::NAME, "StackerCrane");
    assert!(<StackerCrane as Problem>::variant().is_empty());
}

#[test]
fn test_stacker_crane_rejects_non_permutations_and_wrong_lengths() {
    let problem = issue_problem(20);

    assert!(!problem.evaluate(&[0, 2, 1, 4, 4]));
    assert!(!problem.evaluate(&[0, 2, 1, 4, 5]));
    assert!(!problem.evaluate(&[0, 2, 1, 4]));
    assert!(!problem.evaluate(&[0, 2, 1, 4, 3, 0]));
}

#[test]
fn test_stacker_crane_issue_witness_and_tighter_bound() {
    assert!(issue_problem(20).evaluate(&[0, 2, 1, 4, 3]));
    assert!(!issue_problem(19).evaluate(&[0, 2, 1, 4, 3]));
}

#[test]
fn test_stacker_crane_issue_instance_is_unsatisfiable_at_bound_19() {
    let problem = issue_problem(19);
    let solver = BruteForce::new();

    assert!(solver.find_all_satisfying(&problem).is_empty());
}

#[test]
fn test_stacker_crane_paper_example() {
    let problem = issue_problem(20);
    let witness = vec![0, 2, 1, 4, 3];

    assert_eq!(problem.closed_walk_length(&witness), Some(20));
    assert!(problem.evaluate(&witness));

    let solver = BruteForce::new();
    let satisfying = solver.find_all_satisfying(&problem);
    assert!(!satisfying.is_empty());
    assert!(satisfying.contains(&witness));
    for config in &satisfying {
        assert!(problem.evaluate(config));
    }
}

#[test]
fn test_stacker_crane_small_solver_instance() {
    let problem = small_problem();
    let solver = BruteForce::new();

    let satisfying = solver
        .find_satisfying(&problem)
        .expect("small instance should be satisfiable");
    let mut sorted = satisfying.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, vec![0, 1]);
    assert!(problem.evaluate(&satisfying));
}

#[test]
fn test_stacker_crane_serialization_round_trip() {
    let problem = issue_problem(20);
    let json = serde_json::to_string(&problem).unwrap();
    let round_trip: StackerCrane = serde_json::from_str(&json).unwrap();

    assert_eq!(round_trip.num_vertices(), 6);
    assert_eq!(round_trip.num_arcs(), 5);
    assert_eq!(round_trip.num_edges(), 7);
    assert_eq!(round_trip.bound(), 20);
    assert!(round_trip.evaluate(&[0, 2, 1, 4, 3]));
}

#[test]
fn test_stacker_crane_try_new_validation_errors() {
    // Mismatched arc_lengths length
    assert!(StackerCrane::try_new(3, vec![(0, 1)], vec![], vec![1, 2], vec![], 5).is_err());

    // Mismatched edge_lengths length
    assert!(StackerCrane::try_new(3, vec![], vec![(0, 1)], vec![], vec![], 5).is_err());

    // Negative bound
    assert!(StackerCrane::try_new(3, vec![], vec![], vec![], vec![], -1).is_err());

    // Arc endpoint out of range
    assert!(StackerCrane::try_new(2, vec![(0, 5)], vec![], vec![1], vec![], 5).is_err());

    // Edge endpoint out of range
    assert!(StackerCrane::try_new(2, vec![], vec![(0, 5)], vec![], vec![1], 5).is_err());

    // Negative arc length
    assert!(StackerCrane::try_new(3, vec![(0, 1)], vec![], vec![-1], vec![], 5).is_err());

    // Negative edge length
    assert!(StackerCrane::try_new(3, vec![], vec![(0, 1)], vec![], vec![-1], 5).is_err());
}

#[test]
fn test_stacker_crane_unreachable_connector() {
    // Two disconnected components: arc 0→1 and arc 2→3 with no connecting edges.
    let problem = StackerCrane::new(4, vec![(0, 1), (2, 3)], vec![], vec![1, 1], vec![], 100);

    // No permutation can find a connector path from vertex 1 to vertex 2 (or 3 to 0).
    assert_eq!(problem.closed_walk_length(&[0, 1]), None);
    assert_eq!(problem.closed_walk_length(&[1, 0]), None);
    assert!(!problem.evaluate(&[0, 1]));
    assert!(!problem.evaluate(&[1, 0]));
}

#[test]
fn test_stacker_crane_deserialization_rejects_invalid() {
    let bad_json = r#"{"num_vertices":2,"arcs":[[0,5]],"edges":[],"arc_lengths":[1],"edge_lengths":[],"bound":5}"#;
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
        3,
    );

    assert_eq!(problem.num_arcs(), 2);
}
