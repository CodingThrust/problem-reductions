use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn example_instance() -> BoundedDiameterSpanningTree<SimpleGraph, i32> {
    // 5 vertices, 7 edges with weights
    // (0,1,1),(0,2,2),(0,3,1),(1,2,1),(1,4,2),(2,3,1),(3,4,1)
    // B=5, D=3
    BoundedDiameterSpanningTree::new(
        SimpleGraph::new(
            5,
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 4), (2, 3), (3, 4)],
        ),
        vec![1, 2, 1, 1, 2, 1, 1],
        5,
        3,
    )
}

#[test]
fn test_bounded_diameter_spanning_tree_creation() {
    let problem = example_instance();
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 7);
    assert_eq!(problem.weight_bound(), &5);
    assert_eq!(problem.diameter_bound(), 3);
    assert_eq!(problem.dims(), vec![2; 7]);
    assert_eq!(problem.graph().num_vertices(), 5);
    assert_eq!(problem.edge_list().len(), 7);
    assert_eq!(problem.edge_weights().len(), 7);
    assert!(problem.is_weighted());
}

#[test]
fn test_bounded_diameter_spanning_tree_evaluate_valid() {
    let problem = example_instance();
    // Edges: (0,1)=0, (0,2)=1, (0,3)=2, (1,2)=3, (1,4)=4, (2,3)=5, (3,4)=6
    // Select edges 0,2,5,6: (0,1),(0,3),(2,3),(3,4)
    // Weight: 1+1+1+1 = 4 ≤ 5
    // Tree adjacency: 0-{1,3}, 1-{0}, 2-{3}, 3-{0,2,4}, 4-{3}
    // Diameter: longest path is e.g. 1-0-3-2 or 1-0-3-4 = 3 edges ≤ 3
    assert!(problem.evaluate(&[1, 0, 1, 0, 0, 1, 1]));
}

#[test]
fn test_bounded_diameter_spanning_tree_evaluate_exceeds_weight() {
    let problem = example_instance();
    // Select edges 1,2,4,6: (0,2),(0,3),(1,4),(3,4)
    // Weight: 2+1+2+1 = 6 > 5
    assert!(!problem.evaluate(&[0, 1, 1, 0, 1, 0, 1]));
}

#[test]
fn test_bounded_diameter_spanning_tree_evaluate_exceeds_diameter() {
    // Create instance with very tight diameter bound
    let problem = BoundedDiameterSpanningTree::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1, 1, 1],
        10,
        1, // diameter ≤ 1 means all vertices must be distance 1 from each other
    );
    // The only spanning tree is the path 0-1-2-3 with diameter 3
    assert!(!problem.evaluate(&[1, 1, 1]));
}

#[test]
fn test_bounded_diameter_spanning_tree_evaluate_not_tree() {
    let problem = example_instance();
    // Too few edges
    assert!(!problem.evaluate(&[1, 1, 0, 0, 0, 0, 0]));
    // Too many edges
    assert!(!problem.evaluate(&[1, 1, 1, 1, 1, 0, 0]));
}

#[test]
fn test_bounded_diameter_spanning_tree_evaluate_wrong_length() {
    let problem = example_instance();
    assert!(!problem.evaluate(&[0, 1, 0]));
    assert!(!problem.evaluate(&[0, 1, 0, 0, 1, 0, 0, 1]));
}

#[test]
fn test_bounded_diameter_spanning_tree_brute_force() {
    let problem = example_instance();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));
}

#[test]
fn test_bounded_diameter_spanning_tree_infeasible() {
    // Path graph 0-1-2-3-4, all weight 1, weight bound 10 but diameter bound 2
    // Only spanning tree is the path itself with diameter 4 > 2
    let problem = BoundedDiameterSpanningTree::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1, 1, 1, 1],
        10,
        2,
    );
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_bounded_diameter_spanning_tree_serialization() {
    let problem = example_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: BoundedDiameterSpanningTree<SimpleGraph, i32> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vertices(), 5);
    assert_eq!(deserialized.num_edges(), 7);
    assert_eq!(deserialized.weight_bound(), &5);
    assert_eq!(deserialized.diameter_bound(), 3);
}

#[test]
#[should_panic(expected = "diameter_bound must be at least 1")]
fn test_bounded_diameter_spanning_tree_zero_diameter_panics() {
    let _ = BoundedDiameterSpanningTree::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1, 1],
        5,
        0,
    );
}

#[test]
#[should_panic(expected = "edge_weights length must match num_edges")]
fn test_bounded_diameter_spanning_tree_wrong_weights_length_panics() {
    let _ =
        BoundedDiameterSpanningTree::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1], 5, 2);
}
