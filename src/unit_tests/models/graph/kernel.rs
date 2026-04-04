use super::*;
use crate::solvers::BruteForce;
use crate::topology::DirectedGraph;
use crate::traits::Problem;

#[test]
fn test_kernel_creation() {
    let graph = DirectedGraph::new(
        5,
        vec![(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (4, 0), (4, 1)],
    );
    let problem = Kernel::new(graph);
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_arcs(), 7);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2, 2]);
}

#[test]
fn test_kernel_evaluate_valid() {
    // 5 vertices, arcs: (0,1),(0,2),(1,3),(2,3),(3,4),(4,0),(4,1)
    // Kernel: V' = {0, 3} → config [1,0,0,1,0]
    // Independence: no arc between 0 and 3 in either direction
    // Absorption: 1 has arc to 3 (selected), 2 has arc to 3 (selected), 4 has arc to 0 (selected)
    let graph = DirectedGraph::new(
        5,
        vec![(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (4, 0), (4, 1)],
    );
    let problem = Kernel::new(graph);
    assert_eq!(problem.evaluate(&[1, 0, 0, 1, 0]), crate::types::Or(true));
}

#[test]
fn test_kernel_evaluate_not_independent() {
    // Select vertices 0 and 1, but there is arc (0,1), so not independent
    let graph = DirectedGraph::new(
        5,
        vec![(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (4, 0), (4, 1)],
    );
    let problem = Kernel::new(graph);
    assert_eq!(problem.evaluate(&[1, 1, 0, 0, 0]), crate::types::Or(false));
}

#[test]
fn test_kernel_evaluate_not_absorbing() {
    // Select only vertex 0: vertex 1 has no arc to 0 (only 0->1 exists), so not absorbing
    // Actually, let's check: successors of 1 = {3}, and 3 is not selected
    // successors of 2 = {3}, not selected
    // successors of 3 = {4}, not selected
    // successors of 4 = {0, 1}, 0 is selected → ok for 4
    // But 1,2,3 cannot reach any selected vertex → not absorbing
    let graph = DirectedGraph::new(
        5,
        vec![(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (4, 0), (4, 1)],
    );
    let problem = Kernel::new(graph);
    assert_eq!(problem.evaluate(&[1, 0, 0, 0, 0]), crate::types::Or(false));
}

#[test]
fn test_kernel_brute_force() {
    let graph = DirectedGraph::new(
        5,
        vec![(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (4, 0), (4, 1)],
    );
    let problem = Kernel::new(graph);
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).expect("should have a kernel");
    assert_eq!(problem.evaluate(&solution), crate::types::Or(true));
}

#[test]
fn test_kernel_no_solution() {
    // A directed 3-cycle has no kernel:
    // 0->1, 1->2, 2->0
    // Any single vertex is not absorbing (e.g., {0}: vertex 2 has successor 0, ok; vertex 1 has successor 2, not selected → fail)
    // Wait, let's verify: {0}: successors of 1 = {2}, not selected. Not absorbing.
    // {0,1}: arc (0,1) exists. Not independent.
    // No kernel exists for odd cycles.
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = Kernel::new(graph);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_kernel_serialization() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = Kernel::new(graph);
    let json = serde_json::to_value(&problem).unwrap();
    let deserialized: Kernel = serde_json::from_value(json).unwrap();
    assert_eq!(deserialized.num_vertices(), 3);
    assert_eq!(deserialized.num_arcs(), 2);
}

#[test]
fn test_kernel_empty_graph() {
    // A graph with no arcs: every vertex is independent; absorption requires
    // every unselected vertex to have an arc to a selected one, but no arcs exist.
    // So the only kernel is the full vertex set (all selected → no unselected vertices to check).
    let graph = DirectedGraph::new(3, vec![]);
    let problem = Kernel::new(graph);
    // All selected: independent (no arcs), absorbing (no unselected vertices)
    assert_eq!(problem.evaluate(&[1, 1, 1]), crate::types::Or(true));
    // Not all selected: e.g., {0} → vertex 1 has no arc to 0, not absorbing
    assert_eq!(problem.evaluate(&[1, 0, 0]), crate::types::Or(false));
}
