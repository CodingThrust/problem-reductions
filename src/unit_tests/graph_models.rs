//! Unit tests for graph problems.
//!
//! Tests extracted from source files for better compilation times
//! and clearer separation of concerns.

use crate::models::graph::{
    is_independent_set, is_valid_coloring, is_vertex_cover, KColoring, MaximumIndependentSet,
    MinimumVertexCover,
};
use crate::prelude::*;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use crate::variant::{K1, K2, K3, K4};

// =============================================================================
// Independent Set Tests
// =============================================================================

mod maximum_independent_set {
    use super::*;

    #[test]
    fn test_creation() {
        let problem = MaximumIndependentSet::new(
            SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
            vec![1i32; 4],
        );
        assert_eq!(problem.graph().num_vertices(), 4);
        assert_eq!(problem.graph().num_edges(), 3);
        assert_eq!(problem.num_variables(), 4);
    }

    #[test]
    fn test_with_weights() {
        let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1, 2, 3]);
        assert_eq!(problem.weights().to_vec(), vec![1, 2, 3]);
        assert!(problem.is_weighted());
    }

    #[test]
    fn test_unweighted() {
        // i32 type is always considered weighted, even with uniform values
        let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
        assert!(problem.is_weighted());
    }

    #[test]
    fn test_has_edge() {
        let problem =
            MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
        assert!(problem.graph().has_edge(0, 1));
        assert!(problem.graph().has_edge(1, 0)); // Undirected
        assert!(problem.graph().has_edge(1, 2));
        assert!(!problem.graph().has_edge(0, 2));
    }

    #[test]
    fn test_evaluate_valid() {
        let problem =
            MaximumIndependentSet::new(SimpleGraph::new(4, vec![(0, 1), (2, 3)]), vec![1i32; 4]);

        // Valid: select 0 and 2 (not adjacent)
        assert_eq!(problem.evaluate(&[1, 0, 1, 0]), SolutionSize::Valid(2));

        // Valid: select 1 and 3 (not adjacent)
        assert_eq!(problem.evaluate(&[0, 1, 0, 1]), SolutionSize::Valid(2));
    }

    #[test]
    fn test_evaluate_invalid() {
        let problem =
            MaximumIndependentSet::new(SimpleGraph::new(4, vec![(0, 1), (2, 3)]), vec![1i32; 4]);

        // Invalid: 0 and 1 are adjacent - returns Invalid
        assert_eq!(problem.evaluate(&[1, 1, 0, 0]), SolutionSize::Invalid);

        // Invalid: 2 and 3 are adjacent
        assert_eq!(problem.evaluate(&[0, 0, 1, 1]), SolutionSize::Invalid);
    }

    #[test]
    fn test_evaluate_empty() {
        let problem =
            MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
        // Empty selection is valid with size 0
        assert_eq!(problem.evaluate(&[0, 0, 0]), SolutionSize::Valid(0));
    }

    #[test]
    fn test_evaluate_weighted() {
        let problem =
            MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![10, 20, 30]);

        // Select vertex 2 (weight 30)
        assert_eq!(problem.evaluate(&[0, 0, 1]), SolutionSize::Valid(30));

        // Select vertices 0 and 2 (weights 10 + 30 = 40)
        assert_eq!(problem.evaluate(&[1, 0, 1]), SolutionSize::Valid(40));
    }

    #[test]
    fn test_brute_force_triangle() {
        // Triangle graph: maximum IS has size 1
        let problem = MaximumIndependentSet::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
            vec![1i32; 3],
        );
        let solver = BruteForce::new();

        let solutions = solver.find_all_best(&problem);
        // All solutions should have exactly 1 vertex selected
        assert_eq!(solutions.len(), 3); // Three equivalent solutions
        for sol in &solutions {
            assert_eq!(sol.iter().sum::<usize>(), 1);
        }
    }

    #[test]
    fn test_brute_force_path() {
        // Path graph 0-1-2-3: maximum IS = {0,2} or {1,3} or {0,3}
        let problem = MaximumIndependentSet::new(
            SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
            vec![1i32; 4],
        );
        let solver = BruteForce::new();

        let solutions = solver.find_all_best(&problem);
        // Maximum size is 2
        for sol in &solutions {
            let size: usize = sol.iter().sum();
            assert_eq!(size, 2);
            // Verify it's valid (evaluate returns Valid, not Invalid)
            assert_eq!(problem.evaluate(sol), SolutionSize::Valid(2));
        }
    }

    #[test]
    fn test_brute_force_weighted() {
        // Graph with weights: vertex 1 has high weight but is connected to both 0 and 2
        let problem =
            MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1, 100, 1]);
        let solver = BruteForce::new();

        let solutions = solver.find_all_best(&problem);
        assert_eq!(solutions.len(), 1);
        // Should select vertex 1 (weight 100) over vertices 0+2 (weight 2)
        assert_eq!(solutions[0], vec![0, 1, 0]);
    }

    #[test]
    fn test_is_independent_set_function() {
        assert!(is_independent_set(
            &SimpleGraph::new(3, vec![(0, 1)]),
            &[true, false, true]
        ));
        assert!(is_independent_set(
            &SimpleGraph::new(3, vec![(0, 1)]),
            &[false, true, true]
        ));
        assert!(!is_independent_set(
            &SimpleGraph::new(3, vec![(0, 1)]),
            &[true, true, false]
        ));
        assert!(is_independent_set(
            &SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
            &[true, false, true]
        ));
        assert!(!is_independent_set(
            &SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
            &[false, true, true]
        ));
    }

    #[test]
    fn test_direction() {
        let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
        assert_eq!(problem.direction(), Direction::Maximize);
    }

    #[test]
    fn test_edges() {
        let problem =
            MaximumIndependentSet::new(SimpleGraph::new(4, vec![(0, 1), (2, 3)]), vec![1i32; 4]);
        let edges = problem.graph().edges();
        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&(0, 1)) || edges.contains(&(1, 0)));
        assert!(edges.contains(&(2, 3)) || edges.contains(&(3, 2)));
    }

    #[test]
    fn test_with_custom_weights() {
        let problem =
            MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![5, 10, 15]);
        assert_eq!(problem.weights().to_vec(), vec![5, 10, 15]);
    }

    #[test]
    fn test_empty_graph() {
        let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![]), vec![1i32; 3]);
        let solver = BruteForce::new();

        let solutions = solver.find_all_best(&problem);
        assert_eq!(solutions.len(), 1);
        // All vertices can be selected
        assert_eq!(solutions[0], vec![1, 1, 1]);
    }

    #[test]
    fn test_validity_via_evaluate() {
        let problem =
            MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);

        // Valid IS configurations return is_valid() == true
        assert!(problem.evaluate(&[1, 0, 1]).is_valid());
        assert!(problem.evaluate(&[0, 1, 0]).is_valid());
        // Invalid configurations return Invalid
        assert_eq!(problem.evaluate(&[1, 1, 0]), SolutionSize::Invalid);
        assert_eq!(problem.evaluate(&[0, 1, 1]), SolutionSize::Invalid);
    }
}

