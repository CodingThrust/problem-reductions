use super::*;
use crate::solvers::BruteForceProblem as _;

#[test]
fn create_spec_defaults_edge_weights() {
    let p = MaximumEdgeWeightedKClique::try_from(MaximumEdgeWeightedKCliqueCreateSpec::<i64> {
        graph: SimpleGraph::new(2, vec![(0, 1)]),
        edge_weights: None,
        k: 2,
    })
    .unwrap();
    assert_eq!(p.edge_weights(), &[1]);
}
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Max;

/// Canonical instance from issue #1020: 4 vertices, edges
/// {(0,1), (0,2), (1,2), (0,3), (1,3)} with weights [5, 4, -1, 1, 0]
/// and k = 3. Triangles are {0,1,2} (value 8) and {0,1,3} (value 6).
fn issue_instance() -> MaximumEdgeWeightedKClique<i64> {
    MaximumEdgeWeightedKClique::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
        vec![5, 4, -1, 1, 0],
        3,
    )
    .unwrap()
}

#[test]
fn test_maximum_edge_weighted_k_clique_creation() {
    let problem = issue_instance();
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 5);
    assert_eq!(problem.k(), 3);
    assert_eq!(problem.edge_weights(), &[5, 4, -1, 1, 0]);
    assert_eq!(problem.dimensions(), vec![2; 4]);
    assert_eq!(problem.num_variables(), 4);
    assert!(problem.graph().has_edge(0, 1));
    assert!(!problem.graph().has_edge(2, 3));
}

#[test]
fn test_maximum_edge_weighted_k_clique_evaluate_feasible() {
    let problem = issue_instance();

    // Optimum from the issue: S = {0,1,2}, value 5 + 4 + (-1) = 8.
    assert_eq!(
        problem.evaluate(&vec![true, true, true, false]).unwrap(),
        Max(Some(8))
    );
    assert!(problem.is_valid_solution(&[true, true, true, false]));

    // Other feasible 3-clique {0,1,3} with value 5 + 1 + 0 = 6.
    assert_eq!(
        problem.evaluate(&vec![true, true, false, true]).unwrap(),
        Max(Some(6))
    );
    assert!(problem.is_valid_solution(&[true, true, false, true]));
}

#[test]
fn test_maximum_edge_weighted_k_clique_evaluate_infeasible_wrong_size() {
    let problem = issue_instance();

    // |S| = 2 != k = 3 -> infeasible.
    assert_eq!(
        problem.evaluate(&vec![true, true, false, false]).unwrap(),
        Max(None)
    );
    assert!(!problem.is_valid_solution(&[true, true, false, false]));

    // |S| = 4 != k = 3 -> infeasible (also not a 4-clique here).
    assert_eq!(
        problem.evaluate(&vec![true, true, true, true]).unwrap(),
        Max(None)
    );

    // Empty selection: |S| = 0 != k = 3 -> infeasible.
    assert_eq!(
        problem.evaluate(&vec![false, false, false, false]).unwrap(),
        Max(None)
    );
}

#[test]
fn test_maximum_edge_weighted_k_clique_evaluate_infeasible_not_clique() {
    let problem = issue_instance();

    // {0,2,3}: edge (2,3) is not present in E -> not a clique.
    assert_eq!(
        problem.evaluate(&vec![true, false, true, true]).unwrap(),
        Max(None)
    );
    assert!(!problem.is_valid_solution(&[true, false, true, true]));

    // {1,2,3}: edge (2,3) is not present -> not a clique.
    assert_eq!(
        problem.evaluate(&vec![false, true, true, true]).unwrap(),
        Max(None)
    );
}

#[test]
fn test_maximum_edge_weighted_k_clique_brute_force() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    assert_eq!(
        problem
            .evaluate(&solver.solve(&problem).unwrap().unwrap())
            .unwrap(),
        Max(Some(8))
    );

    let witness = solver.solve(&problem).unwrap().expect("witness exists");
    assert!(problem.is_valid_solution(&witness));
    assert_eq!(problem.evaluate(&witness).unwrap(), Max(Some(8)));
}

