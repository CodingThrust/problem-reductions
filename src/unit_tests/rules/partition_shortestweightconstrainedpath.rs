use super::*;
use crate::models::graph::ShortestWeightConstrainedPath;
use crate::models::misc::Partition;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_partition_to_shortestweightconstrainedpath_closed_loop() {
    let source = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    let reduction = ReduceTo::<ShortestWeightConstrainedPath<SimpleGraph, i32>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "Partition -> ShortestWeightConstrainedPath closed loop",
    );
}

#[test]
fn test_partition_to_shortestweightconstrainedpath_structure() {
    let source = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    let reduction = ReduceTo::<ShortestWeightConstrainedPath<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // n=6 elements → 7 vertices, 12 edges
    assert_eq!(target.num_vertices(), 7);
    assert_eq!(target.num_edges(), 12);
    assert_eq!(target.source_vertex(), 0);
    assert_eq!(target.target_vertex(), 6);

    // total_sum = 10, weight_bound = floor(10/2) + 6 = 11
    assert_eq!(*target.weight_bound(), 11);

    // Check edge lengths and weights for first layer (a_0 = 3):
    // Include edge: length=4, weight=1; Exclude edge: length=1, weight=4
    assert_eq!(target.edge_lengths()[0], 4); // include
    assert_eq!(target.edge_weights()[0], 1);
    assert_eq!(target.edge_lengths()[1], 1); // exclude
    assert_eq!(target.edge_weights()[1], 4);
}

#[test]
fn test_partition_to_shortestweightconstrainedpath_unsatisfiable() {
    // Odd total sum → no balanced partition exists
    let source = Partition::new(vec![2, 4, 5]);
    let reduction = ReduceTo::<ShortestWeightConstrainedPath<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // total_sum = 11, weight_bound = floor(11/2) + 3 = 8
    assert_eq!(*target.weight_bound(), 8);

    // The SWCP optimal path exists, but extracting it should not satisfy Partition.
    let best = BruteForce::new()
        .find_witness(target)
        .expect("SWCP target should have an optimal solution");
    let extracted = reduction.extract_solution(&best);
    assert!(!source.evaluate(&extracted));
}

#[test]
fn test_partition_to_shortestweightconstrainedpath_small() {
    // Two elements: [3, 3] → balanced partition exists
    let source = Partition::new(vec![3, 3]);
    let reduction = ReduceTo::<ShortestWeightConstrainedPath<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 3);
    assert_eq!(target.num_edges(), 4);
    // total_sum = 6, weight_bound = 3 + 2 = 5
    assert_eq!(*target.weight_bound(), 5);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "Partition [3,3] -> SWCP",
    );
}

#[test]
fn test_partition_to_shortestweightconstrainedpath_single_element() {
    // Single element → no balanced partition (odd total)
    let source = Partition::new(vec![4]);
    let reduction = ReduceTo::<ShortestWeightConstrainedPath<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 2);
    assert_eq!(target.num_edges(), 2);
    // total_sum = 4, weight_bound = 2 + 1 = 3
    assert_eq!(*target.weight_bound(), 3);
}
