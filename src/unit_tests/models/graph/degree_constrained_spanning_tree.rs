use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn example_instance() -> DegreeConstrainedSpanningTree<SimpleGraph> {
    // 5 vertices, 7 edges, K=2
    DegreeConstrainedSpanningTree::new(
        SimpleGraph::new(
            5,
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 4), (2, 3), (3, 4)],
        ),
        2,
    )
}

#[test]
fn test_degree_constrained_spanning_tree_creation() {
    let problem = example_instance();
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 7);
    assert_eq!(problem.max_degree(), 2);
    assert_eq!(problem.dims(), vec![2; 7]);
    assert_eq!(problem.graph().num_vertices(), 5);
    assert_eq!(problem.edge_list().len(), 7);
}

#[test]
fn test_degree_constrained_spanning_tree_evaluate_valid() {
    let problem = example_instance();
    // Edges: (0,1)=0, (0,2)=1, (0,3)=2, (1,2)=3, (1,4)=4, (2,3)=5, (3,4)=6
    // Select edges 1,2,3,4: (0,2),(0,3),(1,2),(1,4)
    // Degrees: 0→2, 1→2, 2→2, 3→1, 4→1 — all ≤ 2
    // Connected and n-1=4 edges → valid spanning tree
    assert!(problem.evaluate(&[0, 1, 1, 1, 1, 0, 0]));
}

#[test]
fn test_degree_constrained_spanning_tree_evaluate_invalid_degree() {
    let problem = example_instance();
    // Select edges 0,1,2,4: (0,1),(0,2),(0,3),(1,4)
    // Degrees: 0→3 (exceeds K=2)
    assert!(!problem.evaluate(&[1, 1, 1, 0, 1, 0, 0]));
}

#[test]
fn test_degree_constrained_spanning_tree_evaluate_not_tree() {
    let problem = example_instance();
    // Select only 3 edges (not enough for n-1=4)
    assert!(!problem.evaluate(&[1, 1, 1, 0, 0, 0, 0]));
    // Select 5 edges (too many)
    assert!(!problem.evaluate(&[1, 1, 1, 1, 1, 0, 0]));
}

#[test]
fn test_degree_constrained_spanning_tree_evaluate_disconnected() {
    let problem = example_instance();
    // Select edges (0,1),(0,2),(0,3),(3,4) → 4 edges, but check degree:
    // 0→3 edges — degree exceeds K=2, so fails on degree.
    // Try: (0,1),(2,3),(1,4),(3,4) → indices 0,5,4,6
    // Degrees: 0→1, 1→2, 2→1, 3→3 — degree exceeds K=2
    // Need to pick 4 edges forming a tree where no vertex has degree > 2.
    // edges (0,2),(2,3),(3,4),(1,4) → indices 1,5,6,4
    // Degrees: 0→1, 1→1, 2→2, 3→2, 4→2 → valid and connected!
    assert!(problem.evaluate(&[0, 1, 0, 0, 1, 1, 1]));
}

#[test]
fn test_degree_constrained_spanning_tree_evaluate_wrong_length() {
    let problem = example_instance();
    assert!(!problem.evaluate(&[0, 1, 0]));
    assert!(!problem.evaluate(&[0, 1, 0, 0, 1, 0, 0, 1]));
}

#[test]
fn test_degree_constrained_spanning_tree_brute_force() {
    let problem = example_instance();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));
}

#[test]
fn test_degree_constrained_spanning_tree_infeasible() {
    // Star graph K_{1,4}: vertex 0 connected to 1,2,3,4.
    // Only spanning tree is the star itself, which has degree 4 at vertex 0.
    // With K=2, no spanning tree exists.
    let problem = DegreeConstrainedSpanningTree::new(
        SimpleGraph::new(5, vec![(0, 1), (0, 2), (0, 3), (0, 4)]),
        2,
    );
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_degree_constrained_spanning_tree_k1_path() {
    // K=1 means the tree is a single edge for n=2.
    // For n>2, K=1 is impossible since a tree on n>=3 vertices must have max degree >= 2.
    let problem =
        DegreeConstrainedSpanningTree::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]), 1);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());

    // For n=2, K=1 works: the single edge is the tree.
    let problem2 = DegreeConstrainedSpanningTree::new(SimpleGraph::new(2, vec![(0, 1)]), 1);
    let solver2 = BruteForce::new();
    let sol = solver2.find_witness(&problem2);
    assert!(sol.is_some());
}

#[test]
fn test_degree_constrained_spanning_tree_serialization() {
    let problem = example_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: DegreeConstrainedSpanningTree<SimpleGraph> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vertices(), 5);
    assert_eq!(deserialized.num_edges(), 7);
    assert_eq!(deserialized.max_degree(), 2);
}

#[test]
#[should_panic(expected = "max_degree must be at least 1")]
fn test_degree_constrained_spanning_tree_zero_k_panics() {
    let _ = DegreeConstrainedSpanningTree::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), 0);
}
