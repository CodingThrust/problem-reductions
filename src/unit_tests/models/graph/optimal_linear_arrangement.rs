use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

/// Issue example: 6 vertices, 7 edges
fn issue_example() -> OptimalLinearArrangement<SimpleGraph> {
    let graph = SimpleGraph::new(
        6,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 3), (2, 5)],
    );
    OptimalLinearArrangement::new(graph)
}

/// Path graph: 0-1-2-3-4-5
fn path_example() -> OptimalLinearArrangement<SimpleGraph> {
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)]);
    OptimalLinearArrangement::new(graph)
}

#[test]
fn test_optimallineararrangement_basic() {
    let problem = issue_example();

    // Check dims: 6 variables, each with domain size 6
    assert_eq!(problem.dims(), vec![6, 6, 6, 6, 6, 6]);

    // Identity arrangement: f(i) = i
    // Cost: |0-1| + |1-2| + |2-3| + |3-4| + |4-5| + |0-3| + |2-5| = 1+1+1+1+1+3+3 = 11
    let config = vec![0, 1, 2, 3, 4, 5];
    assert_eq!(problem.evaluate(&config), Min(Some(11)));
    assert_eq!(problem.total_edge_length(&config), Some(11));
}

#[test]
fn test_optimallineararrangement_path() {
    let problem = path_example();

    // Identity arrangement on a path: each edge has length 1, total = 5
    let config = vec![0, 1, 2, 3, 4, 5];
    assert_eq!(problem.evaluate(&config), Min(Some(5)));
    assert_eq!(problem.total_edge_length(&config), Some(5));
}

#[test]
fn test_optimallineararrangement_invalid_config() {
    let problem = issue_example();

    // Not a permutation: repeated value
    assert_eq!(problem.evaluate(&[0, 0, 1, 2, 3, 4]), Min(None));
    assert_eq!(problem.total_edge_length(&[0, 0, 1, 2, 3, 4]), None);

    // Out of range
    assert_eq!(problem.evaluate(&[0, 1, 2, 3, 4, 6]), Min(None));
    assert_eq!(problem.total_edge_length(&[0, 1, 2, 3, 4, 6]), None);

    // Wrong length
    assert_eq!(problem.evaluate(&[0, 1, 2]), Min(None));
    assert_eq!(problem.total_edge_length(&[0, 1, 2]), None);
}

#[test]
fn test_optimallineararrangement_serialization() {
    let problem = issue_example();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: OptimalLinearArrangement<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.graph().num_vertices(), 6);
    assert_eq!(deserialized.graph().num_edges(), 7);

    // Verify evaluation is consistent after round-trip
    let config = vec![0, 1, 2, 3, 4, 5];
    assert_eq!(problem.evaluate(&config), deserialized.evaluate(&config));
}

#[test]
fn test_optimallineararrangement_solver() {
    // Small graph: triangle
    // Any permutation of 3 vertices on a triangle has cost 4
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = OptimalLinearArrangement::new(graph);

    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(problem.evaluate(&sol), Min(Some(4)));
}

#[test]
fn test_optimallineararrangement_solver_aggregate() {
    // Triangle: minimum arrangement cost is 4
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = OptimalLinearArrangement::new(graph);

    let solver = BruteForce::new();
    let value = solver.solve(&problem);
    assert_eq!(value, Min(Some(4)));
}

#[test]
fn test_optimallineararrangement_empty_graph() {
    // No edges: any permutation has cost 0
    let graph = SimpleGraph::new(3, vec![]);
    let problem = OptimalLinearArrangement::new(graph);

    let solver = BruteForce::new();
    let value = solver.solve(&problem);
    assert_eq!(value, Min(Some(0)));

    let all_witnesses = solver.find_all_witnesses(&problem);
    // All 3! = 6 permutations should be witnesses (all achieve cost 0)
    assert_eq!(all_witnesses.len(), 6);
    for s in &all_witnesses {
        assert_eq!(problem.evaluate(s), Min(Some(0)));
        assert_eq!(problem.total_edge_length(s), Some(0));
    }
}

