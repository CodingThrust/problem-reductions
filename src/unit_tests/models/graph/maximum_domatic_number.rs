use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Max;

#[test]
fn test_maximum_domatic_number_creation() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = MaximumDomaticNumber::new(graph);
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 3);
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(problem.dims(), vec![4; 4]);
}

#[test]
fn test_maximum_domatic_number_evaluate_optimal() {
    // Graph from issue: 6 vertices
    // Edges: (0,1), (0,2), (0,3), (1,4), (2,5), (3,4), (3,5), (4,5)
    // Config: [0, 1, 2, 0, 2, 1] → 3 non-empty dominating sets → Max(3)
    let graph = SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 4),
            (2, 5),
            (3, 4),
            (3, 5),
            (4, 5),
        ],
    );
    let problem = MaximumDomaticNumber::new(graph);
    let config = vec![0, 1, 2, 0, 2, 1];
    let result = problem.evaluate(&config);
    assert_eq!(result, Max(Some(3)));
}

#[test]
fn test_maximum_domatic_number_evaluate_invalid() {
    // Path graph P3: 0-1-2
    // Config [0, 1, 2]: set {0} = {v0}, set {1} = {v1}, set {2} = {v2}
    // Set {2} = {v2} does NOT dominate v0 (v0 not in set and not adjacent to v2)
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumDomaticNumber::new(graph);
    let config = vec![0, 1, 2];
    let result = problem.evaluate(&config);
    assert_eq!(result, Max(None));
}

#[test]
fn test_maximum_domatic_number_evaluate_trivial() {
    // All vertices in one set → always a dominating set → Max(1)
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = MaximumDomaticNumber::new(graph);
    let config = vec![0, 0, 0, 0];
    let result = problem.evaluate(&config);
    assert_eq!(result, Max(Some(1)));
}

#[test]
fn test_maximum_domatic_number_solver_p3() {
    // Path graph P3: 0-1-2
    // Domatic number = 2: e.g., {0,2} and {1} are both dominating sets
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumDomaticNumber::new(graph);
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    let value = problem.evaluate(&witness);
    assert_eq!(value, Max(Some(2)));
}

#[test]
fn test_maximum_domatic_number_solver_complete_graph() {
    // K4: domatic number = 4 (each vertex is its own dominating set in K4)
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let problem = MaximumDomaticNumber::new(graph);
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    let value = problem.evaluate(&witness);
    assert_eq!(value, Max(Some(4)));
}

#[test]
fn test_maximum_domatic_number_serialization() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumDomaticNumber::new(graph);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MaximumDomaticNumber<SimpleGraph> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.graph().num_vertices(), 3);
    assert_eq!(deserialized.graph().num_edges(), 2);
}

#[test]
fn test_maximum_domatic_number_size_getters() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let problem = MaximumDomaticNumber::new(graph);
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 4);
}

#[test]
fn test_maximum_domatic_number_single_vertex() {
    // Single vertex: domatic number = 1
    let graph = SimpleGraph::new(1, vec![]);
    let problem = MaximumDomaticNumber::new(graph);
    let config = vec![0];
    assert_eq!(problem.evaluate(&config), Max(Some(1)));
}
