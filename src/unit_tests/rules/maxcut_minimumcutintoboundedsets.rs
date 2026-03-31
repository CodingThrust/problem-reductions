use super::*;
use crate::models::graph::{MaxCut, MinimumCutIntoBoundedSets};
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::traits::ReduceTo;
use crate::topology::SimpleGraph;

#[test]
fn test_maxcut_to_minimumcutintoboundedsets_closed_loop() {
    // Triangle K_3 with unit weights: max cut = 2
    let source = MaxCut::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32, 1, 1],
    );
    let reduction = ReduceTo::<MinimumCutIntoBoundedSets<SimpleGraph, i32>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaxCut triangle -> MinCutBounded",
    );
}

#[test]
fn test_maxcut_to_minimumcutintoboundedsets_single_edge() {
    // Single edge K_2: max cut = 1
    let source = MaxCut::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i32]);
    let reduction = ReduceTo::<MinimumCutIntoBoundedSets<SimpleGraph, i32>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaxCut single edge -> MinCutBounded",
    );
}

#[test]
fn test_maxcut_to_minimumcutintoboundedsets_path_p4() {
    // Path P_4: vertices 0-1-2-3, unit weights, max cut = 3 (alternate: 0,1,0,1)
    let source = MaxCut::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32, 1, 1],
    );
    let reduction = ReduceTo::<MinimumCutIntoBoundedSets<SimpleGraph, i32>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaxCut path P4 -> MinCutBounded",
    );
}

#[test]
fn test_maxcut_to_minimumcutintoboundedsets_weighted() {
    // Triangle with weights [1, 2, 3]: max cut = 5 (cut edges with weights 2 and 3)
    let source = MaxCut::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32, 2, 3],
    );
    let reduction = ReduceTo::<MinimumCutIntoBoundedSets<SimpleGraph, i32>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaxCut weighted triangle -> MinCutBounded",
    );
}

#[test]
fn test_maxcut_to_minimumcutintoboundedsets_target_structure() {
    // Verify the target problem structure for a 3-vertex graph
    let source = MaxCut::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32, 1, 1],
    );
    let reduction = ReduceTo::<MinimumCutIntoBoundedSets<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // n=3, n'=3+1=4, N=8
    assert_eq!(target.num_vertices(), 8);
    // Complete graph K_8 has C(8,2) = 28 edges
    assert_eq!(target.num_edges(), 28);
    // source=n'=4, sink=n'+1=5
    assert_eq!(target.source(), 4);
    assert_eq!(target.sink(), 5);
    // size_bound = n' = 4
    assert_eq!(target.size_bound(), 4);
}

#[test]
fn test_maxcut_to_minimumcutintoboundedsets_even_vertices() {
    // Even number of vertices: n=4, n'=4, N=8
    let source = MaxCut::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]),
        vec![1i32, 1, 1, 1],
    );
    let reduction = ReduceTo::<MinimumCutIntoBoundedSets<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // n=4, n'=4, N=8
    assert_eq!(target.num_vertices(), 8);
    assert_eq!(target.num_edges(), 28); // K_8
    assert_eq!(target.source(), 4);
    assert_eq!(target.sink(), 5);
    assert_eq!(target.size_bound(), 4);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaxCut even vertices -> MinCutBounded",
    );
}

#[test]
fn test_maxcut_to_minimumcutintoboundedsets_extract_solution_size() {
    // Verify extract_solution returns only original vertices
    let source = MaxCut::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32, 1, 1],
    );
    let reduction = ReduceTo::<MinimumCutIntoBoundedSets<SimpleGraph, i32>>::reduce_to(&source);

    // Target has 8 vertices, extract should return 3
    let dummy_target_sol = vec![0, 1, 0, 1, 0, 1, 0, 1];
    let extracted = reduction.extract_solution(&dummy_target_sol);
    assert_eq!(extracted.len(), 3);
}

#[test]
fn test_maxcut_to_minimumcutintoboundedsets_weight_inversion() {
    // Verify weight inversion: original edge gets W_max - w, non-edge gets W_max
    // Use n=2 to keep the target small: n'=2, N=4, K_4 has 6 edges
    let source = MaxCut::new(SimpleGraph::new(2, vec![(0, 1)]), vec![5i32]);
    let reduction = ReduceTo::<MinimumCutIntoBoundedSets<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // W_max = 5 + 1 = 6
    // n=2, n'=2, N=4, K_4 has 6 edges
    assert_eq!(target.num_vertices(), 4);
    assert_eq!(target.num_edges(), 6);

    // Edge (0,1) in original: weight = 6 - 5 = 1
    // All other edges: weight = 6
    let edge_weights = target.edge_weights();
    assert_eq!(edge_weights[0], 1); // (0,1): W_max - 5 = 1
    assert_eq!(edge_weights[1], 6); // (0,2): non-edge
    assert_eq!(edge_weights[2], 6); // (0,3): non-edge
    assert_eq!(edge_weights[3], 6); // (1,2): non-edge
    assert_eq!(edge_weights[4], 6); // (1,3): non-edge
    assert_eq!(edge_weights[5], 6); // (2,3): non-edge
}
