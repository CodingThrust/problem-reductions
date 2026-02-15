//! Integration tests for the problemreductions crate.
//!
//! These tests verify that all problem types work correctly with the
//! BruteForce solver and that related problems have consistent solutions.

use problemreductions::models::graph::*;
use problemreductions::models::optimization::*;
use problemreductions::models::satisfiability::*;
use problemreductions::models::set::*;
use problemreductions::models::specialized::*;
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

/// Test that all problem types can be instantiated and solved.
mod all_problems_solvable {
    use super::*;

    #[test]
    fn test_independent_set_solvable() {
        let problem =
            MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.evaluate(sol).is_valid());
        }
    }

    #[test]
    fn test_vertex_covering_solvable() {
        let problem = MinimumVertexCover::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.evaluate(sol).is_valid());
        }
    }

    #[test]
    fn test_max_cut_solvable() {
        let problem = MaxCut::<SimpleGraph, i32>::new(4, vec![(0, 1, 1), (1, 2, 2), (2, 3, 1)]);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);
        assert!(!solutions.is_empty());
    }

    #[test]
    fn test_coloring_solvable() {
        let problem = KColoring::<K3, SimpleGraph>::new(3, vec![(0, 1), (1, 2)]);
        let solver = BruteForce::new();
        // KColoring returns bool, so we can use find_all_satisfying
        let satisfying = solver.find_all_satisfying(&problem);
        assert!(!satisfying.is_empty());
        for sol in &satisfying {
            assert!(problem.evaluate(sol));
        }
    }

    #[test]
    fn test_dominating_set_solvable() {
        let problem =
            MinimumDominatingSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.evaluate(sol).is_valid());
        }
    }

    #[test]
    fn test_maximal_is_solvable() {
        let problem = MaximalIS::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.evaluate(sol).is_valid());
        }
    }

    #[test]
    fn test_matching_solvable() {
        let problem =
            MaximumMatching::<SimpleGraph, i32>::new(4, vec![(0, 1, 1), (1, 2, 2), (2, 3, 1)]);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.evaluate(sol).is_valid());
        }
    }

    #[test]
    fn test_satisfiability_solvable() {
        let problem = Satisfiability::new(
            3,
            vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 3])],
        );
        // Satisfiability returns bool, find satisfying configs manually
        let dims = problem.dims();
        let all_configs: Vec<Vec<usize>> =
            problemreductions::config::DimsIterator::new(dims.clone()).collect();
        let satisfying: Vec<Vec<usize>> = all_configs
            .into_iter()
            .filter(|config| problem.evaluate(config))
            .collect();
        assert!(!satisfying.is_empty());
        for sol in &satisfying {
            assert!(problem.evaluate(sol));
        }
    }

    #[test]
    fn test_spin_glass_solvable() {
        let problem = SpinGlass::new(3, vec![((0, 1), -1.0), ((1, 2), 1.0)], vec![0.5, -0.5, 0.0]);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);
        assert!(!solutions.is_empty());
    }

    #[test]
    fn test_qubo_solvable() {
        let problem = QUBO::from_matrix(vec![
            vec![1.0, -2.0, 0.0],
            vec![0.0, 1.0, -1.0],
            vec![0.0, 0.0, 1.0],
        ]);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);
        assert!(!solutions.is_empty());
    }

    #[test]
    fn test_set_covering_solvable() {
        let problem =
            MinimumSetCovering::<i32>::new(5, vec![vec![0, 1, 2], vec![2, 3, 4], vec![0, 4]]);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.evaluate(sol).is_valid());
        }
    }

    #[test]
    fn test_set_packing_solvable() {
        let problem =
            MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![2, 3], vec![1, 2], vec![4]]);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.evaluate(sol).is_valid());
        }
    }

    #[test]
    fn test_circuit_sat_solvable() {
        let circuit = Circuit::new(vec![Assignment::new(
            vec!["c".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
        )]);
        let problem = CircuitSAT::new(circuit);
        // CircuitSAT returns bool
        let dims = problem.dims();
        let all_configs: Vec<Vec<usize>> =
            problemreductions::config::DimsIterator::new(dims.clone()).collect();
        let satisfying: Vec<Vec<usize>> = all_configs
            .into_iter()
            .filter(|config| problem.evaluate(config))
            .collect();
        assert!(!satisfying.is_empty());
        for sol in &satisfying {
            assert!(problem.evaluate(sol));
        }
    }

    #[test]
    fn test_factoring_solvable() {
        let problem = Factoring::new(15, 2, 2);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.evaluate(sol).is_valid());
        }
    }

    #[test]
    fn test_paintshop_solvable() {
        let problem = PaintShop::new(vec!["a", "b", "a", "b"]);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);
        assert!(!solutions.is_empty());
    }

    #[test]
    fn test_biclique_cover_solvable() {
        // Left vertices: 0, 1; Right vertices: 2, 3
        let problem = BicliqueCover::new(2, 2, vec![(0, 2), (0, 3), (1, 2), (1, 3)], 1);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.evaluate(sol).is_valid());
        }
    }

    #[test]
    fn test_bmf_solvable() {
        let problem = BMF::new(vec![vec![true, true], vec![true, true]], 1);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            // BMF minimizes Hamming distance, all configs are valid (no invalid marker)
            let _ = problem.evaluate(sol);
        }
    }
}

