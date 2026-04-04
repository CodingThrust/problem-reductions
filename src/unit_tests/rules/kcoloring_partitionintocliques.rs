use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::topology::Graph;
use crate::variant::KN;

#[test]
fn test_kcoloring_to_partitionintocliques_closed_loop() {
    let source = KColoring::<KN, _>::with_k(
        SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]),
        3,
    );
    let reduction = ReduceTo::<PartitionIntoCliques<SimpleGraph>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "KColoring->PartitionIntoCliques closed loop",
    );
}

#[test]
fn test_kcoloring_to_partitionintocliques_complement_structure() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]), 2);
    let reduction = ReduceTo::<PartitionIntoCliques<SimpleGraph>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.graph().num_vertices(), 4);
    assert_eq!(target.graph().num_edges(), 3);
    assert_eq!(target.num_cliques(), 2);
    assert!(target.graph().has_edge(0, 2));
    assert!(target.graph().has_edge(0, 3));
    assert!(target.graph().has_edge(1, 3));
}

#[test]
fn test_kcoloring_to_partitionintocliques_extract_solution_identity() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), 2);
    let reduction = ReduceTo::<PartitionIntoCliques<SimpleGraph>>::reduce_to(&source);
    let config = vec![0, 1, 0];

    assert_eq!(reduction.extract_solution(&config), config);
}

#[test]
fn test_kcoloring_to_partitionintocliques_unsat_preserved() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]), 2);
    let reduction = ReduceTo::<PartitionIntoCliques<SimpleGraph>>::reduce_to(&source);
    let solver = BruteForce::new();

    assert!(solver.find_witness(&source).is_none());
    assert!(solver.find_witness(reduction.target_problem()).is_none());
}
