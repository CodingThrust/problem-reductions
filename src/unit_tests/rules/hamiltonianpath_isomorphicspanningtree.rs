use super::*;
use crate::models::graph::{HamiltonianPath, IsomorphicSpanningTree};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_hamiltonianpath_to_isomorphicspanningtree_structure() {
    let source = HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 2)]));
    let reduction: ReductionHamiltonianPathToIsomorphicSpanningTree =
        ReduceTo::<IsomorphicSpanningTree<SimpleGraph>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.graph(), source.graph());
    assert_eq!(target.num_vertices(), source.num_vertices());
    assert_eq!(target.num_edges(), source.num_edges());
    assert_eq!(target.tree(), &SimpleGraph::path(source.num_vertices()));
    assert_eq!(target.tree_edges(), vec![(0, 1), (1, 2), (2, 3)]);
}

#[test]
fn test_hamiltonianpath_to_isomorphicspanningtree_closed_loop() {
    let source = HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 2)]));
    let reduction: ReductionHamiltonianPathToIsomorphicSpanningTree =
        ReduceTo::<IsomorphicSpanningTree<SimpleGraph>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "HamiltonianPath->IsomorphicSpanningTree closed loop",
    );
}

#[test]
fn test_hamiltonianpath_to_isomorphicspanningtree_extract_solution_is_identity_mapping() {
    let source = HamiltonianPath::new(SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (2, 3),
            (3, 4),
            (3, 5),
            (4, 2),
            (5, 1),
        ],
    ));
    let reduction: ReductionHamiltonianPathToIsomorphicSpanningTree =
        ReduceTo::<IsomorphicSpanningTree<SimpleGraph>>::reduce_to(&source);
    let target_solution = vec![0, 2, 4, 3, 1, 5];

    assert!(reduction.target_problem().evaluate(&target_solution));

    let extracted = reduction.extract_solution(&target_solution);

    assert_eq!(extracted, target_solution);
    assert!(source.evaluate(&extracted));
}
