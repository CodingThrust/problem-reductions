use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

/// Instance 1 from issue: hexagonal graph with 3 required edges, B=6
fn hexagon_rpp() -> RuralPostman<SimpleGraph, i32> {
    // 6 vertices, 8 edges
    // Edges: {0,1}:1, {1,2}:1, {2,3}:1, {3,4}:1, {4,5}:1, {5,0}:1, {0,3}:2, {1,4}:2
    let graph = SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 0),
            (0, 3),
            (1, 4),
        ],
    );
    let edge_lengths = vec![1, 1, 1, 1, 1, 1, 2, 2];
    // Required edges: {0,1}=idx 0, {2,3}=idx 2, {4,5}=idx 4
    let required_edges = vec![0, 2, 4];
    let bound = 6;
    RuralPostman::new(graph, edge_lengths, required_edges, bound)
}

/// Instance 3 from issue: C4 cycle, all edges required (Chinese Postman), B=4
fn chinese_postman_rpp() -> RuralPostman<SimpleGraph, i32> {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]);
    let edge_lengths = vec![1, 1, 1, 1];
    let required_edges = vec![0, 1, 2, 3];
    let bound = 4;
    RuralPostman::new(graph, edge_lengths, required_edges, bound)
}

#[test]
fn test_rural_postman_creation() {
    let problem = hexagon_rpp();
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 8);
    assert_eq!(problem.num_required_edges(), 3);
    assert_eq!(problem.dims().len(), 8);
    assert!(problem.dims().iter().all(|&d| d == 3));
}

#[test]
fn test_rural_postman_accessors() {
    let problem = hexagon_rpp();
    assert_eq!(problem.graph().num_vertices(), 6);
    assert_eq!(problem.edge_lengths().len(), 8);
    assert_eq!(problem.required_edges(), &[0, 2, 4]);
    assert_eq!(*problem.bound(), 6);
    assert!(problem.is_weighted());
}

#[test]
fn test_rural_postman_valid_circuit() {
    let problem = hexagon_rpp();
    // Circuit: 0->1->2->3->4->5->0 uses edges 0,1,2,3,4,5 (the hexagon)
    // Total length = 6 * 1 = 6 = B, covers all required edges
    let config = vec![1, 1, 1, 1, 1, 1, 0, 0];
    assert!(problem.evaluate(&config));
}

#[test]
fn test_rural_postman_missing_required_edge() {
    let problem = hexagon_rpp();
    // Select edges but miss required edge 4 ({4,5})
    let config = vec![1, 1, 1, 1, 0, 1, 0, 0];
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_rural_postman_odd_degree() {
    let problem = hexagon_rpp();
    // Select edges 0,2,4 only (the 3 required edges) — disconnected, odd degree
    let config = vec![1, 0, 1, 0, 1, 0, 0, 0];
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_rural_postman_exceeds_bound() {
    // Same graph but with tight bound
    let graph = SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 0),
            (0, 3),
            (1, 4),
        ],
    );
    let edge_lengths = vec![1, 1, 1, 1, 1, 1, 2, 2];
    let required_edges = vec![0, 2, 4];
    let bound = 5; // Too tight — the hexagon cycle costs 6
    let problem = RuralPostman::new(graph, edge_lengths, required_edges, bound);
    // Hexagon cycle costs 6 > 5
    let config = vec![1, 1, 1, 1, 1, 1, 0, 0];
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_rural_postman_chinese_postman_case() {
    let problem = chinese_postman_rpp();
    // Select all edges in the C4 cycle: valid Eulerian circuit, length 4 = B
    let config = vec![1, 1, 1, 1];
    assert!(problem.evaluate(&config));
}

#[test]
fn test_rural_postman_no_edges_no_required() {
    // No required edges, bound 0 — selecting no edges is valid (empty circuit)
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let edge_lengths = vec![1, 1, 1];
    let required_edges = vec![];
    let bound = 0;
    let problem = RuralPostman::new(graph, edge_lengths, required_edges, bound);
    let config = vec![0, 0, 0];
    assert!(problem.evaluate(&config));
}

