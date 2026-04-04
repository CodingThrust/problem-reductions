use crate::models::misc::Clustering;
use crate::solvers::BruteForce;
use crate::traits::Problem;

/// Helper: build the 6-element two-group instance from the issue.
fn two_group_instance() -> Clustering {
    let distances = vec![
        vec![0, 1, 1, 3, 3, 3],
        vec![1, 0, 1, 3, 3, 3],
        vec![1, 1, 0, 3, 3, 3],
        vec![3, 3, 3, 0, 1, 1],
        vec![3, 3, 3, 1, 0, 1],
        vec![3, 3, 3, 1, 1, 0],
    ];
    Clustering::new(distances, 2, 1)
}

#[test]
fn test_clustering_creation() {
    let problem = two_group_instance();
    assert_eq!(problem.num_elements(), 6);
    assert_eq!(problem.num_clusters(), 2);
    assert_eq!(problem.diameter_bound(), 1);
    assert_eq!(problem.distances().len(), 6);
    assert_eq!(problem.dims(), vec![2; 6]);
}

#[test]
fn test_clustering_evaluate_feasible() {
    let problem = two_group_instance();
    // Cluster 0 = {0,1,2}, Cluster 1 = {3,4,5}
    // All intra-cluster distances = 1 ≤ B=1
    let result = problem.evaluate(&[0, 0, 0, 1, 1, 1]);
    assert!(result.0);
}

#[test]
fn test_clustering_evaluate_infeasible_distance() {
    let problem = two_group_instance();
    // Put element 3 (inter-group distance 3) in cluster 0 with {0,1,2}
    // distances[0][3] = 3 > B=1 → infeasible
    let result = problem.evaluate(&[0, 0, 0, 0, 1, 1]);
    assert!(!result.0);
}

#[test]
fn test_clustering_evaluate_all_same_cluster() {
    let problem = two_group_instance();
    // All elements in one cluster → inter-group distance 3 > 1 → infeasible
    let result = problem.evaluate(&[0, 0, 0, 0, 0, 0]);
    assert!(!result.0);
}

#[test]
fn test_clustering_evaluate_wrong_length() {
    let problem = two_group_instance();
    assert!(!problem.evaluate(&[0, 0, 0]).0);
    assert!(!problem.evaluate(&[0, 0, 0, 1, 1, 1, 0]).0);
}

#[test]
fn test_clustering_evaluate_invalid_cluster_index() {
    let problem = two_group_instance();
    // Cluster index 2 is invalid (K=2, valid indices are 0,1)
    assert!(!problem.evaluate(&[0, 0, 2, 1, 1, 1]).0);
}

#[test]
fn test_clustering_trivial_k_ge_n() {
    // K ≥ n: each element in its own cluster → always feasible
    let distances = vec![vec![0, 100, 100], vec![100, 0, 100], vec![100, 100, 0]];
    let problem = Clustering::new(distances, 3, 0);
    // Each element in its own cluster: [0, 1, 2]
    assert!(problem.evaluate(&[0, 1, 2]).0);
}

#[test]
fn test_clustering_solver() {
    let problem = two_group_instance();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()).0);
}

#[test]
fn test_clustering_solver_all_witnesses() {
    // Small instance: 4 elements, K=2, B=1
    let distances = vec![
        vec![0, 1, 3, 3],
        vec![1, 0, 3, 3],
        vec![3, 3, 0, 1],
        vec![3, 3, 1, 0],
    ];
    let problem = Clustering::new(distances, 2, 1);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol).0);
    }
    // Two valid groupings: {0,1} vs {2,3} in either assignment order
    // [0,0,1,1] and [1,1,0,0]
    assert_eq!(solutions.len(), 2);
}

#[test]
fn test_clustering_serialization() {
    let problem = two_group_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: Clustering = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_elements(), 6);
    assert_eq!(deserialized.num_clusters(), 2);
    assert_eq!(deserialized.diameter_bound(), 1);
    // Check round-trip gives same evaluation
    let config = vec![0, 0, 0, 1, 1, 1];
    assert_eq!(
        problem.evaluate(&config).0,
        deserialized.evaluate(&config).0
    );
}

#[test]
fn test_clustering_no_solution() {
    // 3 elements all pairwise distance 5, K=1, B=2 → infeasible
    let distances = vec![vec![0, 5, 5], vec![5, 0, 5], vec![5, 5, 0]];
    let problem = Clustering::new(distances, 1, 2);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
#[should_panic(expected = "symmetric")]
fn test_clustering_asymmetric_panics() {
    let distances = vec![vec![0, 1], vec![2, 0]];
    Clustering::new(distances, 1, 1);
}

#[test]
#[should_panic(expected = "Diagonal")]
fn test_clustering_nonzero_diagonal_panics() {
    let distances = vec![vec![1, 1], vec![1, 0]];
    Clustering::new(distances, 1, 1);
}

#[test]
fn test_clustering_paper_example() {
    // Paper example: 6 elements, K=2, B=1
    let problem = two_group_instance();
    let config = vec![0, 0, 0, 1, 1, 1];
    let result = problem.evaluate(&config);
    assert!(result.0);

    // Verify this is satisfiable
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem);
    assert!(witness.is_some());
    assert!(problem.evaluate(&witness.unwrap()).0);
}
