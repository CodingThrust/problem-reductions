use super::*;
use crate::models::graph::{BoundedComponentSpanningForest, PartitionIntoPathsOfLength2};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_partitionintopathsoflength2_to_boundedcomponentspanningforest_closed_loop() {
    // 6-vertex graph with two P3 paths: 0-1-2 and 3-4-5
    let source =
        PartitionIntoPathsOfLength2::new(SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]));
    let result = ReduceTo::<BoundedComponentSpanningForest<SimpleGraph, i32>>::reduce_to(&source);
    let target = result.target_problem();

    // Check target structure
    assert_eq!(target.num_vertices(), 6);
    assert_eq!(target.num_edges(), 4);
    assert_eq!(target.max_components(), 2); // K = 6/3 = 2
    assert_eq!(*target.max_weight(), 3); // B = 3

    // All weights should be 1
    assert!(target.weights().iter().all(|&w| w == 1));

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &result,
        "PPL2->BCSF closed loop",
    );
}

#[test]
fn test_partitionintopathsoflength2_to_boundedcomponentspanningforest_no_solution() {
    // 6 vertices, only edges within first 3 vertices, none in the second 3.
    // Second triple {3,4,5} has no edges, so it can't form a connected component.
    let source = PartitionIntoPathsOfLength2::new(SimpleGraph::new(
        6,
        vec![(0, 1), (1, 2), (0, 2)], // triangle on {0,1,2}, no edges on {3,4,5}
    ));
    let result = ReduceTo::<BoundedComponentSpanningForest<SimpleGraph, i32>>::reduce_to(&source);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(result.target_problem());
    assert!(
        solutions.is_empty(),
        "No P3-partition exists, so BCSF should have no solution"
    );
}

#[test]
fn test_partitionintopathsoflength2_to_boundedcomponentspanningforest_triangle_partition() {
    // 9-vertex graph from the issue example
    let source = PartitionIntoPathsOfLength2::new(SimpleGraph::new(
        9,
        vec![
            (0, 1),
            (1, 2),
            (0, 2),
            (3, 4),
            (4, 5),
            (6, 7),
            (7, 8),
            (1, 3),
            (2, 6),
            (5, 8),
            (0, 5),
        ],
    ));
    let result = ReduceTo::<BoundedComponentSpanningForest<SimpleGraph, i32>>::reduce_to(&source);
    let target = result.target_problem();

    assert_eq!(target.num_vertices(), 9);
    assert_eq!(target.max_components(), 3); // K = 9/3 = 3
    assert_eq!(*target.max_weight(), 3); // B = 3

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &result,
        "PPL2->BCSF 9-vertex closed loop",
    );
}

#[test]
fn test_partitionintopathsoflength2_to_boundedcomponentspanningforest_extract_solution() {
    // Verify extract_solution is identity
    let source =
        PartitionIntoPathsOfLength2::new(SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]));
    let result = ReduceTo::<BoundedComponentSpanningForest<SimpleGraph, i32>>::reduce_to(&source);

    let target_config = vec![0, 0, 0, 1, 1, 1];
    let extracted = result.extract_solution(&target_config);
    assert_eq!(extracted, vec![0, 0, 0, 1, 1, 1]);

    // Verify the extracted solution is valid in the source
    assert!(source.evaluate(&extracted).0);
}
