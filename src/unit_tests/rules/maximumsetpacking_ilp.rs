use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Max;

#[test]
fn test_reduction_creates_valid_ilp() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let reduction: ReductionSPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 3, "Should have one variable per set");
    // Elements 1 and 2 each appear in 2 sets → 2 element constraints
    assert_eq!(
        ilp.constraints.len(),
        2,
        "Should have one constraint per shared element"
    );
    assert_eq!(ilp.sense, ObjectiveSense::Maximize, "Should maximize");

    for constraint in &ilp.constraints {
        assert!(constraint.terms.len() >= 2);
        assert!((constraint.rhs - 1.0).abs() < 1e-9);
    }
}

#[test]
fn test_reduction_weighted() {
    let problem = MaximumSetPacking::with_weights(vec![vec![0, 1], vec![2, 3]], vec![5, 10]);
    let reduction: ReductionSPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let mut coeffs: Vec<f64> = vec![0.0; 2];
    for &(var, coef) in &ilp.objective {
        coeffs[var] = coef;
    }
    assert!((coeffs[0] - 5.0).abs() < 1e-9);
    assert!((coeffs[1] - 10.0).abs() < 1e-9);
}

#[test]
fn test_maximumsetpacking_to_ilp_closed_loop() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let reduction: ReductionSPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_all_witnesses(&problem);
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    let bf_size: usize = bf_solutions[0].iter().sum();
    let ilp_size: usize = extracted.iter().sum();
    assert_eq!(bf_size, 2);
    assert_eq!(ilp_size, 2);

    assert!(
        problem.evaluate(&extracted).is_valid(),
        "Extracted solution should be valid"
    );
}

#[test]
fn test_ilp_solution_equals_brute_force_weighted() {
    let problem = MaximumSetPacking::with_weights(
        vec![vec![0, 1, 2, 3], vec![0, 1], vec![2, 3]],
        vec![5, 3, 3],
    );
    let reduction: ReductionSPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_all_witnesses(&problem);
    let bf_obj = problem.evaluate(&bf_solutions[0]);

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_obj = problem.evaluate(&extracted);

    assert_eq!(bf_obj, Max(Some(6)));
    assert_eq!(ilp_obj, Max(Some(6)));
    assert_eq!(extracted, vec![0, 1, 1]);
}

#[test]
fn test_solution_extraction() {
    let problem =
        MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![2, 3], vec![4, 5], vec![6, 7]]);
    let reduction: ReductionSPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = vec![1, 0, 1, 0];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![1, 0, 1, 0]);
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_disjoint_sets() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0], vec![1], vec![2], vec![3]]);
    let reduction: ReductionSPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.constraints.len(), 0);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted, vec![1, 1, 1, 1]);
    assert!(problem.evaluate(&extracted).is_valid());
    assert_eq!(problem.evaluate(&extracted), Max(Some(4)));
}

#[test]
fn test_solve_reduced() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);

    let ilp_solver = ILPSolver::new();
    let solution = ilp_solver
        .solve_reduced(&problem)
        .expect("solve_reduced should work");

    assert!(problem.evaluate(&solution).is_valid());
    assert_eq!(problem.evaluate(&solution), Max(Some(2)));
}

#[test]
fn test_maximumsetpacking_to_ilp_bf_vs_ilp() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let reduction: ReductionSPToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}
