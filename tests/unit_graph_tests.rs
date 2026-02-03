//! Unit tests for graph problems.
//!
//! Tests extracted from source files for better compilation times
//! and clearer separation of concerns.

use problemreductions::models::graph::{
    is_independent_set, is_valid_coloring, is_vertex_cover, IndependentSet, KColoring,
    VertexCovering,
};
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

// =============================================================================
// Independent Set Tests
// =============================================================================

mod independent_set {
    use super::*;

    #[test]
    fn test_creation() {
        let problem = IndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        assert_eq!(problem.num_vertices(), 4);
        assert_eq!(problem.num_edges(), 3);
        assert_eq!(problem.num_variables(), 4);
        assert_eq!(problem.num_flavors(), 2);
    }

    #[test]
    fn test_with_weights() {
        let problem = IndependentSet::with_weights(3, vec![(0, 1)], vec![1, 2, 3]);
        assert_eq!(problem.weights(), vec![1, 2, 3]);
        assert!(problem.is_weighted());
    }

    #[test]
    fn test_unweighted() {
        let problem = IndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        assert!(!problem.is_weighted());
    }

    #[test]
    fn test_has_edge() {
        let problem = IndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        assert!(problem.has_edge(0, 1));
        assert!(problem.has_edge(1, 0)); // Undirected
        assert!(problem.has_edge(1, 2));
        assert!(!problem.has_edge(0, 2));
    }

    #[test]
    fn test_solution_size_valid() {
        let problem = IndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (2, 3)]);

        // Valid: select 0 and 2 (not adjacent)
        let sol = problem.solution_size(&[1, 0, 1, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 2);

        // Valid: select 1 and 3 (not adjacent)
        let sol = problem.solution_size(&[0, 1, 0, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 2);
    }

    #[test]
    fn test_solution_size_invalid() {
        let problem = IndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (2, 3)]);

        // Invalid: 0 and 1 are adjacent
        let sol = problem.solution_size(&[1, 1, 0, 0]);
        assert!(!sol.is_valid);
        assert_eq!(sol.size, 2);

        // Invalid: 2 and 3 are adjacent
        let sol = problem.solution_size(&[0, 0, 1, 1]);
        assert!(!sol.is_valid);
    }

    #[test]
    fn test_solution_size_empty() {
        let problem = IndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        let sol = problem.solution_size(&[0, 0, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 0);
    }

    #[test]
    fn test_weighted_solution() {
        let problem = IndependentSet::with_weights(3, vec![(0, 1)], vec![10, 20, 30]);

        // Select vertex 2 (weight 30)
        let sol = problem.solution_size(&[0, 0, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 30);

        // Select vertices 0 and 2 (weights 10 + 30 = 40)
        let sol = problem.solution_size(&[1, 0, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 40);
    }

    #[test]
    fn test_constraints() {
        let problem = IndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        let constraints = problem.constraints();
        assert_eq!(constraints.len(), 2); // One per edge
    }

    #[test]
    fn test_objectives() {
        let problem = IndependentSet::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
        let objectives = problem.objectives();
        assert_eq!(objectives.len(), 3); // One per vertex
    }

    #[test]
    fn test_brute_force_triangle() {
        // Triangle graph: maximum IS has size 1
        let problem = IndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // All solutions should have exactly 1 vertex selected
        assert_eq!(solutions.len(), 3); // Three equivalent solutions
        for sol in &solutions {
            assert_eq!(sol.iter().sum::<usize>(), 1);
        }
    }

    #[test]
    fn test_brute_force_path() {
        // Path graph 0-1-2-3: maximum IS = {0,2} or {1,3} or {0,3}
        let problem = IndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Maximum size is 2
        for sol in &solutions {
            let size: usize = sol.iter().sum();
            assert_eq!(size, 2);
            // Verify it's valid
            let sol_result = problem.solution_size(sol);
            assert!(sol_result.is_valid);
        }
    }

    #[test]
    fn test_brute_force_weighted() {
        // Graph with weights: vertex 1 has high weight but is connected to both 0 and 2
        let problem = IndependentSet::with_weights(3, vec![(0, 1), (1, 2)], vec![1, 100, 1]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        assert_eq!(solutions.len(), 1);
        // Should select vertex 1 (weight 100) over vertices 0+2 (weight 2)
        assert_eq!(solutions[0], vec![0, 1, 0]);
    }

    #[test]
    fn test_is_independent_set_function() {
        assert!(is_independent_set(3, &[(0, 1)], &[true, false, true]));
        assert!(is_independent_set(3, &[(0, 1)], &[false, true, true]));
        assert!(!is_independent_set(3, &[(0, 1)], &[true, true, false]));
        assert!(is_independent_set(
            3,
            &[(0, 1), (1, 2)],
            &[true, false, true]
        ));
        assert!(!is_independent_set(
            3,
            &[(0, 1), (1, 2)],
            &[false, true, true]
        ));
    }

    #[test]
    fn test_problem_size() {
        let problem = IndependentSet::<SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2), (2, 3)]);
        let size = problem.problem_size();
        assert_eq!(size.get("num_vertices"), Some(5));
        assert_eq!(size.get("num_edges"), Some(3));
    }

    #[test]
    fn test_energy_mode() {
        let problem = IndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        assert!(problem.energy_mode().is_maximization());
    }

    #[test]
    fn test_edges() {
        let problem = IndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (2, 3)]);
        let edges = problem.edges();
        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&(0, 1)) || edges.contains(&(1, 0)));
        assert!(edges.contains(&(2, 3)) || edges.contains(&(3, 2)));
    }

    #[test]
    fn test_set_weights() {
        let mut problem = IndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        problem.set_weights(vec![5, 10, 15]);
        assert_eq!(problem.weights(), vec![5, 10, 15]);
    }

    #[test]
    fn test_empty_graph() {
        let problem = IndependentSet::<SimpleGraph, i32>::new(3, vec![]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        assert_eq!(solutions.len(), 1);
        // All vertices can be selected
        assert_eq!(solutions[0], vec![1, 1, 1]);
    }

    #[test]
    fn test_is_satisfied() {
        let problem = IndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        assert!(problem.is_satisfied(&[1, 0, 1])); // Valid IS
        assert!(problem.is_satisfied(&[0, 1, 0])); // Valid IS
        assert!(!problem.is_satisfied(&[1, 1, 0])); // Invalid: 0-1 adjacent
        assert!(!problem.is_satisfied(&[0, 1, 1])); // Invalid: 1-2 adjacent
    }
}

