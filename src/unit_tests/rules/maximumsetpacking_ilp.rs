use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::SolutionSize;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Three sets with two overlapping pairs
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // Check ILP structure
    assert_eq!(ilp.num_vars, 3, "Should have one variable per set");
    assert_eq!(
        ilp.constraints.len(),
        2,
        "Should have one constraint per overlapping pair"
    );
    assert_eq!(ilp.sense, ObjectiveSense::Maximize, "Should maximize");

    // All variables should be binary
    for bound in &ilp.bounds {
        assert_eq!(*bound, VarBounds::binary());
    }

    // Each constraint should be x_i + x_j <= 1
    for constraint in &ilp.constraints {
        assert_eq!(constraint.terms.len(), 2);
        assert!((constraint.rhs - 1.0).abs() < 1e-9);
    }
}

#[test]
fn test_reduction_weighted() {
    let problem = MaximumSetPacking::with_weights(vec![vec![0, 1], vec![2, 3]], vec![5, 10]);
    let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
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
fn test_ilp_solution_equals_brute_force_chain() {
    // Chain: {0,1}, {1,2}, {2,3} - can select at most 2 non-adjacent sets
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve with brute force on original problem
    let bf_solutions = bf.find_all_best(&problem);

    // Solve via ILP reduction
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // Both should find optimal size = 2
    let bf_size: usize = bf_solutions[0].iter().sum();
    let ilp_size: usize = extracted.iter().sum();
    assert_eq!(bf_size, 2);
    assert_eq!(ilp_size, 2);

    // Verify the ILP solution is valid for the original problem
    assert!(
        problem.evaluate(&extracted).is_valid(),
        "Extracted solution should be valid"
    );
}

#[test]
fn test_ilp_solution_equals_brute_force_all_overlap() {
    // All sets share element 0: can only select one
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![0, 2], vec![0, 3]]);
    let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_all_best(&problem);
    let bf_size: usize = bf_solutions[0].iter().sum();

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_size: usize = extracted.iter().sum();

    assert_eq!(bf_size, 1);
    assert_eq!(ilp_size, 1);

    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_ilp_solution_equals_brute_force_weighted() {
    // Weighted problem: single heavy set vs multiple light sets
    // Set 0 covers all elements but has weight 5
    // Sets 1 and 2 are disjoint and together have weight 6
    let problem = MaximumSetPacking::with_weights(
        vec![vec![0, 1, 2, 3], vec![0, 1], vec![2, 3]],
        vec![5, 3, 3],
    );
    let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_all_best(&problem);
    let bf_obj = problem.evaluate(&bf_solutions[0]);

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_obj = problem.evaluate(&extracted);

    assert_eq!(bf_obj, SolutionSize::Valid(6));
    assert_eq!(ilp_obj, SolutionSize::Valid(6));

    // Should select sets 1 and 2
    assert_eq!(extracted, vec![0, 1, 1]);
}

#[test]
fn test_solution_extraction() {
    let problem =
        MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![2, 3], vec![4, 5], vec![6, 7]]);
    let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);

    // Test that extraction works correctly (1:1 mapping)
    let ilp_solution = vec![1, 0, 1, 0];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![1, 0, 1, 0]);

    // Verify this is a valid packing (sets 0 and 2 are disjoint)
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_ilp_structure() {
    let problem =
        MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]);
    let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 4);
    // 3 overlapping pairs: (0,1), (1,2), (2,3)
    assert_eq!(ilp.constraints.len(), 3);
}

#[test]
fn test_disjoint_sets() {
    // All sets are disjoint: no overlapping pairs
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0], vec![1], vec![2], vec![3]]);
    let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.constraints.len(), 0);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // All sets should be selected
    assert_eq!(extracted, vec![1, 1, 1, 1]);

    assert!(problem.evaluate(&extracted).is_valid());
    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(4));
}

#[test]
fn test_empty_sets() {
    let problem = MaximumSetPacking::<i32>::new(vec![]);
    let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 0);
    assert_eq!(ilp.constraints.len(), 0);
}

#[test]
fn test_solve_reduced() {
    // Test the ILPSolver::solve_reduced method
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);

    let ilp_solver = ILPSolver::new();
    let solution = ilp_solver
        .solve_reduced(&problem)
        .expect("solve_reduced should work");

    assert!(problem.evaluate(&solution).is_valid());
    assert_eq!(problem.evaluate(&solution), SolutionSize::Valid(2));
}

#[test]
fn test_all_sets_overlap_pairwise() {
    // All pairs overlap: can only select one set
    // Sets: {0,1}, {0,2}, {1,2} - each pair shares one element
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![0, 2], vec![1, 2]]);
    let reduction: ReductionSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 3 overlapping pairs
    assert_eq!(ilp.constraints.len(), 3);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(problem.evaluate(&extracted).is_valid());
    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(1));
}
