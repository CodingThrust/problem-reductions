use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

/// Issue example: 6 vertices, 7 edges, bound K=11 (YES instance)
fn issue_example_yes() -> OptimalLinearArrangement<SimpleGraph> {
    let graph = SimpleGraph::new(
        6,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 3), (2, 5)],
    );
    OptimalLinearArrangement::new(graph, 11)
}

/// Issue example: same graph, bound K=9 (NO instance)
fn issue_example_no() -> OptimalLinearArrangement<SimpleGraph> {
    let graph = SimpleGraph::new(
        6,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 3), (2, 5)],
    );
    OptimalLinearArrangement::new(graph, 9)
}

/// Path graph: 0-1-2-3-4-5, bound K=5
fn path_example() -> OptimalLinearArrangement<SimpleGraph> {
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)]);
    OptimalLinearArrangement::new(graph, 5)
}

#[test]
fn test_optimallineararrangement_basic() {
    let problem = issue_example_yes();

    // Check dims: 6 variables, each with domain size 6
    assert_eq!(problem.dims(), vec![6, 6, 6, 6, 6, 6]);

    // Identity arrangement: f(i) = i
    // Cost: |0-1| + |1-2| + |2-3| + |3-4| + |4-5| + |0-3| + |2-5| = 1+1+1+1+1+3+3 = 11
    let config = vec![0, 1, 2, 3, 4, 5];
    assert!(problem.evaluate(&config));
    assert_eq!(problem.total_edge_length(&config), Some(11));
}

#[test]
fn test_optimallineararrangement_no_instance() {
    let problem = issue_example_no();

    // Identity arrangement has cost 11 > 9
    let config = vec![0, 1, 2, 3, 4, 5];
    assert!(!problem.evaluate(&config));
    assert_eq!(problem.total_edge_length(&config), Some(11));

    // Brute-force confirms no arrangement achieves cost <= 9
    let solver = BruteForce::new();
    assert!(solver.find_satisfying(&problem).is_none());
}

#[test]
fn test_optimallineararrangement_path() {
    let problem = path_example();

    // Identity arrangement on a path: each edge has length 1, total = 5
    let config = vec![0, 1, 2, 3, 4, 5];
    assert!(problem.evaluate(&config));
    assert_eq!(problem.total_edge_length(&config), Some(5));
}

#[test]
fn test_optimallineararrangement_invalid_config() {
    let problem = issue_example_yes();

    // Not a permutation: repeated value
    assert!(!problem.evaluate(&[0, 0, 1, 2, 3, 4]));
    assert_eq!(problem.total_edge_length(&[0, 0, 1, 2, 3, 4]), None);

    // Out of range
    assert!(!problem.evaluate(&[0, 1, 2, 3, 4, 6]));
    assert_eq!(problem.total_edge_length(&[0, 1, 2, 3, 4, 6]), None);

    // Wrong length
    assert!(!problem.evaluate(&[0, 1, 2]));
    assert_eq!(problem.total_edge_length(&[0, 1, 2]), None);
}

#[test]
fn test_optimallineararrangement_serialization() {
    let problem = issue_example_yes();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: OptimalLinearArrangement<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.graph().num_vertices(), 6);
    assert_eq!(deserialized.graph().num_edges(), 7);
    assert_eq!(deserialized.bound(), 11);

    // Verify evaluation is consistent after round-trip
    let config = vec![0, 1, 2, 3, 4, 5];
    assert_eq!(problem.evaluate(&config), deserialized.evaluate(&config));
}

#[test]
fn test_optimallineararrangement_solver() {
    // Small graph: triangle, bound = 4
    // Any permutation of 3 vertices on a triangle has cost 4
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = OptimalLinearArrangement::new(graph, 4);

    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert!(problem.evaluate(&sol));

    // All satisfying solutions should be valid
    let all_sat = solver.find_all_satisfying(&problem);
    assert!(!all_sat.is_empty());
    for s in &all_sat {
        assert!(problem.evaluate(s));
    }
}

