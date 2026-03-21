use crate::models::graph::NetworkReliability;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;

fn issue_235_example() -> NetworkReliability {
    NetworkReliability::new(
        SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (0, 2),
                (1, 3),
                (2, 3),
                (1, 4),
                (3, 4),
                (3, 5),
                (4, 5),
            ],
        ),
        vec![0, 5],
        vec![0.1; 8],
        0.95,
    )
}

#[test]
fn test_network_reliability_creation_and_getters() {
    let problem = issue_235_example();

    assert_eq!(problem.graph().num_vertices(), 6);
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 8);
    assert_eq!(problem.num_terminals(), 2);
    assert_eq!(problem.terminals(), &[0, 5]);
    assert_eq!(problem.failure_probs(), &[0.1; 8]);
    assert_eq!(problem.threshold(), 0.95);
    assert_eq!(problem.dims(), vec![2; 8]);
}

#[test]
fn test_network_reliability_evaluate_terminal_connectivity() {
    let problem = issue_235_example();

    let connected_config = vec![1, 0, 1, 0, 0, 0, 1, 0];
    let disconnected_config = vec![1, 0, 0, 0, 0, 0, 1, 0];

    assert!(problem.evaluate(&connected_config));
    assert!(problem.is_valid_solution(&connected_config));
    assert!(!problem.evaluate(&disconnected_config));
    assert!(!problem.is_valid_solution(&disconnected_config));
}

#[test]
fn test_network_reliability_exact_reliability_matches_issue_example() {
    let problem = issue_235_example();

    let reliability = problem.reliability();
    assert!((reliability - 0.968425).abs() < 1e-6);
    assert!(problem.meets_threshold());
}

#[cfg(feature = "example-db")]
#[test]
fn test_network_reliability_paper_example() {
    let problem = issue_235_example();
    let witness_config = vec![1, 0, 1, 0, 0, 0, 1, 0];

    assert!(problem.evaluate(&witness_config));
    assert!((problem.reliability() - 0.96842547).abs() < 1e-9);
    assert!(problem.meets_threshold());

    let specs = super::canonical_model_example_specs();
    assert_eq!(specs.len(), 1);
    assert_eq!(specs[0].optimal_config, witness_config);
    assert_eq!(specs[0].optimal_value, serde_json::json!(true));
}

#[test]
#[should_panic(expected = "failure_probs length must match num_edges")]
fn test_network_reliability_rejects_bad_probability_length() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let _ = NetworkReliability::new(graph, vec![0, 2], vec![0.1], 0.5);
}

#[test]
#[should_panic(expected = "failure probability")]
fn test_network_reliability_rejects_probability_out_of_range() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let _ = NetworkReliability::new(graph, vec![0, 2], vec![0.1, 1.2], 0.5);
}

#[test]
#[should_panic(expected = "threshold must be in [0, 1]")]
fn test_network_reliability_rejects_threshold_out_of_range() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let _ = NetworkReliability::new(graph, vec![0, 2], vec![0.1, 0.2], 1.1);
}

#[test]
#[should_panic(expected = "terminals must be distinct")]
fn test_network_reliability_rejects_duplicate_terminals() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let _ = NetworkReliability::new(graph, vec![0, 0], vec![0.1, 0.2], 0.5);
}

#[test]
#[should_panic(expected = "terminal 3 out of range")]
fn test_network_reliability_rejects_terminal_out_of_bounds() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let _ = NetworkReliability::new(graph, vec![0, 3], vec![0.1, 0.2], 0.5);
}

#[test]
#[should_panic(expected = "at least 2 terminals required")]
fn test_network_reliability_rejects_too_few_terminals() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let _ = NetworkReliability::new(graph, vec![0], vec![0.1, 0.2], 0.5);
}
