//! Integration tests for the problemreductions crate.
//!
//! These tests verify that all problem types work correctly with the
//! BruteForce solver and that related problems have consistent solutions.

use problemreductions::prelude::*;
use problemreductions::models::graph::*;
use problemreductions::models::optimization::*;
use problemreductions::models::satisfiability::*;
use problemreductions::models::set::*;
use problemreductions::models::specialized::*;

/// Test that all problem types can be instantiated and solved.
mod all_problems_solvable {
    use super::*;

    #[test]
    fn test_independent_set_solvable() {
        let problem = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_vertex_covering_solvable() {
        let problem = VertexCovering::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_max_cut_solvable() {
        let problem = MaxCut::<i32>::new(4, vec![(0, 1, 1), (1, 2, 2), (2, 3, 1)]);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        assert!(!solutions.is_empty());
    }

    #[test]
    fn test_coloring_solvable() {
        let problem = Coloring::new(3, 3, vec![(0, 1), (1, 2)]);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_dominating_set_solvable() {
        let problem = DominatingSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_maximal_is_solvable() {
        let problem = MaximalIS::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_matching_solvable() {
        let problem = Matching::<i32>::new(4, vec![(0, 1, 1), (1, 2, 2), (2, 3, 1)]);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_satisfiability_solvable() {
        let problem = Satisfiability::<i32>::new(
            3,
            vec![
                CNFClause::new(vec![1, 2]),
                CNFClause::new(vec![-1, 3]),
            ],
        );
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_spin_glass_solvable() {
        let problem = SpinGlass::new(3, vec![((0, 1), -1.0), ((1, 2), 1.0)], vec![0.5, -0.5, 0.0]);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
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
        let solutions = solver.find_best(&problem);
        assert!(!solutions.is_empty());
    }

    #[test]
    fn test_set_covering_solvable() {
        let problem = SetCovering::<i32>::new(
            5,
            vec![vec![0, 1, 2], vec![2, 3, 4], vec![0, 4]],
        );
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_set_packing_solvable() {
        let problem = SetPacking::<i32>::new(
            vec![vec![0, 1], vec![2, 3], vec![1, 2], vec![4]],
        );
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_circuit_sat_solvable() {
        let circuit = Circuit::new(vec![
            Assignment::new(
                vec!["c".to_string()],
                BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
            ),
        ]);
        let problem = CircuitSAT::<i32>::new(circuit);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_factoring_solvable() {
        let problem = Factoring::new(15, 2, 2);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_paintshop_solvable() {
        let problem = PaintShop::new(vec!["a", "b", "a", "b"]);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        assert!(!solutions.is_empty());
    }

    #[test]
    fn test_biclique_cover_solvable() {
        // Left vertices: 0, 1; Right vertices: 2, 3
        let problem = BicliqueCover::new(2, 2, vec![(0, 2), (0, 3), (1, 2), (1, 3)], 1);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_bmf_solvable() {
        let problem = BMF::new(vec![vec![true, true], vec![true, true]], 1);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
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

        let is_problem = IndependentSet::<i32>::new(n, edges.clone());
        let vc_problem = VertexCovering::<i32>::new(n, edges);

        let solver = BruteForce::new();
        let is_solutions = solver.find_best(&is_problem);
        let vc_solutions = solver.find_best(&vc_problem);

        let max_is_size = is_solutions[0].iter().sum::<usize>();
        let min_vc_size = vc_solutions[0].iter().sum::<usize>();

        // IS complement is a valid VC and vice versa
        assert_eq!(max_is_size + min_vc_size, n);
    }

    /// MaximalIS solutions are a subset of IndependentSet solutions (valid IS).
    #[test]
    fn test_maximal_is_is_independent_set() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let n = 4;

        let maximal_is = MaximalIS::new(n, edges.clone());
        let is_problem = IndependentSet::<i32>::new(n, edges);

        let solver = BruteForce::new();
        let maximal_solutions = solver.find_best(&maximal_is);

        // Every maximal IS is also a valid IS
        for sol in &maximal_solutions {
            assert!(is_problem.solution_size(sol).is_valid);
        }
    }

    /// SAT clauses with all positive literals have the all-true assignment as solution.
    #[test]
    fn test_sat_positive_clauses() {
        let problem = Satisfiability::<i32>::new(
            3,
            vec![
                CNFClause::new(vec![1, 2]),
                CNFClause::new(vec![2, 3]),
                CNFClause::new(vec![1, 3]),
            ],
        );

        // All true should satisfy
        let all_true = vec![1, 1, 1];
        assert!(problem.solution_size(&all_true).is_valid);
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
        let solutions = solver.find_best(&problem);

        // Optimal should be all same spin (all 0 or all 1)
        for sol in &solutions {
            let all_same = sol.iter().all(|&s| s == sol[0]);
            assert!(all_same, "Ferromagnetic ground state should have aligned spins");
        }
    }

    /// SetCovering and SetPacking on disjoint sets.
    #[test]
    fn test_set_covering_packing_disjoint() {
        // Three disjoint sets covering universe {0,1,2,3,4,5}
        let sets = vec![vec![0, 1], vec![2, 3], vec![4, 5]];

        let covering = SetCovering::<i32>::new(6, sets.clone());
        let packing = SetPacking::<i32>::new(sets);

        let solver = BruteForce::new();

        // All sets needed for cover
        let cover_solutions = solver.find_best(&covering);
        assert_eq!(cover_solutions[0].iter().sum::<usize>(), 3);

        // All sets can be packed (no overlap)
        let pack_solutions = solver.find_best(&packing);
        assert_eq!(pack_solutions[0].iter().sum::<usize>(), 3);
    }
}

/// Tests for edge cases and boundary conditions.
mod edge_cases {
    use super::*;

    #[test]
    fn test_empty_graph_independent_set() {
        let problem = IndependentSet::<i32>::new(3, vec![]);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);

        // All vertices can be in IS when no edges
        assert_eq!(solutions[0].iter().sum::<usize>(), 3);
    }

    #[test]
    fn test_complete_graph_independent_set() {
        // K4 - complete graph on 4 vertices
        let edges = vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
        let problem = IndependentSet::<i32>::new(4, edges);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);

        // Maximum IS in complete graph is 1
        assert_eq!(solutions[0].iter().sum::<usize>(), 1);
    }

    #[test]
    fn test_single_clause_sat() {
        let problem = Satisfiability::<i32>::new(2, vec![CNFClause::new(vec![1, -2])]);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);

        // (x1 OR NOT x2) is satisfied by 3 of 4 assignments
        assert!(solutions.len() >= 1);
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_trivial_factoring() {
        // Factor 4 = 2 * 2
        let problem = Factoring::new(4, 2, 2);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);

        assert!(!solutions.is_empty());
        for sol in &solutions {
            let sol_size = problem.solution_size(sol);
            assert!(sol_size.is_valid);
        }
    }

    #[test]
    fn test_single_car_paintshop() {
        let problem = PaintShop::new(vec!["a", "a"]);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);

        // Single car always has 1 switch (color must change)
        assert_eq!(problem.count_switches(&solutions[0]), 1);
    }
}

/// Tests for weighted problems.
mod weighted_problems {
    use super::*;

    #[test]
    fn test_weighted_independent_set() {
        let mut problem = IndependentSet::<i32>::new(3, vec![(0, 1)]);
        problem.set_weights(vec![10, 1, 1]);

        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);

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
        let mut problem = VertexCovering::<i32>::new(3, vec![(0, 1), (1, 2)]);
        problem.set_weights(vec![1, 10, 1]);

        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);

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
        let solutions = solver.find_best(&problem);

        // Maximum cut should include the heavy edge (0,1)
        let cut_value = problem.solution_size(&solutions[0]).size;
        assert!(cut_value >= 10);
    }

    #[test]
    fn test_weighted_sat() {
        let mut problem = Satisfiability::<i32>::new(
            2,
            vec![
                CNFClause::new(vec![1]),   // x1
                CNFClause::new(vec![-1]),  // NOT x1
            ],
        );
        problem.set_weights(vec![10, 1]);

        let solver = BruteForce::new().valid_only(false);
        let solutions = solver.find_best(&problem);

        // Can't satisfy both, but x1=true satisfies weight 10
        let best_weight = problem.solution_size(&solutions[0]).size;
        assert_eq!(best_weight, 10);
    }
}

/// Tests for Problem trait consistency.
mod trait_consistency {
    use super::*;

    fn check_problem_trait<P: Problem>(problem: &P, name: &str)
    where
        P::Size: std::fmt::Debug,
    {
        assert!(problem.num_variables() > 0 || name.contains("empty"), "{} should have variables", name);
        assert!(problem.num_flavors() >= 2, "{} should have at least 2 flavors", name);

        let size = problem.problem_size();
        // Check that problem_size returns some meaningful data
        assert!(size.get("num_vertices").is_some()
            || size.get("num_vars").is_some()
            || size.get("num_sets").is_some()
            || size.get("num_cars").is_some()
            || size.get("rows").is_some()
            || size.get("left_size").is_some()
            || size.get("target").is_some()
            || size.get("num_variables").is_some()
            || size.get("num_colors").is_some()
            || size.get("num_spins").is_some()
            || size.get("num_edges").is_some(),
            "{} problem_size should have meaningful data", name);
    }

    #[test]
    fn test_all_problems_implement_trait_correctly() {
        check_problem_trait(&IndependentSet::<i32>::new(3, vec![(0, 1)]), "IndependentSet");
        check_problem_trait(&VertexCovering::<i32>::new(3, vec![(0, 1)]), "VertexCovering");
        check_problem_trait(&MaxCut::<i32>::new(3, vec![(0, 1, 1)]), "MaxCut");
        check_problem_trait(&Coloring::new(3, 3, vec![(0, 1)]), "Coloring");
        check_problem_trait(&DominatingSet::<i32>::new(3, vec![(0, 1)]), "DominatingSet");
        check_problem_trait(&MaximalIS::new(3, vec![(0, 1)]), "MaximalIS");
        check_problem_trait(&Matching::<i32>::new(3, vec![(0, 1, 1)]), "Matching");
        check_problem_trait(&Satisfiability::<i32>::new(3, vec![CNFClause::new(vec![1])]), "SAT");
        check_problem_trait(&SpinGlass::new(3, vec![((0, 1), 1.0)], vec![0.0; 3]), "SpinGlass");
        check_problem_trait(&QUBO::from_matrix(vec![vec![1.0; 3]; 3]), "QUBO");
        check_problem_trait(&SetCovering::<i32>::new(3, vec![vec![0, 1]]), "SetCovering");
        check_problem_trait(&SetPacking::<i32>::new(vec![vec![0, 1]]), "SetPacking");
        check_problem_trait(&PaintShop::new(vec!["a", "a"]), "PaintShop");
        check_problem_trait(&BMF::new(vec![vec![true]], 1), "BMF");
        check_problem_trait(&BicliqueCover::new(2, 2, vec![(0, 2)], 1), "BicliqueCover");
        check_problem_trait(&Factoring::new(6, 2, 2), "Factoring");

        let circuit = Circuit::new(vec![Assignment::new(vec!["x".to_string()], BooleanExpr::constant(true))]);
        check_problem_trait(&CircuitSAT::<i32>::new(circuit), "CircuitSAT");
    }

    #[test]
    fn test_energy_modes() {
        // Minimization problems
        assert!(VertexCovering::<i32>::new(2, vec![(0, 1)]).energy_mode().is_minimization());
        assert!(DominatingSet::<i32>::new(2, vec![(0, 1)]).energy_mode().is_minimization());
        assert!(SetCovering::<i32>::new(2, vec![vec![0, 1]]).energy_mode().is_minimization());
        assert!(PaintShop::new(vec!["a", "a"]).energy_mode().is_minimization());
        assert!(QUBO::from_matrix(vec![vec![1.0]]).energy_mode().is_minimization());
        assert!(SpinGlass::new(1, vec![], vec![0.0]).energy_mode().is_minimization());
        assert!(BMF::new(vec![vec![true]], 1).energy_mode().is_minimization());
        assert!(Factoring::new(6, 2, 2).energy_mode().is_minimization());
        assert!(Coloring::new(2, 2, vec![(0, 1)]).energy_mode().is_minimization());
        assert!(BicliqueCover::new(2, 2, vec![(0, 2)], 1).energy_mode().is_minimization());

        // Maximization problems
        assert!(IndependentSet::<i32>::new(2, vec![(0, 1)]).energy_mode().is_maximization());
        assert!(MaximalIS::new(2, vec![(0, 1)]).energy_mode().is_maximization());
        assert!(MaxCut::<i32>::new(2, vec![(0, 1, 1)]).energy_mode().is_maximization());
        assert!(Matching::<i32>::new(2, vec![(0, 1, 1)]).energy_mode().is_maximization());
        assert!(SetPacking::<i32>::new(vec![vec![0]]).energy_mode().is_maximization());
        assert!(Satisfiability::<i32>::new(1, vec![CNFClause::new(vec![1])]).energy_mode().is_maximization());

        let circuit = Circuit::new(vec![]);
        assert!(CircuitSAT::<i32>::new(circuit).energy_mode().is_maximization());
    }
}
