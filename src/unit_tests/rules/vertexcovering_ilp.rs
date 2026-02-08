use super::*;
use crate::solvers::{BruteForce, ILPSolver, Solver};

#[test]
fn test_reduction_creates_valid_ilp() {
    // Triangle graph: 3 vertices, 3 edges
    let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction: ReductionVCToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // Check ILP structure
    assert_eq!(ilp.num_vars, 3, "Should have one variable per vertex");
    assert_eq!(
        ilp.constraints.len(),
        3,
        "Should have one constraint per edge"
    );
    assert_eq!(ilp.sense, ObjectiveSense::Minimize, "Should minimize");

    // All variables should be binary
    for bound in &ilp.bounds {
        assert_eq!(*bound, VarBounds::binary());
    }

    // Each constraint should be x_i + x_j >= 1
    for constraint in &ilp.constraints {
        assert_eq!(constraint.terms.len(), 2);
        assert!((constraint.rhs - 1.0).abs() < 1e-9);
    }
}

#[test]
fn test_reduction_weighted() {
    let problem = VertexCovering::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
    let reduction: ReductionVCToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // Check that weights are correctly transferred to objective
    let mut coeffs: Vec<f64> = vec![0.0; 3];
    for &(var, coef) in &ilp.objective {
        coeffs[var] = coef;
    }
    assert!((coeffs[0] - 5.0).abs() < 1e-9);
    assert!((coeffs[1] - 10.0).abs() < 1e-9);
    assert!((coeffs[2] - 15.0).abs() < 1e-9);
}

#[test]
fn test_ilp_solution_equals_brute_force_triangle() {
    // Triangle graph: min VC = 2 vertices
    let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction: ReductionVCToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve with brute force on original problem
    let bf_solutions = bf.find_best(&problem);

    // Solve via ILP reduction
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // Both should find optimal size = 2
    let bf_size: usize = bf_solutions[0].iter().sum();
    let ilp_size: usize = extracted.iter().sum();
    assert_eq!(bf_size, 2);
    assert_eq!(ilp_size, 2);

    // Verify the ILP solution is valid for the original problem
    let sol_result = problem.solution_size(&extracted);
    assert!(sol_result.is_valid, "Extracted solution should be valid");
}

#[test]
fn test_ilp_solution_equals_brute_force_path() {
    // Path graph 0-1-2-3: min VC = 2 (e.g., {1, 2} or {0, 2} or {1, 3})
    let problem = VertexCovering::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let reduction: ReductionVCToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve with brute force
    let bf_solutions = bf.find_best(&problem);
    let bf_size: usize = bf_solutions[0].iter().sum();

    // Solve via ILP
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_size: usize = extracted.iter().sum();

    assert_eq!(bf_size, 2);
    assert_eq!(ilp_size, 2);

    // Verify validity
    let sol_result = problem.solution_size(&extracted);
    assert!(sol_result.is_valid);
}

#[test]
fn test_ilp_solution_equals_brute_force_weighted() {
    // Weighted problem: vertex 1 has low weight and covers both edges
    // 0 -- 1 -- 2
    // Weights: [100, 1, 100]
    // Min VC by weight: just vertex 1 (weight 1) beats 0+2 (weight 200)
    let problem = VertexCovering::with_weights(3, vec![(0, 1), (1, 2)], vec![100, 1, 100]);
    let reduction: ReductionVCToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_best(&problem);
    let bf_obj = problem.solution_size(&bf_solutions[0]).size;

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_obj = problem.solution_size(&extracted).size;

    assert_eq!(bf_obj, 1);
    assert_eq!(ilp_obj, 1);

    // Verify the solution selects vertex 1
    assert_eq!(extracted, vec![0, 1, 0]);
}