#[test]
fn test_optimallineararrangement_solver_no_solution() {
    // Triangle with very tight bound
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    // Minimum cost for triangle is 4, so bound 3 should have no solution
    let problem = OptimalLinearArrangement::new(graph, 3);

    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_none());

    let all_sat = solver.find_all_satisfying(&problem);
    assert!(all_sat.is_empty());
}

#[test]
fn test_optimallineararrangement_empty_graph() {
    // No edges: any permutation has cost 0
    let graph = SimpleGraph::new(3, vec![]);
    let problem = OptimalLinearArrangement::new(graph, 0);

    let solver = BruteForce::new();
    let all_sat = solver.find_all_satisfying(&problem);
    // All 3! = 6 permutations should be valid
    assert_eq!(all_sat.len(), 6);
    for s in &all_sat {
        assert!(problem.evaluate(s));
        assert_eq!(problem.total_edge_length(s), Some(0));
    }
}

#[test]
fn test_optimallineararrangement_single_vertex() {
    let graph = SimpleGraph::new(1, vec![]);
    let problem = OptimalLinearArrangement::new(graph, 0);

    assert_eq!(problem.dims(), vec![1]);
    assert!(problem.evaluate(&[0]));
    assert_eq!(problem.total_edge_length(&[0]), Some(0));
}

#[test]
fn test_optimallineararrangement_size_getters() {
    let problem = issue_example_yes();
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 7);
    assert_eq!(problem.bound(), 11);
}

#[test]
fn test_optimallineararrangement_graph_accessor() {
    let problem = issue_example_yes();
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
    // Single edge: 0-1, bound = 1
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let problem = OptimalLinearArrangement::new(graph, 1);

    // Both permutations [0,1] and [1,0] have cost 1
    assert!(problem.evaluate(&[0, 1]));
    assert!(problem.evaluate(&[1, 0]));
    assert_eq!(problem.total_edge_length(&[0, 1]), Some(1));
    assert_eq!(problem.total_edge_length(&[1, 0]), Some(1));
}

#[test]
fn test_optimallineararrangement_permutation_matters() {
    // Path 0-1-2-3, bound = 4
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = OptimalLinearArrangement::new(graph, 4);

    // Identity: cost = 1+1+1 = 3 <= 4, valid
    assert!(problem.evaluate(&[0, 1, 2, 3]));
    assert_eq!(problem.total_edge_length(&[0, 1, 2, 3]), Some(3));

    // Reversed: cost = 1+1+1 = 3 <= 4, valid
    assert!(problem.evaluate(&[3, 2, 1, 0]));
    assert_eq!(problem.total_edge_length(&[3, 2, 1, 0]), Some(3));

    // Scrambled: [2, 0, 3, 1] -> f(0)=2, f(1)=0, f(2)=3, f(3)=1
    // |2-0| + |0-3| + |3-1| = 2+3+2 = 7 > 4
    let scrambled = vec![2, 0, 3, 1];
    assert!(!problem.evaluate(&scrambled));
    assert_eq!(problem.total_edge_length(&scrambled), Some(7));
}

#[test]
fn test_optimallineararrangement_is_valid_solution() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = OptimalLinearArrangement::new(graph, 2);

    // Valid permutation, cost = 2 <= 2
    assert!(problem.is_valid_solution(&[0, 1, 2]));
    // Valid permutation, cost = 2 <= 2
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
    // K4: all 6 edges present, bound = 10
    // For K4, any linear arrangement has cost 1+2+3+1+2+1 = 10
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let problem = OptimalLinearArrangement::new(graph, 10);

    let solver = BruteForce::new();
    let all_sat = solver.find_all_satisfying(&problem);
    // All 4! = 24 permutations should be valid since all have cost 10
    assert_eq!(all_sat.len(), 24);
    for sol in &all_sat {
        assert!(problem.evaluate(sol));
        assert_eq!(problem.total_edge_length(sol), Some(10));
    }
}
