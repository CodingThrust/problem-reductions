use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::{ObjectiveProblem, Problem};
use crate::types::ExtremumSense;

#[test]
fn test_min_sum_multicenter_creation() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = MinimumSumMulticenter::new(graph, vec![1i32; 4], vec![1i32; 3], 2);
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 3);
    assert_eq!(problem.k(), 2);
    assert_eq!(problem.vertex_weights(), &[1, 1, 1, 1]);
    assert_eq!(problem.edge_lengths(), &[1, 1, 1]);
}

#[test]
fn test_min_sum_multicenter_size_getters() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let problem = MinimumSumMulticenter::new(graph, vec![1i32; 5], vec![1i32; 4], 2);
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 4);
    assert_eq!(problem.num_centers(), 2);
}

#[test]
fn test_min_sum_multicenter_direction() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumSumMulticenter::new(graph, vec![1i32; 3], vec![1i32; 2], 1);
    assert_eq!(problem.direction(), ExtremumSense::Minimize);
}

#[test]
fn test_min_sum_multicenter_evaluate_path() {
    // Path: 0-1-2, unit weights and lengths, K=1
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumSumMulticenter::new(graph, vec![1i32; 3], vec![1i32; 2], 1);

    // Center at vertex 1: distances = [1, 0, 1], total = 2
    let result = problem.evaluate(&[0, 1, 0]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 2);

    // Center at vertex 0: distances = [0, 1, 2], total = 3
    let result = problem.evaluate(&[1, 0, 0]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 3);
}

#[test]
fn test_min_sum_multicenter_wrong_k() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumSumMulticenter::new(graph, vec![1i32; 3], vec![1i32; 2], 2);

    // Only 1 center selected when K=2
    let result = problem.evaluate(&[0, 1, 0]);
    assert!(!result.is_valid());

    // 3 centers selected when K=2
    let result = problem.evaluate(&[1, 1, 1]);
    assert!(!result.is_valid());

    // No centers selected
    let result = problem.evaluate(&[0, 0, 0]);
    assert!(!result.is_valid());
}

#[test]
fn test_min_sum_multicenter_weighted() {
    // Path: 0-1-2, vertex weights = [3, 1, 2], edge lengths = [1, 1], K=1
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumSumMulticenter::new(graph, vec![3i32, 1, 2], vec![1i32; 2], 1);

    // Center at 0: distances = [0, 1, 2], total = 3*0 + 1*1 + 2*2 = 5
    assert_eq!(problem.evaluate(&[1, 0, 0]).unwrap(), 5);

    // Center at 1: distances = [1, 0, 1], total = 3*1 + 1*0 + 2*1 = 5
    assert_eq!(problem.evaluate(&[0, 1, 0]).unwrap(), 5);

    // Center at 2: distances = [2, 1, 0], total = 3*2 + 1*1 + 2*0 = 7
    assert_eq!(problem.evaluate(&[0, 0, 1]).unwrap(), 7);
}

#[test]
fn test_min_sum_multicenter_weighted_edges() {
    // Triangle: 0-1 (len 1), 1-2 (len 3), 0-2 (len 2), K=1
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = MinimumSumMulticenter::new(graph, vec![1i32; 3], vec![1, 3, 2], 1);

    // Center at 0: d(0)=0, d(1)=1, d(2)=2, total=3
    assert_eq!(problem.evaluate(&[1, 0, 0]).unwrap(), 3);

    // Center at 1: d(1)=0, d(0)=1, d(2)=min(3, 1+2)=3, total=4
    assert_eq!(problem.evaluate(&[0, 1, 0]).unwrap(), 4);
}

#[test]
fn test_min_sum_multicenter_two_centers() {
    // Path: 0-1-2-3-4, unit weights and lengths, K=2
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let problem = MinimumSumMulticenter::new(graph, vec![1i32; 5], vec![1i32; 4], 2);

    // Centers at {1, 3}: d = [1, 0, 1, 0, 1], total = 3
    assert_eq!(problem.evaluate(&[0, 1, 0, 1, 0]).unwrap(), 3);

    // Centers at {0, 4}: d = [0, 1, 2, 1, 0], total = 4
    assert_eq!(problem.evaluate(&[1, 0, 0, 0, 1]).unwrap(), 4);
}

#[test]
fn test_min_sum_multicenter_solver() {
    // Issue example: 7 vertices, 8 edges, unit weights, K=2
    let graph = SimpleGraph::new(
        7,
        vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 6),
            (0, 6),
            (2, 5),
        ],
    );
    let problem = MinimumSumMulticenter::new(graph, vec![1i32; 7], vec![1i32; 8], 2);

    let solver = BruteForce::new();
    let best = solver.find_witness(&problem).unwrap();
    let best_cost = problem.evaluate(&best).unwrap();

    // Optimal cost should be 6 (centers at {2, 5})
    assert_eq!(best_cost, 6);
}

