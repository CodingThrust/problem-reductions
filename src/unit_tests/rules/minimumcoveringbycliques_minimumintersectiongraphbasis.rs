use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::topology::Graph;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_minimumcoveringbycliques_to_minimumintersectiongraphbasis_closed_loop() {
    let source =
        MinimumCoveringByCliques::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (2, 3)]));
    let reduction = ReduceTo::<MinimumIntersectionGraphBasis<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MinimumCoveringByCliques -> MinimumIntersectionGraphBasis closed loop",
    );
}

#[test]
fn test_minimumcoveringbycliques_to_minimumintersectiongraphbasis_structure_identity() {
    let source =
        MinimumCoveringByCliques::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (2, 3)]));
    let reduction = ReduceTo::<MinimumIntersectionGraphBasis<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), source.num_vertices());
    assert_eq!(target.num_edges(), source.num_edges());
    assert_eq!(target.graph().edges(), source.graph().edges());
}

#[test]
fn test_minimumcoveringbycliques_to_minimumintersectiongraphbasis_issue_example_extraction() {
    let source =
        MinimumCoveringByCliques::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (2, 3)]));
    let reduction = ReduceTo::<MinimumIntersectionGraphBasis<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    let target_solution = intersection_basis_config(target.graph(), &[&[0], &[0], &[0, 1], &[1]]);

    assert_eq!(target.evaluate(&target_solution).unwrap(), Min(Some(2)));

    let extracted = reduction.extract_solution(&target_solution).unwrap();

    assert_eq!(extracted, vec![0, 0, 0, 1]);
    assert_eq!(source.evaluate(&extracted).unwrap(), Min(Some(2)));
}

#[test]
fn test_minimumcoveringbycliques_to_minimumintersectiongraphbasis_invalid_target_rejected() {
    let source = MinimumCoveringByCliques::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let reduction = ReduceTo::<MinimumIntersectionGraphBasis<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    let invalid_target_solution = vec![1; 6];

    assert_eq!(
        target.evaluate(&invalid_target_solution).unwrap(),
        Min(None)
    );

    let error = reduction
        .extract_solution(&invalid_target_solution)
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "target configuration is not a valid intersection graph basis"
    );
}

#[test]
fn test_minimumcoveringbycliques_to_minimumintersectiongraphbasis_empty_graph() {
    let source = MinimumCoveringByCliques::new(SimpleGraph::new(3, vec![]));
    let reduction = ReduceTo::<MinimumIntersectionGraphBasis<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.evaluate(&[]).unwrap(), Min(Some(0)));
    assert_eq!(
        reduction.extract_solution(&[]).unwrap(),
        Vec::<usize>::new()
    );
    assert_eq!(source.evaluate(&[]).unwrap(), Min(Some(0)));
}
