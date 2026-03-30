use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn two_triangle_instance() -> PartitionIntoForests<SimpleGraph> {
    // Two disjoint triangles + bridge edge (2,3)
    // Triangle A: 0-1-2-0, Triangle B: 3-4-5-3
    PartitionIntoForests::new(
        SimpleGraph::new(
            6,
            vec![(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 5), (5, 3)],
        ),
        2,
    )
}

#[test]
fn test_partition_into_forests_creation() {
    let problem = two_triangle_instance();
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 7);
    assert_eq!(problem.num_forests(), 2);
    assert_eq!(problem.dims(), vec![2; 6]);
    assert_eq!(problem.graph().num_vertices(), 6);
}

#[test]
fn test_partition_into_forests_evaluate_positive() {
    let problem = two_triangle_instance();

    // config [0,1,1,0,1,1]: class0={0,3} (no intra-class edges), class1={1,2,4,5} (edges 1-2, 4-5, both trees)
    assert!(problem.evaluate(&[0, 1, 1, 0, 1, 1]));

    // config [0,0,1,1,0,1]: class0={0,1,4} (edge 0-1 → path), class1={2,3,5} (no triangle edges remain)
    assert!(problem.evaluate(&[0, 0, 1, 1, 0, 1]));
}

#[test]
fn test_partition_into_forests_evaluate_negative_k1() {
    // K=1: must put all vertices in one class; two triangles create cycles
    let problem = PartitionIntoForests::new(
        SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)]),
        1,
    );

    // Any single-class assignment must include a triangle → cycle
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0, 0]));
}

#[test]
fn test_partition_into_forests_evaluate_cycle_in_class() {
    let problem = two_triangle_instance();

    // config [0,0,0,1,1,1]: class0={0,1,2} contains triangle 0-1-2-0 → cycle
    assert!(!problem.evaluate(&[0, 0, 0, 1, 1, 1]));
}

#[test]
fn test_partition_into_forests_evaluate_wrong_config_length() {
    let problem = two_triangle_instance();
    assert!(!problem.evaluate(&[0, 1, 0]));
    assert!(!problem.evaluate(&[0, 1, 0, 0, 1, 1, 0]));
}

#[test]
fn test_partition_into_forests_evaluate_out_of_range_class() {
    let problem = two_triangle_instance();
    // Class 2 doesn't exist (num_forests=2)
    assert!(!problem.evaluate(&[0, 1, 2, 0, 1, 1]));
}

#[test]
fn test_partition_into_forests_brute_force_finds_solution() {
    // Small instance: 4-cycle (no triangle), K=2 should work easily
    let problem =
        PartitionIntoForests::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]), 2);
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));
}

#[test]
fn test_partition_into_forests_brute_force_no_solution() {
    // Single triangle, K=1: impossible
    let problem = PartitionIntoForests::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]), 1);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_partition_into_forests_brute_force_all_valid() {
    // Small acyclic graph (path 0-1-2), K=1: every assignment is valid
    let problem = PartitionIntoForests::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), 1);
    let solutions = BruteForce::new().find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_partition_into_forests_serialization() {
    let problem = two_triangle_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: PartitionIntoForests<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vertices(), 6);
    assert_eq!(deserialized.num_edges(), 7);
    assert_eq!(deserialized.num_forests(), 2);
}

#[test]
#[should_panic(expected = "num_forests must be at least 1")]
fn test_partition_into_forests_rejects_zero_forests() {
    let _ = PartitionIntoForests::new(SimpleGraph::new(2, vec![(0, 1)]), 0);
}