/// Tests verifying relationships between related problems.
mod problem_relationships {
    use super::*;

    /// Independent Set and Vertex Cover are complements on the same graph.
    /// For any graph, IS size + VC size = n (number of vertices).
    #[test]
    fn test_independent_set_vertex_cover_complement() {
        let edges = vec![(0, 1), (1, 2), (2, 3), (0, 3)];
        let n = 4;

        let is_problem = MaximumIndependentSet::<SimpleGraph, i32>::new(n, edges.clone());
        let vc_problem = MinimumVertexCover::<SimpleGraph, i32>::new(n, edges);

        let solver = BruteForce::new();
        let is_solutions = solver.find_all_best(&is_problem);
        let vc_solutions = solver.find_all_best(&vc_problem);

        let max_is_size = is_solutions[0].iter().sum::<usize>();
        let min_vc_size = vc_solutions[0].iter().sum::<usize>();

        // IS complement is a valid VC and vice versa
        assert_eq!(max_is_size + min_vc_size, n);
    }

    /// MaximalIS solutions are a subset of MaximumIndependentSet solutions (valid IS).
    #[test]
    fn test_maximal_is_is_independent_set() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let n = 4;

        let maximal_is = MaximalIS::<SimpleGraph, i32>::new(n, edges.clone());
        let is_problem = MaximumIndependentSet::<SimpleGraph, i32>::new(n, edges);

        let solver = BruteForce::new();
        let maximal_solutions = solver.find_all_best(&maximal_is);

        // Every maximal IS is also a valid IS
        for sol in &maximal_solutions {
            assert!(is_problem.evaluate(sol).is_valid());
        }
    }

    /// SAT clauses with all positive literals have the all-true assignment as solution.
    #[test]
    fn test_sat_positive_clauses() {
        let problem = Satisfiability::new(
            3,
            vec![
                CNFClause::new(vec![1, 2]),
                CNFClause::new(vec![2, 3]),
                CNFClause::new(vec![1, 3]),
            ],
        );

        // All true should satisfy
        let all_true = vec![1, 1, 1];
        assert!(problem.evaluate(&all_true));
    }

    /// SpinGlass with all ferromagnetic (negative J) interactions prefers aligned spins.
    #[test]
    fn test_spin_glass_ferromagnetic() {
        // All negative J -> spins want to align
        let problem = SpinGlass::new(
            3,
            vec![((0, 1), -1.0), ((1, 2), -1.0), ((0, 2), -1.0)],
            vec![0.0, 0.0, 0.0],
        );

        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);

        // Optimal should be all same spin (all 0 or all 1)
        for sol in &solutions {
            let all_same = sol.iter().all(|&s| s == sol[0]);
            assert!(
                all_same,
                "Ferromagnetic ground state should have aligned spins"
            );
        }
    }

    /// MinimumSetCovering and MaximumSetPacking on disjoint sets.
    #[test]
    fn test_set_covering_packing_disjoint() {
        // Three disjoint sets covering universe {0,1,2,3,4,5}
        let sets = vec![vec![0, 1], vec![2, 3], vec![4, 5]];

        let covering = MinimumSetCovering::<i32>::new(6, sets.clone());
        let packing = MaximumSetPacking::<i32>::new(sets);

        let solver = BruteForce::new();

        // All sets needed for cover
        let cover_solutions = solver.find_all_best(&covering);
        assert_eq!(cover_solutions[0].iter().sum::<usize>(), 3);

        // All sets can be packed (no overlap)
        let pack_solutions = solver.find_all_best(&packing);
        assert_eq!(pack_solutions[0].iter().sum::<usize>(), 3);
    }
}

