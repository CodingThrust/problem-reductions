use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Sequence: A, B, A, B => 2 cars, 4 positions
    let problem = PaintShop::new(vec!["A", "B", "A", "B"]);
    let reduction: ReductionPaintShopToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 2 car vars + 4 k vars + 4 c vars = 10
    assert_eq!(ilp.num_vars(), 10);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_paintshop_to_ilp_closed_loop() {
    let problem = PaintShop::new(vec!["A", "B", "A", "C", "B", "C"]);
    let reduction: ReductionPaintShopToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_optimization_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "PaintShop->ILP closed loop",
    );
}

#[test]
fn test_paintshop_to_ilp_bf_vs_ilp() {
    let problem = PaintShop::new(vec!["A", "B", "A", "C", "B", "C"]);
    let reduction: ReductionPaintShopToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_witnesses(&problem);
    let bf_value = problem.evaluate(&bf_solutions[0]);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(bf_value, ilp_value);
}

#[test]
fn test_solution_extraction() {
    // Minimal: A, A => 1 car
    let problem = PaintShop::new(vec!["A", "A"]);
    let reduction: ReductionPaintShopToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted.len(), 1);
    // Either 0 or 1 is valid; coloring is [x, 1-x], switches = 1
    assert!(problem.evaluate(&extracted).is_valid());
}