// =============================================================================
// Vertex Covering Tests
// =============================================================================

mod vertex_covering {
    use super::*;

    #[test]
    fn test_creation() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        assert_eq!(problem.num_vertices(), 4);
        assert_eq!(problem.num_edges(), 3);
        assert_eq!(problem.num_variables(), 4);
        assert_eq!(problem.num_flavors(), 2);
    }

    #[test]
    fn test_with_weights() {
        let problem = VertexCovering::with_weights(3, vec![(0, 1)], vec![1, 2, 3]);
        assert_eq!(problem.weights(), vec![1, 2, 3]);
        assert!(problem.is_weighted());
    }

    #[test]
    fn test_solution_size_valid() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        // Valid: select vertex 1 (covers both edges)
        let sol = problem.solution_size(&[0, 1, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 1);

        // Valid: select all vertices
        let sol = problem.solution_size(&[1, 1, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 3);
    }

    #[test]
    fn test_solution_size_invalid() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        // Invalid: no vertex selected
        let sol = problem.solution_size(&[0, 0, 0]);
        assert!(!sol.is_valid);

        // Invalid: only vertex 0 selected (edge 1-2 not covered)
        let sol = problem.solution_size(&[1, 0, 0]);
        assert!(!sol.is_valid);
    }

    #[test]
    fn test_brute_force_path() {
        // Path graph 0-1-2: minimum vertex cover is {1}
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![0, 1, 0]);
    }

    #[test]
    fn test_brute_force_triangle() {
        // Triangle: minimum vertex cover has size 2
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // There are 3 minimum covers of size 2
        assert_eq!(solutions.len(), 3);
        for sol in &solutions {
            assert_eq!(sol.iter().sum::<usize>(), 2);
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_brute_force_weighted() {
        // Weighted: prefer selecting low-weight vertices
        let problem = VertexCovering::with_weights(3, vec![(0, 1), (1, 2)], vec![100, 1, 100]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        assert_eq!(solutions.len(), 1);
        // Should select vertex 1 (weight 1) instead of 0 and 2 (total 200)
        assert_eq!(solutions[0], vec![0, 1, 0]);
    }

    #[test]
    fn test_is_vertex_cover_function() {
        assert!(is_vertex_cover(3, &[(0, 1), (1, 2)], &[false, true, false]));
        assert!(is_vertex_cover(3, &[(0, 1), (1, 2)], &[true, false, true]));
        assert!(!is_vertex_cover(
            3,
            &[(0, 1), (1, 2)],
            &[true, false, false]
        ));
        assert!(!is_vertex_cover(
            3,
            &[(0, 1), (1, 2)],
            &[false, false, false]
        ));
    }

    #[test]
    fn test_constraints() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        let constraints = problem.constraints();
        assert_eq!(constraints.len(), 2);
    }

    #[test]
    fn test_energy_mode() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        assert!(problem.energy_mode().is_minimization());
    }

    #[test]
    fn test_empty_graph() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // No edges means empty cover is valid and optimal
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![0, 0, 0]);
    }

    #[test]
    fn test_single_edge() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(2, vec![(0, 1)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Either vertex covers the single edge
        assert_eq!(solutions.len(), 2);
    }

    #[test]
    fn test_is_satisfied() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        assert!(problem.is_satisfied(&[0, 1, 0])); // Valid cover
        assert!(problem.is_satisfied(&[1, 0, 1])); // Valid cover
        assert!(!problem.is_satisfied(&[1, 0, 0])); // Edge 1-2 uncovered
        assert!(!problem.is_satisfied(&[0, 0, 1])); // Edge 0-1 uncovered
    }

    #[test]
    fn test_complement_relationship() {
        // For a graph, if S is an independent set, then V\S is a vertex cover
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let is_problem = IndependentSet::<SimpleGraph, i32>::new(4, edges.clone());
        let vc_problem = VertexCovering::<SimpleGraph, i32>::new(4, edges);

        let solver = BruteForce::new();

        let is_solutions = solver.find_best(&is_problem);
        for is_sol in &is_solutions {
            // Complement should be a valid vertex cover
            let vc_config: Vec<usize> = is_sol.iter().map(|&x| 1 - x).collect();
            assert!(vc_problem.solution_size(&vc_config).is_valid);
        }
    }

    #[test]
    fn test_objectives() {
        let problem = VertexCovering::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
        let objectives = problem.objectives();
        assert_eq!(objectives.len(), 3);
    }

    #[test]
    fn test_set_weights() {
        let mut problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        assert!(!problem.is_weighted()); // Initially uniform
        problem.set_weights(vec![1, 2, 3]);
        assert!(problem.is_weighted());
        assert_eq!(problem.weights(), vec![1, 2, 3]);
    }

    #[test]
    fn test_is_weighted_empty() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(0, vec![]);
        assert!(!problem.is_weighted());
    }

    #[test]
    fn test_is_vertex_cover_wrong_len() {
        // Wrong length should return false
        assert!(!is_vertex_cover(3, &[(0, 1)], &[true, false]));
    }
}

