use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_maximumindependentset_to_integralflowbundles_closed_loop() {
    // Path graph: 0-1-2-3-4
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1i32; 5],
    );
    let reduction = ReduceTo::<IntegralFlowBundles>::reduce_to(&source);
    let target = reduction.target_problem();

    // n + 2 = 7 vertices, 2n = 10 arcs, m + n = 4 + 5 = 9 bundles
    assert_eq!(target.num_vertices(), 7);
    assert_eq!(target.num_arcs(), 10);
    assert_eq!(target.num_bundles(), 9);

    assert_optimization_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "MIS->IntegralFlowBundles closed loop (path)",
    );
}

#[test]
fn test_maximumindependentset_to_integralflowbundles_triangle() {
    // Triangle: 0-1-2-0, unit weights. Optimal MIS = 1.
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32; 3],
    );
    let reduction = ReduceTo::<IntegralFlowBundles>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 5);
    assert_eq!(target.num_arcs(), 6);
    assert_eq!(target.num_bundles(), 6); // 3 edges + 3 vertices
    assert_eq!(target.requirement(), 1);

    // Each optimal MIS witness (single vertex) should correspond to a feasible flow
    assert_optimization_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "MIS->IntegralFlowBundles closed loop (triangle)",
    );
}

#[test]
fn test_maximumindependentset_to_integralflowbundles_cycle5() {
    // C5 (5-cycle): 5 vertices, 5 edges, unit weights. Optimal MIS = 2.
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]),
        vec![1i32; 5],
    );
    let reduction = ReduceTo::<IntegralFlowBundles>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 7); // 5 + 2
    assert_eq!(target.num_arcs(), 10); // 2 * 5
    assert_eq!(target.num_bundles(), 10); // 5 edges + 5 vertices
    assert_eq!(target.requirement(), 2);

    assert_optimization_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "MIS->IntegralFlowBundles closed loop (C5)",
    );
}

#[test]
fn test_maximumindependentset_to_integralflowbundles_empty_graph() {
    // Empty graph (no edges): all vertices form an IS. Optimal = n.
    let source = MaximumIndependentSet::new(SimpleGraph::new(3, vec![]), vec![1i32; 3]);
    let reduction = ReduceTo::<IntegralFlowBundles>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 5);
    assert_eq!(target.num_arcs(), 6);
    assert_eq!(target.num_bundles(), 3); // 0 edges + 3 vertices
    assert_eq!(target.requirement(), 3);

    assert_optimization_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "MIS->IntegralFlowBundles closed loop (empty graph)",
    );
}

#[test]
fn test_maximumindependentset_to_integralflowbundles_single_vertex() {
    // Single vertex, no edges. Optimal MIS = 1.
    let source = MaximumIndependentSet::new(SimpleGraph::new(1, vec![]), vec![1i32]);
    let reduction = ReduceTo::<IntegralFlowBundles>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 3);
    assert_eq!(target.num_arcs(), 2);
    assert_eq!(target.num_bundles(), 1); // 0 edges + 1 vertex
    assert_eq!(target.requirement(), 1);

    let solver = BruteForce::new();
    let witnesses = solver.find_all_witnesses(target);
    assert!(!witnesses.is_empty());

    // Extract all source solutions
    for w in &witnesses {
        let source_config = reduction.extract_solution(w);
        let value = source.evaluate(&source_config);
        assert!(value.is_valid());
        assert_eq!(value.unwrap(), 1);
    }
}

#[test]
fn test_maximumindependentset_to_integralflowbundles_structure() {
    // Verify the graph structure of the reduction for a simple case: K2
    let source = MaximumIndependentSet::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i32; 2]);
    let reduction = ReduceTo::<IntegralFlowBundles>::reduce_to(&source);
    let target = reduction.target_problem();

    // 4 vertices: s=0, w_0=1, w_1=2, t=3
    assert_eq!(target.num_vertices(), 4);
    // 4 arcs: (0,1), (1,3), (0,2), (2,3)
    assert_eq!(target.num_arcs(), 4);
    // 3 bundles: 1 edge bundle + 2 vertex bundles
    assert_eq!(target.num_bundles(), 3);
    assert_eq!(target.source(), 0);
    assert_eq!(target.sink(), 3);
    assert_eq!(target.requirement(), 1);

    // Edge bundle: {arc_out_0, arc_out_1} = {1, 3}
    // Vertex bundle 0: {arc_in_0, arc_out_0} = {0, 1}
    // Vertex bundle 1: {arc_in_1, arc_out_1} = {2, 3}
    let bundles = target.bundles();
    assert_eq!(bundles[0], vec![1, 3]); // edge bundle
    assert_eq!(bundles[1], vec![0, 1]); // vertex 0 bundle
    assert_eq!(bundles[2], vec![2, 3]); // vertex 1 bundle

    let caps = target.bundle_capacities();
    assert_eq!(caps[0], 1); // edge bundle cap
    assert_eq!(caps[1], 2); // vertex bundle cap
    assert_eq!(caps[2], 2); // vertex bundle cap
}
