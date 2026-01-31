//! Testing utilities and macros for problem implementations.
//!
//! This module provides macros and helpers to reduce test boilerplate by ~90%
//! when implementing new problems. Instead of writing 300+ lines of tests per
//! problem, you can use these macros to generate comprehensive test suites.
//!
//! # Macros
//!
//! ## `graph_problem_tests!`
//!
//! Generates a complete test suite for graph problems:
//!
//! ```rust,ignore
//! use problemreductions::graph_problem_tests;
//!
//! graph_problem_tests! {
//!     problem_type: IndependentSetT<i32>,
//!     constraint_type: IndependentSetConstraint,
//!     test_cases: [
//!         // (name, num_vertices, edges, valid_solution, expected_size, is_maximization)
//!         (triangle, 3, [(0, 1), (1, 2), (0, 2)], [1, 0, 0], 1, true),
//!         (path, 3, [(0, 1), (1, 2)], [1, 0, 1], 2, true),
//!     ]
//! }
//! ```
//!
//! This generates tests for:
//! - Problem creation and metadata
//! - Solution validity and size computation
//! - Energy mode (maximization vs minimization)
//! - CSP interface (constraints, objectives)
//! - Brute force solving (for small instances)
//!
//! ## `complement_test!`
//!
//! Tests that two problems are complements (e.g., IS + VC = n):
//!
//! ```rust,ignore
//! use problemreductions::complement_test;
//!
//! complement_test! {
//!     name: test_is_vc_complement,
//!     problem_a: IndependentSetT<i32>,
//!     problem_b: VertexCoverT<i32>,
//!     test_graphs: [
//!         (3, [(0, 1), (1, 2)]),
//!         (4, [(0, 1), (1, 2), (2, 3)]),
//!     ]
//! }
//! ```
//!
//! ## `quick_problem_test!`
//!
//! Quick single-instance validation:
//!
//! ```rust,ignore
//! use problemreductions::quick_problem_test;
//!
//! quick_problem_test!(
//!     IndependentSetT<i32>,
//!     new(3, vec![(0, 1)]),
//!     solution: [0, 0, 1],
//!     expected_size: 1,
//!     is_valid: true
//! );
//! ```
//!
//! # Test Case Types
//!
//! - [`GraphTestCase`] - Structured test case for graph problems
//! - [`SatTestCase`] - Structured test case for SAT problems

#[macro_use]
mod macros;

/// A test case for a graph problem.
#[derive(Debug, Clone)]
pub struct GraphTestCase<W> {
    /// Number of vertices.
    pub num_vertices: usize,
    /// Edge list.
    pub edges: Vec<(usize, usize)>,
    /// Vertex weights (if any).
    pub weights: Option<Vec<W>>,
    /// A known valid solution.
    pub valid_solution: Vec<usize>,
    /// The expected objective value for the valid solution.
    pub expected_size: W,
    /// The optimal objective value (for brute force testing).
    pub optimal_size: Option<W>,
}

impl<W: Clone> GraphTestCase<W> {
    /// Create a new test case with unit weights.
    pub fn new(
        num_vertices: usize,
        edges: Vec<(usize, usize)>,
        valid_solution: Vec<usize>,
        expected_size: W,
    ) -> Self {
        Self {
            num_vertices,
            edges,
            weights: None,
            valid_solution,
            expected_size,
            optimal_size: None,
        }
    }

    /// Create a new test case with custom weights.
    pub fn with_weights(
        num_vertices: usize,
        edges: Vec<(usize, usize)>,
        weights: Vec<W>,
        valid_solution: Vec<usize>,
        expected_size: W,
    ) -> Self {
        Self {
            num_vertices,
            edges,
            weights: Some(weights),
            valid_solution,
            expected_size,
            optimal_size: None,
        }
    }

    /// Set the optimal objective value.
    pub fn with_optimal(mut self, optimal: W) -> Self {
        self.optimal_size = Some(optimal);
        self
    }
}

/// A test case for a SAT problem.
#[derive(Debug, Clone)]
pub struct SatTestCase {
    /// Number of variables.
    pub num_vars: usize,
    /// Clauses as lists of literals (positive = true, negative = negated).
    pub clauses: Vec<Vec<i32>>,
    /// A known satisfying assignment (if satisfiable).
    pub satisfying_assignment: Option<Vec<usize>>,
    /// Whether the formula is satisfiable.
    pub is_satisfiable: bool,
}

impl SatTestCase {
    /// Create a satisfiable test case.
    pub fn satisfiable(
        num_vars: usize,
        clauses: Vec<Vec<i32>>,
        satisfying_assignment: Vec<usize>,
    ) -> Self {
        Self {
            num_vars,
            clauses,
            satisfying_assignment: Some(satisfying_assignment),
            is_satisfiable: true,
        }
    }

    /// Create an unsatisfiable test case.
    pub fn unsatisfiable(num_vars: usize, clauses: Vec<Vec<i32>>) -> Self {
        Self {
            num_vars,
            clauses,
            satisfying_assignment: None,
            is_satisfiable: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_test_case() {
        let case = GraphTestCase::new(3, vec![(0, 1), (1, 2)], vec![1, 0, 1], 2);
        assert_eq!(case.num_vertices, 3);
        assert_eq!(case.edges.len(), 2);
        assert!(case.weights.is_none());
        assert!(case.optimal_size.is_none());
    }

    #[test]
    fn test_graph_test_case_with_weights() {
        let case = GraphTestCase::with_weights(3, vec![(0, 1)], vec![1, 2, 3], vec![0, 0, 1], 3);
        assert!(case.weights.is_some());
        assert_eq!(case.weights.as_ref().unwrap(), &vec![1, 2, 3]);
    }

    #[test]
    fn test_graph_test_case_with_optimal() {
        let case = GraphTestCase::new(3, vec![(0, 1)], vec![0, 0, 1], 1).with_optimal(2);
        assert_eq!(case.optimal_size, Some(2));
    }

    #[test]
    fn test_sat_test_case_satisfiable() {
        let case = SatTestCase::satisfiable(2, vec![vec![1, 2], vec![-1]], vec![0, 1]);
        assert!(case.is_satisfiable);
        assert!(case.satisfying_assignment.is_some());
    }

    #[test]
    fn test_sat_test_case_unsatisfiable() {
        let case = SatTestCase::unsatisfiable(1, vec![vec![1], vec![-1]]);
        assert!(!case.is_satisfiable);
        assert!(case.satisfying_assignment.is_none());
    }
}
