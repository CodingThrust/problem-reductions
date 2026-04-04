use super::*;
use crate::models::algebraic::ILP;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;

#[test]
fn test_sequencingtominimizemaximumcumulativecost_to_ilp_closed_loop() {
    let problem = SequencingToMinimizeMaximumCumulativeCost::new(vec![2, -1, 3, -2], vec![(0, 2)]);
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    // Brute-force the source to get the optimal value
    let bf = BruteForce::new();
    let bf_solution = bf.find_witness(&problem).expect("brute-force optimum");
    let bf_value = problem.evaluate(&bf_solution);

    // Solve the ILP target with the ILP solver
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert!(
        ilp_value.0.is_some(),
        "Extracted solution should be feasible"
    );
    assert_eq!(
        ilp_value, bf_value,
        "ILP-extracted solution should match brute-force optimum"
    );
}

#[test]
fn test_sequencingtominimizemaximumcumulativecost_to_ilp_bf_vs_ilp() {
    let problem = SequencingToMinimizeMaximumCumulativeCost::new(vec![2, -1, 3, -2], vec![(0, 2)]);
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    let bf_witness = BruteForce::new()
        .find_witness(&problem)
        .expect("should be feasible");
    assert!(problem.evaluate(&bf_witness).0.is_some());

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert!(problem.evaluate(&extracted).0.is_some());
}

#[test]
fn test_sequencingtominimizemaximumcumulativecost_to_ilp_no_precedences() {
    let problem = SequencingToMinimizeMaximumCumulativeCost::new(vec![3, -2, 1], vec![]);
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert!(problem.evaluate(&extracted).0.is_some());
}