#[test]
fn test_rural_postman_disconnected_selection() {
    // Select two disconnected triangles — even degree but not connected
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)]);
    let edge_lengths = vec![1, 1, 1, 1, 1, 1];
    let required_edges = vec![0, 3]; // edges in different components
    let bound = 100;
    let problem = RuralPostman::new(graph, edge_lengths, required_edges, bound);
    // Select both triangles: even degree but disconnected
    let config = vec![1, 1, 1, 1, 1, 1];
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_rural_postman_brute_force_finds_solution() {
    let problem = chinese_postman_rpp();
    let solver = BruteForce::new();
    let result = solver.find_witness(&problem);
    assert!(result.is_some());
    let sol = result.unwrap();
    assert!(problem.evaluate(&sol));
}

#[test]
fn test_rural_postman_brute_force_hexagon() {
    let problem = hexagon_rpp();
    let solver = BruteForce::new();
    let result = solver.find_witness(&problem);
    assert!(result.is_some());
    let sol = result.unwrap();
    assert!(problem.evaluate(&sol));
}

#[test]
fn test_rural_postman_brute_force_no_solution() {
    // Instance 2 from issue: no feasible circuit with B=4
    let graph = SimpleGraph::new(
        6,
        vec![(0, 1), (1, 2), (2, 3), (3, 0), (3, 4), (4, 5), (5, 3)],
    );
    let edge_lengths = vec![1, 1, 1, 1, 3, 1, 3];
    let required_edges = vec![0, 5]; // {0,1} and {4,5}
    let bound = 4;
    let problem = RuralPostman::new(graph, edge_lengths, required_edges, bound);
    let solver = BruteForce::new();
    let result = solver.find_witness(&problem);
    assert!(result.is_none());
}

#[test]
fn test_rural_postman_find_all_witnesses() {
    // Issue #248 instance 1: hexagonal graph, 6 vertices, 8 edges
    // Required edges E'={{0,1},{2,3},{4,5}}, B=6
    // Search space = 3^8 = 6561
    let problem = hexagon_rpp();
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
    // The issue witness (hexagon cycle, all multiplicity 1) must be among solutions
    assert!(solutions.contains(&vec![1, 1, 1, 1, 1, 1, 0, 0]));
    // Only the hexagon cycle (cost 6 = B) satisfies; diagonals cost 2 each
    assert_eq!(solutions.len(), 1);
}

#[test]
fn test_rural_postman_find_all_witnesses_empty() {
    // Issue #248 instance 2: required edges {0,1} and {4,5} are far apart
    // Minimum circuit cost ≥ 8 > B=4
    let graph = SimpleGraph::new(
        6,
        vec![(0, 1), (1, 2), (2, 3), (3, 0), (3, 4), (4, 5), (5, 3)],
    );
    let edge_lengths = vec![1, 1, 1, 1, 3, 1, 3];
    let required_edges = vec![0, 5];
    let bound = 4;
    let problem = RuralPostman::new(graph, edge_lengths, required_edges, bound);
    let solver = BruteForce::new();
    assert!(solver.find_all_witnesses(&problem).is_empty());
}

#[test]
fn test_rural_postman_serialization() {
    let problem = chinese_postman_rpp();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: RuralPostman<SimpleGraph, i32> = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_vertices(), problem.num_vertices());
    assert_eq!(restored.num_edges(), problem.num_edges());
    assert_eq!(restored.num_required_edges(), problem.num_required_edges());
    assert_eq!(restored.required_edges(), problem.required_edges());
}

#[test]
fn test_rural_postman_problem_name() {
    assert_eq!(
        <RuralPostman<SimpleGraph, i32> as Problem>::NAME,
        "RuralPostman"
    );
}

#[test]
fn test_rural_postman_set_weights() {
    let mut problem = chinese_postman_rpp();
    problem.set_weights(vec![2, 2, 2, 2]);
    assert_eq!(problem.weights(), vec![2, 2, 2, 2]);
}

#[test]
fn test_rural_postman_size_getters() {
    let problem = hexagon_rpp();
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 8);
    assert_eq!(problem.num_required_edges(), 3);
}
