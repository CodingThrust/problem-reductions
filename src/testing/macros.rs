//! Testing macros for problem implementations.

/// Generate standard tests for a graph problem using the template.
///
/// This macro generates tests for:
/// - Problem creation
/// - Solution validity
/// - Brute force solving (for small instances)
/// - CSP interface
/// - Metadata (if ProblemMetadata is implemented)
///
/// # Example
///
/// ```text
/// // Macro usage example - users customize for their tests
/// use problemreductions::graph_problem_tests;
/// use problemreductions::models::graph::{IndependentSetT, IndependentSetConstraint};
///
/// graph_problem_tests! {
///     problem_type: IndependentSetT,
///     constraint_type: IndependentSetConstraint,
///     test_cases: [
///         // (name, num_vertices, edges, valid_solution, expected_size, is_maximization)
///         (triangle, 3, [(0, 1), (1, 2), (0, 2)], [1, 0, 0], 1, true),
///         (path3, 3, [(0, 1), (1, 2)], [1, 0, 1], 2, true),
///     ]
/// }
/// ```
#[macro_export]
macro_rules! graph_problem_tests {
    (
        problem_type: $problem:ty,
        constraint_type: $constraint:ty,
        test_cases: [
            $(
                ($name:ident, $n:expr, [$($edge:expr),*], [$($sol:expr),*], $size:expr, $is_max:expr)
            ),* $(,)?
        ]
    ) => {
        mod generated_tests {
            use super::*;
            use $crate::prelude::*;
            use $crate::registry::ProblemMetadata;

            $(
                mod $name {
                    use super::*;

                    fn create_problem() -> $problem {
                        <$problem>::new($n, vec![$($edge),*])
                    }

                    #[test]
                    fn test_creation() {
                        let problem = create_problem();
                        assert_eq!(problem.num_variables(), $n);
                        assert_eq!(problem.num_flavors(), 2);
                    }

                    #[test]
                    fn test_solution_validity() {
                        let problem = create_problem();
                        let solution = vec![$($sol),*];
                        let result = problem.solution_size(&solution);
                        assert!(result.is_valid, "Solution should be valid");
                        assert_eq!(result.size, $size, "Solution size mismatch");
                    }

                    #[test]
                    fn test_energy_mode() {
                        let problem = create_problem();
                        if $is_max {
                            assert!(problem.energy_mode().is_maximization());
                        } else {
                            assert!(problem.energy_mode().is_minimization());
                        }
                    }

                    #[test]
                    fn test_csp_interface() {
                        let problem = create_problem();
                        let solution = vec![$($sol),*];

                        // Check constraints are generated
                        let constraints = problem.constraints();
                        let edge_count = vec![$($edge),*].len();
                        assert_eq!(constraints.len(), edge_count);

                        // Check objectives are generated
                        let objectives = problem.objectives();
                        assert_eq!(objectives.len(), $n);

                        // Check is_satisfied matches solution_size validity
                        assert_eq!(
                            problem.is_satisfied(&solution),
                            problem.solution_size(&solution).is_valid
                        );
                    }

                    #[test]
                    fn test_brute_force() {
                        if $n <= 15 {
                            let problem = create_problem();
                            let solver = BruteForce::new();
                            let solutions = solver.find_best(&problem);

                            // All solutions should be valid
                            for sol in &solutions {
                                assert!(problem.solution_size(sol).is_valid);
                            }

                            // All solutions should have the same (optimal) size
                            if solutions.len() > 1 {
                                let first_size = problem.solution_size(&solutions[0]).size;
                                for sol in &solutions[1..] {
                                    assert_eq!(problem.solution_size(sol).size, first_size);
                                }
                            }
                        }
                    }
                }
            )*

            #[test]
            fn test_problem_metadata() {
                let info = <$problem as ProblemMetadata>::problem_info();
                assert!(!info.name.is_empty());
                assert!(!info.description.is_empty());

                let category = <$problem as ProblemMetadata>::category();
                assert_eq!(category.name(), "graph");
            }
        }
    };
}

