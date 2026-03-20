use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_partition_into_paths_basic() {
    // 9-vertex graph: three P3 paths: 0-1-2, 3-4-5, 6-7-8
    let graph = SimpleGraph::new(
        9,
        vec![
            (0, 1),
            (1, 2),
            (3, 4),
            (4, 5),
            (6, 7),
            (7, 8),
            (0, 3),
            (2, 5),
            (3, 6),
            (5, 8),
        ],
    );
    let problem = PartitionIntoPathsOfLength2::new(graph);

    assert_eq!(problem.num_vertices(), 9);
    assert_eq!(problem.num_edges(), 10);
    assert_eq!(problem.num_groups(), 3);
    assert_eq!(problem.dims(), vec![3; 9]);

    // Valid partition: {0,1,2}, {3,4,5}, {6,7,8}
    // Config: vertex i -> group i/3
    let valid_config = vec![0, 0, 0, 1, 1, 1, 2, 2, 2];
    assert!(problem.evaluate(&valid_config));

    // Alternative valid partition: {0,1,3}, {2,4,5}, {6,7,8}
    // Group {0,1,3}: edges (0,1) and (0,3) — 2 edges, valid
    // Group {2,4,5}: edges (4,5) and (2,5) — 2 edges, valid
    let another_config = vec![0, 0, 1, 0, 1, 1, 2, 2, 2];
    assert!(problem.evaluate(&another_config));
}

#[test]
fn test_partition_into_paths_no_solution() {
    // 6-vertex graph where no valid partition exists
    // Edges: {0,1}, {2,3}, {0,4}, {1,5}
    let graph = SimpleGraph::new(6, vec![(0, 1), (2, 3), (0, 4), (1, 5)]);
    let problem = PartitionIntoPathsOfLength2::new(graph);

    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_groups(), 2);

    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_none(), "Expected no solution for this graph");
}

#[test]
fn test_partition_into_paths_solver() {
    // Simple 6-vertex graph with obvious partition: 0-1-2 and 3-4-5
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]);
    let problem = PartitionIntoPathsOfLength2::new(graph);

    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert!(!solutions.is_empty(), "Expected at least one solution");

    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_partition_into_paths_invalid_group_size() {
    // 6-vertex path: 0-1-2-3-4-5
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)]);
    let problem = PartitionIntoPathsOfLength2::new(graph);

    // Config where group 0 has 4 vertices and group 1 has 2 vertices
    let bad_config = vec![0, 0, 0, 0, 1, 1];
    assert!(!problem.evaluate(&bad_config));
}

#[test]
fn test_partition_into_paths_insufficient_edges() {
    // 6 vertices, only 2 edges — not enough for any group to have 2 edges
    let graph = SimpleGraph::new(6, vec![(0, 1), (3, 4)]);
    let problem = PartitionIntoPathsOfLength2::new(graph);

    // Even a well-sized partition fails because groups lack edges
    let config = vec![0, 0, 0, 1, 1, 1];
    // Group {0,1,2}: only edge (0,1) — 1 edge < 2, invalid
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_partition_into_paths_triangle() {
    // Triangle group: 3 vertices, 3 edges — also valid (>= 2 edges)
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = PartitionIntoPathsOfLength2::new(graph);

    // Single group with all 3 vertices forming a triangle
    let config = vec![0, 0, 0];
    assert!(problem.evaluate(&config));
}

#[test]
fn test_partition_into_paths_serialization() {
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]);
    let problem = PartitionIntoPathsOfLength2::new(graph);

    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: PartitionIntoPathsOfLength2<SimpleGraph> =
        serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.num_vertices(), 6);
    assert_eq!(deserialized.num_edges(), 4);
    assert_eq!(deserialized.num_groups(), 2);

    // Verify evaluation is consistent
    let config = vec![0, 0, 0, 1, 1, 1];
    assert_eq!(problem.evaluate(&config), deserialized.evaluate(&config));
}

#[test]
#[should_panic(expected = "must be divisible by 3")]
fn test_partition_into_paths_invalid_vertex_count() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let _problem = PartitionIntoPathsOfLength2::new(graph);
}

#[test]
fn test_partition_into_paths_size_getters() {
    let graph = SimpleGraph::new(9, vec![(0, 1), (1, 2), (3, 4), (4, 5), (6, 7), (7, 8)]);
    let problem = PartitionIntoPathsOfLength2::new(graph);
    assert_eq!(problem.num_vertices(), 9);
    assert_eq!(problem.num_edges(), 6);
    assert_eq!(problem.num_groups(), 3);
}

#[test]
fn test_partition_into_paths_out_of_range_group() {
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]);
    let problem = PartitionIntoPathsOfLength2::new(graph);

    // Group index out of range (q=2, so valid groups are 0 and 1)
    let config = vec![0, 0, 0, 2, 2, 2];
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_partition_into_paths_is_valid_partition() {
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]);
    let problem = PartitionIntoPathsOfLength2::new(graph);

    assert!(problem.is_valid_partition(&[0, 0, 0, 1, 1, 1]));
    assert!(!problem.is_valid_partition(&[0, 0, 1, 1, 1, 1])); // Wrong group sizes
    assert!(!problem.is_valid_partition(&[0, 0, 0])); // Wrong length
}
