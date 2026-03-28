use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;

#[test]
fn test_rootedtreearrangement_to_rootedtreestorageassignment_closed_loop() {
    // Path graph P4: 0-1-2-3, bound K=5
    // Optimal chain tree gives total distance 3 <= 5
    let source = RootedTreeArrangement::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]), 5);
    let reduction = ReduceTo::<RootedTreeStorageAssignment>::reduce_to(&source);
    assert_satisfaction_round_trip_from_satisfaction_target(&source, &reduction, "P4 path graph");
}

#[test]
fn test_rootedtreearrangement_to_rootedtreestorageassignment_target_structure() {
    // Triangle graph: 3 vertices, 3 edges, bound K=6
    let source = RootedTreeArrangement::new(SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]), 6);
    let reduction = ReduceTo::<RootedTreeStorageAssignment>::reduce_to(&source);
    let target = reduction.target_problem();

    // Universe size = num_vertices = 3
    assert_eq!(target.universe_size(), 3);
    // Num subsets = num_edges = 3
    assert_eq!(target.num_subsets(), 3);
    // Bound = K - |E| = 6 - 3 = 3
    assert_eq!(target.bound(), 3);
    // Each subset is a 2-element set from an edge
    for subset in target.subsets() {
        assert_eq!(subset.len(), 2);
    }
}

#[test]
fn test_rootedtreearrangement_to_rootedtreestorageassignment_star_graph() {
    // Star graph K_{1,3}: center=0, leaves=1,2,3
    // Bound K=3 (optimal: root at 0, each leaf distance 1, total=3)
    let source = RootedTreeArrangement::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]), 3);
    let reduction = ReduceTo::<RootedTreeStorageAssignment>::reduce_to(&source);
    let target = reduction.target_problem();

    // K' = 3 - 3 = 0 (no extensions needed for a star rooted at center)
    assert_eq!(target.bound(), 0);
    assert_satisfaction_round_trip_from_satisfaction_target(&source, &reduction, "star K_{1,3}");
}

#[test]
fn test_rootedtreearrangement_to_rootedtreestorageassignment_unsatisfiable() {
    // K4 with tight bound: any tree on 4 vertices has at most 3 edges on
    // root-to-leaf paths. K4 has 6 edges, and its minimum total stretch
    // on a chain tree is 1+1+1+2+2+3=10. With K=7 it should be infeasible.
    let source = RootedTreeArrangement::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        7,
    );
    let reduction = ReduceTo::<RootedTreeStorageAssignment>::reduce_to(&source);
    let target = reduction.target_problem();

    // K' = 7 - 6 = 1
    assert_eq!(target.bound(), 1);

    // Both source and target should be unsatisfiable
    let solver = BruteForce::new();
    assert!(
        solver.find_witness(&source).is_none(),
        "K4 with K=7 should be unsatisfiable"
    );
    assert!(
        solver.find_witness(target).is_none(),
        "target should also be unsatisfiable"
    );
}

#[test]
fn test_rootedtreearrangement_to_rootedtreestorageassignment_solution_extraction() {
    // Simple edge: 2 vertices, 1 edge {0,1}, bound K=1
    let source = RootedTreeArrangement::new(SimpleGraph::new(2, vec![(0, 1)]), 1);
    let reduction = ReduceTo::<RootedTreeStorageAssignment>::reduce_to(&source);
    let target = reduction.target_problem();

    // Target: universe_size=2, subsets={{0,1}}, bound=0
    assert_eq!(target.universe_size(), 2);
    assert_eq!(target.bound(), 0);

    // Target solution: parent array [0, 0] means tree rooted at 0 with 1->0
    let target_config = vec![0, 0];
    let source_config = reduction.extract_solution(&target_config);

    // Source config should be [parent_array | identity_mapping] = [0, 0, 0, 1]
    assert_eq!(source_config, vec![0, 0, 0, 1]);
    // Verify it's valid for the source
    assert!(source.is_valid_solution(&source_config));
}

#[test]
fn test_rootedtreearrangement_to_rootedtreestorageassignment_empty_graph() {
    // Graph with no edges
    let source = RootedTreeArrangement::new(SimpleGraph::new(3, vec![]), 0);
    let reduction = ReduceTo::<RootedTreeStorageAssignment>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.universe_size(), 3);
    assert_eq!(target.num_subsets(), 0);
    assert_eq!(target.bound(), 0);

    assert_satisfaction_round_trip_from_satisfaction_target(&source, &reduction, "empty graph");
}
