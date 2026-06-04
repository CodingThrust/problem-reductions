use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

// Canonical instance from the issue:
// V = {0,1,2,3}, E = {(0,1),(0,2),(1,2),(2,3)} — triangle on {0,1,2} with leaf 3.
fn canonical_problem() -> HighlyConnectedDeletion<SimpleGraph> {
    HighlyConnectedDeletion::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (2, 3)]))
}

// Discriminatory instance: a "double triangle" — two K3's joined by a bridge edge (0,3).
// V = {0,1,2,3,4,5}, E = {(0,1),(0,2),(1,2),(0,3),(3,4),(3,5),(4,5)} — 7 edges.
// Optimum: delete the bridge edge (0,3) at edge index 3 → two K3 components → value 1.
fn double_triangle_problem() -> HighlyConnectedDeletion<SimpleGraph> {
    HighlyConnectedDeletion::new(SimpleGraph::new(
        6,
        vec![(0, 1), (0, 2), (1, 2), (0, 3), (3, 4), (3, 5), (4, 5)],
    ))
}

#[test]
fn test_highly_connected_deletion_creation() {
    let problem = canonical_problem();
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 4);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 4);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2]);
    assert_eq!(problem.num_variables(), 4);
}

#[test]
fn test_highly_connected_deletion_problem_name() {
    assert_eq!(
        <HighlyConnectedDeletion<SimpleGraph> as Problem>::NAME,
        "HighlyConnectedDeletion"
    );
}

#[test]
fn test_highly_connected_deletion_evaluate_optimum() {
    // Delete only the leaf edge (2,3) at index 3 → K3 on {0,1,2} + isolated {3}.
    let problem = canonical_problem();
    let config = vec![0, 0, 0, 1];
    assert_eq!(problem.evaluate(&config), Min(Some(1)));
    assert!(problem.is_valid_solution(&config));
}

#[test]
fn test_highly_connected_deletion_evaluate_zero_deletions_infeasible() {
    // No deletions: the whole graph on 4 vertices has min cut 1 (vertex 3 has degree 1),
    // and 2*1 = 2 <= 4, so the unique component is not highly connected → infeasible.
    let problem = canonical_problem();
    let config = vec![0, 0, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(None));
    assert!(!problem.is_valid_solution(&config));
}

#[test]
fn test_highly_connected_deletion_evaluate_delete_all_feasible() {
    // Deleting every edge yields 4 isolated vertices — all singletons are allowed.
    let problem = canonical_problem();
    let config = vec![1, 1, 1, 1];
    assert_eq!(problem.evaluate(&config), Min(Some(4)));
    assert!(problem.is_valid_solution(&config));
}

#[test]
fn test_highly_connected_deletion_evaluate_two_vertex_component_infeasible() {
    // Delete (0,1),(0,2),(1,2); keep only (2,3).
    // Components: {0}, {1}, {2,3}. The 2-vertex component {2,3} is never a valid cluster.
    let problem = canonical_problem();
    let config = vec![1, 1, 1, 0];
    assert_eq!(problem.evaluate(&config), Min(None));
    assert!(!problem.is_valid_solution(&config));
}

#[test]
fn test_highly_connected_deletion_evaluate_path_component_infeasible() {
    // Delete (0,1),(1,2); keep (0,2),(2,3).
    // Components: {1}, {0,2,3} with edges (0,2),(2,3) — a path P_3.
    // λ(P_3) = 1, 2*1 = 2 <= 3 → not highly connected → infeasible.
    let problem = canonical_problem();
    let config = vec![1, 0, 1, 0];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_highly_connected_deletion_evaluate_wrong_config_length() {
    // A config whose length disagrees with the number of edges is rejected by the
    // feasibility check (it can never describe a valid deletion).
    let problem = canonical_problem();
    let too_short = vec![0, 0, 0];
    assert_eq!(problem.evaluate(&too_short), Min(None));
    assert!(!problem.is_valid_solution(&too_short));
}

#[test]
fn test_highly_connected_deletion_brute_force_canonical() {
    // Brute force over 2^4 = 16 configs; optimum is delete only edge (2,3) → value 1.
    let problem = canonical_problem();
    assert_eq!(BruteForce::new().solve(&problem), Min(Some(1)));
    let witness = BruteForce::new().find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&witness), Min(Some(1)));
}

