use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;

#[test]
fn test_partitionintotriangles_basic() {
    use crate::traits::Problem;

    // 9-vertex YES instance: three disjoint triangles
    // Triangle 1: 0-1-2, Triangle 2: 3-4-5, Triangle 3: 6-7-8
    let graph = SimpleGraph::new(
        9,
        vec![
            (0, 1),
            (1, 2),
            (0, 2),
            (3, 4),
            (4, 5),
            (3, 5),
            (6, 7),
            (7, 8),
            (6, 8),
        ],
    );
    let problem = PartitionIntoTriangles::new(graph);

    assert_eq!(problem.num_vertices(), 9);
    assert_eq!(problem.num_edges(), 9);
    assert_eq!(problem.dims(), vec![3; 9]);

    // Valid partition: vertices 0,1,2 in group 0; 3,4,5 in group 1; 6,7,8 in group 2
    assert!(problem.evaluate(&[0, 0, 0, 1, 1, 1, 2, 2, 2]));

    // Invalid: wrong grouping (vertices 0,1,3 are not a triangle)
    assert!(!problem.evaluate(&[0, 0, 1, 0, 1, 1, 2, 2, 2]));

    // Invalid: group sizes wrong (4 in group 0, 2 in group 1)
    assert!(!problem.evaluate(&[0, 0, 0, 0, 1, 1, 2, 2, 2]));
}

#[test]
fn test_partitionintotriangles_no_solution() {
    use crate::traits::Problem;

    // 6-vertex NO instance: path graph has no triangles at all
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)]);
    let problem = PartitionIntoTriangles::new(graph);

    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.dims(), vec![2; 6]);

    // No valid partition exists since there are no triangles
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_none());
}

#[test]
fn test_partitionintotriangles_solver() {
    use crate::traits::Problem;

    // Single triangle
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = PartitionIntoTriangles::new(graph);

    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert!(problem.evaluate(&sol));

    // All solutions should be valid
    let all = solver.find_all_witnesses(&problem);
    assert!(!all.is_empty());
    for s in &all {
        assert!(problem.evaluate(s));
    }
}

#[test]
fn test_partitionintotriangles_serialization() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = PartitionIntoTriangles::new(graph);

    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: PartitionIntoTriangles<SimpleGraph> = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.num_vertices(), 3);
    assert_eq!(deserialized.num_edges(), 3);
}

#[test]
#[should_panic(expected = "must be divisible by 3")]
fn test_partitionintotriangles_invalid_vertex_count() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let _ = PartitionIntoTriangles::new(graph);
}

#[test]
fn test_partitionintotriangles_config_out_of_range() {
    use crate::traits::Problem;

    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = PartitionIntoTriangles::new(graph);

    // q = 1, so only group 0 is valid; group 1 is out of range
    assert!(!problem.evaluate(&[0, 0, 1]));
}

#[test]
fn test_partitionintotriangles_wrong_config_length() {
    use crate::traits::Problem;

    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = PartitionIntoTriangles::new(graph);

    assert!(!problem.evaluate(&[0, 0]));
    assert!(!problem.evaluate(&[0, 0, 0, 0]));
}

#[test]
fn test_partitionintotriangles_size_getters() {
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (0, 2), (3, 4), (4, 5), (3, 5)]);
    let problem = PartitionIntoTriangles::new(graph);
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 6);
}

#[test]
fn test_partitionintotriangles_paper_example() {
    use crate::traits::Problem;
    // Paper: 6 vertices, two triangles + cross-edge (0,3)
    let graph = SimpleGraph::new(
        6,
        vec![(0, 1), (0, 2), (1, 2), (3, 4), (3, 5), (4, 5), (0, 3)],
    );
    let problem = PartitionIntoTriangles::new(graph);
    // Valid partition: {0,1,2} in group 0, {3,4,5} in group 1
    assert!(problem.evaluate(&[0, 0, 0, 1, 1, 1]));

    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
}
