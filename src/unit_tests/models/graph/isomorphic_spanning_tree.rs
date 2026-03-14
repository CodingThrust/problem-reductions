use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_isomorphicspanningtree_basic() {
    // Triangle graph, path tree
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let tree = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = IsomorphicSpanningTree::new(graph, tree);

    assert_eq!(problem.dims(), vec![3, 3, 3]);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_graph_edges(), 3);
    assert_eq!(problem.num_tree_edges(), 2);
    assert_eq!(IsomorphicSpanningTree::NAME, "IsomorphicSpanningTree");
}

#[test]
fn test_isomorphicspanningtree_evaluation_yes() {
    // Host graph: 0-1, 1-2, 0-2 (triangle)
    // Tree: 0-1, 1-2 (path)
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let tree = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = IsomorphicSpanningTree::new(graph, tree);

    // Identity mapping: π = [0, 1, 2]
    // Tree edges: (0,1) -> (0,1) ✓, (1,2) -> (1,2) ✓
    assert!(problem.evaluate(&[0, 1, 2]));

    // Reversed: π = [2, 1, 0]
    // Tree edges: (0,1) -> (2,1) ✓, (1,2) -> (1,0) ✓
    assert!(problem.evaluate(&[2, 1, 0]));
}

#[test]
fn test_isomorphicspanningtree_evaluation_no() {
    // Host graph: path 0-1-2 (no edge 0-2)
    // Tree: star with center 1: edges (0,1), (1,2) -- this is also a path, same structure
    // Actually let's make a case where it fails:
    // Host graph: 0-1, 2-3 (disconnected, 2 components)
    // But wait, the tree must span, so let's use a connected graph where the tree doesn't fit.

    // Host graph: path 0-1-2-3 (edges: 0-1, 1-2, 2-3)
    // Tree: star K_{1,3} center=0, leaves=1,2,3 (edges: 0-1, 0-2, 0-3)
    // No vertex in graph has degree 3, so no valid mapping exists
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let tree = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]);
    let problem = IsomorphicSpanningTree::new(graph, tree);

    // No permutation should work
    assert!(!problem.evaluate(&[0, 1, 2, 3]));
    assert!(!problem.evaluate(&[1, 0, 2, 3]));
    assert!(!problem.evaluate(&[2, 1, 0, 3]));
}

#[test]
fn test_isomorphicspanningtree_invalid_configs() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let tree = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = IsomorphicSpanningTree::new(graph, tree);

    // Not a permutation: repeated value
    assert!(!problem.evaluate(&[0, 0, 1]));
    // Out of range
    assert!(!problem.evaluate(&[0, 1, 3]));
    // Wrong length
    assert!(!problem.evaluate(&[0, 1]));
}

#[test]
fn test_isomorphicspanningtree_solver_yes() {
    // Complete graph K4, any tree with 4 vertices should have a solution
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let tree = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]); // path
    let problem = IsomorphicSpanningTree::new(graph, tree);

    let solver = BruteForce::new();
    let sol = solver.find_satisfying(&problem);
    assert!(sol.is_some());
    assert!(problem.evaluate(&sol.unwrap()));

    // All satisfying solutions should be valid
    let all = solver.find_all_satisfying(&problem);
    assert!(!all.is_empty());
    for s in &all {
        assert!(problem.evaluate(s));
    }
}

#[test]
fn test_isomorphicspanningtree_solver_no() {
    // Path graph 0-1-2-3, star tree K_{1,3}
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let tree = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]);
    let problem = IsomorphicSpanningTree::new(graph, tree);

    let solver = BruteForce::new();
    let sol = solver.find_satisfying(&problem);
    assert!(sol.is_none());

    let all = solver.find_all_satisfying(&problem);
    assert!(all.is_empty());
}

#[test]
fn test_isomorphicspanningtree_serialization() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let tree = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = IsomorphicSpanningTree::new(graph, tree);

    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: IsomorphicSpanningTree = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.num_vertices(), 3);
    assert_eq!(deserialized.num_graph_edges(), 3);
    assert_eq!(deserialized.num_tree_edges(), 2);
    // Verify same evaluation
    assert!(deserialized.evaluate(&[0, 1, 2]));
}

#[test]
fn test_isomorphicspanningtree_caterpillar_example() {
    // Example from the issue: 7-vertex graph with caterpillar tree
    let graph = SimpleGraph::new(
        7,
        vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 2),
            (1, 4),
            (2, 3),
            (2, 5),
            (3, 6),
            (4, 5),
            (4, 6),
            (5, 6),
            (1, 3),
        ],
    );
    // Caterpillar tree: a-b, b-c, c-d, d-e, b-f, c-g
    // Using vertex indices: 0-1, 1-2, 2-3, 3-4, 1-5, 2-6
    let tree = SimpleGraph::new(7, vec![(0, 1), (1, 2), (2, 3), (3, 4), (1, 5), (2, 6)]);
    let problem = IsomorphicSpanningTree::new(graph, tree);

    // The issue gives solution: a→0, b→1, c→2, d→3, e→6, f→4, g→5
    // As config: π = [0, 1, 2, 3, 6, 4, 5]
    assert!(problem.evaluate(&[0, 1, 2, 3, 6, 4, 5]));
}

#[test]
fn test_isomorphicspanningtree_variant() {
    assert!(IsomorphicSpanningTree::variant().is_empty());
}

#[test]
#[should_panic(expected = "graph and tree must have the same number of vertices")]
fn test_isomorphicspanningtree_mismatched_sizes() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let tree = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    IsomorphicSpanningTree::new(graph, tree);
}

#[test]
#[should_panic(expected = "tree must have exactly n-1 edges")]
fn test_isomorphicspanningtree_not_a_tree() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    // Not a tree: 3 edges for 3 vertices (has a cycle)
    let tree = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    IsomorphicSpanningTree::new(graph, tree);
}