// =============================================================================
// Vertex Covering Tests
// =============================================================================

mod minimum_vertex_cover {
    use super::*;

    #[test]
    fn test_creation() {
        let problem = MinimumVertexCover::new(
            SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
            vec![1i32; 4],
        );
        assert_eq!(problem.graph().num_vertices(), 4);
        assert_eq!(problem.graph().num_edges(), 3);
        assert_eq!(problem.num_variables(), 4);
    }

    #[test]
    fn test_with_weights() {
        let problem = MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1, 2, 3]);
        assert_eq!(problem.weights().to_vec(), vec![1, 2, 3]);
        assert!(problem.is_weighted());
    }

    #[test]
    fn test_evaluate_valid() {
        let problem =
            MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);

        // Valid: select vertex 1 (covers both edges)
        assert_eq!(problem.evaluate(&[0, 1, 0]), SolutionSize::Valid(1));

        // Valid: select all vertices
        assert_eq!(problem.evaluate(&[1, 1, 1]), SolutionSize::Valid(3));
    }

    #[test]
    fn test_evaluate_invalid() {
        let problem =
            MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);

        // Invalid: no vertex selected - returns Invalid for minimization
        assert_eq!(problem.evaluate(&[0, 0, 0]), SolutionSize::Invalid);

        // Invalid: only vertex 0 selected (edge 1-2 not covered)
        assert_eq!(problem.evaluate(&[1, 0, 0]), SolutionSize::Invalid);
    }

    #[test]
    fn test_brute_force_path() {
        // Path graph 0-1-2: minimum vertex cover is {1}
        let problem =
            MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
        let solver = BruteForce::new();

        let solutions = solver.find_all_best(&problem);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![0, 1, 0]);
    }

    #[test]
    fn test_brute_force_triangle() {
        // Triangle: minimum vertex cover has size 2
        let problem = MinimumVertexCover::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
            vec![1i32; 3],
        );
        let solver = BruteForce::new();

        let solutions = solver.find_all_best(&problem);
        // There are 3 minimum covers of size 2
        assert_eq!(solutions.len(), 3);
        for sol in &solutions {
            assert_eq!(sol.iter().sum::<usize>(), 2);
            // Verify valid (not Invalid)
            assert!(problem.evaluate(sol).is_valid());
        }
    }

    #[test]
    fn test_brute_force_weighted() {
        // Weighted: prefer selecting low-weight vertices
        let problem =
            MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![100, 1, 100]);
        let solver = BruteForce::new();

        let solutions = solver.find_all_best(&problem);
        assert_eq!(solutions.len(), 1);
        // Should select vertex 1 (weight 1) instead of 0 and 2 (total 200)
        assert_eq!(solutions[0], vec![0, 1, 0]);
    }

    #[test]
    fn test_is_vertex_cover_function() {
        assert!(is_vertex_cover(
            &SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
            &[false, true, false]
        ));
        assert!(is_vertex_cover(
            &SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
            &[true, false, true]
        ));
        assert!(!is_vertex_cover(
            &SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
            &[true, false, false]
        ));
        assert!(!is_vertex_cover(
            &SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
            &[false, false, false]
        ));
    }

    #[test]
    fn test_direction() {
        let problem = MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
        assert_eq!(problem.direction(), Direction::Minimize);
    }

    #[test]
    fn test_empty_graph() {
        let problem = MinimumVertexCover::new(SimpleGraph::new(3, vec![]), vec![1i32; 3]);
        let solver = BruteForce::new();

        let solutions = solver.find_all_best(&problem);
        // No edges means empty cover is valid and optimal
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![0, 0, 0]);
    }

    #[test]
    fn test_single_edge() {
        let problem = MinimumVertexCover::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i32; 2]);
        let solver = BruteForce::new();

        let solutions = solver.find_all_best(&problem);
        // Either vertex covers the single edge
        assert_eq!(solutions.len(), 2);
    }

    #[test]
    fn test_validity_via_evaluate() {
        let problem =
            MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);

        // Valid cover configurations return is_valid() == true
        assert!(problem.evaluate(&[0, 1, 0]).is_valid());
        assert!(problem.evaluate(&[1, 0, 1]).is_valid());
        // Invalid configurations return Invalid
        assert_eq!(problem.evaluate(&[1, 0, 0]), SolutionSize::Invalid);
        assert_eq!(problem.evaluate(&[0, 0, 1]), SolutionSize::Invalid);
    }

    #[test]
    fn test_complement_relationship() {
        // For a graph, if S is an independent set, then V\S is a vertex cover
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let is_problem =
            MaximumIndependentSet::new(SimpleGraph::new(4, edges.clone()), vec![1i32; 4]);
        let vc_problem = MinimumVertexCover::new(SimpleGraph::new(4, edges), vec![1i32; 4]);

        let solver = BruteForce::new();

        let is_solutions = solver.find_all_best(&is_problem);
        for is_sol in &is_solutions {
            // Complement should be a valid vertex cover
            let vc_config: Vec<usize> = is_sol.iter().map(|&x| 1 - x).collect();
            // Valid cover returns is_valid() == true
            assert!(vc_problem.evaluate(&vc_config).is_valid());
        }
    }

    #[test]
    fn test_with_custom_weights() {
        let problem = MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1, 2, 3]);
        assert!(problem.is_weighted());
        assert_eq!(problem.weights().to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_is_weighted_empty() {
        // i32 type is always considered weighted, even with empty weights
        let problem = MinimumVertexCover::new(SimpleGraph::new(0, vec![]), vec![0i32; 0]);
        assert!(problem.is_weighted());
    }

    #[test]
    #[should_panic(expected = "selected length must match num_vertices")]
    fn test_is_vertex_cover_wrong_len() {
        // Wrong length should panic
        is_vertex_cover(&SimpleGraph::new(3, vec![(0, 1)]), &[true, false]);
    }
}

