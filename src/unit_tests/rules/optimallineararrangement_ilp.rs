use super::*;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Path P4: 0-1-2-3
    let problem = OptimalLinearArrangement::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction: ReductionOLAToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // num_x=16, p_v=4, z_e=3, total=23
    assert_eq!(ilp.num_vars, 23);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_optimallineararrangement_to_ilp_closed_loop() {
    // Path graph (identity permutation achieves cost 3)
    let problem = OptimalLinearArrangement::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    // BruteForce on source to verify feasibility
    let bf = BruteForce::new();
    let bf_solution = bf
        .find_witness(&problem)
        .expect("brute-force should find a solution");
    assert!(problem.evaluate(&bf_solution).0.is_some());

    // Solve via ILP
    let reduction: ReductionOLAToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert!(
        problem.evaluate(&extracted).0.is_some(),
        "ILP solution should produce a valid arrangement"
    );
}

#[test]
fn test_optimallineararrangement_to_ilp_with_chords() {
    // 6 vertices, path + chords
    let problem = OptimalLinearArrangement::new(SimpleGraph::new(
        6,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 3), (2, 5)],
    ));

    // BruteForce on source
    let bf = BruteForce::new();
    let bf_solution = bf
        .find_witness(&problem)
        .expect("brute-force should find a solution");
    assert!(problem.evaluate(&bf_solution).0.is_some());

    // Solve via ILP
    let reduction: ReductionOLAToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert!(problem.evaluate(&extracted).0.is_some());
}

#[test]
fn test_optimallineararrangement_to_ilp_optimization() {
    // Path P4: optimal cost is 3
    let problem = OptimalLinearArrangement::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction: ReductionOLAToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    // Cannot brute-force ILP<i32> (integer domain too large), so compare BF source vs ILP solver
    let bf = BruteForce::new();
    let bf_value = bf.solve(&problem);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);
    assert_eq!(
        bf_value, ilp_value,
        "BF and ILP should agree on optimal value"
    );
}

#[test]
fn test_solution_extraction() {
    let problem = OptimalLinearArrangement::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction: ReductionOLAToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert!(problem.evaluate(&extracted).0.is_some());
}

#[test]
fn test_optimallineararrangement_to_ilp_bf_vs_ilp() {
    let problem = OptimalLinearArrangement::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction: ReductionOLAToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}
