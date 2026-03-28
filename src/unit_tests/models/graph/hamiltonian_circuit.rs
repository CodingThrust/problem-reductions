use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_hamiltonian_circuit_basic() {
    // Prism graph: 6 vertices, 9 edges
    // Two triangles (0,1,2) and (3,4,5) connected by edges (0,3), (1,4), (2,5)
    let graph = SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (1, 2),
            (2, 0),
            (3, 4),
            (4, 5),
            (5, 3),
            (0, 3),
            (1, 4),
            (2, 5),
        ],
    );
    let problem = HamiltonianCircuit::new(graph);

    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 9);
    assert_eq!(problem.dims(), vec![6; 6]);

    // Valid Hamiltonian circuit: 0->1->2->5->4->3->0
    // Edges used: (0,1), (1,2), (2,5), (5,4), (4,3), (3,0) -- all present
    assert!(problem.evaluate(&[0, 1, 2, 5, 4, 3]));

    // Invalid: 0->1->2->3 requires edge (2,3) which is NOT in the edge list
    assert!(!problem.evaluate(&[0, 1, 2, 3, 4, 5]));

    // Invalid: duplicate vertex 0 -- not a valid permutation
    assert!(!problem.evaluate(&[0, 0, 1, 2, 3, 4]));

    // Invalid: wrong-length config
    assert!(!problem.evaluate(&[0, 1]));

    // Invalid: vertex out of range
    assert!(!problem.evaluate(&[0, 1, 2, 3, 4, 99]));
}

#[test]
fn test_hamiltonian_circuit_small_graphs() {
    // Empty graph (0 vertices): n < 3, no circuit possible
    let graph = SimpleGraph::new(0, vec![]);
    let problem = HamiltonianCircuit::new(graph);
    assert!(!problem.evaluate(&[]));

    // Single vertex: n < 3
    let graph = SimpleGraph::new(1, vec![]);
    let problem = HamiltonianCircuit::new(graph);
    assert!(!problem.evaluate(&[0]));

    // Two vertices with edge: n < 3
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let problem = HamiltonianCircuit::new(graph);
    assert!(!problem.evaluate(&[0, 1]));

    // Triangle (K3): smallest valid Hamiltonian circuit
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = HamiltonianCircuit::new(graph);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    // K3 has 6 directed Hamiltonian circuits: 3 rotations x 2 directions
    assert_eq!(solutions.len(), 6);
}

#[test]
fn test_hamiltonian_circuit_complete_graph_k4() {
    // K4: complete graph on 4 vertices
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let problem = HamiltonianCircuit::new(graph);

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    // K4 has 3 distinct undirected Hamiltonian circuits, each yielding
    // 4 rotations x 2 directions = 8 directed permutations => 24 total
    assert_eq!(solutions.len(), 24);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_hamiltonian_circuit_no_solution() {
    // Path graph on 4 vertices: no Hamiltonian circuit possible
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = HamiltonianCircuit::new(graph);

    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
    assert!(solver.find_all_witnesses(&problem).is_empty());
}

#[test]
fn test_hamiltonian_circuit_solver() {
    // Cycle on 4 vertices (square): edges {0,1}, {1,2}, {2,3}, {3,0}
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]);
    let problem = HamiltonianCircuit::new(graph);

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);

    // 4-cycle has 8 Hamiltonian circuits: 4 starting positions x 2 directions
    assert_eq!(solutions.len(), 8);

    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_hamiltonian_circuit_serialization() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]);
    let problem = HamiltonianCircuit::new(graph);

    let json = serde_json::to_string(&problem).unwrap();
    let restored: HamiltonianCircuit<SimpleGraph> = serde_json::from_str(&json).unwrap();

    assert_eq!(problem.dims(), restored.dims());

    // Valid circuit gives the same result on both instances
    assert_eq!(
        problem.evaluate(&[0, 1, 2, 3]),
        restored.evaluate(&[0, 1, 2, 3])
    );
    // Invalid config gives the same result on both instances
    assert_eq!(
        problem.evaluate(&[0, 0, 1, 2]),
        restored.evaluate(&[0, 0, 1, 2])
    );
}
