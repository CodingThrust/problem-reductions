use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Or;

fn issue_instance() -> VertexCover<SimpleGraph> {
    // Triangle {0,1,2} with pendant edge to 3
    VertexCover::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (0, 2), (2, 3)]), 2)
}

#[test]
fn test_vertex_cover_creation() {
    let problem = issue_instance();
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 4);
    assert_eq!(problem.k(), 2);
    assert_eq!(problem.dims(), vec![2; 4]);
}

#[test]
fn test_vertex_cover_evaluate_valid() {
    let problem = issue_instance();
    // {0, 2} covers all edges with size 2 ≤ k=2
    assert_eq!(problem.evaluate(&[1, 0, 1, 0]), Or(true));
}

#[test]
fn test_vertex_cover_evaluate_too_large() {
    let problem = issue_instance();
    // {0, 1, 2} is a valid cover but size 3 > k=2
    assert_eq!(problem.evaluate(&[1, 1, 1, 0]), Or(false));
}

#[test]
fn test_vertex_cover_evaluate_not_covering() {
    let problem = issue_instance();
    // {0} doesn't cover edge (1,2)
    assert_eq!(problem.evaluate(&[1, 0, 0, 0]), Or(false));
}

#[test]
fn test_vertex_cover_evaluate_k1_impossible() {
    // Same graph but k=1 — impossible for triangle
    let problem = VertexCover::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (0, 2), (2, 3)]), 1);
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem);
    assert!(witness.is_none());
}

#[test]
fn test_vertex_cover_solver() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem);
    assert!(witness.is_some());
    let w = witness.unwrap();
    assert_eq!(problem.evaluate(&w), Or(true));
    // Cover size should be ≤ k=2
    let count: usize = w.iter().filter(|&&v| v == 1).count();
    assert!(count <= 2);
}

#[test]
fn test_vertex_cover_all_witnesses() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    let witnesses = solver.find_all_witnesses(&problem);
    // For k=2 on triangle+pendant: covers of size ≤2 that cover all edges
    // Valid size-2 covers: {0,2}, {1,2} (vertex 2 covers pendant edge)
    // {0,1} doesn't cover (2,3)
    assert!(witnesses.len() >= 2);
    for w in &witnesses {
        assert_eq!(problem.evaluate(w), Or(true));
    }
}

#[test]
fn test_vertex_cover_serialization() {
    let problem = issue_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let restored: VertexCover<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.num_vertices(), 4);
    assert_eq!(restored.k(), 2);
    assert_eq!(restored.evaluate(&[1, 0, 1, 0]), Or(true));
}

#[test]
fn test_vertex_cover_path_graph() {
    // Path 0-1-2: minimum cover is {1}, size 1
    let problem = VertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), 1);
    assert_eq!(problem.evaluate(&[0, 1, 0]), Or(true));
    assert_eq!(problem.evaluate(&[1, 0, 0]), Or(false)); // Doesn't cover (1,2)
}

#[test]
#[should_panic(expected = "k must be positive")]
fn test_vertex_cover_k_zero() {
    VertexCover::new(SimpleGraph::new(3, vec![(0, 1)]), 0);
}