#[test]
fn test_highly_connected_deletion_brute_force_double_triangle() {
    // The discriminatory instance: deleting only the bridge edge (0,3) at index 3
    // splits the graph into two K3 components, giving the unique optimum value 1.
    // Keeping any extra edge of either triangle either leaves the whole graph
    // connected (which is infeasible) or creates a non-highly-connected component.
    let problem = double_triangle_problem();
    assert_eq!(BruteForce::new().solve(&problem), Min(Some(1)));

    // Verify the named optimal config evaluates to 1.
    let bridge_only = vec![0, 0, 0, 1, 0, 0, 0];
    assert_eq!(problem.evaluate(&bridge_only), Min(Some(1)));

    // The all-zero config is infeasible because the bridge gives the union min cut 1.
    let no_deletions = vec![0; 7];
    assert_eq!(problem.evaluate(&no_deletions), Min(None));

    // Deleting one extra triangle edge in addition to the bridge breaks one K3 into
    // a 3-vertex path, which is no longer highly connected → infeasible.
    let bridge_plus_one = vec![1, 0, 0, 1, 0, 0, 0];
    assert_eq!(problem.evaluate(&bridge_plus_one), Min(None));
}

#[test]
fn test_highly_connected_deletion_serialization() {
    let problem = canonical_problem();
    let json = serde_json::to_string(&problem).unwrap();
    let restored: HighlyConnectedDeletion<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.graph().num_vertices(), 4);
    assert_eq!(restored.graph().num_edges(), 4);
    // Evaluating on the canonical optimum still yields 1 after the round trip.
    assert_eq!(restored.evaluate(&[0, 0, 0, 1]), Min(Some(1)));
}

#[test]
fn test_highly_connected_deletion_variant() {
    let variant = <HighlyConnectedDeletion<SimpleGraph> as Problem>::variant();
    assert_eq!(variant, vec![("graph", "SimpleGraph")]);
}

// ── edge_connectivity helper tests ───────────────────────────────────────────

#[test]
fn test_edge_connectivity_single_vertex() {
    // A trivial component of size 1 has edge connectivity 0 by convention.
    let adj: Vec<Vec<usize>> = vec![vec![]];
    assert_eq!(edge_connectivity_for_tests(&[0], &adj), 0);
}

#[test]
fn test_edge_connectivity_single_edge() {
    // K2: the only cut separates the two vertices with a single edge.
    let adj: Vec<Vec<usize>> = vec![vec![1], vec![0]];
    assert_eq!(edge_connectivity_for_tests(&[0, 1], &adj), 1);
}

#[test]
fn test_edge_connectivity_path_p3() {
    // P3: 0-1-2. Removing the single edge incident to an endpoint disconnects it,
    // so λ(P3) = 1.
    let adj: Vec<Vec<usize>> = vec![vec![1], vec![0, 2], vec![1]];
    assert_eq!(edge_connectivity_for_tests(&[0, 1, 2], &adj), 1);
}

#[test]
fn test_edge_connectivity_k3() {
    // K3 (triangle): every cut must remove at least 2 edges, so λ(K3) = 2.
    let adj: Vec<Vec<usize>> = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
    assert_eq!(edge_connectivity_for_tests(&[0, 1, 2], &adj), 2);
}

#[test]
fn test_edge_connectivity_k4() {
    // K4 is 3-edge-connected: every vertex has degree 3 and that is also the edge
    // connectivity.
    let adj: Vec<Vec<usize>> = vec![vec![1, 2, 3], vec![0, 2, 3], vec![0, 1, 3], vec![0, 1, 2]];
    assert_eq!(edge_connectivity_for_tests(&[0, 1, 2, 3], &adj), 3);
}