// =============================================================================
// KColoring Tests
// =============================================================================

mod kcoloring {
    use super::*;

    #[test]
    fn test_creation() {
        let problem = KColoring::<K3, _>::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
        assert_eq!(problem.graph().num_vertices(), 4);
        assert_eq!(problem.graph().num_edges(), 3);
        assert_eq!(problem.num_colors(), 3);
        assert_eq!(problem.num_variables(), 4);
    }

    #[test]
    fn test_evaluate_valid() {
        let problem = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));

        // Valid: different colors on adjacent vertices - returns true
        assert!(problem.evaluate(&[0, 1, 0]));
        assert!(problem.evaluate(&[0, 1, 2]));
    }

    #[test]
    fn test_evaluate_invalid() {
        let problem = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));

        // Invalid: adjacent vertices have same color
        assert!(!problem.evaluate(&[0, 0, 1])); // 0-1 conflict
        assert!(!problem.evaluate(&[0, 0, 0])); // Multiple conflicts
    }

    #[test]
    fn test_brute_force_path() {
        // Path graph can be 2-colored
        let problem = KColoring::<K2, _>::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
        let solver = BruteForce::new();

        let solutions = solver.find_all_satisfying(&problem);
        // All solutions should be valid
        for sol in &solutions {
            assert!(problem.evaluate(sol));
        }
    }

    #[test]
    fn test_brute_force_triangle() {
        // Triangle needs 3 colors
        let problem = KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
        let solver = BruteForce::new();

        let solutions = solver.find_all_satisfying(&problem);
        for sol in &solutions {
            assert!(problem.evaluate(sol));
            // All three vertices have different colors
            assert_ne!(sol[0], sol[1]);
            assert_ne!(sol[1], sol[2]);
            assert_ne!(sol[0], sol[2]);
        }
    }

    #[test]
    fn test_triangle_2_colors_unsat() {
        // Triangle cannot be 2-colored
        let problem = KColoring::<K2, _>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
        let solver = BruteForce::new();

        // No satisfying assignments
        let solution = solver.find_satisfying(&problem);
        assert!(solution.is_none());
    }

    #[test]
    fn test_is_valid_coloring_function() {
        let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);

        assert!(is_valid_coloring(&graph, &[0, 1, 0], 2));
        assert!(is_valid_coloring(&graph, &[0, 1, 2], 3));
        assert!(!is_valid_coloring(&graph, &[0, 0, 1], 2)); // 0-1 conflict
        assert!(!is_valid_coloring(&graph, &[0, 1, 1], 2)); // 1-2 conflict
        assert!(!is_valid_coloring(&graph, &[0, 2, 0], 2)); // Color out of range
    }

    #[test]
    #[should_panic(expected = "coloring length must match num_vertices")]
    fn test_is_valid_coloring_wrong_len() {
        let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
        is_valid_coloring(&graph, &[0, 1], 2); // Wrong length
    }

    #[test]
    fn test_empty_graph() {
        let problem = KColoring::<K1, _>::new(SimpleGraph::new(3, vec![]));
        let solver = BruteForce::new();

        let solutions = solver.find_all_satisfying(&problem);
        // Any coloring is valid when there are no edges
        assert!(problem.evaluate(&solutions[0]));
    }

    #[test]
    fn test_complete_graph_k4() {
        // K4 needs 4 colors
        let problem = KColoring::<K4, _>::new(SimpleGraph::new(
            4,
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
        ));
        let solver = BruteForce::new();

        let solutions = solver.find_all_satisfying(&problem);
        for sol in &solutions {
            assert!(problem.evaluate(sol));
        }
    }
}
