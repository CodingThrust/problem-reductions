use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::{Max, One};
use crate::variant::KN;
use crate::Solver;

fn c5() -> SimpleGraph {
    SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)])
}

fn issue_instance() -> MaximumCoKPlex<SimpleGraph, i32, KN> {
    MaximumCoKPlex::<_, i32, KN>::with_k(c5(), vec![5, 1, 4, 1, 3], 2)
}

#[test]
fn test_maximum_co_k_plex_creation() {
    let problem = issue_instance();
    assert_eq!(problem.graph().num_vertices(), 5);
    assert_eq!(problem.graph().num_edges(), 5);
    assert_eq!(problem.weights(), &[5, 1, 4, 1, 3]);
    assert_eq!(problem.bound_k(), 2);
    assert_eq!(problem.dims(), vec![2; 5]);
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 5);
    assert!(problem.is_weighted());
}

#[test]
fn test_maximum_co_k_plex_evaluate_feasible() {
    let problem = issue_instance();

    // Optimum from the issue: x = (1,0,1,0,1), S = {0,2,4}.
    // Induced subgraph has only the edge (4,0); induced degrees (1,0,1) all <= 1.
    assert_eq!(problem.evaluate(&[1, 0, 1, 0, 1]), Max(Some(5 + 4 + 3)));

    // S = {0,1}: induced edge (0,1), induced degrees (1,1) -- still feasible at k=2.
    assert_eq!(problem.evaluate(&[1, 1, 0, 0, 0]), Max(Some(5 + 1)));

    // Empty set: always feasible.
    assert_eq!(problem.evaluate(&[0; 5]), Max(Some(0)));
}

#[test]
fn test_maximum_co_k_plex_evaluate_infeasible() {
    let problem = issue_instance();

    // S = {0,1,2}: vertex 1 has induced degree 2 > k-1 = 1.
    assert_eq!(problem.evaluate(&[1, 1, 1, 0, 0]), Max(None));
    assert!(!problem.is_valid_solution(&[1, 1, 1, 0, 0]));

    // Whole 5-cycle: every vertex has induced degree 2 > 1.
    assert_eq!(problem.evaluate(&[1; 5]), Max(None));
}

#[test]
fn test_maximum_co_k_plex_brute_force() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    let aggregate = solver.solve(&problem);
    assert_eq!(aggregate, Max(Some(12)));

    let witness = solver.find_witness(&problem).expect("witness exists");
    assert!(problem.is_valid_solution(&witness));
    assert_eq!(problem.evaluate(&witness), Max(Some(12)));
}

#[test]
fn test_maximum_co_k_plex_k_equals_1_is_independent_set() {
    // For k = 1 the co-k-plex constraint forces an independent set.
    // 5-cycle MIS has size 2, so unit-weight optimum is 2.
    let problem = MaximumCoKPlex::<_, One, KN>::with_k(c5(), vec![One; 5], 1);
    let solver = BruteForce::new();
    assert_eq!(solver.solve(&problem), Max(Some(2)));

    // Picking adjacent vertices violates the k=1 constraint.
    assert_eq!(problem.evaluate(&[1, 1, 0, 0, 0]), Max(None));
    // Any two non-adjacent vertices is feasible.
    assert_eq!(problem.evaluate(&[1, 0, 1, 0, 0]), Max(Some(2)));
}

#[test]
fn test_maximum_co_k_plex_serialization_roundtrip() {
    let problem = issue_instance();
    let json = serde_json::to_value(&problem).expect("serialize");
    let restored: MaximumCoKPlex<SimpleGraph, i32, KN> =
        serde_json::from_value(json).expect("deserialize");
    assert_eq!(restored.graph().num_vertices(), 5);
    assert_eq!(restored.weights(), &[5, 1, 4, 1, 3]);
    assert_eq!(restored.bound_k(), 2);
    assert_eq!(restored.evaluate(&[1, 0, 1, 0, 1]), Max(Some(12)));
}

#[test]
fn test_maximum_co_k_plex_problem_name_and_variant() {
    assert_eq!(
        <MaximumCoKPlex<SimpleGraph, One, KN> as Problem>::NAME,
        "MaximumCoKPlex"
    );
    let v = <MaximumCoKPlex<SimpleGraph, One, KN> as Problem>::variant();
    assert!(v.contains(&("graph", "SimpleGraph")));
    assert!(v.contains(&("weight", "One")));
    assert!(v.contains(&("k", "KN")));
}

#[test]
#[should_panic(expected = "co-k-plex parameter k must be at least 1")]
fn test_maximum_co_k_plex_rejects_zero_k() {
    let _ = MaximumCoKPlex::<_, One, KN>::with_k(c5(), vec![One; 5], 0);
}

#[test]
#[should_panic(expected = "weights length must match graph num_vertices")]
fn test_maximum_co_k_plex_rejects_weight_length_mismatch() {
    let _ = MaximumCoKPlex::<_, One, KN>::with_k(c5(), vec![One; 4], 2);
}

#[test]
fn test_maximum_co_k_plex_rejects_missing_bound_k_on_load() {
    // A JSON payload missing `bound_k` must fail to deserialize with a
    // clear error, instead of silently defaulting to 0 and producing
    // degenerate `Max(None)` results from `evaluate()` (KN variant has no
    // compile-time K).
    let bad_json = serde_json::json!({
        "graph": {
            "num_vertices": 5,
            "edges": [[0, 1], [1, 2], [2, 3], [3, 4], [4, 0]]
        },
        "weights": [5, 1, 4, 1, 3]
        // bound_k intentionally omitted
    });
    let err = serde_json::from_value::<MaximumCoKPlex<SimpleGraph, i32, KN>>(bad_json)
        .expect_err("missing bound_k must fail to deserialize");
    let msg = err.to_string();
    assert!(
        msg.contains("bound_k"),
        "error should mention the missing field `bound_k`, got: {msg}"
    );
}
