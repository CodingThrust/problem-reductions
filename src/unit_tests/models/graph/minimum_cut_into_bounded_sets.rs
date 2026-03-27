use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::{Aggregate, Min};
use crate::Solver;

/// Build the example instance from issue #228:
/// 8 vertices, 12 edges, s=0, t=7, B=5
fn example_instance() -> MinimumCutIntoBoundedSets<SimpleGraph, i32> {
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
    MinimumCutIntoBoundedSets::new(graph, edge_weights, 0, 7, 5)
}

#[test]
fn test_minimumcutintoboundedsets_basic() {
    let problem = example_instance();
    assert_eq!(problem.num_vertices(), 8);
    assert_eq!(problem.num_edges(), 12);
    assert_eq!(problem.source(), 0);
    assert_eq!(problem.sink(), 7);
    assert_eq!(problem.size_bound(), 5);
    assert_eq!(problem.dims(), vec![2; 8]);
}

#[test]
fn test_minimumcutintoboundedsets_evaluation_valid_partition() {
    let problem = example_instance();
    // V1={0,1,2,3}, V2={4,5,6,7}
    // Cut edges: (2,4)=2, (3,5)=1, (3,6)=3 => cut=6
    let config = vec![0, 0, 0, 0, 1, 1, 1, 1];
    assert_eq!(problem.evaluate(&config), Min(Some(6)));
}

#[test]
fn test_minimumcutintoboundedsets_evaluation_different_partition() {
    let problem = example_instance();
    // V1={0,1,2}, V2={3,4,5,6,7}
    // Cut edges: (1,3)=4, (2,4)=2 => cut=6
    let config = vec![0, 0, 0, 1, 1, 1, 1, 1];
    assert_eq!(problem.evaluate(&config), Min(Some(6)));
}

#[test]
fn test_minimumcutintoboundedsets_wrong_source() {
    let problem = example_instance();
    // Source (0) not in V1 (config[0]=1 instead of 0)
    let config = vec![1, 0, 0, 0, 1, 1, 1, 1];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimumcutintoboundedsets_wrong_sink() {
    let problem = example_instance();
    // Sink (7) not in V2 (config[7]=0 instead of 1)
    let config = vec![0, 0, 0, 0, 1, 1, 1, 0];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimumcutintoboundedsets_size_bound_violated() {
    // Use B=3 so that |V1|=4 violates the bound
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
    let problem = MinimumCutIntoBoundedSets::new(graph, edge_weights, 0, 7, 3);
    // V1={0,1,2,3} has 4 > B=3
    let config = vec![0, 0, 0, 0, 1, 1, 1, 1];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimumcutintoboundedsets_wrong_config_length() {
    let problem = example_instance();
    let config = vec![0, 0, 1]; // too short
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimumcutintoboundedsets_serialization() {
    let problem = example_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinimumCutIntoBoundedSets<SimpleGraph, i32> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vertices(), 8);
    assert_eq!(deserialized.num_edges(), 12);
    assert_eq!(deserialized.source(), 0);
    assert_eq!(deserialized.sink(), 7);
    assert_eq!(deserialized.size_bound(), 5);
    // Verify same evaluation
    let config = vec![0, 0, 0, 0, 1, 1, 1, 1];
    assert_eq!(deserialized.evaluate(&config), Min(Some(6)));
}

#[test]
fn test_minimumcutintoboundedsets_solver() {
    let problem = example_instance();
    let solver = BruteForce::new();
    let value = solver.solve(&problem);
    assert_eq!(value, Min(Some(6)));
    let witness = solver.find_witness(&problem);
    assert!(witness.is_some());
    let sol = witness.unwrap();
    assert!(problem.evaluate(&sol).0.is_some());
}

#[test]
fn test_minimumcutintoboundedsets_small_graph() {
    // Simple 3-vertex path: 0-1-2, s=0, t=2, B=2
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumCutIntoBoundedSets::new(graph, vec![1, 1], 0, 2, 2);
    // V1={0,1}, V2={2}: cut edge (1,2)=1
    assert_eq!(problem.evaluate(&[0, 0, 1]), Min(Some(1)));
    // V1={0}, V2={1,2}: cut edge (0,1)=1
    assert_eq!(problem.evaluate(&[0, 1, 1]), Min(Some(1)));
}

#[test]
fn test_minimumcutintoboundedsets_edge_weights_accessor() {
    let problem = example_instance();
    assert_eq!(problem.edge_weights().len(), 12);
    assert_eq!(problem.edge_weights()[0], 2);
}

#[test]
fn test_minimumcutintoboundedsets_graph_accessor() {
    let problem = example_instance();
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 8);
    assert_eq!(graph.num_edges(), 12);
}

#[test]
fn test_minimumcutintoboundedsets_variant() {
    let variant = MinimumCutIntoBoundedSets::<SimpleGraph, i32>::variant();
    assert_eq!(variant.len(), 2);
    assert!(variant.iter().any(|(k, _)| *k == "graph"));
    assert!(variant.iter().any(|(k, _)| *k == "weight"));
}

#[test]
fn test_minimumcutintoboundedsets_supports_witnesses() {
    assert!(<MinimumCutIntoBoundedSets<SimpleGraph, i32> as Problem>::Value::supports_witnesses());
}
