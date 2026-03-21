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