/// Tests for edge cases and boundary conditions.
mod edge_cases {
    use super::*;

    #[test]
    fn test_empty_graph_independent_set() {
        let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![]);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);

        // All vertices can be in IS when no edges
        assert_eq!(solutions[0].iter().sum::<usize>(), 3);
    }

    #[test]
    fn test_complete_graph_independent_set() {
        // K4 - complete graph on 4 vertices
        let edges = vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
        let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(4, edges);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);

        // Maximum IS in complete graph is 1
        assert_eq!(solutions[0].iter().sum::<usize>(), 1);
    }

    #[test]
    fn test_single_clause_sat() {
        let problem = Satisfiability::new(2, vec![CNFClause::new(vec![1, -2])]);
        // Find satisfying configs
        let dims = problem.dims();
        let all_configs: Vec<Vec<usize>> =
            problemreductions::config::DimsIterator::new(dims.clone()).collect();
        let satisfying: Vec<Vec<usize>> = all_configs
            .into_iter()
            .filter(|config| problem.evaluate(config))
            .collect();

        // (x1 OR NOT x2) is satisfied by 3 of 4 assignments
        assert!(!satisfying.is_empty());
        for sol in &satisfying {
            assert!(problem.evaluate(sol));
        }
    }

    #[test]
    fn test_trivial_factoring() {
        // Factor 4 = 2 * 2
        let problem = Factoring::new(4, 2, 2);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);

        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.evaluate(sol).is_valid());
        }
    }

    #[test]
    fn test_single_car_paintshop() {
        let problem = PaintShop::new(vec!["a", "a"]);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);

        // Single car always has 1 switch (color must change)
        assert_eq!(problem.count_switches(&solutions[0]), 1);
    }
}

/// Tests for weighted problems.
mod weighted_problems {
    use super::*;

    #[test]
    fn test_weighted_independent_set() {
        let problem = MaximumIndependentSet::<SimpleGraph, i32>::with_weights(
            3,
            vec![(0, 1)],
            vec![10, 1, 1],
        );

        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);

        // Should prefer vertex 0 (weight 10) over vertex 1 (weight 1)
        // Optimal: {0, 2} with weight 11
        let best_weight: i32 = solutions[0]
            .iter()
            .enumerate()
            .map(|(i, &s)| if s == 1 { problem.weights()[i] } else { 0 })
            .sum();
        assert_eq!(best_weight, 11);
    }

    #[test]
    fn test_weighted_vertex_cover() {
        let problem = MinimumVertexCover::<SimpleGraph, i32>::with_weights(
            3,
            vec![(0, 1), (1, 2)],
            vec![1, 10, 1],
        );

        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);

        // Prefer {0, 2} over {1} because {0,2} has weight 2 vs {1} has weight 10
        let best_weight: i32 = solutions[0]
            .iter()
            .enumerate()
            .map(|(i, &s)| if s == 1 { problem.weights()[i] } else { 0 })
            .sum();
        assert_eq!(best_weight, 2);
    }

    #[test]
    fn test_weighted_max_cut() {
        let problem = MaxCut::new(3, vec![(0, 1, 10), (1, 2, 1)]);
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&problem);

        // Maximum cut should include the heavy edge (0,1)
        let cut_value = problem.evaluate(&solutions[0]);
        // cut_value should be >= 10
        assert!(cut_value.is_valid() && cut_value.unwrap() >= 10);
    }

    #[test]
    fn test_unsatisfiable_sat() {
        // This formula is unsatisfiable: x1 AND NOT x1
        let problem = Satisfiability::new(
            2,
            vec![
                CNFClause::new(vec![1]),  // x1
                CNFClause::new(vec![-1]), // NOT x1
            ],
        );

        // Find satisfying configs
        let dims = problem.dims();
        let all_configs: Vec<Vec<usize>> =
            problemreductions::config::DimsIterator::new(dims.clone()).collect();
        let satisfying: Vec<Vec<usize>> = all_configs
            .into_iter()
            .filter(|config| problem.evaluate(config))
            .collect();

        // Can't satisfy both - no solution satisfies all clauses
        assert!(satisfying.is_empty());
    }
}