#[test]
fn test_maximum_edge_weighted_k_clique_k_zero_returns_zero() {
    // With k = 0 the unique feasible config selects no vertices and the
    // induced edge set is empty, so the objective is 0.
    let problem = MaximumEdgeWeightedKClique::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
        vec![5, 4, -1, 1, 0],
        0,
    )
    .unwrap();
    assert_eq!(
        problem.evaluate(&vec![false, false, false, false]).unwrap(),
        Max(Some(0))
    );
    // Any nonempty selection violates |S| = k = 0.
    assert_eq!(
        problem.evaluate(&vec![true, false, false, false]).unwrap(),
        Max(None)
    );
    assert_eq!(
        problem
            .evaluate(&BruteForce::new().solve(&problem).unwrap().unwrap())
            .unwrap(),
        Max(Some(0))
    );
}

#[test]
fn test_maximum_edge_weighted_k_clique_k_one_returns_zero() {
    // For k = 1 every single-vertex selection is a trivial clique and the
    // induced edge set is empty regardless of edge weights, so all feasible
    // configurations evaluate to 0.
    let problem = MaximumEdgeWeightedKClique::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
        vec![5, 4, -1, 1, 0],
        1,
    )
    .unwrap();
    assert_eq!(
        problem.evaluate(&vec![true, false, false, false]).unwrap(),
        Max(Some(0))
    );
    assert_eq!(
        problem.evaluate(&vec![false, false, true, false]).unwrap(),
        Max(Some(0))
    );
    // |S| = 0 != 1 -> infeasible.
    assert_eq!(
        problem.evaluate(&vec![false, false, false, false]).unwrap(),
        Max(None)
    );
    assert_eq!(
        problem
            .evaluate(&BruteForce::new().solve(&problem).unwrap().unwrap())
            .unwrap(),
        Max(Some(0))
    );
}

#[test]
fn test_maximum_edge_weighted_k_clique_f64_variant() {
    // f64 variant exercises the additional registered weight type.
    let problem = MaximumEdgeWeightedKClique::<f64>::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
        vec![5.0, 4.0, -1.0, 1.0, 0.0],
        3,
    )
    .unwrap();
    assert_eq!(
        problem.evaluate(&vec![true, true, true, false]).unwrap(),
        Max(Some(8.0))
    );
    assert_eq!(
        problem
            .evaluate(&BruteForce::new().solve(&problem).unwrap().unwrap())
            .unwrap(),
        Max(Some(8.0))
    );
}

#[test]
fn test_maximum_edge_weighted_k_clique_serialization_roundtrip() {
    let problem = issue_instance();
    let json = serde_json::to_value(&problem).expect("serialize");
    let restored: MaximumEdgeWeightedKClique<i64> =
        serde_json::from_value(json).expect("deserialize");
    assert_eq!(restored.num_vertices(), 4);
    assert_eq!(restored.num_edges(), 5);
    assert_eq!(restored.k(), 3);
    assert_eq!(restored.edge_weights(), &[5, 4, -1, 1, 0]);
    assert_eq!(
        restored.evaluate(&vec![true, true, true, false]).unwrap(),
        Max(Some(8))
    );
}

#[test]
fn test_maximum_edge_weighted_k_clique_problem_name_and_variant() {
    assert_eq!(
        <MaximumEdgeWeightedKClique<i64> as Problem>::NAME,
        "MaximumEdgeWeightedKClique"
    );
    let v = <MaximumEdgeWeightedKClique<i64> as Problem>::variant();
    assert!(v.contains(&("weight", "i64")));
}

#[test]
fn test_maximum_edge_weighted_k_clique_rejects_weight_length_mismatch() {
    let error = MaximumEdgeWeightedKClique::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
        vec![1, 2, 3, 4], // length 4 != 5 edges
        3,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        crate::registry::ConstructionError::Conversion(message)
            if message == "edge_weights length must match graph num_edges"
    ));
}

#[test]
fn test_maximum_edge_weighted_k_clique_rejects_k_greater_than_n() {
    let error = MaximumEdgeWeightedKClique::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
        vec![5, 4, -1, 1, 0],
        5,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        crate::registry::ConstructionError::Conversion(message)
            if message == "k = 5 must be <= num_vertices = 4"
    ));
}

#[test]
fn test_maximum_edge_weighted_k_clique_rejects_non_finite_weight() {
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    assert!(MaximumEdgeWeightedKClique::new(graph, vec![f64::NAN], 2).is_err());
}
