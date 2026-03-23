use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::{Problem, WitnessProblem};

/// Build the example instance from issue #228:
/// 8 vertices, 12 edges, s=0, t=7, B=5
fn example_instance(cut_bound: i32) -> MinimumCutIntoBoundedSets<SimpleGraph, i32> {
    let graph = SimpleGraph::new(
        8,
        vec![
            (0, 1),
            (0, 2),
            (1, 2),
            (1, 3),
            (2, 4),
            (3, 5),
            (3, 6),
            (4, 5),
            (4, 6),
            (5, 7),
            (6, 7),
            (5, 6),
        ],
    );
    let edge_weights = vec![2, 3, 1, 4, 2, 1, 3, 2, 1, 2, 3, 1];
    MinimumCutIntoBoundedSets::new(graph, edge_weights, 0, 7, 5, cut_bound)
}

#[test]
fn test_minimumcutintoboundedsets_basic() {
    let problem = example_instance(6);
    assert_eq!(problem.num_vertices(), 8);
    assert_eq!(problem.num_edges(), 12);
    assert_eq!(problem.source(), 0);
    assert_eq!(problem.sink(), 7);
    assert_eq!(problem.size_bound(), 5);
    assert_eq!(problem.cut_bound(), &6);
    assert_eq!(problem.dims(), vec![2; 8]);
}

#[test]
fn test_minimumcutintoboundedsets_evaluation_yes() {
    let problem = example_instance(6);
    // V1={0,1,2,3}, V2={4,5,6,7}
    // Cut edges: (2,4)=2, (3,5)=1, (3,6)=3 => cut=6 <= K=6
    let config = vec![0, 0, 0, 0, 1, 1, 1, 1];
    assert!(problem.evaluate(&config));
}

#[test]
fn test_minimumcutintoboundedsets_evaluation_no() {
    let problem = example_instance(5);
    // Same partition: cut=6 > K=5
    let config = vec![0, 0, 0, 0, 1, 1, 1, 1];
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_minimumcutintoboundedsets_wrong_source() {
    let problem = example_instance(6);
    // Source (0) not in V1 (config[0]=1 instead of 0)
    let config = vec![1, 0, 0, 0, 1, 1, 1, 1];
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_minimumcutintoboundedsets_wrong_sink() {
    let problem = example_instance(6);
    // Sink (7) not in V2 (config[7]=0 instead of 1)
    let config = vec![0, 0, 0, 0, 1, 1, 1, 0];
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_minimumcutintoboundedsets_size_bound_violated() {
    // Use B=3 so that |V1|=4 or |V2|=4 violates the bound
    let graph = SimpleGraph::new(
        8,
        vec![
            (0, 1),
            (0, 2),
            (1, 2),
            (1, 3),
            (2, 4),
            (3, 5),
            (3, 6),
            (4, 5),
            (4, 6),
            (5, 7),
            (6, 7),
            (5, 6),
        ],
    );
    let edge_weights = vec![2, 3, 1, 4, 2, 1, 3, 2, 1, 2, 3, 1];
    let problem = MinimumCutIntoBoundedSets::new(graph, edge_weights, 0, 7, 3, 100);
    // V1={0,1,2,3} has 4 > B=3
    let config = vec![0, 0, 0, 0, 1, 1, 1, 1];
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_minimumcutintoboundedsets_wrong_config_length() {
    let problem = example_instance(6);
    let config = vec![0, 0, 1]; // too short
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_minimumcutintoboundedsets_serialization() {
    let problem = example_instance(6);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinimumCutIntoBoundedSets<SimpleGraph, i32> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vertices(), 8);
    assert_eq!(deserialized.num_edges(), 12);
    assert_eq!(deserialized.source(), 0);
    assert_eq!(deserialized.sink(), 7);
    assert_eq!(deserialized.size_bound(), 5);
    assert_eq!(deserialized.cut_bound(), &6);
    // Verify same evaluation
    let config = vec![0, 0, 0, 0, 1, 1, 1, 1];
    assert!(deserialized.evaluate(&config));
}

#[test]
fn test_minimumcutintoboundedsets_solver_satisfying() {
    let problem = example_instance(6);
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(
        solution.is_some(),
        "Should find a satisfying partition for K=6"
    );
    let sol = solution.unwrap();
    assert!(problem.evaluate(&sol));
}

#[test]
fn test_minimumcutintoboundedsets_solver_no_solution() {
    // K=0 with non-trivial graph: no partition with cut=0 can have s and t separated
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = MinimumCutIntoBoundedSets::new(graph, vec![1, 1, 1], 0, 3, 3, 0);
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(
        solution.is_none(),
        "Should find no satisfying partition for K=0"
    );
}

#[test]
fn test_minimumcutintoboundedsets_small_graph() {
    // Simple 3-vertex path: 0-1-2, s=0, t=2, B=2, K=1
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumCutIntoBoundedSets::new(graph, vec![1, 1], 0, 2, 2, 1);
    // V1={0,1}, V2={2}: cut edge (1,2)=1 <= K=1
    assert!(problem.evaluate(&[0, 0, 1]));
    // V1={0}, V2={1,2}: cut edge (0,1)=1 <= K=1
    assert!(problem.evaluate(&[0, 1, 1]));
}

#[test]
fn test_minimumcutintoboundedsets_edge_weights_accessor() {
    let problem = example_instance(6);
    assert_eq!(problem.edge_weights().len(), 12);
    assert_eq!(problem.edge_weights()[0], 2);
}

#[test]
fn test_minimumcutintoboundedsets_graph_accessor() {
    let problem = example_instance(6);
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 8);
    assert_eq!(graph.num_edges(), 12);
}

#[test]
fn test_minimumcutintoboundedsets_all_satisfying() {
    // Small graph: 3-vertex path, s=0, t=2, B=2, K=1
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumCutIntoBoundedSets::new(graph, vec![1, 1], 0, 2, 2, 1);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    // Two valid partitions: {0,1}|{2} and {0}|{1,2}
    assert_eq!(solutions.len(), 2);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_minimumcutintoboundedsets_variant() {
    let variant = MinimumCutIntoBoundedSets::<SimpleGraph, i32>::variant();
    assert_eq!(variant.len(), 2);
    assert!(variant.iter().any(|(k, _)| *k == "graph"));
    assert!(variant.iter().any(|(k, _)| *k == "weight"));
}

#[test]
fn test_minimumcutintoboundedsets_solver_no_solution_issue_instance() {
    // Issue #228 NO instance: K=5 on the 8-vertex graph has no valid partition
    let problem = example_instance(5);
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(
        solution.is_none(),
        "Should find no satisfying partition for K=5 on the 8-vertex instance"
    );
}

// Verify WitnessProblem marker trait is implemented
#[test]
fn test_minimumcutintoboundedsets_is_satisfaction_problem() {
    fn assert_satisfaction<T: WitnessProblem>() {}
    assert_satisfaction::<MinimumCutIntoBoundedSets<SimpleGraph, i32>>();
}
