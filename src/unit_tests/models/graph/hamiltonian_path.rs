use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;

#[test]
fn test_hamiltonian_path_basic() {
    use crate::traits::Problem;

    // Path graph: 0-1-2-3 (has Hamiltonian path)
    let problem = HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.dims(), vec![4, 4, 4, 4]);

    // Valid path: 0->1->2->3
    assert!(problem.evaluate(&[0, 1, 2, 3]));
    // Valid path: 3->2->1->0 (reversed)
    assert!(problem.evaluate(&[3, 2, 1, 0]));
    // Invalid: 0->1->3->2 (no edge 1-3)
    assert!(!problem.evaluate(&[0, 1, 3, 2]));
    // Invalid: not a permutation (repeated vertex)
    assert!(!problem.evaluate(&[0, 1, 1, 2]));
}

#[test]
fn test_hamiltonian_path_no_solution() {
    // K4 on {0,1,2,3} + two isolated vertices {4,5}
    let problem = HamiltonianPath::new(SimpleGraph::new(
        6,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    ));
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(
        solution.is_none(),
        "Graph with isolated vertices has no Hamiltonian path"
    );
}

#[test]
fn test_hamiltonian_path_brute_force() {
    use crate::traits::Problem;

    // Path graph P4: 0-1-2-3
    let problem = HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let solver = BruteForce::new();

    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));

    let all = solver.find_all_satisfying(&problem);
    // Path graph P4 has exactly 2 Hamiltonian paths: 0-1-2-3 and 3-2-1-0
    assert_eq!(all.len(), 2);
    for sol in &all {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_hamiltonian_path_nontrivial() {
    use crate::traits::Problem;

    // Instance 2 from issue: 6 vertices, 8 edges
    let problem = HamiltonianPath::new(SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (2, 3),
            (3, 4),
            (3, 5),
            (4, 2),
            (5, 1),
        ],
    ));
    // Hamiltonian path: 0->2->4->3->1->5
    assert!(problem.evaluate(&[0, 2, 4, 3, 1, 5]));
}

#[test]
fn test_hamiltonian_path_complete_graph() {
    // Complete graph K4: every permutation is a Hamiltonian path
    let problem = HamiltonianPath::new(SimpleGraph::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    ));
    let solver = BruteForce::new();
    let all = solver.find_all_satisfying(&problem);
    // K4 has 4! = 24 Hamiltonian paths (all permutations)
    assert_eq!(all.len(), 24);
}

#[test]
fn test_is_valid_hamiltonian_path_function() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    assert!(is_valid_hamiltonian_path(&graph, &[0, 1, 2, 3]));
    assert!(is_valid_hamiltonian_path(&graph, &[3, 2, 1, 0]));
    assert!(!is_valid_hamiltonian_path(&graph, &[0, 1, 3, 2]));
    // Wrong length
    assert!(!is_valid_hamiltonian_path(&graph, &[0, 1, 2]));
    // Vertex out of range
    assert!(!is_valid_hamiltonian_path(&graph, &[0, 1, 2, 4]));
}

#[test]
fn test_hamiltonian_path_serialization() {
    let problem = HamiltonianPath::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let json = serde_json::to_value(&problem).unwrap();
    let deserialized: HamiltonianPath<SimpleGraph> = serde_json::from_value(json).unwrap();
    assert_eq!(deserialized.num_vertices(), 3);
    assert_eq!(deserialized.num_edges(), 2);
}

#[test]
fn test_is_valid_solution() {
    let problem = HamiltonianPath::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    assert!(problem.is_valid_solution(&[0, 1, 2]));
    assert!(!problem.is_valid_solution(&[0, 2, 1])); // no edge 0-2
}

#[test]
fn test_size_getters() {
    let problem = HamiltonianPath::new(SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]));
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 4);
}

#[test]
fn test_hamiltonianpath_paper_example() {
    use crate::traits::Problem;

    // Paper/issue #217: 6 vertices, 8 edges
    let problem = HamiltonianPath::new(SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (2, 3),
            (3, 4),
            (3, 5),
            (4, 2),
            (5, 1),
        ],
    ));

    // Hamiltonian path: 0→2→4→3→1→5
    assert!(problem.evaluate(&[0, 2, 4, 3, 1, 5]));

    // Verify with brute force
    let solver = BruteForce::new();
    let all = solver.find_all_satisfying(&problem);
    assert!(!all.is_empty());
    for sol in &all {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_single_vertex() {
    use crate::traits::Problem;

    // Single vertex graph: trivially has a Hamiltonian "path" (just the vertex)
    let problem = HamiltonianPath::new(SimpleGraph::new(1, vec![]));
    assert!(problem.evaluate(&[0]));
    let solver = BruteForce::new();
    let all = solver.find_all_satisfying(&problem);
    assert_eq!(all.len(), 1);
}
