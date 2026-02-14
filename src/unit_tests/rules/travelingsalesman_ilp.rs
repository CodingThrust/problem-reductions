use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::SolutionSize;

#[test]
fn test_reduction_creates_valid_ilp_c4() {
    // C4 cycle: 4 vertices, 4 edges. Unique Hamiltonian cycle (the cycle itself).
    let problem =
        TravelingSalesman::<SimpleGraph, i32>::unweighted(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]);
    let reduction: ReductionTSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // n=4, m=4: num_vars = 16 + 2*4*4 = 48
    assert_eq!(ilp.num_vars, 48);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);

    // All variables should be binary
    for bound in &ilp.bounds {
        assert_eq!(*bound, VarBounds::binary());
    }
}

#[test]
fn test_reduction_c4_closed_loop() {
    // C4 cycle with unit weights: optimal tour cost = 4
    let problem =
        TravelingSalesman::<SimpleGraph, i32>::unweighted(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]);
    let reduction: ReductionTSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // Verify extracted solution is valid on source problem
    let metric = problem.evaluate(&extracted);
    assert!(metric.is_valid(), "Extracted solution must be valid");
    assert_eq!(metric, SolutionSize::Valid(4));
}

#[test]
fn test_reduction_k4_weighted_closed_loop() {
    // K4 weighted: find minimum weight Hamiltonian cycle
    let problem = TravelingSalesman::<SimpleGraph, i32>::new(
        4,
        vec![
            (0, 1, 10),
            (0, 2, 15),
            (0, 3, 20),
            (1, 2, 35),
            (1, 3, 25),
            (2, 3, 30),
        ],
    );

    // Solve via ILP reduction
    let reduction: ReductionTSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // Solve via brute force for cross-check
    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_best(&problem);
    let bf_metric = problem.evaluate(&bf_solutions[0]);
    let ilp_metric = problem.evaluate(&extracted);

    assert!(ilp_metric.is_valid());
    assert_eq!(
        ilp_metric, bf_metric,
        "ILP and brute force must agree on optimal cost"
    );
}

#[test]
fn test_reduction_c5_unweighted_closed_loop() {
    // C5 cycle with unit weights
    let problem = TravelingSalesman::<SimpleGraph, i32>::unweighted(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)],
    );

    let reduction: ReductionTSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    let metric = problem.evaluate(&extracted);
    assert!(metric.is_valid());
    assert_eq!(metric, SolutionSize::Valid(5));
}

#[test]
fn test_no_hamiltonian_cycle_infeasible() {
    // Path graph 0-1-2-3: no Hamiltonian cycle exists
    let problem =
        TravelingSalesman::<SimpleGraph, i32>::unweighted(4, vec![(0, 1), (1, 2), (2, 3)]);

    let reduction: ReductionTSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solver = ILPSolver::new();
    let result = ilp_solver.solve(ilp);

    assert!(
        result.is_none(),
        "Path graph should have no Hamiltonian cycle (infeasible ILP)"
    );
}

#[test]
fn test_solution_extraction_structure() {
    // C4 cycle: verify extraction produces correct edge selection format
    let problem =
        TravelingSalesman::<SimpleGraph, i32>::unweighted(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]);
    let reduction: ReductionTSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // Should have one value per edge
    assert_eq!(extracted.len(), 4);
    // All edges should be selected (C4 has unique cycle = all edges)
    assert_eq!(extracted.iter().sum::<usize>(), 4);
}

#[test]
fn test_solve_reduced() {
    // Test via ILPSolver::solve_reduced
    let problem = TravelingSalesman::<SimpleGraph, i32>::new(
        4,
        vec![
            (0, 1, 10),
            (0, 2, 15),
            (0, 3, 20),
            (1, 2, 35),
            (1, 3, 25),
            (2, 3, 30),
        ],
    );

    let ilp_solver = ILPSolver::new();
    let solution = ilp_solver
        .solve_reduced(&problem)
        .expect("solve_reduced should work");

    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());

    // Cross-check with brute force
    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_best(&problem);
    assert_eq!(metric, problem.evaluate(&bf_solutions[0]));
}
