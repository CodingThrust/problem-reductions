use super::*;
use crate::rules::ilp_helpers::permutation_to_lehmer;
use crate::solvers::BruteForce;
use crate::topology::DirectedGraph;
use crate::traits::Problem;

fn encode(perm: &[usize]) -> Vec<usize> {
    permutation_to_lehmer(perm)
}

#[test]
fn test_directed_hamiltonian_path_creation() {
    // Simple directed path: 0->1->2->3
    let graph = DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = DirectedHamiltonianPath::new(graph);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_arcs(), 3);
    // Lehmer dims: [4, 3, 2, 1]
    assert_eq!(problem.dims(), vec![4, 3, 2, 1]);
}

#[test]
fn test_directed_hamiltonian_path_evaluate_valid() {
    // Directed path: 0->1->2->3
    let graph = DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = DirectedHamiltonianPath::new(graph);

    // Path [0, 1, 2, 3]: Lehmer code [0, 0, 0, 0]
    assert_eq!(
        problem.evaluate(&encode(&[0, 1, 2, 3])),
        crate::types::Or(true)
    );
    // Path [3, 2, 1, 0]: no arcs in reverse, invalid
    assert_eq!(
        problem.evaluate(&encode(&[3, 2, 1, 0])),
        crate::types::Or(false)
    );
}

#[test]
fn test_directed_hamiltonian_path_evaluate_invalid_no_arc() {
    // Only arc 0->1 and 2->3, not 1->2
    let graph = DirectedGraph::new(4, vec![(0, 1), (2, 3)]);
    let problem = DirectedHamiltonianPath::new(graph);
    // No Hamiltonian path should be valid
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_directed_hamiltonian_path_brute_force() {
    // Simple directed path graph
    let graph = DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = DirectedHamiltonianPath::new(graph);
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should have a Hamiltonian path");
    assert_eq!(problem.evaluate(&solution), crate::types::Or(true));
}

#[test]
fn test_directed_hamiltonian_path_issue_example() {
    // 6-vertex example from issue #813
    // Arcs: [(0,1),(0,3),(1,3),(1,4),(2,0),(2,4),(3,2),(3,5),(4,5),(5,1)]
    // Hamiltonian path: [0, 1, 3, 2, 4, 5]
    let graph = DirectedGraph::new(
        6,
        vec![
            (0, 1),
            (0, 3),
            (1, 3),
            (1, 4),
            (2, 0),
            (2, 4),
            (3, 2),
            (3, 5),
            (4, 5),
            (5, 1),
        ],
    );
    let problem = DirectedHamiltonianPath::new(graph);
    let path = vec![0usize, 1, 3, 2, 4, 5];
    assert_eq!(
        problem.evaluate(&encode(&path)),
        crate::types::Or(true),
        "Path [0,1,3,2,4,5] should be a valid Hamiltonian path"
    );
}

#[test]
fn test_directed_hamiltonian_path_no_solution() {
    // Directed graph with no Hamiltonian path: 0->1, 0->2, no outgoing from 1 or 2
    let graph = DirectedGraph::new(3, vec![(0, 1), (0, 2)]);
    let problem = DirectedHamiltonianPath::new(graph);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_directed_hamiltonian_path_single_vertex() {
    let graph = DirectedGraph::new(1, vec![]);
    let problem = DirectedHamiltonianPath::new(graph);
    // Single vertex: trivially Hamiltonian
    assert_eq!(problem.evaluate(&[0]), crate::types::Or(true));
    let solver = BruteForce::new();
    let sol = solver.find_witness(&problem);
    assert!(sol.is_some());
}

#[test]
fn test_directed_hamiltonian_path_serialization() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = DirectedHamiltonianPath::new(graph);
    let json = serde_json::to_value(&problem).unwrap();
    let deserialized: DirectedHamiltonianPath = serde_json::from_value(json).unwrap();
    assert_eq!(deserialized.num_vertices(), 3);
    assert_eq!(deserialized.num_arcs(), 2);
}

#[test]
fn test_is_valid_solution() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = DirectedHamiltonianPath::new(graph);
    // Valid: path [0, 1, 2]
    assert!(problem.is_valid_solution(&encode(&[0, 1, 2])));
    // Invalid: path [0, 2, 1] (no arc 0->2)
    assert!(!problem.is_valid_solution(&encode(&[0, 2, 1])));
}

#[test]
fn test_size_getters() {
    let graph = DirectedGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let problem = DirectedHamiltonianPath::new(graph);
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_arcs(), 4);
    // Lehmer dims: [5, 4, 3, 2, 1]
    assert_eq!(problem.dims(), vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_decode_lehmer_identity() {
    // Lehmer code [0,0,...,0] should decode to identity permutation [0,1,...,n-1]
    let code = vec![0usize; 5];
    let perm = decode_lehmer(&code);
    assert_eq!(perm, vec![0, 1, 2, 3, 4]);
}
