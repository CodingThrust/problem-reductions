use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

/// Star graph S4: center 0 connected to 1, 2, 3
fn star_example() -> MinimumGraphBandwidth<SimpleGraph> {
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]);
    MinimumGraphBandwidth::new(graph)
}

/// Path graph P4: 0-1-2-3
fn path_example() -> MinimumGraphBandwidth<SimpleGraph> {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    MinimumGraphBandwidth::new(graph)
}

#[test]
fn test_minimumgraphbandwidth_creation() {
    let problem = star_example();
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.dims(), vec![4, 4, 4, 4]);
}

#[test]
fn test_minimumgraphbandwidth_evaluate_valid() {
    let problem = star_example();
    // Config [1,0,2,3]: f(0)=1, f(1)=0, f(2)=2, f(3)=3
    // Edges: (0,1): |1-0|=1, (0,2): |1-2|=1, (0,3): |1-3|=2
    // Bandwidth = max(1, 1, 2) = 2
    assert_eq!(problem.evaluate(&[1, 0, 2, 3]), Min(Some(2)));
    assert_eq!(problem.bandwidth(&[1, 0, 2, 3]), Some(2));
}

#[test]
fn test_minimumgraphbandwidth_evaluate_invalid() {
    let problem = star_example();

    // Not a permutation: repeated value
    assert_eq!(problem.evaluate(&[0, 0, 1, 2]), Min(None));
    assert_eq!(problem.bandwidth(&[0, 0, 1, 2]), None);

    // Out of range
    assert_eq!(problem.evaluate(&[0, 1, 2, 4]), Min(None));
    assert_eq!(problem.bandwidth(&[0, 1, 2, 4]), None);

    // Wrong length
    assert_eq!(problem.evaluate(&[0, 1, 2]), Min(None));
    assert_eq!(problem.bandwidth(&[0, 1, 2]), None);
}

#[test]
fn test_minimumgraphbandwidth_evaluate_optimal() {
    let problem = star_example();
    // For S4 (star with 4 vertices), optimal bandwidth is 2.
    // Center (vertex 0) placed at position 1: [1, 0, 2, 3]
    // Edges: (0,1): |1-0|=1, (0,2): |1-2|=1, (0,3): |1-3|=2 → max = 2
    let solver = BruteForce::new();
    let value = solver.solve(&problem);
    assert_eq!(value, Min(Some(2)));
}

#[test]
fn test_minimumgraphbandwidth_solver() {
    let problem = path_example();
    // Path graph P4: optimal bandwidth is 1 (identity permutation)
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(problem.evaluate(&sol), Min(Some(1)));

    let value = solver.solve(&problem);
    assert_eq!(value, Min(Some(1)));
}

#[test]
fn test_minimumgraphbandwidth_serialization() {
    let problem = star_example();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinimumGraphBandwidth<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.graph().num_vertices(), 4);
    assert_eq!(deserialized.graph().num_edges(), 3);

    // Verify evaluation is consistent after round-trip
    let config = vec![1, 0, 2, 3];
    assert_eq!(problem.evaluate(&config), deserialized.evaluate(&config));
}

#[test]
fn test_minimumgraphbandwidth_single_vertex() {
    let graph = SimpleGraph::new(1, vec![]);
    let problem = MinimumGraphBandwidth::new(graph);
    assert_eq!(problem.dims(), vec![1]);
    assert_eq!(problem.evaluate(&[0]), Min(Some(0)));
    assert_eq!(problem.bandwidth(&[0]), Some(0));
}

#[test]
fn test_minimumgraphbandwidth_empty_graph() {
    // No edges: any permutation has bandwidth 0
    let graph = SimpleGraph::new(3, vec![]);
    let problem = MinimumGraphBandwidth::new(graph);

    let solver = BruteForce::new();
    let value = solver.solve(&problem);
    assert_eq!(value, Min(Some(0)));

    let all_witnesses = solver.find_all_witnesses(&problem);
    assert_eq!(all_witnesses.len(), 6); // 3! = 6
    for s in &all_witnesses {
        assert_eq!(problem.evaluate(s), Min(Some(0)));
    }
}

#[test]
fn test_minimumgraphbandwidth_complete_graph_k4() {
    // K4: bandwidth is always 3 (max position difference in any permutation)
    // Actually for K4, bandwidth = n-1 = 3 for any arrangement since edge (first, last) exists.
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let problem = MinimumGraphBandwidth::new(graph);

    let solver = BruteForce::new();
    let value = solver.solve(&problem);
    assert_eq!(value, Min(Some(3)));
}

#[test]
fn test_minimumgraphbandwidth_problem_name() {
    assert_eq!(
        <MinimumGraphBandwidth<SimpleGraph> as Problem>::NAME,
        "MinimumGraphBandwidth"
    );
}

#[test]
fn test_minimumgraphbandwidth_size_getters() {
    let problem = star_example();
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
}

#[test]
fn test_minimumgraphbandwidth_graph_accessor() {
    let problem = star_example();
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 4);
    assert_eq!(graph.num_edges(), 3);
}

#[test]
fn test_minimumgraphbandwidth_permutation_matters() {
    let problem = star_example();

    // Center at position 0: [0, 1, 2, 3]
    // Edges: (0,1): |0-1|=1, (0,2): |0-2|=2, (0,3): |0-3|=3 → max = 3
    assert_eq!(problem.evaluate(&[0, 1, 2, 3]), Min(Some(3)));

    // Center at position 1: [1, 0, 2, 3]
    // Edges: (0,1): |1-0|=1, (0,2): |1-2|=1, (0,3): |1-3|=2 → max = 2
    assert_eq!(problem.evaluate(&[1, 0, 2, 3]), Min(Some(2)));
}
