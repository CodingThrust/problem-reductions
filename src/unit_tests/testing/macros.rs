use crate::prelude::*;
use crate::topology::SimpleGraph;
use crate::types::SolutionSize;

// Test the quick_problem_test macro
#[test]
fn test_quick_problem_test_macro() {
    // Test a valid solution
    {
        let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
        let solution = vec![1, 0, 1];
        let result = problem.evaluate(&solution);
        assert_eq!(result, SolutionSize::Valid(2));
    }

    // Test an invalid solution (adjacent vertices selected) -> returns Invalid
    {
        let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1i32; 3]);
        let solution = vec![1, 1, 0];
        let result = problem.evaluate(&solution);
        assert_eq!(result, SolutionSize::Invalid);
    }
}

// Test the complement_test macro - manually implemented since MIS constructor changed
#[test]
fn test_is_vc_complement() {
    use crate::prelude::*;

    for (n, edges) in [
        (3usize, vec![(0, 1), (1, 2)]),
        (4usize, vec![(0, 1), (1, 2), (2, 3), (0, 3)]),
    ] {
        let problem_a = MaximumIndependentSet::new(SimpleGraph::new(n, edges.clone()), vec![1i32; n]);
        let problem_b = MinimumVertexCover::<SimpleGraph, i32>::new(n, edges);

        let solver = BruteForce::new();
        let solutions_a = solver.find_all_best(&problem_a);
        let solutions_b = solver.find_all_best(&problem_b);

        let size_a: usize = solutions_a[0].iter().sum();
        let size_b: usize = solutions_b[0].iter().sum();

        assert_eq!(
            size_a + size_b,
            n,
            "Complement relationship failed for graph with {} vertices",
            n
        );

        for sol_a in &solutions_a {
            let complement: Vec<usize> = sol_a.iter().map(|&x| 1 - x).collect();
            let value = problem_b.evaluate(&complement);
            assert!(
                value.is_valid(),
                "Complement of A solution should be valid for B"
            );
        }
    }
}