#[test]
fn test_optimallineararrangement_single_vertex() {
    let graph = SimpleGraph::new(1, vec![]);
    let problem = OptimalLinearArrangement::new(graph);

    assert_eq!(problem.dims(), vec![1]);
    assert_eq!(problem.evaluate(&[0]), Min(Some(0)));
    assert_eq!(problem.total_edge_length(&[0]), Some(0));
}

#[test]
fn test_optimallineararrangement_size_getters() {
    let problem = issue_example();
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 7);
}

#[test]
fn test_optimallineararrangement_graph_accessor() {
    let problem = issue_example();
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 6);
    assert_eq!(graph.num_edges(), 7);
}

#[test]
fn test_optimallineararrangement_problem_name() {
    assert_eq!(
        <OptimalLinearArrangement<SimpleGraph> as Problem>::NAME,
        "OptimalLinearArrangement"
    );
}

#[test]
fn test_optimallineararrangement_two_vertices() {
    // Single edge: 0-1
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let problem = OptimalLinearArrangement::new(graph);

    // Both permutations [0,1] and [1,0] have cost 1
    assert_eq!(problem.evaluate(&[0, 1]), Min(Some(1)));
    assert_eq!(problem.evaluate(&[1, 0]), Min(Some(1)));
    assert_eq!(problem.total_edge_length(&[0, 1]), Some(1));
    assert_eq!(problem.total_edge_length(&[1, 0]), Some(1));
}

#[test]
fn test_optimallineararrangement_permutation_matters() {
    // Path 0-1-2-3
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = OptimalLinearArrangement::new(graph);

    // Identity: cost = 1+1+1 = 3
    assert_eq!(problem.evaluate(&[0, 1, 2, 3]), Min(Some(3)));
    assert_eq!(problem.total_edge_length(&[0, 1, 2, 3]), Some(3));

    // Reversed: cost = 1+1+1 = 3
    assert_eq!(problem.evaluate(&[3, 2, 1, 0]), Min(Some(3)));
    assert_eq!(problem.total_edge_length(&[3, 2, 1, 0]), Some(3));

    // Scrambled: [2, 0, 3, 1] -> f(0)=2, f(1)=0, f(2)=3, f(3)=1
    // |2-0| + |0-3| + |3-1| = 2+3+2 = 7
    let scrambled = vec![2, 0, 3, 1];
    assert_eq!(problem.evaluate(&scrambled), Min(Some(7)));
    assert_eq!(problem.total_edge_length(&scrambled), Some(7));
}

#[test]
fn test_optimallineararrangement_is_valid_solution() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = OptimalLinearArrangement::new(graph);

    // Valid permutation
    assert!(problem.is_valid_solution(&[0, 1, 2]));
    assert!(problem.is_valid_solution(&[2, 1, 0]));
    // Not a permutation
    assert!(!problem.is_valid_solution(&[0, 0, 1]));
    // Wrong length
    assert!(!problem.is_valid_solution(&[0, 1]));
    // Out of range
    assert!(!problem.is_valid_solution(&[0, 1, 3]));
}

#[test]
fn test_optimallineararrangement_complete_graph_k4() {
    // K4: all 6 edges present
    // For K4, any linear arrangement has cost 1+2+3+1+2+1 = 10
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let problem = OptimalLinearArrangement::new(graph);

    let solver = BruteForce::new();
    let value = solver.solve(&problem);
    assert_eq!(value, Min(Some(10)));

    let all_witnesses = solver.find_all_witnesses(&problem);
    // All 4! = 24 permutations should be witnesses since all have cost 10
    assert_eq!(all_witnesses.len(), 24);
    for sol in &all_witnesses {
        assert_eq!(problem.evaluate(sol), Min(Some(10)));
        assert_eq!(problem.total_edge_length(sol), Some(10));
    }
}
