use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::SolutionSize;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Universe: {0, 1, 2}, Sets: S0={0,1}, S1={1,2}
    let problem = MinimumSetCovering::<i32>::new(3, vec![vec![0, 1], vec![1, 2]]);
    let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // Check ILP structure
    assert_eq!(ilp.num_vars, 2, "Should have one variable per set");
    assert_eq!(
        ilp.constraints.len(),
        3,
        "Should have one constraint per element"
    );
    assert_eq!(ilp.sense, ObjectiveSense::Minimize, "Should minimize");

    // All variables should be binary
    for bound in &ilp.bounds {
        assert_eq!(*bound, VarBounds::binary());
    }

    // Each constraint should be sum >= 1
    for constraint in &ilp.constraints {
        assert!((constraint.rhs - 1.0).abs() < 1e-9);
    }
}

#[test]
fn test_reduction_weighted() {
    let problem = MinimumSetCovering::with_weights(3, vec![vec![0, 1], vec![1, 2]], vec![5, 10]);
    let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
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
fn test_ilp_solution_equals_brute_force_simple() {
    // Universe: {0, 1, 2}, Sets: S0={0,1}, S1={1,2}, S2={0,2}
    // Minimum cover: any 2 sets work
    let problem = MinimumSetCovering::<i32>::new(3, vec![vec![0, 1], vec![1, 2], vec![0, 2]]);
    let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
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
    assert!(problem.evaluate(&extracted).is_valid(), "Extracted solution should be valid");
}

#[test]
fn test_ilp_solution_equals_brute_force_weighted() {
    // Weighted problem: prefer lighter sets
    // Universe: {0,1,2}, Sets: S0={0,1,2}, S1={0,1}, S2={2}
    // Weights: [10, 3, 3]
    // Optimal: select S1 and S2 (weight 6) instead of S0 (weight 10)
    let problem = MinimumSetCovering::with_weights(
        3,
        vec![vec![0, 1, 2], vec![0, 1], vec![2]],
        vec![10, 3, 3],
    );
    let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
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

    // Verify the solution selects S1 and S2
    assert_eq!(extracted, vec![0, 1, 1]);
}

#[test]
fn test_solution_extraction() {
    let problem = MinimumSetCovering::<i32>::new(4, vec![vec![0, 1], vec![2, 3]]);
    let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);

    // Test that extraction works correctly (1:1 mapping)
    let ilp_solution = vec![1, 1];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![1, 1]);

    // Verify this is a valid set cover
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_ilp_structure() {
    let problem =
        MinimumSetCovering::<i32>::new(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]);
    let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 4);
    assert_eq!(ilp.constraints.len(), 5);
}

#[test]
fn test_single_set_covers_all() {
    // Single set covers entire universe
    let problem = MinimumSetCovering::<i32>::new(3, vec![vec![0, 1, 2], vec![0], vec![1], vec![2]]);

    let ilp_solver = ILPSolver::new();
    let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // First set alone covers everything with weight 1
    assert_eq!(extracted, vec![1, 0, 0, 0]);

    assert!(problem.evaluate(&extracted).is_valid());
    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(1));
}

#[test]
fn test_overlapping_sets() {
    // All sets overlap on element 1
    let problem = MinimumSetCovering::<i32>::new(3, vec![vec![0, 1], vec![1, 2]]);

    let ilp_solver = ILPSolver::new();
    let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // Need both sets to cover all elements
    assert_eq!(extracted, vec![1, 1]);

    assert!(problem.evaluate(&extracted).is_valid());
    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(2));
}

#[test]
fn test_empty_universe() {
    // Empty universe is trivially covered
    let problem = MinimumSetCovering::<i32>::new(0, vec![]);
    let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 0);
    assert_eq!(ilp.constraints.len(), 0);
}

#[test]
fn test_solve_reduced() {
    // Test the ILPSolver::solve_reduced method
    let problem =
        MinimumSetCovering::<i32>::new(4, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 3]]);

    let ilp_solver = ILPSolver::new();
    let solution = ilp_solver
        .solve_reduced(&problem)
        .expect("solve_reduced should work");

    assert!(problem.evaluate(&solution).is_valid());
    assert_eq!(problem.evaluate(&solution), SolutionSize::Valid(2));
}

#[test]
fn test_constraint_structure() {
    // Universe: {0, 1, 2}
    // Sets: S0={0}, S1={0,1}, S2={1,2}
    // Element 0 is in S0, S1 -> constraint: x0 + x1 >= 1
    // Element 1 is in S1, S2 -> constraint: x1 + x2 >= 1
    // Element 2 is in S2 -> constraint: x2 >= 1
    let problem = MinimumSetCovering::<i32>::new(3, vec![vec![0], vec![0, 1], vec![1, 2]]);
    let reduction: ReductionSCToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.constraints.len(), 3);

    // Check constraint for element 0: should involve sets 0 and 1
    let c0 = &ilp.constraints[0];
    let vars0: Vec<usize> = c0.terms.iter().map(|&(v, _)| v).collect();
    assert!(vars0.contains(&0));
    assert!(vars0.contains(&1));
    assert!(!vars0.contains(&2));

    // Check constraint for element 1: should involve sets 1 and 2
    let c1 = &ilp.constraints[1];
    let vars1: Vec<usize> = c1.terms.iter().map(|&(v, _)| v).collect();
    assert!(!vars1.contains(&0));
    assert!(vars1.contains(&1));
    assert!(vars1.contains(&2));

    // Check constraint for element 2: should involve only set 2
    let c2 = &ilp.constraints[2];
    let vars2: Vec<usize> = c2.terms.iter().map(|&(v, _)| v).collect();
    assert!(!vars2.contains(&0));
    assert!(!vars2.contains(&1));
    assert!(vars2.contains(&2));
}