/// Generate tests for verifying complement relationships between problems.
///
/// # Example
///
/// ```text
/// // Macro usage example - users customize for their tests
/// use problemreductions::complement_test;
/// use problemreductions::prelude::{IndependentSet, VertexCovering};
///
/// complement_test! {
///     name: is_vc_complement,
///     problem_a: IndependentSet,
///     problem_b: VertexCovering,
///     test_graphs: [
///         (3, [(0, 1), (1, 2)]),
///         (4, [(0, 1), (1, 2), (2, 3), (0, 3)]),
///     ]
/// }
/// ```
#[macro_export]
macro_rules! complement_test {
    (
        name: $name:ident,
        problem_a: $prob_a:ty,
        problem_b: $prob_b:ty,
        test_graphs: [
            $(($n:expr, [$($edge:expr),*])),* $(,)?
        ]
    ) => {
        #[test]
        fn $name() {
            use $crate::prelude::*;

            $(
                {
                    let edges = vec![$($edge),*];
                    let n = $n;

                    let problem_a = <$prob_a>::new(n, edges.clone());
                    let problem_b = <$prob_b>::new(n, edges);

                    let solver = BruteForce::new();
                    let solutions_a = solver.find_best(&problem_a);
                    let solutions_b = solver.find_best(&problem_b);

                    // Get optimal sizes
                    let size_a: usize = solutions_a[0].iter().sum();
                    let size_b: usize = solutions_b[0].iter().sum();

                    // For complement problems: size_a + size_b = n
                    assert_eq!(
                        size_a + size_b,
                        n,
                        "Complement relationship failed for graph with {} vertices",
                        n
                    );

                    // Verify that complement of solution_a is valid for problem_b
                    for sol_a in &solutions_a {
                        let complement: Vec<usize> = sol_a.iter().map(|&x| 1 - x).collect();
                        assert!(
                            problem_b.solution_size(&complement).is_valid,
                            "Complement of A solution should be valid for B"
                        );
                    }
                }
            )*
        }
    };
}

/// Quick test for a single problem instance.
///
/// # Example
///
/// ```text
/// // Macro usage example - users customize for their tests
/// use problemreductions::quick_problem_test;
/// use problemreductions::prelude::IndependentSet;
///
/// quick_problem_test!(
///     IndependentSet,
///     new(3, vec![(0, 1), (1, 2)]),
///     solution: [1, 0, 1],
///     expected_size: 2,
///     is_valid: true
/// );
/// ```
#[macro_export]
macro_rules! quick_problem_test {
    (
        $problem_type:ty,
        $constructor:ident($($args:expr),*),
        solution: [$($sol:expr),*],
        expected_size: $size:expr,
        is_valid: $valid:expr
    ) => {
        {
            let problem = <$problem_type>::$constructor($($args),*);
            let solution = vec![$($sol),*];
            let result = problem.solution_size(&solution);
            assert_eq!(result.size, $size);
            assert_eq!(result.is_valid, $valid);
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::models::graph::{IndependentSetT, VertexCoverT};
    use crate::prelude::*;
    use crate::topology::SimpleGraph;

    // Test the quick_problem_test macro
    #[test]
    fn test_quick_problem_test_macro() {
        quick_problem_test!(
            IndependentSetT<SimpleGraph, i32>,
            new(3, vec![(0, 1), (1, 2)]),
            solution: [1, 0, 1],
            expected_size: 2,
            is_valid: true
        );

        quick_problem_test!(
            IndependentSetT<SimpleGraph, i32>,
            new(3, vec![(0, 1), (1, 2)]),
            solution: [1, 1, 0],
            expected_size: 2,
            is_valid: false
        );
    }

    // Test the complement_test macro
    complement_test! {
        name: test_is_vc_complement,
        problem_a: IndependentSetT<SimpleGraph, i32>,
        problem_b: VertexCoverT<SimpleGraph, i32>,
        test_graphs: [
            (3, [(0, 1), (1, 2)]),
            (4, [(0, 1), (1, 2), (2, 3), (0, 3)]),
        ]
    }
}
