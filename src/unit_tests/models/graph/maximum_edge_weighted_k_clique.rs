use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Max;
use crate::Solver;

/// Canonical instance from issue #1020: 4 vertices, edges
/// {(0,1), (0,2), (1,2), (0,3), (1,3)} with weights [5, 4, -1, 1, 0]
/// and k = 3. Triangles are {0,1,2} (value 8) and {0,1,3} (value 6).
fn issue_instance() -> MaximumEdgeWeightedKClique<i32> {
    MaximumEdgeWeightedKClique::<i32>::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
        vec![5, 4, -1, 1, 0],
        3,
    )
}

#[test]
fn test_maximum_edge_weighted_k_clique_creation() {
    let problem = issue_instance();
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 5);
    assert_eq!(problem.k(), 3);
    assert_eq!(problem.edge_weights(), &[5, 4, -1, 1, 0]);
    assert_eq!(problem.dims(), vec![2; 4]);
    assert_eq!(problem.num_variables(), 4);
    assert!(problem.graph().has_edge(0, 1));
    assert!(!problem.graph().has_edge(2, 3));
}

#[test]
fn test_maximum_edge_weighted_k_clique_evaluate_feasible() {
    let problem = issue_instance();

    // Optimum from the issue: S = {0,1,2}, value 5 + 4 + (-1) = 8.
    assert_eq!(problem.evaluate(&[1, 1, 1, 0]), Max(Some(8)));
    assert!(problem.is_valid_solution(&[1, 1, 1, 0]));

    // Other feasible 3-clique {0,1,3} with value 5 + 1 + 0 = 6.
    assert_eq!(problem.evaluate(&[1, 1, 0, 1]), Max(Some(6)));
    assert!(problem.is_valid_solution(&[1, 1, 0, 1]));
}

#[test]
fn test_maximum_edge_weighted_k_clique_evaluate_infeasible_wrong_size() {
    let problem = issue_instance();

    // |S| = 2 != k = 3 -> infeasible.
    assert_eq!(problem.evaluate(&[1, 1, 0, 0]), Max(None));
    assert!(!problem.is_valid_solution(&[1, 1, 0, 0]));

    // |S| = 4 != k = 3 -> infeasible (also not a 4-clique here).
    assert_eq!(problem.evaluate(&[1, 1, 1, 1]), Max(None));

    // Empty selection: |S| = 0 != k = 3 -> infeasible.
    assert_eq!(problem.evaluate(&[0, 0, 0, 0]), Max(None));
}

#[test]
fn test_maximum_edge_weighted_k_clique_evaluate_infeasible_not_clique() {
    let problem = issue_instance();

    // {0,2,3}: edge (2,3) is not present in E -> not a clique.
    assert_eq!(problem.evaluate(&[1, 0, 1, 1]), Max(None));
    assert!(!problem.is_valid_solution(&[1, 0, 1, 1]));

    // {1,2,3}: edge (2,3) is not present -> not a clique.
    assert_eq!(problem.evaluate(&[0, 1, 1, 1]), Max(None));
}

#[test]
fn test_maximum_edge_weighted_k_clique_brute_force() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    assert_eq!(solver.solve(&problem), Max(Some(8)));

    let witness = solver.find_witness(&problem).expect("witness exists");
    assert!(problem.is_valid_solution(&witness));
    assert_eq!(problem.evaluate(&witness), Max(Some(8)));
}

#[test]
fn test_maximum_edge_weighted_k_clique_k_zero_returns_zero() {
    // With k = 0 the unique feasible config selects no vertices and the
    // induced edge set is empty, so the objective is 0.
    let problem = MaximumEdgeWeightedKClique::<i32>::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
        vec![5, 4, -1, 1, 0],
        0,
    );
    assert_eq!(problem.evaluate(&[0, 0, 0, 0]), Max(Some(0)));
    // Any nonempty selection violates |S| = k = 0.
    assert_eq!(problem.evaluate(&[1, 0, 0, 0]), Max(None));
    assert_eq!(BruteForce::new().solve(&problem), Max(Some(0)));
}

#[test]
fn test_maximum_edge_weighted_k_clique_k_one_returns_zero() {
    // For k = 1 every single-vertex selection is a trivial clique and the
    // induced edge set is empty regardless of edge weights, so all feasible
    // configurations evaluate to 0.
    let problem = MaximumEdgeWeightedKClique::<i32>::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
        vec![5, 4, -1, 1, 0],
        1,
    );
    assert_eq!(problem.evaluate(&[1, 0, 0, 0]), Max(Some(0)));
    assert_eq!(problem.evaluate(&[0, 0, 1, 0]), Max(Some(0)));
    // |S| = 0 != 1 -> infeasible.
    assert_eq!(problem.evaluate(&[0, 0, 0, 0]), Max(None));
    assert_eq!(BruteForce::new().solve(&problem), Max(Some(0)));
}

#[test]
fn test_maximum_edge_weighted_k_clique_f64_variant() {
    // f64 variant exercises the additional registered weight type.
    let problem = MaximumEdgeWeightedKClique::<f64>::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
        vec![5.0, 4.0, -1.0, 1.0, 0.0],
        3,
    );
    assert_eq!(problem.evaluate(&[1, 1, 1, 0]), Max(Some(8.0)));
    assert_eq!(BruteForce::new().solve(&problem), Max(Some(8.0)));
}

#[test]
fn test_maximum_edge_weighted_k_clique_serialization_roundtrip() {
    let problem = issue_instance();
    let json = serde_json::to_value(&problem).expect("serialize");
    let restored: MaximumEdgeWeightedKClique<i32> =
        serde_json::from_value(json).expect("deserialize");
    assert_eq!(restored.num_vertices(), 4);
    assert_eq!(restored.num_edges(), 5);
    assert_eq!(restored.k(), 3);
    assert_eq!(restored.edge_weights(), &[5, 4, -1, 1, 0]);
    assert_eq!(restored.evaluate(&[1, 1, 1, 0]), Max(Some(8)));
}

#[test]
fn test_maximum_edge_weighted_k_clique_problem_name_and_variant() {
    assert_eq!(
        <MaximumEdgeWeightedKClique<i32> as Problem>::NAME,
        "MaximumEdgeWeightedKClique"
    );
    let v = <MaximumEdgeWeightedKClique<i32> as Problem>::variant();
    assert!(v.contains(&("weight", "i32")));
}

#[test]
#[should_panic(expected = "edge_weights length must match graph num_edges")]
fn test_maximum_edge_weighted_k_clique_rejects_weight_length_mismatch() {
    let _ = MaximumEdgeWeightedKClique::<i32>::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
        vec![1, 2, 3, 4], // length 4 != 5 edges
        3,
    );
}

#[test]
#[should_panic(expected = "k = 5 must be <= num_vertices = 4")]
fn test_maximum_edge_weighted_k_clique_rejects_k_greater_than_n() {
    let _ = MaximumEdgeWeightedKClique::<i32>::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (0, 3), (1, 3)]),
        vec![5, 4, -1, 1, 0],
        5,
    );
}