#[test]
fn test_min_sum_multicenter_disconnected() {
    // Two disconnected components: 0-1 and 2-3, K=1
    let graph = SimpleGraph::new(4, vec![(0, 1), (2, 3)]);
    let problem = MinimumSumMulticenter::new(graph, vec![1i32; 4], vec![1i32; 2], 1);

    // Center at 0: vertex 2 and 3 are unreachable
    let result = problem.evaluate(&[1, 0, 0, 0]);
    assert!(!result.is_valid());

    // With K=2, centers at {0, 2}: all reachable
    let graph2 = SimpleGraph::new(4, vec![(0, 1), (2, 3)]);
    let problem2 = MinimumSumMulticenter::new(graph2, vec![1i32; 4], vec![1i32; 2], 2);
    let result2 = problem2.evaluate(&[1, 0, 1, 0]);
    assert!(result2.is_valid());
    assert_eq!(result2.unwrap(), 2); // d = [0, 1, 0, 1]
}

#[test]
fn test_min_sum_multicenter_single_vertex() {
    let graph = SimpleGraph::new(1, vec![]);
    let problem = MinimumSumMulticenter::new(graph, vec![5i32], vec![], 1);
    let result = problem.evaluate(&[1]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 0); // Only vertex is the center, distance = 0
}

#[test]
fn test_min_sum_multicenter_all_centers() {
    // K = num_vertices: all vertices are centers, total distance = 0
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumSumMulticenter::new(graph, vec![1i32; 3], vec![1i32; 2], 3);
    let result = problem.evaluate(&[1, 1, 1]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 0);
}

#[test]
#[should_panic(expected = "vertex_weights length must match num_vertices")]
fn test_min_sum_multicenter_wrong_vertex_weights_len() {
    let graph = SimpleGraph::new(3, vec![(0, 1)]);
    MinimumSumMulticenter::new(graph, vec![1i32; 2], vec![1i32; 1], 1);
}

#[test]
#[should_panic(expected = "edge_lengths length must match num_edges")]
fn test_min_sum_multicenter_wrong_edge_lengths_len() {
    let graph = SimpleGraph::new(3, vec![(0, 1)]);
    MinimumSumMulticenter::new(graph, vec![1i32; 3], vec![1i32; 2], 1);
}

#[test]
#[should_panic(expected = "k must be positive")]
fn test_min_sum_multicenter_k_zero() {
    let graph = SimpleGraph::new(3, vec![(0, 1)]);
    MinimumSumMulticenter::new(graph, vec![1i32; 3], vec![1i32; 1], 0);
}

#[test]
#[should_panic(expected = "k must not exceed num_vertices")]
fn test_min_sum_multicenter_k_too_large() {
    let graph = SimpleGraph::new(3, vec![(0, 1)]);
    MinimumSumMulticenter::new(graph, vec![1i32; 3], vec![1i32; 1], 4);
}

#[test]
fn test_min_sum_multicenter_paper_example() {
    // Paper/issue #399: 7 vertices, 8 edges, unit weights, K=2
    let graph = SimpleGraph::new(
        7,
        vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 6),
            (0, 6),
            (2, 5),
        ],
    );
    let problem = MinimumSumMulticenter::new(graph, vec![1i32; 7], vec![1i32; 8], 2);

    // Optimal: centers at {2, 5}, config [0,0,1,0,0,1,0]
    // Distances: d(0)=2, d(1)=1, d(2)=0, d(3)=1, d(4)=1, d(5)=0, d(6)=1
    // Total = 6
    let result = problem.evaluate(&[0, 0, 1, 0, 0, 1, 0]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 6);

    // Verify optimality with brute force
    let solver = BruteForce::new();
    let best = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&best).unwrap(), 6);
}

#[test]
fn test_min_sum_multicenter_dims() {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let problem = MinimumSumMulticenter::new(graph, vec![1i32; 5], vec![1i32; 4], 2);
    assert_eq!(problem.dims(), vec![2; 5]);
}

#[test]
fn test_min_sum_multicenter_find_all_witnesses() {
    // Path: 0-1-2, unit weights, K=1. Center at 1 is optimal (cost 2)
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumSumMulticenter::new(graph, vec![1i32; 3], vec![1i32; 2], 1);

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0, 1, 0]);
}

#[test]
fn test_min_sum_multicenter_serialization() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumSumMulticenter::new(graph, vec![1i32; 3], vec![1i32; 2], 1);

    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinimumSumMulticenter<SimpleGraph, i32> =
        serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.graph().num_vertices(), 3);
    assert_eq!(deserialized.graph().num_edges(), 2);
    assert_eq!(deserialized.vertex_weights(), &[1, 1, 1]);
    assert_eq!(deserialized.edge_lengths(), &[1, 1]);
    assert_eq!(deserialized.k(), 1);

    // Verify evaluation produces same results
    let config = vec![0, 1, 0];
    assert_eq!(
        problem.evaluate(&config).unwrap(),
        deserialized.evaluate(&config).unwrap()
    );
}
