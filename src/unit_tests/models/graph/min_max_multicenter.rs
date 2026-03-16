use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

/// Helper: build the canonical example instance.
/// 6 vertices, 7 edges [{0,1},{1,2},{2,3},{3,4},{4,5},{0,5},{1,4}],
/// unit weights/lengths, K=2, B=1.
fn example_instance() -> MinMaxMulticenter<SimpleGraph, i32> {
    let graph = SimpleGraph::new(
        6,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 5), (1, 4)],
    );
    MinMaxMulticenter::new(graph, vec![1i32; 6], vec![1i32; 7], 2, 1)
}

#[test]
fn test_minmaxmulticenter_basic() {
    let problem = example_instance();
    assert_eq!(problem.graph().num_vertices(), 6);
    assert_eq!(problem.graph().num_edges(), 7);
    assert_eq!(problem.k(), 2);
    assert_eq!(*problem.bound(), 1);
    assert_eq!(problem.vertex_weights(), &[1, 1, 1, 1, 1, 1]);
    assert_eq!(problem.edge_lengths(), &[1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(problem.dims(), vec![2; 6]);
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 7);
    assert_eq!(problem.num_centers(), 2);
}

#[test]
fn test_minmaxmulticenter_evaluate_valid() {
    let problem = example_instance();
    // Centers at vertices 1 and 4:
    // Distances: d(0)=1, d(1)=0, d(2)=1, d(3)=1, d(4)=0, d(5)=1
    // Max weighted distance = 1*1 = 1 <= B=1
    assert!(problem.evaluate(&[0, 1, 0, 0, 1, 0]));
}

#[test]
fn test_minmaxmulticenter_evaluate_invalid_count() {
    let problem = example_instance();
    // 3 centers selected when K=2
    assert!(!problem.evaluate(&[1, 1, 1, 0, 0, 0]));
}

#[test]
fn test_minmaxmulticenter_evaluate_invalid_distance() {
    let problem = example_instance();
    // Centers at 0 and 5 (adjacent via edge {0,5}):
    // Distances: d(0)=0, d(1)=1, d(2)=2, d(3)=2, d(4)=1, d(5)=0
    // Max weighted distance = 1*2 = 2 > B=1
    assert!(!problem.evaluate(&[1, 0, 0, 0, 0, 1]));
}

#[test]
fn test_minmaxmulticenter_evaluate_no_centers() {
    let problem = example_instance();
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0, 0]));
}

#[test]
fn test_minmaxmulticenter_serialization() {
    let problem = example_instance();

    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinMaxMulticenter<SimpleGraph, i32> = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.graph().num_vertices(), 6);
    assert_eq!(deserialized.graph().num_edges(), 7);
    assert_eq!(deserialized.vertex_weights(), &[1, 1, 1, 1, 1, 1]);
    assert_eq!(deserialized.edge_lengths(), &[1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(deserialized.k(), 2);
    assert_eq!(*deserialized.bound(), 1);

    // Verify evaluation produces same results
    let config = vec![0, 1, 0, 0, 1, 0];
    assert_eq!(problem.evaluate(&config), deserialized.evaluate(&config));
}

#[test]
fn test_minmaxmulticenter_solver() {
    let problem = example_instance();

    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);

    // All solutions should evaluate to true
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }

    // Centers at {1, 4} should be among the solutions
    assert!(solutions.contains(&vec![0, 1, 0, 0, 1, 0]));
}

#[test]
fn test_minmaxmulticenter_disconnected() {
    // Two disconnected components: 0-1 and 2-3, K=1, B=1
    let graph = SimpleGraph::new(4, vec![(0, 1), (2, 3)]);
    let problem = MinMaxMulticenter::new(graph, vec![1i32; 4], vec![1i32; 2], 1, 1);

    // Center at 0: vertices 2 and 3 are unreachable -> false
    assert!(!problem.evaluate(&[1, 0, 0, 0]));

    // With K=2, centers at {0, 2}: all reachable, max distance = 1 <= B=1
    let graph2 = SimpleGraph::new(4, vec![(0, 1), (2, 3)]);
    let problem2 = MinMaxMulticenter::new(graph2, vec![1i32; 4], vec![1i32; 2], 2, 1);
    assert!(problem2.evaluate(&[1, 0, 1, 0]));
}

#[test]
fn test_minmaxmulticenter_weighted() {
    // Path: 0-1-2, vertex weights = [3, 1, 2], edge lengths = [1, 1], K=1, B=3
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinMaxMulticenter::new(graph, vec![3i32, 1, 2], vec![1i32; 2], 1, 3);

    // Center at 1: d(0)=1, d(1)=0, d(2)=1
    // w(0)*d(0) = 3*1 = 3, w(1)*d(1) = 0, w(2)*d(2) = 2*1 = 2
    // max = 3 <= B=3 -> true
    assert!(problem.evaluate(&[0, 1, 0]));

    // Center at 0: d(0)=0, d(1)=1, d(2)=2
    // w(0)*d(0) = 0, w(1)*d(1) = 1, w(2)*d(2) = 4
    // max = 4 > B=3 -> false
    assert!(!problem.evaluate(&[1, 0, 0]));
}

#[test]
fn test_minmaxmulticenter_single_vertex() {
    let graph = SimpleGraph::new(1, vec![]);
    let problem = MinMaxMulticenter::new(graph, vec![5i32], vec![], 1, 0);
    // Only vertex is the center, max weighted distance = 0 <= B=0
    assert!(problem.evaluate(&[1]));
}

#[test]
fn test_minmaxmulticenter_all_centers() {
    // K = num_vertices: all vertices are centers, max distance = 0
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinMaxMulticenter::new(graph, vec![1i32; 3], vec![1i32; 2], 3, 0);
    assert!(problem.evaluate(&[1, 1, 1]));
}

#[test]
#[should_panic(expected = "vertex_weights length must match num_vertices")]
fn test_minmaxmulticenter_wrong_vertex_weights_len() {
    let graph = SimpleGraph::new(3, vec![(0, 1)]);
    MinMaxMulticenter::new(graph, vec![1i32; 2], vec![1i32; 1], 1, 0);
}

#[test]
#[should_panic(expected = "edge_lengths length must match num_edges")]
fn test_minmaxmulticenter_wrong_edge_lengths_len() {
    let graph = SimpleGraph::new(3, vec![(0, 1)]);
    MinMaxMulticenter::new(graph, vec![1i32; 3], vec![1i32; 2], 1, 0);
}

#[test]
#[should_panic(expected = "k must be positive")]
fn test_minmaxmulticenter_k_zero() {
    let graph = SimpleGraph::new(3, vec![(0, 1)]);
    MinMaxMulticenter::new(graph, vec![1i32; 3], vec![1i32; 1], 0, 0);
}

#[test]
#[should_panic(expected = "k must not exceed num_vertices")]
fn test_minmaxmulticenter_k_too_large() {
    let graph = SimpleGraph::new(3, vec![(0, 1)]);
    MinMaxMulticenter::new(graph, vec![1i32; 3], vec![1i32; 1], 4, 0);
}