// =============================================================================
// KColoring Tests
// =============================================================================

mod kcoloring {
    use super::*;

    #[test]
    fn test_creation() {
        let problem = KColoring::<3, SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        assert_eq!(problem.num_vertices(), 4);
        assert_eq!(problem.num_edges(), 3);
        assert_eq!(problem.num_colors(), 3);
        assert_eq!(problem.num_variables(), 4);
        assert_eq!(problem.num_flavors(), 3);
    }

    #[test]
    fn test_solution_size_valid() {
        let problem = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        // Valid: different colors on adjacent vertices
        let sol = problem.solution_size(&[0, 1, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 0);

        let sol = problem.solution_size(&[0, 1, 2]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 0);
    }

    #[test]
    fn test_solution_size_invalid() {
        let problem = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        // Invalid: adjacent vertices have same color
        let sol = problem.solution_size(&[0, 0, 1]);
        assert!(!sol.is_valid);
        assert_eq!(sol.size, 1); // 1 conflict

        let sol = problem.solution_size(&[0, 0, 0]);
        assert!(!sol.is_valid);
        assert_eq!(sol.size, 2); // 2 conflicts
    }

    #[test]
    fn test_brute_force_path() {
        // Path graph can be 2-colored
        let problem = KColoring::<2, SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // All solutions should be valid (0 conflicts)
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_brute_force_triangle() {
        // Triangle needs 3 colors
        let problem = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
            // All three vertices have different colors
            assert_ne!(sol[0], sol[1]);
            assert_ne!(sol[1], sol[2]);
            assert_ne!(sol[0], sol[2]);
        }
    }

    #[test]
    fn test_triangle_2_colors() {
        // Triangle cannot be 2-colored
        let problem = KColoring::<2, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Best we can do is 1 conflict
        for sol in &solutions {
            assert!(!problem.solution_size(sol).is_valid);
            assert_eq!(problem.solution_size(sol).size, 1);
        }
    }

    #[test]
    fn test_constraints() {
        let problem = KColoring::<2, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        let constraints = problem.constraints();
        assert_eq!(constraints.len(), 2); // One per edge
    }

    #[test]
    fn test_energy_mode() {
        let problem = KColoring::<2, SimpleGraph, i32>::new(2, vec![(0, 1)]);
        assert!(problem.energy_mode().is_minimization());
    }

    #[test]
    fn test_is_valid_coloring_function() {
        let edges = vec![(0, 1), (1, 2)];

        assert!(is_valid_coloring(3, &edges, &[0, 1, 0], 2));
        assert!(is_valid_coloring(3, &edges, &[0, 1, 2], 3));
        assert!(!is_valid_coloring(3, &edges, &[0, 0, 1], 2)); // 0-1 conflict
        assert!(!is_valid_coloring(3, &edges, &[0, 1, 1], 2)); // 1-2 conflict
        assert!(!is_valid_coloring(3, &edges, &[0, 1], 2)); // Wrong length
        assert!(!is_valid_coloring(3, &edges, &[0, 2, 0], 2)); // Color out of range
    }

    #[test]
    fn test_empty_graph() {
        let problem = KColoring::<1, SimpleGraph, i32>::new(3, vec![]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Any coloring is valid when there are no edges
        assert!(problem.solution_size(&solutions[0]).is_valid);
    }

    #[test]
    fn test_complete_graph_k4() {
        // K4 needs 4 colors
        let problem = KColoring::<4, SimpleGraph, i32>::new(
            4,
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
        );
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_is_satisfied() {
        let problem = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        assert!(problem.is_satisfied(&[0, 1, 0]));
        assert!(problem.is_satisfied(&[0, 1, 2]));
        assert!(!problem.is_satisfied(&[0, 0, 1]));
    }

    #[test]
    fn test_problem_size() {
        let problem = KColoring::<3, SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2)]);
        let size = problem.problem_size();
        assert_eq!(size.get("num_vertices"), Some(5));
        assert_eq!(size.get("num_edges"), Some(2));
        assert_eq!(size.get("num_colors"), Some(3));
    }

    #[test]
    fn test_csp_methods() {
        let problem = KColoring::<2, SimpleGraph, i32>::new(3, vec![(0, 1)]);

        // KColoring has no objectives (pure CSP)
        let objectives = problem.objectives();
        assert!(objectives.is_empty());

        // KColoring has no weights
        let weights: Vec<i32> = problem.weights();
        assert!(weights.is_empty());

        // is_weighted should return false
        assert!(!problem.is_weighted());
    }

    #[test]
    fn test_set_weights() {
        let mut problem = KColoring::<2, SimpleGraph, i32>::new(3, vec![(0, 1)]);
        // set_weights does nothing for KColoring
        problem.set_weights(vec![1, 2, 3]);
        assert!(!problem.is_weighted());
    }
}
