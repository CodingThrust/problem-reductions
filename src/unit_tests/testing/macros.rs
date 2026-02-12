use crate::prelude::*;
use crate::topology::SimpleGraph;
use crate::types::SolutionSize;

// Test the quick_problem_test macro
#[test]
fn test_quick_problem_test_macro() {
    // Test a valid solution
    quick_problem_test!(
        MaximumIndependentSet<SimpleGraph, i32>,
        new(3, vec![(0, 1), (1, 2)]),
        solution: [1, 0, 1],
        expected_value: SolutionSize::Valid(2),
        is_max: true
    );

    // Test an invalid solution (adjacent vertices selected) -> returns Invalid
    quick_problem_test!(
        MaximumIndependentSet<SimpleGraph, i32>,
        new(3, vec![(0, 1), (1, 2)]),
        solution: [1, 1, 0],
        expected_value: SolutionSize::Invalid,
        is_max: true
    );
}

// Test the complement_test macro
complement_test! {
    name: test_is_vc_complement,
    problem_a: MaximumIndependentSet<SimpleGraph, i32>,
    problem_b: MinimumVertexCover<SimpleGraph, i32>,
    test_graphs: [
        (3, [(0, 1), (1, 2)]),
        (4, [(0, 1), (1, 2), (2, 3), (0, 3)]),
    ]
}
