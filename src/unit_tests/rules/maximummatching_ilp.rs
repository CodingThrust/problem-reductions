use super::*;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::traits::Problem;
use crate::types::SolutionSize;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Triangle graph: 3 vertices, 3 edges
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // Check ILP structure
    assert_eq!(ilp.num_vars, 3, "Should have one variable per edge");
    // Each vertex has degree 2, so 3 constraints (one per vertex)
    assert_eq!(
        ilp.constraints.len(),
        3,
        "Should have one constraint per vertex"
    );
    assert_eq!(ilp.sense, ObjectiveSense::Maximize, "Should maximize");

    // All variables should be binary
    for bound in &ilp.bounds {
        assert_eq!(*bound, VarBounds::binary());
    }

    // Each constraint should be sum of incident edge vars <= 1
    for constraint in &ilp.constraints {
        assert!((constraint.rhs - 1.0).abs() < 1e-9);
    }
}

#[test]
fn test_reduction_weighted() {
    let problem = MaximumMatching::new(3, vec![(0, 1, 5), (1, 2, 10)]);
    let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // Check that weights are correctly transferred to objective
    let mut coeffs: Vec<f64> = vec![0.0; 2];
    for &(var, coef) in &ilp.objective {
        coeffs[var] = coef;
    }
    assert!((coeffs[0] - 5.0).abs() < 1e-9);
    assert!((coeffs[1] - 10.0).abs() < 1e-9);
}

#[test]
fn test_ilp_solution_equals_brute_force_triangle() {
    // Triangle graph: max matching = 1 edge
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve with brute force on original problem
    let bf_solutions = bf.find_best(&problem);

    // Solve via ILP reduction
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // Both should find optimal size = 1 (one edge)
    let bf_size = problem.evaluate(&bf_solutions[0]);
    let ilp_size = problem.evaluate(&extracted);
    assert_eq!(bf_size, SolutionSize::Valid(1));
    assert_eq!(ilp_size, SolutionSize::Valid(1));

    // Verify the ILP solution is valid for the original problem
    assert!(problem.evaluate(&extracted).is_valid(), "Extracted solution should be valid");
}

#[test]
fn test_ilp_solution_equals_brute_force_path() {
    // Path graph 0-1-2-3: max matching = 2 (edges {0-1, 2-3})
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(4, vec![(0, 1), (1, 2), (2, 3)]);
    let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve with brute force
    let bf_solutions = bf.find_best(&problem);
    let bf_size = problem.evaluate(&bf_solutions[0]);

    // Solve via ILP
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_size = problem.evaluate(&extracted);

    assert_eq!(bf_size, SolutionSize::Valid(2));
    assert_eq!(ilp_size, SolutionSize::Valid(2));

    // Verify validity
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_ilp_solution_equals_brute_force_weighted() {
    // Weighted matching: edge 0-1 has high weight
    // 0 -- 1 -- 2
    // Weights: [100, 1]
    // Max matching by weight: just edge 0-1 (weight 100) beats edge 1-2 (weight 1)
    let problem = MaximumMatching::new(3, vec![(0, 1, 100), (1, 2, 1)]);
    let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_best(&problem);
    let bf_obj = problem.evaluate(&bf_solutions[0]);

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_obj = problem.evaluate(&extracted);

    assert_eq!(bf_obj, SolutionSize::Valid(100));
    assert_eq!(ilp_obj, SolutionSize::Valid(100));

    // Verify the solution selects edge 0 (0-1)
    assert_eq!(extracted, vec![1, 0]);
}

#[test]
fn test_solution_extraction() {
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(4, vec![(0, 1), (2, 3)]);
    let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);

    // Test that extraction works correctly (1:1 mapping)
    let ilp_solution = vec![1, 1];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![1, 1]);

    // Verify this is a valid matching (edges 0-1 and 2-3 are disjoint)
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_ilp_structure() {
    let problem =
        MaximumMatching::<SimpleGraph, i32>::unweighted(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 4);
    // Constraints: one per vertex with degree >= 1
    // Vertices 0,1,2,3,4 have degrees 1,2,2,2,1 respectively
    assert_eq!(ilp.constraints.len(), 5);
}

#[test]
fn test_empty_graph() {
    // Graph with no edges: empty matching
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(3, vec![]);
    let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 0);
    assert_eq!(ilp.constraints.len(), 0);

    assert!(problem.evaluate(&[]).is_valid());
    assert_eq!(problem.evaluate(&[]), SolutionSize::Valid(0));
}

#[test]
fn test_k4_perfect_matching() {
    // Complete graph K4: can have perfect matching (2 edges covering all 4 vertices)
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    );
    let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 6 edges, 4 vertices with constraints
    assert_eq!(ilp.num_vars, 6);
    assert_eq!(ilp.constraints.len(), 4);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(problem.evaluate(&extracted).is_valid());
    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(2)); // Perfect matching has 2 edges

    // Verify all vertices are matched
    let sum: usize = extracted.iter().sum();
    assert_eq!(sum, 2);
}

#[test]
fn test_star_graph() {
    // Star graph with center vertex 0 connected to 1, 2, 3
    // Max matching = 1 (only one edge can be selected)
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(4, vec![(0, 1), (0, 2), (0, 3)]);
    let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(problem.evaluate(&extracted).is_valid());
    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(1));
}

#[test]
fn test_bipartite_graph() {
    // Bipartite graph: {0,1} and {2,3} with all cross edges
    // Max matching = 2 (one perfect matching)
    let problem =
        MaximumMatching::<SimpleGraph, i32>::unweighted(4, vec![(0, 2), (0, 3), (1, 2), (1, 3)]);
    let reduction: ReductionMatchingToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(problem.evaluate(&extracted).is_valid());
    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(2));
}

#[test]
fn test_solve_reduced() {
    // Test the ILPSolver::solve_reduced method
    let problem = MaximumMatching::<SimpleGraph, i32>::unweighted(4, vec![(0, 1), (1, 2), (2, 3)]);

    let ilp_solver = ILPSolver::new();
    let solution = ilp_solver
        .solve_reduced(&problem)
        .expect("solve_reduced should work");

    assert!(problem.evaluate(&solution).is_valid());
    assert_eq!(problem.evaluate(&solution), SolutionSize::Valid(2));
}
