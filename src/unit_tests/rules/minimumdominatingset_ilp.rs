use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::SolutionSize;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Triangle graph: 3 vertices, 3 edges
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32; 3],
    );
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
    let problem = MinimumDominatingSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![5, 10, 15]);
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
fn test_minimumdominatingset_to_ilp_closed_loop() {
    // Star graph: center vertex 0 connected to all others
    // Minimum dominating set is just the center (weight 1)
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]),
        vec![1i32; 4],
    );
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve with brute force on original problem
    let bf_solutions = bf.find_all_best(&problem);
    let bf_size = problem.evaluate(&bf_solutions[0]);

    // Solve via ILP reduction
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_size = problem.evaluate(&extracted);

    // Both should find optimal size = 1 (just the center)
    assert_eq!(bf_size, SolutionSize::Valid(1));
    assert_eq!(ilp_size, SolutionSize::Valid(1));

    // Verify the ILP solution is valid for the original problem
    assert!(
        problem.evaluate(&extracted).is_valid(),
        "Extracted solution should be valid"
    );
}

#[test]
fn test_ilp_solution_equals_brute_force_path() {
    // Path graph 0-1-2-3-4: min DS = 2 (e.g., vertices 1 and 3)
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1i32; 5],
    );
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve with brute force
    let bf_solutions = bf.find_all_best(&problem);
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
    // Star with heavy center: prefer selecting all leaves (total weight 3)
    // over center (weight 100)
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]),
        vec![100, 1, 1, 1],
    );
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_all_best(&problem);
    let bf_obj = problem.evaluate(&bf_solutions[0]);

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_obj = problem.evaluate(&extracted);

    assert_eq!(bf_obj, SolutionSize::Valid(3));
    assert_eq!(ilp_obj, SolutionSize::Valid(3));

    // Verify the solution selects all leaves
    assert_eq!(extracted, vec![0, 1, 1, 1]);
}

#[test]
fn test_solution_extraction() {
    let problem =
        MinimumDominatingSet::new(SimpleGraph::new(4, vec![(0, 1), (2, 3)]), vec![1i32; 4]);
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);

    // Test that extraction works correctly (1:1 mapping)
    let ilp_solution = vec![1, 0, 1, 0];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![1, 0, 1, 0]);

    // Verify this is a valid DS (0 dominates 0,1 and 2 dominates 2,3)
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_ilp_structure() {
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1i32; 5],
    );
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 5);
    assert_eq!(ilp.constraints.len(), 5); // one per vertex
}

#[test]
fn test_isolated_vertices() {
    // Graph with isolated vertex 2: it must be in the dominating set
    let problem = MinimumDominatingSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // Vertex 2 must be selected (isolated)
    assert_eq!(extracted[2], 1);

    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_complete_graph() {
    // Complete graph K4: min DS = 1 (any vertex dominates all)
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        vec![1i32; 4],
    );
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(problem.evaluate(&extracted).is_valid());
    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(1));
}

#[test]
fn test_single_vertex() {
    // Single vertex with no edges: must be in dominating set
    let problem = MinimumDominatingSet::new(SimpleGraph::new(1, vec![]), vec![1i32; 1]);
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted, vec![1]);

    assert!(problem.evaluate(&extracted).is_valid());
    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(1));
}

#[test]
fn test_cycle_graph() {
    // Cycle C5: 0-1-2-3-4-0
    // Minimum dominating set size = 2
    let problem = MinimumDominatingSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]),
        vec![1i32; 5],
    );
    let reduction: ReductionDSToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_all_best(&problem);
    let bf_size = problem.evaluate(&bf_solutions[0]);

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_size = problem.evaluate(&extracted);

    assert_eq!(bf_size, ilp_size);

    assert!(problem.evaluate(&extracted).is_valid());
}
