use crate::prelude::*;
use crate::topology::SimpleGraph;

// Test the quick_problem_test macro
#[test]
fn test_quick_problem_test_macro() {
    quick_problem_test!(
        IndependentSet<SimpleGraph, i32>,
        new(3, vec![(0, 1), (1, 2)]),
        solution: [1, 0, 1],
        expected_size: 2,
        is_valid: true
    );

    quick_problem_test!(
        IndependentSet<SimpleGraph, i32>,
        new(3, vec![(0, 1), (1, 2)]),
        solution: [1, 1, 0],
        expected_size: 2,
        is_valid: false
    );
}

// Test the complement_test macro
complement_test! {
    name: test_is_vc_complement,
    problem_a: IndependentSet<SimpleGraph, i32>,
    problem_b: VertexCovering<SimpleGraph, i32>,
    test_graphs: [
        (3, [(0, 1), (1, 2)]),
        (4, [(0, 1), (1, 2), (2, 3), (0, 3)]),
    ]
}
