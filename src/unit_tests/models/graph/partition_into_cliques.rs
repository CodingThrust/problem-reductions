use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn two_triangle_instance() -> PartitionIntoCliques<SimpleGraph> {
    // Two triangles: 0-1-2 and 3-4-5, plus cross edges 0-3, 1-4, 2-5
    PartitionIntoCliques::new(
        SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (0, 2),
                (1, 2),
                (3, 4),
                (3, 5),
                (4, 5),
                (0, 3),
                (1, 4),
                (2, 5),
            ],
        ),
        3,
    )
}

#[test]
fn test_partition_into_cliques_creation() {
    let problem = two_triangle_instance();
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 9);
    assert_eq!(problem.num_cliques(), 3);
    assert_eq!(problem.dims(), vec![3; 6]);
    assert_eq!(problem.graph().num_vertices(), 6);
}

#[test]
fn test_partition_into_cliques_evaluate_positive() {
    let problem = two_triangle_instance();

    // Group 0 = {0,1,2} (triangle), Group 1 = {3,4,5} (triangle)
    assert!(problem.evaluate(&[0, 0, 0, 1, 1, 1]));

    // Each vertex in its own group (trivially valid)
    assert!(problem.evaluate(&[0, 1, 2, 0, 1, 2]));
}

#[test]
fn test_partition_into_cliques_evaluate_negative() {
    let problem = two_triangle_instance();

    // Group 0 = {0,1,2,3}: 0-1, 0-2, 1-2, 0-3 present, but 1-3 missing
    assert!(!problem.evaluate(&[0, 0, 0, 0, 1, 2]));
}

#[test]
fn test_partition_into_cliques_evaluate_wrong_config_length() {
    let problem = two_triangle_instance();
    assert!(!problem.evaluate(&[0, 1, 0]));
    assert!(!problem.evaluate(&[0, 1, 0, 0, 1, 1, 0]));
}

#[test]
fn test_partition_into_cliques_evaluate_out_of_range_group() {
    let problem = two_triangle_instance();
    // Group 3 doesn't exist (num_cliques=3, valid groups are 0,1,2)
    assert!(!problem.evaluate(&[0, 1, 3, 0, 1, 2]));
}

#[test]
fn test_partition_into_cliques_brute_force_finds_solution() {
    // Complete graph K4, K=2: can partition into two cliques
    let problem = PartitionIntoCliques::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        2,
    );
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));
}

#[test]
fn test_partition_into_cliques_brute_force_no_solution() {
    // Path 0-1-2, K=1: {0,1,2} not a clique (missing edge 0-2)
    let problem = PartitionIntoCliques::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), 1);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_partition_into_cliques_brute_force_all_valid() {
    // Complete graph K3, K=3: every assignment is valid
    let problem = PartitionIntoCliques::new(SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]), 3);
    let solutions = BruteForce::new().find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_partition_into_cliques_serialization() {
    let problem = two_triangle_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: PartitionIntoCliques<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vertices(), 6);
    assert_eq!(deserialized.num_edges(), 9);
    assert_eq!(deserialized.num_cliques(), 3);
}

#[test]
#[should_panic(expected = "num_cliques must be at least 1")]
fn test_partition_into_cliques_rejects_zero() {
    let _ = PartitionIntoCliques::new(SimpleGraph::new(2, vec![(0, 1)]), 0);
}

#[test]
#[should_panic(expected = "num_cliques must be at most num_vertices")]
fn test_partition_into_cliques_rejects_too_many() {
    let _ = PartitionIntoCliques::new(SimpleGraph::new(2, vec![(0, 1)]), 3);
}
