use super::*;
use crate::solvers::{BruteForce, ILPSolver, Solver};

#[test]
fn test_reduction_creates_valid_ilp() {
    // Triangle graph: 3 vertices, 3 edges
    let problem = DominatingSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // Check ILP structure
    assert_eq!(ilp.num_vars, 3, "Should have one variable per vertex");
    assert_eq!(
        ilp.constraints.len(),
        3,
        "Should have one constraint per vertex"
    );
    assert_eq!(ilp.sense, ObjectiveSense::Minimize, "Should minimize");

    // All variables should be binary
    for bound in &ilp.bounds {
        assert_eq!(*bound, VarBounds::binary());
    }

    // Each constraint should be x_v + sum_{u in N(v)} x_u >= 1
    for constraint in &ilp.constraints {
        assert!(!constraint.terms.is_empty());
        assert!((constraint.rhs - 1.0).abs() < 1e-9);
    }
}

#[test]
fn test_reduction_weighted() {
    let problem = DominatingSet::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
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
fn test_ilp_solution_equals_brute_force_star() {
    // Star graph: center vertex 0 connected to all others
    // Minimum dominating set is just the center (weight 1)
    let problem = DominatingSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (0, 2), (0, 3)]);
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve with brute force on original problem
    let bf_solutions = bf.find_best(&problem);
    let bf_size = problem.solution_size(&bf_solutions[0]).size;

    // Solve via ILP reduction
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_size = problem.solution_size(&extracted).size;

    // Both should find optimal size = 1 (just the center)
    assert_eq!(bf_size, 1);
    assert_eq!(ilp_size, 1);

    // Verify the ILP solution is valid for the original problem
    let sol_result = problem.solution_size(&extracted);
    assert!(sol_result.is_valid, "Extracted solution should be valid");
}

#[test]
fn test_ilp_solution_equals_brute_force_path() {
    // Path graph 0-1-2-3-4: min DS = 2 (e.g., vertices 1 and 3)
    let problem = DominatingSet::<SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve with brute force
    let bf_solutions = bf.find_best(&problem);
    let bf_size = problem.solution_size(&bf_solutions[0]).size;

    // Solve via ILP
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_size = problem.solution_size(&extracted).size;

    assert_eq!(bf_size, 2);
    assert_eq!(ilp_size, 2);

    // Verify validity
    let sol_result = problem.solution_size(&extracted);
    assert!(sol_result.is_valid);
}

#[test]
fn test_ilp_solution_equals_brute_force_weighted() {
    // Star with heavy center: prefer selecting all leaves (total weight 3)
    // over center (weight 100)
    let problem =
        DominatingSet::with_weights(4, vec![(0, 1), (0, 2), (0, 3)], vec![100, 1, 1, 1]);
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_best(&problem);
    let bf_obj = problem.solution_size(&bf_solutions[0]).size;

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_obj = problem.solution_size(&extracted).size;

    assert_eq!(bf_obj, 3);
    assert_eq!(ilp_obj, 3);

    // Verify the solution selects all leaves
    assert_eq!(extracted, vec![0, 1, 1, 1]);
}

#[test]
fn test_solution_extraction() {
    let problem = DominatingSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (2, 3)]);
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);

    // Test that extraction works correctly (1:1 mapping)
    let ilp_solution = vec![1, 0, 1, 0];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![1, 0, 1, 0]);

    // Verify this is a valid DS (0 dominates 0,1 and 2 dominates 2,3)
    let sol_result = problem.solution_size(&extracted);
    assert!(sol_result.is_valid);
}

#[test]
fn test_source_and_target_size() {
    let problem = DominatingSet::<SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);

    let source_size = reduction.source_size();
    let target_size = reduction.target_size();

    assert_eq!(source_size.get("num_vertices"), Some(5));
    assert_eq!(source_size.get("num_edges"), Some(4));

    assert_eq!(target_size.get("num_vars"), Some(5));
    assert_eq!(target_size.get("num_constraints"), Some(5)); // one per vertex
}

#[test]
fn test_isolated_vertices() {
    // Graph with isolated vertex 2: it must be in the dominating set
    let problem = DominatingSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // Vertex 2 must be selected (isolated)
    assert_eq!(extracted[2], 1);

    let sol_result = problem.solution_size(&extracted);
    assert!(sol_result.is_valid);
}

#[test]
fn test_complete_graph() {
    // Complete graph K4: min DS = 1 (any vertex dominates all)
    let problem =
        DominatingSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    let sol_result = problem.solution_size(&extracted);
    assert!(sol_result.is_valid);
    assert_eq!(sol_result.size, 1);
}

#[test]
fn test_single_vertex() {
    // Single vertex with no edges: must be in dominating set
    let problem = DominatingSet::<SimpleGraph, i32>::new(1, vec![]);
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted, vec![1]);

    let sol_result = problem.solution_size(&extracted);
    assert!(sol_result.is_valid);
    assert_eq!(sol_result.size, 1);
}

#[test]
fn test_cycle_graph() {
    // Cycle C5: 0-1-2-3-4-0
    // Minimum dominating set size = 2
    let problem = DominatingSet::<SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]);
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_best(&problem);
    let bf_size = problem.solution_size(&bf_solutions[0]).size;

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_size = problem.solution_size(&extracted).size;

    assert_eq!(bf_size, ilp_size);

    let sol_result = problem.solution_size(&extracted);
    assert!(sol_result.is_valid);
}
