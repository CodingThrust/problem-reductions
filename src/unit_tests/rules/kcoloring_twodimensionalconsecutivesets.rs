use super::*;
use crate::models::graph::KColoring;
use crate::models::set::TwoDimensionalConsecutiveSets;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::traits::ReduceTo;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::variant::K3;

#[test]
fn test_kcoloring_to_twodimensionalconsecutivesets_closed_loop() {
    // Triangle graph: 3-colorable
    let source = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
    let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "K3-coloring triangle -> TDCS",
    );
}

#[test]
fn test_kcoloring_to_tdcs_target_structure() {
    // Graph with 4 vertices and 3 edges: path 0-1-2-3
    let source = KColoring::<K3, _>::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source);
    let target = reduction.target_problem();

    // Alphabet: 4 vertices + 3 edges = 7
    assert_eq!(target.alphabet_size(), 7);
    // One subset per edge
    assert_eq!(target.num_subsets(), 3);
    // Each subset has size 3
    for subset in target.subsets() {
        assert_eq!(subset.len(), 3);
    }
}

#[test]
fn test_kcoloring_to_tdcs_non_3colorable() {
    // K3 with an extra vertex connected to all 3: K4 restricted to 3 vertices + 1
    // Use K_3 + edge to make a non-3-colorable subgraph: vertex 0 connected to 1, 2;
    // vertex 1 connected to 2; all three connected to vertex 3
    // This is K4 but we only check source side (target brute-force too slow).
    let source = KColoring::<K3, _>::new(SimpleGraph::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    ));

    let solver = BruteForce::new();
    let source_solutions = solver.find_all_witnesses(&source);
    assert!(source_solutions.is_empty(), "K4 is not 3-colorable");

    // Verify the reduction produces the correct structure
    let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source);
    let target = reduction.target_problem();
    assert_eq!(target.alphabet_size(), 10); // 4 vertices + 6 edges
    assert_eq!(target.num_subsets(), 6);
}

#[test]
fn test_kcoloring_to_tdcs_bipartite() {
    // Path 0-1-2: bipartite, 2-colorable (hence 3-colorable)
    let source = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "K3-coloring path -> TDCS",
    );
}

#[test]
fn test_kcoloring_to_tdcs_single_edge() {
    // Single edge: trivially 3-colorable
    let source = KColoring::<K3, _>::new(SimpleGraph::new(2, vec![(0, 1)]));
    let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.alphabet_size(), 3); // 2 vertices + 1 edge
    assert_eq!(target.num_subsets(), 1);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "K3-coloring single edge -> TDCS",
    );
}

#[test]
fn test_kcoloring_to_tdcs_extract_solution_valid() {
    // Triangle: verify extracted coloring is valid
    let source = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
    let reduction = ReduceTo::<TwoDimensionalConsecutiveSets>::reduce_to(&source);

    let solver = BruteForce::new();
    let target_solutions = solver.find_all_witnesses(reduction.target_problem());

    for target_sol in &target_solutions {
        let source_sol = reduction.extract_solution(target_sol);
        assert_eq!(source_sol.len(), 3);
        // Verify it is a valid coloring
        assert!(
            source.evaluate(&source_sol).0,
            "Extracted coloring must be valid: {:?}",
            source_sol
        );
    }
}
