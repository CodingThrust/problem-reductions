use super::*;
use crate::solvers::BruteForceProblem as _;
#[test]
fn create_spec_rejects_existing_potential_edge() {
    assert!(
        BiconnectivityAugmentation::try_from(BiconnectivityAugmentationCreateSpec {
            graph: vec![(0, 1)],
            num_vertices: None,
            potential_weights: vec![(0, 1, 2)],
            budget: 3
        })
        .is_err()
    );
}
use crate::solvers::BruteForce;
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
    assert_eq!(problem.dimensions(), vec![2, 2]);
    assert_eq!(problem.num_variables(), 2);
    assert!(problem.is_weighted());
    assert_eq!(
        <BiconnectivityAugmentation<SimpleGraph, i64> as Problem>::NAME,
        "BiconnectivityAugmentation"
    );
    assert_eq!(
        <BiconnectivityAugmentation<SimpleGraph, i64> as Problem>::variant(),
        vec![("graph", "SimpleGraph"), ("weight", "i64")]
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

    assert!(!problem.evaluate(&vec![false, false, false]).unwrap());
    assert!(!problem.evaluate(&vec![false, true, false]).unwrap());
    assert!(problem.evaluate(&vec![false, false, true]).unwrap());
    assert!(!problem.evaluate(&vec![false, true, true]).unwrap());
    assert!(crate::registry::DynProblem::evaluate_dyn(
        &problem,
        &serde_json::json!([2, false, false])
    )
    .is_err());
    assert!(matches!(
        problem.evaluate(&vec![true, false]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_biconnectivity_augmentation_serialization() {
    let problem =
        BiconnectivityAugmentation::new(SimpleGraph::path(4), vec![(0, 3, 2), (1, 3, 1)], 2);

    let json = serde_json::to_value(&problem).unwrap();
    let restored: BiconnectivityAugmentation<SimpleGraph, i64> =
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
        .solve(&problem)
        .unwrap()
        .expect("expected a satisfying augmentation");
    assert_eq!(solution, vec![false, false, true]);

    let all_solutions = solver.find_all_witnesses(&problem).unwrap();
    assert_eq!(all_solutions, vec![vec![false, false, true]]);
}

#[test]
fn test_biconnectivity_augmentation_no_solution() {
    let problem = BiconnectivityAugmentation::new(SimpleGraph::path(4), vec![(0, 2, 1)], 1);
    let solver = BruteForce::new();

    assert!(solver.solve(&problem).unwrap().is_none());
    assert!(solver.find_all_witnesses(&problem).unwrap().is_empty());
}

#[test]
fn test_biconnectivity_augmentation_paper_example() {
    let problem = example_instance();
    let solver = BruteForce::new();
    let satisfying_config = vec![true, false, false, true, false, false, true, false, true];
    let satisfying_solutions = solver.find_all_witnesses(&problem).unwrap();

    assert!(problem.evaluate(&satisfying_config).unwrap());
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
    assert!(!over_budget_problem.evaluate(&satisfying_config).unwrap());
    assert!(solver.solve(&over_budget_problem).unwrap().is_none());
}

#[test]
fn test_is_biconnected() {
    assert!(is_biconnected(&SimpleGraph::cycle(4)));
    assert!(is_biconnected(&SimpleGraph::complete(3)));
    assert!(!is_biconnected(&SimpleGraph::path(4)));
    assert!(!is_biconnected(&SimpleGraph::new(4, vec![(0, 1), (2, 3)])));
}
