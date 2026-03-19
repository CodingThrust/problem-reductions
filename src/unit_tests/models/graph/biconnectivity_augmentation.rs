use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::One;

#[test]
fn test_biconnectivity_augmentation_creation() {
    let graph = SimpleGraph::path(4);
    let problem = BiconnectivityAugmentation::new(graph.clone(), vec![(0, 3, 2), (1, 3, 1)], 2);

    assert_eq!(problem.graph(), &graph);
    assert_eq!(problem.potential_weights(), &[(0, 3, 2), (1, 3, 1)]);
    assert_eq!(problem.budget(), &2);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.num_potential_edges(), 2);
    assert_eq!(problem.dims(), vec![2, 2]);
    assert_eq!(problem.num_variables(), 2);
    assert!(problem.is_weighted());
    assert_eq!(
        <BiconnectivityAugmentation<SimpleGraph, i32> as Problem>::NAME,
        "BiconnectivityAugmentation"
    );
    assert_eq!(
        <BiconnectivityAugmentation<SimpleGraph, i32> as Problem>::variant(),
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    );

    let unit_problem =
        BiconnectivityAugmentation::<_, One>::new(SimpleGraph::path(3), vec![(0, 2, One)], 1);
    assert!(!unit_problem.is_weighted());
}

#[test]
#[should_panic(expected = "references vertex >= num_vertices")]
fn test_biconnectivity_augmentation_creation_rejects_invalid_potential_edge() {
    BiconnectivityAugmentation::new(SimpleGraph::path(4), vec![(0, 4, 1)], 1);
}

#[test]
#[should_panic(expected = "already exists in the graph")]
fn test_biconnectivity_augmentation_creation_rejects_existing_edge_candidate() {
    BiconnectivityAugmentation::new(SimpleGraph::path(4), vec![(1, 2, 1)], 1);
}

#[test]
#[should_panic(expected = "is duplicated")]
fn test_biconnectivity_augmentation_creation_rejects_duplicate_candidate() {
    BiconnectivityAugmentation::new(SimpleGraph::path(4), vec![(0, 3, 1), (3, 0, 2)], 2);
}

#[test]
fn test_biconnectivity_augmentation_evaluation() {
    let problem = BiconnectivityAugmentation::new(
        SimpleGraph::path(4),
        vec![(0, 2, 5), (1, 3, 1), (0, 3, 2)],
        2,
    );

    assert!(!problem.evaluate(&[0, 0, 0]));
    assert!(!problem.evaluate(&[0, 1, 0]));
    assert!(problem.evaluate(&[0, 0, 1]));
    assert!(!problem.evaluate(&[0, 1, 1]));
    assert!(!problem.evaluate(&[2, 0, 0]));
    assert!(!problem.evaluate(&[1, 0]));
}

#[test]
fn test_biconnectivity_augmentation_serialization() {
    let problem =
        BiconnectivityAugmentation::new(SimpleGraph::path(4), vec![(0, 3, 2), (1, 3, 1)], 2);

    let json = serde_json::to_value(&problem).unwrap();
    let restored: BiconnectivityAugmentation<SimpleGraph, i32> =
        serde_json::from_value(json).unwrap();

    assert_eq!(restored.graph(), problem.graph());
    assert_eq!(restored.potential_weights(), problem.potential_weights());
    assert_eq!(restored.budget(), problem.budget());
}

#[test]
fn test_biconnectivity_augmentation_solver() {
    let problem = BiconnectivityAugmentation::new(
        SimpleGraph::path(4),
        vec![(0, 2, 5), (1, 3, 1), (0, 3, 2)],
        2,
    );
    let solver = BruteForce::new();

    let solution = solver
        .find_satisfying(&problem)
        .expect("expected a satisfying augmentation");
    assert_eq!(solution, vec![0, 0, 1]);

    let all_solutions = solver.find_all_satisfying(&problem);
    assert_eq!(all_solutions, vec![vec![0, 0, 1]]);
}

#[test]
fn test_biconnectivity_augmentation_no_solution() {
    let problem = BiconnectivityAugmentation::new(SimpleGraph::path(4), vec![(0, 2, 1)], 1);
    let solver = BruteForce::new();

    assert!(solver.find_satisfying(&problem).is_none());
    assert!(solver.find_all_satisfying(&problem).is_empty());
}

#[test]
fn test_biconnectivity_augmentation_paper_example() {
    let problem = example_instance();
    let solver = BruteForce::new();
    let satisfying_config = vec![1, 0, 0, 1, 0, 0, 1, 0, 1];
    let satisfying_solutions = solver.find_all_satisfying(&problem);

    assert!(problem.evaluate(&satisfying_config));
    assert!(satisfying_solutions.contains(&satisfying_config));

    let over_budget_problem = BiconnectivityAugmentation::new(
        SimpleGraph::path(6),
        vec![
            (0, 2, 1),
            (0, 3, 2),
            (0, 4, 3),
            (1, 3, 1),
            (1, 4, 2),
            (1, 5, 3),
            (2, 4, 1),
            (2, 5, 2),
            (3, 5, 1),
        ],
        3,
    );
    assert!(!over_budget_problem.evaluate(&satisfying_config));
    assert!(solver.find_satisfying(&over_budget_problem).is_none());
}

#[test]
fn test_is_biconnected() {
    assert!(is_biconnected(&SimpleGraph::cycle(4)));
    assert!(is_biconnected(&SimpleGraph::complete(3)));
    assert!(!is_biconnected(&SimpleGraph::path(4)));
    assert!(!is_biconnected(&SimpleGraph::new(4, vec![(0, 1), (2, 3)])));
}
