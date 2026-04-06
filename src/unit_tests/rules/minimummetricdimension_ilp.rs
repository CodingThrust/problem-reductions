use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_minimummetricdimension_to_ilp_closed_loop() {
    // House graph: metric dimension = 2
    let problem = MinimumMetricDimension::new(SimpleGraph::new(
        5,
        vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)],
    ));
    let reduction: ReductionMDToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve with brute force on original problem
    let bf_solutions = bf.find_all_witnesses(&problem);
    let bf_size = problem.evaluate(&bf_solutions[0]);

    // Solve via ILP reduction
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_size = problem.evaluate(&extracted);

    // Both should find optimal size = 2
    assert_eq!(bf_size, Min(Some(2)));
    assert_eq!(ilp_size, Min(Some(2)));

    // Verify the ILP solution is valid for the original problem
    assert!(
        problem.evaluate(&extracted).is_valid(),
        "Extracted solution should be valid"
    );
}

#[test]
fn test_minimummetricdimension_to_ilp_structure() {
    // Path graph P3: 3 vertices
    let problem = MinimumMetricDimension::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let reduction: ReductionMDToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // Check ILP structure
    assert_eq!(ilp.num_vars, 3, "Should have one variable per vertex");
    // C(3,2) = 3 pairs
    assert_eq!(
        ilp.constraints.len(),
        3,
        "Should have one constraint per vertex pair"
    );
    assert_eq!(ilp.sense, ObjectiveSense::Minimize, "Should minimize");

    // Each constraint should have rhs = 1
    for constraint in &ilp.constraints {
        assert!(!constraint.terms.is_empty());
        assert!((constraint.rhs - 1.0).abs() < 1e-9);
    }
}

#[test]
fn test_minimummetricdimension_to_ilp_bf_vs_ilp() {
    // House graph
    let problem = MinimumMetricDimension::new(SimpleGraph::new(
        5,
        vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)],
    ));
    let reduction: ReductionMDToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_minimummetricdimension_to_ilp_path_graph() {
    // Path P4: 0-1-2-3, metric dimension = 1 (any endpoint resolves)
    let problem = MinimumMetricDimension::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction: ReductionMDToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(problem.evaluate(&extracted).is_valid());
    assert_eq!(problem.evaluate(&extracted), Min(Some(1)));
}

#[test]
fn test_minimummetricdimension_to_ilp_complete_graph() {
    // K4: metric dimension = 3 (n-1)
    let problem = MinimumMetricDimension::new(SimpleGraph::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    ));
    let reduction: ReductionMDToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_all_witnesses(&problem);
    let bf_size = problem.evaluate(&bf_solutions[0]);

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_size = problem.evaluate(&extracted);

    assert_eq!(bf_size, Min(Some(3)));
    assert_eq!(ilp_size, Min(Some(3)));
}

#[test]
fn test_minimummetricdimension_to_ilp_solution_extraction() {
    let problem = MinimumMetricDimension::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let reduction: ReductionMDToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // Test that extraction works correctly (1:1 mapping)
    let ilp_solution = vec![1, 0, 0];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![1, 0, 0]);

    // Verify this is a valid resolving set
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_minimummetricdimension_to_ilp_cycle() {
    // C5: metric dimension = 2
    let problem = MinimumMetricDimension::new(SimpleGraph::new(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)],
    ));
    let reduction: ReductionMDToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(problem.evaluate(&extracted).is_valid());
    assert_eq!(problem.evaluate(&extracted), Min(Some(2)));
}
