//! Testing macros for problem implementations.

/// Generate standard tests for a graph problem.
///
/// This macro generates tests for:
/// - Problem creation
/// - Solution evaluation
/// - Brute force solving (for small instances)
/// - Metadata (if ProblemMetadata is implemented)
///
/// # Example
///
/// ```text
/// // Macro usage example - users customize for their tests
/// use problemreductions::graph_problem_tests;
/// use problemreductions::models::graph::MaximumIndependentSet;
/// use problemreductions::topology::SimpleGraph;
///
/// graph_problem_tests! {
///     problem_type: MaximumIndependentSet<SimpleGraph, i32>,
///     test_cases: [
///         // (name, num_vertices, edges, valid_solution, expected_value, is_maximization)
///         (triangle, 3, [(0, 1), (1, 2), (0, 2)], [1, 0, 0], 1, true),
///         (path3, 3, [(0, 1), (1, 2)], [1, 0, 1], 2, true),
///     ]
/// }
/// ```
#[macro_export]
macro_rules! graph_problem_tests {
    (
        problem_type: $problem:ty,
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
            use $crate::types::Direction;

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
                    }

                    #[test]
                    fn test_solution_evaluation() {
                        let problem = create_problem();
                        let solution = vec![$($sol),*];
                        let value = problem.evaluate(&solution);
                        assert_eq!(value, $size, "Solution value mismatch");
                    }

                    #[test]
                    fn test_direction() {
                        let problem = create_problem();
                        if $is_max {
                            assert_eq!(problem.direction(), Direction::Maximize);
                        } else {
                            assert_eq!(problem.direction(), Direction::Minimize);
                        }
                    }

                    #[test]
                    fn test_brute_force() {
                        if $n <= 15 {
                            let problem = create_problem();
                            let solver = BruteForce::new();
                            let solutions = solver.find_all_best(&problem);

                            // All solutions should have the same (optimal) value
                            if solutions.len() > 1 {
                                let first_value = problem.evaluate(&solutions[0]);
                                for sol in &solutions[1..] {
                                    assert_eq!(problem.evaluate(sol), first_value);
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
/// For complement problems (like MIS and MVC), the optimal solutions are complements
/// of each other: if S is a maximum independent set, then V-S is a minimum vertex cover.
///
/// # Example
///
/// ```text
/// // Macro usage example - users customize for their tests
/// use problemreductions::complement_test;
/// use problemreductions::prelude::{MaximumIndependentSet, MinimumVertexCover};
///
/// complement_test! {
///     name: is_vc_complement,
///     problem_a: MaximumIndependentSet,
///     problem_b: MinimumVertexCover,
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
                    let solutions_a = solver.find_all_best(&problem_a);
                    let solutions_b = solver.find_all_best(&problem_b);

                    // Get optimal sizes (count of selected vertices)
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
                    // (i.e., evaluates to a valid value, is_valid() returns true)
                    for sol_a in &solutions_a {
                        let complement: Vec<usize> = sol_a.iter().map(|&x| 1 - x).collect();
                        let value = problem_b.evaluate(&complement);
                        assert!(
                            value.is_valid(),
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
/// For maximization problems, invalid solutions evaluate to i32::MIN.
/// For minimization problems, invalid solutions evaluate to i32::MAX.
///
/// # Example
///
/// ```text
/// // Macro usage example - users customize for their tests
/// use problemreductions::quick_problem_test;
/// use problemreductions::prelude::MaximumIndependentSet;
///
/// // Test a valid solution (is_max=true means maximization problem)
/// quick_problem_test!(
///     MaximumIndependentSet,
///     new(3, vec![(0, 1), (1, 2)]),
///     solution: [1, 0, 1],
///     expected_value: 2,
///     is_max: true
/// );
///
/// // Test an invalid solution (adjacent vertices selected)
/// quick_problem_test!(
///     MaximumIndependentSet,
///     new(3, vec![(0, 1), (1, 2)]),
///     solution: [1, 1, 0],
///     expected_value: i32::MIN,
///     is_max: true
/// );
/// ```
#[macro_export]
macro_rules! quick_problem_test {
    (
        $problem_type:ty,
        $constructor:ident($($args:expr),*),
        solution: [$($sol:expr),*],
        expected_value: $value:expr,
        is_max: $is_max:expr
    ) => {
        {
            let problem = <$problem_type>::$constructor($($args),*);
            let solution = vec![$($sol),*];
            let result = problem.evaluate(&solution);
            assert_eq!(result, $value);
        }
    };
}

#[cfg(test)]
#[path = "../unit_tests/testing/macros.rs"]
mod tests;
