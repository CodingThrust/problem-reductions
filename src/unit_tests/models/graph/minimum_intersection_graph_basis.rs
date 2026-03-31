use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_minimum_intersection_graph_basis_creation() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumIntersectionGraphBasis::new(graph);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_edges(), 2);
    // 3 vertices * 2 edges = 6 binary variables
    assert_eq!(problem.num_variables(), 6);
    assert_eq!(problem.dims(), vec![2; 6]);
}

#[test]
fn test_minimum_intersection_graph_basis_p3() {
    // Path P3: 0-1-2, edges (0,1) and (1,2)
    // Intersection number = 2: S[0]={0}, S[1]={0,1}, S[2]={1}
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumIntersectionGraphBasis::new(graph);

    // Valid config: S[0]={0}, S[1]={0,1}, S[2]={1} -> [1,0, 1,1, 0,1]
    let config = vec![1, 0, 1, 1, 0, 1];
    assert_eq!(problem.evaluate(&config), Min(Some(2)));

    // Brute force should find optimal = 2
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Min(Some(2)));
}

#[test]
fn test_minimum_intersection_graph_basis_single_edge() {
    // Single edge: 0-1
    // Intersection number = 1: S[0]={0}, S[1]={0}
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let problem = MinimumIntersectionGraphBasis::new(graph);

    // Valid: S[0]={0}, S[1]={0} -> [1, 1]
    assert_eq!(problem.evaluate(&[1, 1]), Min(Some(1)));

    // Invalid: S[0]={}, S[1]={0} -> edge not covered
    assert_eq!(problem.evaluate(&[0, 1]), Min(None));

    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Min(Some(1)));
}

#[test]
fn test_minimum_intersection_graph_basis_triangle() {
    // Triangle K3: edges (0,1),(1,2),(0,2)
    // Intersection number = 1 for K3: all vertices share one element.
    // S[0]={0}, S[1]={0}, S[2]={0} — all pairs intersect, which matches K3.
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = MinimumIntersectionGraphBasis::new(graph);

    // 3 vertices * 3 edges = 9 binary variables
    assert_eq!(problem.dims(), vec![2; 9]);

    // Valid: S[0]={0}, S[1]={0}, S[2]={0}
    // config: v0: [1,0,0], v1: [1,0,0], v2: [1,0,0]
    let config = vec![1, 0, 0, 1, 0, 0, 1, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(Some(1)));

    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Min(Some(1)));
}

#[test]
fn test_minimum_intersection_graph_basis_empty_graph() {
    // No edges: universe size 0
    let graph = SimpleGraph::new(3, vec![]);
    let problem = MinimumIntersectionGraphBasis::new(graph);
    assert_eq!(problem.evaluate(&[]), Min(Some(0)));
}

#[test]
fn test_minimum_intersection_graph_basis_wrong_length() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumIntersectionGraphBasis::new(graph);
    assert_eq!(problem.evaluate(&[1, 0, 1]), Min(None));
}

#[test]
fn test_minimum_intersection_graph_basis_invalid_nonadjacent_intersect() {
    // P3: edges (0,1),(1,2). Vertices 0 and 2 are NOT adjacent.
    // If S[0] and S[2] intersect, it's invalid.
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumIntersectionGraphBasis::new(graph);

    // S[0]={0,1}, S[1]={0,1}, S[2]={0,1} -> 0 and 2 share elements -> invalid
    let config = vec![1, 1, 1, 1, 1, 1];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimum_intersection_graph_basis_invalid_edge_not_covered() {
    // P3: edges (0,1),(1,2).
    // S[0]={0}, S[1]={1}, S[2]={1} -> edge (0,1) not covered (no intersection)
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumIntersectionGraphBasis::new(graph);

    let config = vec![1, 0, 0, 1, 0, 1];
    assert_eq!(problem.evaluate(&config), Min(None));
}