#[test]
fn test_solution_extraction() {
    let problem = VertexCovering::<SimpleGraph, i32>::new(4, vec![(0, 1), (2, 3)]);
    let reduction: ReductionVCToILP = ReduceTo::<ILP>::reduce_to(&problem);

    // Test that extraction works correctly (1:1 mapping)
    let ilp_solution = vec![1, 0, 0, 1];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![1, 0, 0, 1]);

    // Verify this is a valid VC (covers edges 0-1 and 2-3)
    let sol_result = problem.solution_size(&extracted);
    assert!(sol_result.is_valid);
}

#[test]
fn test_source_and_target_size() {
    let problem = VertexCovering::<SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let reduction: ReductionVCToILP = ReduceTo::<ILP>::reduce_to(&problem);

    let source_size = reduction.source_size();
    let target_size = reduction.target_size();

    assert_eq!(source_size.get("num_vertices"), Some(5));
    assert_eq!(source_size.get("num_edges"), Some(4));

    assert_eq!(target_size.get("num_vars"), Some(5));
    assert_eq!(target_size.get("num_constraints"), Some(4));
}

#[test]
fn test_empty_graph() {
    // Graph with no edges: empty cover is valid
    let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![]);
    let reduction: ReductionVCToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.constraints.len(), 0);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // No vertices should be selected
    assert_eq!(extracted, vec![0, 0, 0]);

    let sol_result = problem.solution_size(&extracted);
    assert!(sol_result.is_valid);
    assert_eq!(sol_result.size, 0);
}

#[test]
fn test_complete_graph() {
    // Complete graph K4: min VC = 3 (all but one vertex)
    let problem =
        VertexCovering::<SimpleGraph, i32>::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let reduction: ReductionVCToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.constraints.len(), 6);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    let sol_result = problem.solution_size(&extracted);
    assert!(sol_result.is_valid);
    assert_eq!(sol_result.size, 3);
}

#[test]
fn test_solve_reduced() {
    // Test the ILPSolver::solve_reduced method
    let problem = VertexCovering::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);

    let ilp_solver = ILPSolver::new();
    let solution = ilp_solver
        .solve_reduced(&problem)
        .expect("solve_reduced should work");

    let sol_result = problem.solution_size(&solution);
    assert!(sol_result.is_valid);
    assert_eq!(sol_result.size, 2);
}

#[test]
fn test_bipartite_graph() {
    // Bipartite graph: 0-2, 0-3, 1-2, 1-3 (complete bipartite K_{2,2})
    // Min VC = 2 (either side of the bipartition)
    let problem = VertexCovering::<SimpleGraph, i32>::new(4, vec![(0, 2), (0, 3), (1, 2), (1, 3)]);
    let reduction: ReductionVCToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    let sol_result = problem.solution_size(&extracted);
    assert!(sol_result.is_valid);
    assert_eq!(sol_result.size, 2);

    // Should select either {0, 1} or {2, 3}
    let sum: usize = extracted.iter().sum();
    assert_eq!(sum, 2);
}

#[test]
fn test_single_edge() {
    // Single edge: min VC = 1
    let problem = VertexCovering::<SimpleGraph, i32>::new(2, vec![(0, 1)]);
    let reduction: ReductionVCToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_best(&problem);
    let bf_size: usize = bf_solutions[0].iter().sum();

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_size: usize = extracted.iter().sum();

    assert_eq!(bf_size, 1);
    assert_eq!(ilp_size, 1);
}

#[test]
fn test_star_graph() {
    // Star graph: center vertex 0 connected to all others
    // Min VC = 1 (just the center)
    let problem = VertexCovering::<SimpleGraph, i32>::new(5, vec![(0, 1), (0, 2), (0, 3), (0, 4)]);
    let reduction: ReductionVCToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_best(&problem);
    let bf_size: usize = bf_solutions[0].iter().sum();

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_size: usize = extracted.iter().sum();

    assert_eq!(bf_size, 1);
    assert_eq!(ilp_size, 1);

    // The optimal solution should select vertex 0
    assert_eq!(extracted[0], 1);
}
