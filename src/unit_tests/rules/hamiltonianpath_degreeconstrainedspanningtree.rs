use crate::models::graph::{DegreeConstrainedSpanningTree, HamiltonianPath};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;

fn edge_config(graph: &SimpleGraph, selected_edges: &[(usize, usize)]) -> Vec<usize> {
    graph
        .edges()
        .into_iter()
        .map(|(u, v)| {
            usize::from(
                selected_edges
                    .iter()
                    .any(|&(a, b)| (a == u && b == v) || (a == v && b == u)),
            )
        })
        .collect()
}

#[test]
fn test_hamiltonianpath_to_degreeconstrainedspanningtree_structure() {
    let source = HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 2)]));
    let reduction = ReduceTo::<DegreeConstrainedSpanningTree<SimpleGraph>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.graph(), source.graph());
    assert_eq!(target.num_vertices(), source.num_vertices());
    assert_eq!(target.num_edges(), source.num_edges());
    assert_eq!(target.max_degree(), 2);
}

#[test]
fn test_hamiltonianpath_to_degreeconstrainedspanningtree_closed_loop() {
    let source = HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 2)]));
    let reduction = ReduceTo::<DegreeConstrainedSpanningTree<SimpleGraph>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "HamiltonianPath->DegreeConstrainedSpanningTree closed loop",
    );
}

#[test]
fn test_hamiltonianpath_to_degreeconstrainedspanningtree_extract_solution_reconstructs_order() {
    let source = HamiltonianPath::new(SimpleGraph::path(4));
    let reduction = ReduceTo::<DegreeConstrainedSpanningTree<SimpleGraph>>::reduce_to(&source);
    let target_solution = edge_config(
        reduction.target_problem().graph(),
        &[(0, 1), (1, 2), (2, 3)],
    );

    let extracted = reduction.extract_solution(&target_solution);

    assert_eq!(extracted, vec![0, 1, 2, 3]);
    assert!(source.evaluate(&extracted));
}
