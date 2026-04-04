use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Max;

fn sample_graph() -> SimpleGraph {
    SimpleGraph::new(5, vec![(0, 1), (1, 4), (0, 2), (2, 4), (0, 3), (3, 4)])
}

fn sample_problem() -> LengthBoundedDisjointPaths<SimpleGraph> {
    // max_paths = min(deg(0), deg(4)) = min(3, 3) = 3
    LengthBoundedDisjointPaths::new(sample_graph(), 0, 4, 3)
}

fn encode_paths(num_vertices: usize, max_paths: usize, slots: &[&[usize]]) -> Vec<usize> {
    let mut config = vec![0; num_vertices * max_paths];
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
    let problem = sample_problem();
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 6);
    assert_eq!(problem.max_paths(), 3);
    assert_eq!(problem.max_length(), 3);
    // 3 slots * 5 vertices = 15 binary variables
    assert_eq!(problem.dims(), vec![2; 15]);
}

#[test]
fn test_length_bounded_disjoint_paths_allows_large_bounds() {
    let problem = LengthBoundedDisjointPaths::new(sample_graph(), 0, 4, 10);
    let config = encode_paths(5, 3, &[&[0, 1, 4], &[0, 2, 4]]);
    assert_eq!(problem.evaluate(&config), Max(Some(2)));
}

#[test]
#[should_panic(expected = "source must be a valid graph vertex")]
fn test_length_bounded_disjoint_paths_creation_rejects_invalid_source() {
    let _ = LengthBoundedDisjointPaths::new(sample_graph(), 5, 4, 3);
}

#[test]
#[should_panic(expected = "sink must be a valid graph vertex")]
fn test_length_bounded_disjoint_paths_creation_rejects_invalid_sink() {
    let _ = LengthBoundedDisjointPaths::new(sample_graph(), 0, 5, 3);
}

#[test]
#[should_panic(expected = "source and sink must be distinct")]
fn test_length_bounded_disjoint_paths_creation_rejects_equal_terminals() {
    let _ = LengthBoundedDisjointPaths::new(sample_graph(), 0, 0, 3);
}

#[test]
#[should_panic(expected = "max_length must be positive")]
fn test_length_bounded_disjoint_paths_creation_rejects_zero_bound() {
    let _ = LengthBoundedDisjointPaths::new(sample_graph(), 0, 4, 0);
}

#[test]
fn test_length_bounded_disjoint_paths_evaluate_optimal() {
    let problem = sample_problem();
    // All 3 paths used
    let config = encode_paths(5, 3, &[&[0, 1, 4], &[0, 2, 4], &[0, 3, 4]]);
    assert_eq!(problem.evaluate(&config), Max(Some(3)));
}

#[test]
fn test_length_bounded_disjoint_paths_evaluate_partial() {
    let problem = sample_problem();
    // Only 2 of 3 slots used, third slot empty
    let config = encode_paths(5, 3, &[&[0, 1, 4], &[0, 2, 4]]);
    assert_eq!(problem.evaluate(&config), Max(Some(2)));
}

#[test]
fn test_length_bounded_disjoint_paths_evaluate_single_path() {
    let problem = sample_problem();
    // Only 1 slot used
    let config = encode_paths(5, 3, &[&[0, 1, 4]]);
    assert_eq!(problem.evaluate(&config), Max(Some(1)));
}

#[test]
fn test_length_bounded_disjoint_paths_evaluate_empty_config() {
    let problem = sample_problem();
    // All slots empty → 0 paths
    let config = vec![0; 15];
    assert_eq!(problem.evaluate(&config), Max(Some(0)));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_missing_terminal() {
    let problem = sample_problem();
    // Slot 1 is non-empty but missing sink
    let config = encode_paths(5, 3, &[&[0, 1], &[0, 2, 4]]);
    assert_eq!(problem.evaluate(&config), Max(None));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_disconnected_slot() {
    let problem = sample_problem();
    // Slot has non-adjacent vertices (0 and 3 are adjacent, but 3 and 1 are not)
    let config = encode_paths(5, 3, &[&[0, 1, 3, 4]]);
    assert_eq!(problem.evaluate(&config), Max(None));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_overlong_slot() {
    // Use a graph where a path has 3 edges but max_length=1
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]);
    // max_paths = min(deg(0), deg(3)) = min(2, 2) = 2
    let problem = LengthBoundedDisjointPaths::new(graph, 0, 3, 1);
    // Path [0,1,2,3] has 3 edges but max_length=1
    let config = encode_paths(4, 2, &[&[0, 1, 2, 3]]);
    assert_eq!(problem.evaluate(&config), Max(None));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_shared_internal_vertices() {
    let problem = sample_problem();
    // Two slots share internal vertex 1
    let config = encode_paths(5, 3, &[&[0, 1, 4], &[0, 1, 4]]);
    assert_eq!(problem.evaluate(&config), Max(None));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_reused_direct_edge() {
    let problem = LengthBoundedDisjointPaths::new(SimpleGraph::new(2, vec![(0, 1)]), 0, 1, 1);
    // max_paths = min(deg(0), deg(1)) = 1, so only 1 slot
    let config = encode_paths(2, 1, &[&[0, 1]]);
    assert_eq!(problem.evaluate(&config), Max(Some(1)));
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_non_binary_entries() {
    let problem = sample_problem();
    let mut config = encode_paths(5, 3, &[&[0, 1, 4], &[0, 2, 4]]);
    config[3] = 2;
    assert_eq!(problem.evaluate(&config), Max(None));
}

#[test]
fn test_length_bounded_disjoint_paths_solver() {
    let problem = sample_problem();
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&witness), Max(Some(3)));
}

#[test]
fn test_length_bounded_disjoint_paths_serialization() {
    let problem = sample_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let round_trip: LengthBoundedDisjointPaths<SimpleGraph> = serde_json::from_value(json).unwrap();
    assert_eq!(round_trip.num_vertices(), 5);
    assert_eq!(round_trip.source(), 0);
    assert_eq!(round_trip.sink(), 4);
    assert_eq!(round_trip.max_paths(), 3);
    assert_eq!(round_trip.max_length(), 3);
}

#[test]
fn test_length_bounded_disjoint_paths_graph_getter() {
    let problem = sample_problem();
    assert_eq!(problem.graph().num_vertices(), 5);
    assert_eq!(problem.graph().num_edges(), 6);
}

#[test]
fn test_length_bounded_disjoint_paths_num_variables() {
    let problem = sample_problem();
    assert_eq!(problem.num_variables(), 15);
}

#[test]
fn test_length_bounded_disjoint_paths_rejects_wrong_length_config() {
    let problem = sample_problem();
    assert_eq!(problem.evaluate(&[0, 1, 0]), Max(None));
}
