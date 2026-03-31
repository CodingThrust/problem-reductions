use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;

#[test]
fn test_hamiltonian_path_between_two_vertices_basic() {
    use crate::traits::Problem;

    // Path graph: 0-1-2-3, source=0, target=3
    let problem = HamiltonianPathBetweenTwoVertices::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        0,
        3,
    );
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.source_vertex(), 0);
    assert_eq!(problem.target_vertex(), 3);
    assert_eq!(problem.dims(), vec![4, 4, 4, 4]);

    // Valid path: 0->1->2->3
    assert!(problem.evaluate(&[0, 1, 2, 3]));
    // Reversed path fails (wrong source/target)
    assert!(!problem.evaluate(&[3, 2, 1, 0]));
    // Invalid: wrong start vertex
    assert!(!problem.evaluate(&[1, 0, 2, 3]));
    // Invalid: wrong end vertex
    assert!(!problem.evaluate(&[0, 1, 3, 2]));
    // Invalid: not a permutation
    assert!(!problem.evaluate(&[0, 1, 1, 3]));
}

#[test]
fn test_hamiltonian_path_between_two_vertices_no_solution() {
    // C5 cycle: s=0, t=2 has no Hamiltonian s-t path (from issue #831)
    let problem = HamiltonianPathBetweenTwoVertices::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]),
        0,
        2,
    );
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(
        solution.is_none(),
        "C5 with s=0, t=2 has no Hamiltonian s-t path"
    );
}

#[test]
fn test_hamiltonian_path_between_two_vertices_brute_force() {
    use crate::traits::Problem;

    // Issue #831 Example 1: 6 vertices, 8 edges, s=0, t=5
    let problem = HamiltonianPathBetweenTwoVertices::new(
        SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (0, 3),
                (1, 2),
                (1, 4),
                (2, 5),
                (3, 4),
                (4, 5),
                (2, 3),
            ],
        ),
        0,
        5,
    );

    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert!(problem.evaluate(&sol));
    assert_eq!(sol[0], 0, "Path must start at source vertex 0");
    assert_eq!(sol[5], 5, "Path must end at target vertex 5");

    // Issue says there are exactly 4 distinct Hamiltonian s-t paths
    let all = solver.find_all_witnesses(&problem);
    assert_eq!(all.len(), 4);
    for path in &all {
        assert!(problem.evaluate(path));
        assert_eq!(path[0], 0);
        assert_eq!(path[5], 5);
    }
}

#[test]
fn test_hamiltonian_path_between_two_vertices_is_valid_solution() {
    let problem =
        HamiltonianPathBetweenTwoVertices::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), 0, 2);
    assert!(problem.is_valid_solution(&[0, 1, 2]));
    assert!(!problem.is_valid_solution(&[2, 1, 0])); // wrong direction
    assert!(!problem.is_valid_solution(&[0, 2, 1])); // no edge 0-2
}

#[test]
fn test_hamiltonian_path_between_two_vertices_is_valid_st_path_function() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    // Valid: 0->1->2->3 with source=0, target=3
    assert!(is_valid_hamiltonian_st_path(&graph, &[0, 1, 2, 3], 0, 3));
    // Invalid: reversed (source=3 but we pass source=0)
    assert!(!is_valid_hamiltonian_st_path(&graph, &[3, 2, 1, 0], 0, 3));
    // Invalid: edge 0->2 doesn't exist
    assert!(!is_valid_hamiltonian_st_path(&graph, &[0, 2, 1, 3], 0, 3));
    // Invalid: wrong length
    assert!(!is_valid_hamiltonian_st_path(&graph, &[0, 1, 3], 0, 3));
    // Invalid: vertex out of range
    assert!(!is_valid_hamiltonian_st_path(&graph, &[0, 1, 2, 4], 0, 3));
}

#[test]
fn test_hamiltonian_path_between_two_vertices_serialization() {
    let problem =
        HamiltonianPathBetweenTwoVertices::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), 0, 2);
    let json = serde_json::to_value(&problem).unwrap();
    let deserialized: HamiltonianPathBetweenTwoVertices<SimpleGraph> =
        serde_json::from_value(json).unwrap();
    assert_eq!(deserialized.num_vertices(), 3);
    assert_eq!(deserialized.num_edges(), 2);
    assert_eq!(deserialized.source_vertex(), 0);
    assert_eq!(deserialized.target_vertex(), 2);
}

#[test]
fn test_hamiltonian_path_between_two_vertices_paper_example() {
    use crate::traits::Problem;

    // Instance from issue #831: 6 vertices, s=0, t=5
    let problem = HamiltonianPathBetweenTwoVertices::new(
        SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (0, 3),
                (1, 2),
                (1, 4),
                (2, 5),
                (3, 4),
                (4, 5),
                (2, 3),
            ],
        ),
        0,
        5,
    );

    // Issue-specified solution: 0 -> 3 -> 2 -> 1 -> 4 -> 5
    assert!(problem.evaluate(&[0, 3, 2, 1, 4, 5]));

    // Verify edge-by-edge
    let path = [0usize, 3, 2, 1, 4, 5];
    assert_eq!(path[0], 0, "path starts at s=0");
    assert_eq!(path[5], 5, "path ends at t=5");

    // Verify brute force confirms the problem
    let solver = BruteForce::new();
    let all = solver.find_all_witnesses(&problem);
    assert_eq!(all.len(), 4, "issue says 4 Hamiltonian s-t paths exist");
    assert!(all.contains(&vec![0, 3, 2, 1, 4, 5]));
}
