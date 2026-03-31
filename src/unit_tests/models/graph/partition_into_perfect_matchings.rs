use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn four_vertex_instance() -> PartitionIntoPerfectMatchings<SimpleGraph> {
    // 4 vertices with edges: (0,1),(2,3),(0,2),(1,3)
    PartitionIntoPerfectMatchings::new(SimpleGraph::new(4, vec![(0, 1), (2, 3), (0, 2), (1, 3)]), 2)
}

#[test]
fn test_partition_into_perfect_matchings_creation() {
    let problem = four_vertex_instance();
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 4);
    assert_eq!(problem.num_matchings(), 2);
    assert_eq!(problem.dims(), vec![2; 4]);
    assert_eq!(problem.graph().num_vertices(), 4);
}

#[test]
fn test_partition_into_perfect_matchings_evaluate_positive() {
    let problem = four_vertex_instance();

    // Group 0 = {0,1} (edge 0-1), Group 1 = {2,3} (edge 2-3)
    assert!(problem.evaluate(&[0, 0, 1, 1]));

    // Group 0 = {0,2} (edge 0-2), Group 1 = {1,3} (edge 1-3)
    assert!(problem.evaluate(&[0, 1, 0, 1]));
}

#[test]
fn test_partition_into_perfect_matchings_evaluate_negative() {
    let problem = four_vertex_instance();

    // Group 0 = {0,1,2}: vertex 0 has neighbors 1 and 2 both in group => degree 2, not 1
    assert!(!problem.evaluate(&[0, 0, 0, 1]));

    // All in one group: each vertex has 2 neighbors in the group
    assert!(!problem.evaluate(&[0, 0, 0, 0]));
}

#[test]
fn test_partition_into_perfect_matchings_evaluate_odd_group() {
    // A group with an odd number of members can never be a perfect matching
    let problem = four_vertex_instance();
    // Group 0 = {0,1,2} (3 vertices), Group 1 = {3} (1 vertex)
    assert!(!problem.evaluate(&[0, 0, 0, 1]));
}

#[test]
fn test_partition_into_perfect_matchings_evaluate_wrong_config_length() {
    let problem = four_vertex_instance();
    assert!(!problem.evaluate(&[0, 1]));
    assert!(!problem.evaluate(&[0, 1, 0, 0, 1]));
}

#[test]
fn test_partition_into_perfect_matchings_evaluate_out_of_range_group() {
    let problem = four_vertex_instance();
    // Group 2 doesn't exist (num_matchings=2, valid groups are 0,1)
    assert!(!problem.evaluate(&[0, 1, 2, 0]));
}

#[test]
fn test_partition_into_perfect_matchings_brute_force_finds_solution() {
    let problem = four_vertex_instance();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));
}

#[test]
fn test_partition_into_perfect_matchings_brute_force_no_solution() {
    // Path 0-1-2: no perfect matching partition possible with K=1
    // Group {0,1,2} has 3 vertices (odd) so cannot be a perfect matching
    let problem = PartitionIntoPerfectMatchings::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), 1);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_partition_into_perfect_matchings_brute_force_all_valid() {
    // 2 vertices with edge (0,1), K=2: group {0,1} is a perfect matching
    let problem = PartitionIntoPerfectMatchings::new(SimpleGraph::new(2, vec![(0, 1)]), 2);
    let solutions = BruteForce::new().find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_partition_into_perfect_matchings_serialization() {
    let problem = four_vertex_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: PartitionIntoPerfectMatchings<SimpleGraph> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vertices(), 4);
    assert_eq!(deserialized.num_edges(), 4);
    assert_eq!(deserialized.num_matchings(), 2);
}

#[test]
#[should_panic(expected = "num_matchings must be at least 1")]
fn test_partition_into_perfect_matchings_rejects_zero() {
    let _ = PartitionIntoPerfectMatchings::new(SimpleGraph::new(2, vec![(0, 1)]), 0);
}

#[test]
#[should_panic(expected = "num_matchings must be at most num_vertices")]
fn test_partition_into_perfect_matchings_rejects_too_many() {
    let _ = PartitionIntoPerfectMatchings::new(SimpleGraph::new(2, vec![(0, 1)]), 3);
}
