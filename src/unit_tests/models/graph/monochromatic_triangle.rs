use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn k4_instance() -> MonochromaticTriangle<SimpleGraph> {
    // K4: complete graph on 4 vertices, 6 edges
    MonochromaticTriangle::new(SimpleGraph::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    ))
}

#[test]
fn test_monochromatic_triangle_creation() {
    let problem = k4_instance();
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 6);
    // K4 has 4 triangles
    assert_eq!(problem.triangles().len(), 4);
    // One binary variable per edge
    assert_eq!(problem.dims(), vec![2; 6]);
    assert_eq!(problem.graph().num_vertices(), 4);
}

#[test]
fn test_monochromatic_triangle_evaluate_valid() {
    let problem = k4_instance();
    // Edges: (0,1),(0,2),(0,3),(1,2),(1,3),(2,3)
    // Config [0,0,1,1,0,1]:
    //   Triangle (0,1,2): edges 0,1,3 -> 0,0,1 -> mixed
    //   Triangle (0,1,3): edges 0,2,4 -> 0,1,0 -> mixed
    //   Triangle (0,2,3): edges 1,2,5 -> 0,1,1 -> mixed
    //   Triangle (1,2,3): edges 3,4,5 -> 1,0,1 -> mixed
    assert!(problem.evaluate(&[0, 0, 1, 1, 0, 1]));
}

#[test]
fn test_monochromatic_triangle_evaluate_invalid() {
    let problem = k4_instance();
    // All edges color 0: every triangle is monochromatic
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0, 0]));
    // All edges color 1: every triangle is monochromatic
    assert!(!problem.evaluate(&[1, 1, 1, 1, 1, 1]));
}

#[test]
fn test_monochromatic_triangle_evaluate_wrong_length() {
    let problem = k4_instance();
    assert!(!problem.evaluate(&[0, 1, 0]));
    assert!(!problem.evaluate(&[0, 1, 0, 0, 1, 1, 0]));
}

#[test]
fn test_monochromatic_triangle_triangle_free_graph() {
    // A path graph 0-1-2 has no triangles, so any coloring is valid.
    let problem = MonochromaticTriangle::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    assert_eq!(problem.triangles().len(), 0);
    assert!(problem.evaluate(&[0, 0]));
    assert!(problem.evaluate(&[1, 1]));
    assert!(problem.evaluate(&[0, 1]));
}

#[test]
fn test_monochromatic_triangle_brute_force_k4() {
    let problem = k4_instance();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));
}

#[test]
fn test_monochromatic_triangle_brute_force_k6_no_solution() {
    // By Ramsey theory R(3,3)=6, K6 has no 2-coloring avoiding monochromatic triangles.
    let mut edges = Vec::new();
    for u in 0..6 {
        for v in (u + 1)..6 {
            edges.push((u, v));
        }
    }
    let problem = MonochromaticTriangle::new(SimpleGraph::new(6, edges));
    assert_eq!(problem.num_edges(), 15);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_monochromatic_triangle_brute_force_k5_has_solution() {
    // K5 has valid 2-colorings (R(3,3)=6, so K5 can be 2-colored).
    let mut edges = Vec::new();
    for u in 0..5 {
        for v in (u + 1)..5 {
            edges.push((u, v));
        }
    }
    let problem = MonochromaticTriangle::new(SimpleGraph::new(5, edges));
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));
}

#[test]
fn test_monochromatic_triangle_serialization() {
    let problem = k4_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MonochromaticTriangle<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vertices(), 4);
    assert_eq!(deserialized.num_edges(), 6);
    assert_eq!(deserialized.triangles().len(), 4);
}
