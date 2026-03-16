use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn sample_yes_graph() -> SimpleGraph {
    SimpleGraph::new(
        7,
        vec![
            (0, 1),
            (1, 6),
            (0, 2),
            (2, 3),
            (3, 6),
            (0, 4),
            (4, 5),
            (5, 6),
        ],
    )
}

fn sample_yes_problem() -> LengthBoundedDisjointPaths<SimpleGraph> {
    LengthBoundedDisjointPaths::new(sample_yes_graph(), 0, 6, 2, 3)
}

fn encode_paths(num_vertices: usize, slots: &[&[usize]]) -> Vec<usize> {
    let mut config = vec![0; num_vertices * slots.len()];
    for (slot_index, slot_vertices) in slots.iter().enumerate() {
        let offset = slot_index * num_vertices;
        for &vertex in *slot_vertices {
            config[offset + vertex] = 1;
        }
    }
    config
}

#[test]
fn test_length_bounded_disjoint_paths_creation() {
    let problem = sample_yes_problem();
    assert_eq!(problem.num_vertices(), 7);
    assert_eq!(problem.num_edges(), 8);
    assert_eq!(problem.num_paths_required(), 2);
    assert_eq!(problem.max_length(), 3);
    assert_eq!(problem.dims(), vec![2; 14]);
}

#[test]
fn test_length_bounded_disjoint_paths_allows_large_bounds() {
    let problem = LengthBoundedDisjointPaths::new(sample_yes_graph(), 0, 6, 2, 10);
    let config = encode_paths(7, &[&[0, 1, 6], &[0, 2, 3, 6]]);
    assert!(problem.evaluate(&config));
}

#[test]
#[should_panic(expected = "source must be a valid graph vertex")]
fn test_length_bounded_disjoint_paths_creation_rejects_invalid_source() {
    let _ = LengthBoundedDisjointPaths::new(sample_yes_graph(), 7, 6, 2, 3);
}

#[test]
#[should_panic(expected = "sink must be a valid graph vertex")]
fn test_length_bounded_disjoint_paths_creation_rejects_invalid_sink() {
    let _ = LengthBoundedDisjointPaths::new(sample_yes_graph(), 0, 7, 2, 3);
}

#[test]
#[should_panic(expected = "source and sink must be distinct")]
fn test_length_bounded_disjoint_paths_creation_rejects_equal_terminals() {
    let _ = LengthBoundedDisjointPaths::new(sample_yes_graph(), 0, 0, 2, 3);
}

#[test]
#[should_panic(expected = "num_paths_required must be positive")]
fn test_length_bounded_disjoint_paths_creation_rejects_zero_paths() {
    let _ = LengthBoundedDisjointPaths::new(sample_yes_graph(), 0, 6, 0, 3);
}

#[test]
#[should_panic(expected = "max_length must be positive")]
fn test_length_bounded_disjoint_paths_creation_rejects_zero_bound() {
    let _ = LengthBoundedDisjointPaths::new(sample_yes_graph(), 0, 6, 2, 0);
}

#[test]
fn test_length_bounded_disjoint_paths_evaluation() {
    let problem = sample_yes_problem();
    let config = encode_paths(7, &[&[0, 1, 6], &[0, 2, 3, 6]]);
    assert!(problem.evaluate(&config));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_missing_terminal() {
    let problem = sample_yes_problem();
    let config = encode_paths(7, &[&[0, 1], &[0, 2, 3, 6]]);
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_disconnected_slot() {
    let problem = sample_yes_problem();
    let config = encode_paths(7, &[&[0, 1, 3, 6], &[0, 4, 5, 6]]);
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_overlong_slot() {
    let problem = LengthBoundedDisjointPaths::new(sample_yes_graph(), 0, 6, 2, 2);
    let config = encode_paths(7, &[&[0, 1, 6], &[0, 2, 3, 6]]);
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_shared_internal_vertices() {
    let problem = sample_yes_problem();
    let config = encode_paths(7, &[&[0, 2, 3, 6], &[0, 2, 3, 6]]);
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_reused_direct_edge() {
    let problem = LengthBoundedDisjointPaths::new(SimpleGraph::new(2, vec![(0, 1)]), 0, 1, 2, 1);
    let config = encode_paths(2, &[&[0, 1], &[0, 1]]);
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_non_binary_entries() {
    let problem = sample_yes_problem();
    let mut config = encode_paths(7, &[&[0, 1, 6], &[0, 2, 3, 6]]);
    config[4] = 2;
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_length_bounded_disjoint_paths_solver_yes_and_no() {
    let yes_problem = sample_yes_problem();
    let solver = BruteForce::new();
    assert!(solver.find_satisfying(&yes_problem).is_some());

    let no_problem = LengthBoundedDisjointPaths::new(sample_yes_graph(), 0, 6, 2, 2);
    assert!(solver.find_satisfying(&no_problem).is_none());
}

#[test]
fn test_length_bounded_disjoint_paths_serialization() {
    let problem = sample_yes_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let round_trip: LengthBoundedDisjointPaths<SimpleGraph> = serde_json::from_value(json).unwrap();
    assert_eq!(round_trip.num_vertices(), 7);
    assert_eq!(round_trip.source(), 0);
    assert_eq!(round_trip.sink(), 6);
    assert_eq!(round_trip.num_paths_required(), 2);
    assert_eq!(round_trip.max_length(), 3);
}

#[test]
fn test_length_bounded_disjoint_paths_paper_example() {
    let problem = sample_yes_problem();
    let config = encode_paths(7, &[&[0, 1, 6], &[0, 2, 3, 6]]);
    assert!(problem.evaluate(&config));

    let satisfying = BruteForce::new().find_all_satisfying(&problem);
    assert_eq!(satisfying.len(), 6);
    assert!(satisfying
        .iter()
        .all(|candidate| problem.evaluate(candidate)));
}
