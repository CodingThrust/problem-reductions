use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::variant::K3;

#[test]
fn test_kcoloring_to_clustering_closed_loop() {
    let source = KColoring::<K3, _>::new(SimpleGraph::cycle(5));
    let reduction = ReduceTo::<Clustering>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "KColoring->Clustering closed loop",
    );
}

#[test]
fn test_kcoloring_to_clustering_distance_matrix() {
    let source = KColoring::<K3, _>::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction = ReduceTo::<Clustering>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_elements(), 4);
    assert_eq!(target.num_clusters(), 3);
    assert_eq!(target.diameter_bound(), 0);
    assert_eq!(
        target.distances(),
        &[
            vec![0, 1, 0, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 0, 1],
            vec![0, 0, 1, 0],
        ]
    );
}

#[test]
fn test_kcoloring_to_clustering_extract_solution_identity() {
    let source = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let reduction = ReduceTo::<Clustering>::reduce_to(&source);
    let config = vec![0, 1, 0];

    assert_eq!(reduction.extract_solution(&config), config);
}

#[test]
fn test_kcoloring_to_clustering_unsat_preserved() {
    let source = KColoring::<K3, _>::new(SimpleGraph::complete(4));
    let reduction = ReduceTo::<Clustering>::reduce_to(&source);
    let solver = BruteForce::new();

    assert!(solver.find_witness(&source).is_none());
    assert!(solver.find_witness(reduction.target_problem()).is_none());
}

#[test]
fn test_kcoloring_to_clustering_empty_graph() {
    let source = KColoring::<K3, _>::new(SimpleGraph::new(0, vec![]));
    let reduction = ReduceTo::<Clustering>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_elements(), 1);
    assert_eq!(target.num_clusters(), 3);
    assert_eq!(target.diameter_bound(), 0);
    assert_eq!(reduction.extract_solution(&[2]), Vec::<usize>::new());
    assert_satisfaction_round_trip_from_satisfaction_target(&source, &reduction, "empty graph");
}
