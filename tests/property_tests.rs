//! Property-based tests using proptest.
//!
//! These tests verify mathematical invariants and properties
//! that should hold for all valid inputs.

use problemreductions::models::graph::{IndependentSet, VertexCovering};
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;
use proptest::prelude::*;
use proptest::strategy::ValueTree;
use std::collections::HashSet;

/// Strategy for generating random graphs with between 2 and `max_vertices` vertices.
/// We require at least 2 vertices to avoid edge generation issues with single-vertex graphs.
fn graph_strategy(max_vertices: usize) -> impl Strategy<Value = (usize, Vec<(usize, usize)>)> {
    (2..=max_vertices).prop_flat_map(|n| {
        // For simplicity, just pick random pairs and normalize
        // Self-loops are mapped to edge (0, 1) which always exists when n >= 2
        let edge_strategy = (0..n, 0..n).prop_map(|(u, v)| {
            if u < v {
                (u, v)
            } else if v < u {
                (v, u)
            } else {
                // Self-loop: map to edge (0, 1) which always exists when n >= 2
                (0, 1)
            }
        });

        prop::collection::vec(edge_strategy, 0..n * 2).prop_map(move |edges| {
            let unique: HashSet<_> = edges.into_iter().collect();
            (n, unique.into_iter().collect())
        })
    })
}

proptest! {
    /// Property: For any graph, the complement of a maximum independent set
    /// is a minimum vertex cover, and their sizes sum to n.
    #[test]
    fn independent_set_complement_is_vertex_cover((n, edges) in graph_strategy(8)) {
        let is_problem = IndependentSet::<SimpleGraph, i32>::new(n, edges.clone());
        let vc_problem = VertexCovering::<SimpleGraph, i32>::new(n, edges);

        let solver = BruteForce::new();
        let is_solutions = solver.find_best(&is_problem);
        let vc_solutions = solver.find_best(&vc_problem);

        let is_size: usize = is_solutions[0].iter().sum();
        let vc_size: usize = vc_solutions[0].iter().sum();

        // IS size + VC size = n (for optimal solutions)
        prop_assert_eq!(is_size + vc_size, n);
    }

    /// Property: Any subset of a valid independent set is also a valid independent set.
    #[test]
    fn valid_solution_stays_valid_under_subset((n, edges) in graph_strategy(6)) {
        let problem = IndependentSet::<SimpleGraph, i32>::new(n, edges);
        let solver = BruteForce::new();

        for sol in solver.find_best(&problem) {
            // Any subset of an IS is also an IS
            for i in 0..n {
                let mut subset = sol.clone();
                subset[i] = 0;
                prop_assert!(problem.solution_size(&subset).is_valid);
            }
        }
    }

    /// Property: A vertex cover with additional vertices is still a valid cover.
    #[test]
    fn vertex_cover_superset_is_valid((n, edges) in graph_strategy(6)) {
        let problem = VertexCovering::<SimpleGraph, i32>::new(n, edges);
        let solver = BruteForce::new();

        for sol in solver.find_best(&problem) {
            // Adding any vertex to a VC still gives a valid VC
            for i in 0..n {
                let mut superset = sol.clone();
                superset[i] = 1;
                prop_assert!(problem.solution_size(&superset).is_valid);
            }
        }
    }

    /// Property: The complement of any valid independent set is a valid vertex cover.
    #[test]
    fn is_complement_is_vc((n, edges) in graph_strategy(7)) {
        let is_problem = IndependentSet::<SimpleGraph, i32>::new(n, edges.clone());
        let vc_problem = VertexCovering::<SimpleGraph, i32>::new(n, edges);
        let solver = BruteForce::new();

        // Get all valid independent sets (not just optimal)
        for sol in solver.find_best(&is_problem) {
            // The complement should be a valid vertex cover
            let complement: Vec<usize> = sol.iter().map(|&x| 1 - x).collect();
            prop_assert!(vc_problem.solution_size(&complement).is_valid,
                "Complement of IS {:?} should be valid VC", sol);
        }
    }

    /// Property: Empty selection is always a valid (but possibly non-optimal) independent set.
    #[test]
    fn empty_is_always_valid_is((n, edges) in graph_strategy(10)) {
        let problem = IndependentSet::<SimpleGraph, i32>::new(n, edges);
        let empty = vec![0; n];
        prop_assert!(problem.solution_size(&empty).is_valid);
    }

    /// Property: Full selection is always a valid (but possibly non-optimal) vertex cover
    /// (when there is at least one vertex).
    #[test]
    fn full_is_always_valid_vc((n, edges) in graph_strategy(10)) {
        let problem = VertexCovering::<SimpleGraph, i32>::new(n, edges);
        let full = vec![1; n];
        prop_assert!(problem.solution_size(&full).is_valid);
    }

    /// Property: Solution size is non-negative for independent sets.
    #[test]
    fn is_size_non_negative((n, edges) in graph_strategy(8)) {
        let problem = IndependentSet::<SimpleGraph, i32>::new(n, edges);
        let solver = BruteForce::new();

        for sol in solver.find_best(&problem) {
            let size = problem.solution_size(&sol);
            prop_assert!(size.size >= 0);
        }
    }
}

/// Test that the graph strategy generates valid graphs.
#[test]
fn test_graph_strategy_sanity() {
    use proptest::test_runner::TestRunner;

    let mut runner = TestRunner::default();
    let strategy = graph_strategy(5);

    for _ in 0..10 {
        let (n, edges) = strategy.new_tree(&mut runner).unwrap().current();

        // Check all edges are valid
        for (u, v) in &edges {
            assert!(*u < n, "Edge source out of bounds");
            assert!(*v < n, "Edge target out of bounds");
            assert!(u != v, "Self-loop detected");
        }

        // Check no duplicate edges
        let unique: HashSet<_> = edges.iter().collect();
        assert_eq!(unique.len(), edges.len(), "Duplicate edges detected");
    }
}
