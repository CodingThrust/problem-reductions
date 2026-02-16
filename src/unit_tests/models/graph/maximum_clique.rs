use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::types::SolutionSize;

#[test]
fn test_clique_creation() {
    use crate::traits::Problem;

    let problem = MaximumClique::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32; 4],
    );
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 3);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2]);
}

#[test]
fn test_clique_with_weights() {
    let problem = MaximumClique::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1, 2, 3]);
    assert_eq!(problem.weights().to_vec(), vec![1, 2, 3]);
    assert!(problem.is_weighted());
}

#[test]
fn test_clique_unweighted() {
    // i32 type is always considered weighted, even with uniform values
    let problem = MaximumClique::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    assert!(problem.is_weighted());
}

#[test]
fn test_has_edge() {
    let problem = MaximumClique::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
    assert!(problem.graph().has_edge(0, 1));
    assert!(problem.graph().has_edge(1, 0)); // Undirected
    assert!(problem.graph().has_edge(1, 2));
    assert!(!problem.graph().has_edge(0, 2));
}

#[test]
fn test_evaluate_valid() {
    use crate::traits::Problem;

    // Complete graph K3 (triangle)
    let problem = MaximumClique::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32; 3],
    );

    // Valid: all three form a clique
    assert_eq!(problem.evaluate(&[1, 1, 1]), SolutionSize::Valid(3));

    // Valid: any pair
    assert_eq!(problem.evaluate(&[1, 1, 0]), SolutionSize::Valid(2));
}

#[test]
fn test_evaluate_invalid() {
    use crate::traits::Problem;

    // Path graph: 0-1-2 (no edge between 0 and 2)
    let problem = MaximumClique::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);

    // Invalid: 0 and 2 are not adjacent - returns Invalid
    assert_eq!(problem.evaluate(&[1, 0, 1]), SolutionSize::Invalid);

    // Invalid: all three selected but not a clique
    assert_eq!(problem.evaluate(&[1, 1, 1]), SolutionSize::Invalid);
}

#[test]
fn test_evaluate_empty() {
    use crate::traits::Problem;

    let problem = MaximumClique::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
    // Empty set is a valid clique with size 0
    assert_eq!(problem.evaluate(&[0, 0, 0]), SolutionSize::Valid(0));
}

#[test]
fn test_weighted_solution() {
    use crate::traits::Problem;

    let problem = MaximumClique::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![10, 20, 30],
    );

    // Select vertex 2 (weight 30)
    assert_eq!(problem.evaluate(&[0, 0, 1]), SolutionSize::Valid(30));

    // Select all three (weights 10 + 20 + 30 = 60)
    assert_eq!(problem.evaluate(&[1, 1, 1]), SolutionSize::Valid(60));
}

#[test]
fn test_brute_force_triangle() {
    // Triangle graph (K3): max clique is all 3 vertices
    let problem = MaximumClique::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32; 3],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 1, 1]);
}

#[test]
fn test_brute_force_path() {
    use crate::traits::Problem;

    // Path graph 0-1-2: max clique is any adjacent pair
    let problem = MaximumClique::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Maximum size is 2
    for sol in &solutions {
        let size: usize = sol.iter().sum();
        assert_eq!(size, 2);
        // Verify it's valid
        assert!(problem.evaluate(sol).is_valid());
    }
}

#[test]
fn test_brute_force_weighted() {
    use crate::traits::Problem;

    // Path with weights: vertex 1 has high weight
    let problem = MaximumClique::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1, 100, 1]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Should select {0, 1} (weight 101) or {1, 2} (weight 101)
    assert!(solutions.len() == 2);
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), SolutionSize::Valid(101));
    }
}

#[test]
fn test_is_clique_function() {
    // Triangle
    assert!(is_clique(
        &SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        &[true, true, true]
    ));
    assert!(is_clique(
        &SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        &[true, true, false]
    ));

    // Path - not all pairs adjacent
    assert!(!is_clique(
        &SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        &[true, false, true]
    ));
    assert!(is_clique(
        &SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        &[true, true, false]
    )); // Adjacent pair
}

#[test]
fn test_direction() {
    use crate::traits::OptimizationProblem;
    use crate::types::Direction;

    let problem = MaximumClique::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    assert_eq!(problem.direction(), Direction::Maximize);
}

#[test]
fn test_edges() {
    let problem = MaximumClique::new(SimpleGraph::new(4, vec![(0, 1), (2, 3)]), vec![1i32; 4]);
    let edges = problem.graph().edges();
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_empty_graph() {
    // No edges means any single vertex is a max clique
    let problem = MaximumClique::new(SimpleGraph::new(3, vec![]), vec![1i32; 3]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    assert_eq!(solutions.len(), 3);
    // Each solution should have exactly one vertex selected
    for sol in &solutions {
        assert_eq!(sol.iter().sum::<usize>(), 1);
    }
}

#[test]
fn test_is_clique_method() {
    use crate::traits::Problem;

    let problem = MaximumClique::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);

    // Valid clique - returns Valid
    assert!(problem.evaluate(&[1, 1, 0]).is_valid());
    assert!(problem.evaluate(&[0, 1, 1]).is_valid());
    // Invalid: 0-2 not adjacent - returns Invalid
    assert_eq!(problem.evaluate(&[1, 0, 1]), SolutionSize::Invalid);
}

#[test]
fn test_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumClique::new(graph, vec![1, 2, 3]);
    assert_eq!(problem.graph().num_vertices(), 3);
    assert_eq!(problem.weights().to_vec(), vec![1, 2, 3]);
}

#[test]
fn test_graph_accessor() {
    let problem = MaximumClique::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    let graph = problem.graph();
    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.num_edges(), 1);
}

#[test]
fn test_weights_ref() {
    let problem = MaximumClique::new(SimpleGraph::new(3, vec![(0, 1)]), vec![5, 10, 15]);
    assert_eq!(problem.weights(), &[5, 10, 15]);
}

#[test]
#[should_panic(expected = "selected length must match num_vertices")]
fn test_is_clique_wrong_len() {
    // Wrong length should panic
    is_clique(&SimpleGraph::new(3, vec![(0, 1)]), &[true, false]);
}

#[test]
fn test_complete_graph() {
    // K4 - complete graph with 4 vertices
    let problem = MaximumClique::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        vec![1i32; 4],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 1, 1, 1]); // All vertices form a clique
}

#[test]
fn test_clique_problem() {
    use crate::traits::{OptimizationProblem, Problem};
    use crate::types::Direction;

    // Triangle graph: all pairs connected
    let p = MaximumClique::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32; 3],
    );
    assert_eq!(p.dims(), vec![2, 2, 2]);
    // Valid clique: select all 3 vertices (triangle is a clique)
    assert_eq!(p.evaluate(&[1, 1, 1]), SolutionSize::Valid(3));
    // Valid clique: select just vertex 0
    assert_eq!(p.evaluate(&[1, 0, 0]), SolutionSize::Valid(1));
    assert_eq!(p.direction(), Direction::Maximize);
}

#[test]
fn test_is_valid_solution() {
    // Triangle: 0-1-2 all connected
    let problem = MaximumClique::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32; 3],
    );
    // Valid: all three form a clique
    assert!(problem.is_valid_solution(&[1, 1, 1]));
    // Now path graph: 0-1-2 (no 0-2 edge)
    let problem2 = MaximumClique::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
    // Invalid: {0, 2} not adjacent
    assert!(!problem2.is_valid_solution(&[1, 0, 1]));
}
