use super::*;
use crate::models::algebraic::MinimumMatrixCover;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_minimum_matrix_cover_to_ilp_closed_loop() {
    let problem = MinimumMatrixCover::new(vec![
        vec![0, 3, 1, 0],
        vec![3, 0, 0, 2],
        vec![1, 0, 0, 4],
        vec![0, 2, 4, 0],
    ]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_optimization_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "MinimumMatrixCover->ILP closed loop",
    );

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let value = problem.evaluate(&extracted);
    assert_eq!(value, Min(Some(-20)));
}

#[test]
fn test_minimum_matrix_cover_to_ilp_structure() {
    let problem = MinimumMatrixCover::new(vec![vec![0, 3], vec![2, 0]]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // n=2: 2 sign vars + 1 auxiliary = 3 vars
    assert_eq!(ilp.num_vars, 3);
    // 3 constraints per pair, 1 pair
    assert_eq!(ilp.constraints.len(), 3);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);

    // y_{01} coefficient: 4*(a_01 + a_10) = 4*(3+2) = 20
    // x_0 coefficient: -2*(a_01+a_10) = -2*(3+2) = -10
    // x_1 coefficient: -2*(a_10+a_01) = -2*(2+3) = -10
    let obj_map: std::collections::HashMap<usize, f64> = ilp.objective.iter().copied().collect();
    assert_eq!(*obj_map.get(&0).unwrap_or(&0.0), -10.0);
    assert_eq!(*obj_map.get(&1).unwrap_or(&0.0), -10.0);
    assert_eq!(*obj_map.get(&2).unwrap_or(&0.0), 20.0);
}

#[test]
fn test_minimum_matrix_cover_to_ilp_bf_vs_ilp() {
    let problem = MinimumMatrixCover::new(vec![
        vec![0, 3, 1, 0],
        vec![3, 0, 0, 2],
        vec![1, 0, 0, 4],
        vec![0, 2, 4, 0],
    ]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let bf_value = BruteForce::new().solve(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(bf_value, ilp_value);
}

#[test]
fn test_minimum_matrix_cover_to_ilp_2x2() {
    let problem = MinimumMatrixCover::new(vec![vec![0, 3], vec![2, 0]]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let value = problem.evaluate(&extracted);
    // Optimal: different signs → value = -(3+2) = -5
    assert_eq!(value, Min(Some(-5)));
}

#[test]
fn test_minimum_matrix_cover_to_ilp_1x1() {
    let problem = MinimumMatrixCover::new(vec![vec![5]]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 1 variable, 0 pairs → 0 auxiliaries
    assert_eq!(ilp.num_vars, 1);
    assert_eq!(ilp.constraints.len(), 0);

    // For 1×1, f(1)²=1, value = 5 regardless. Objective should be constant (no x terms).
    // x_0 coefficient: -2*Σ_{j≠0} (a_0j+a_j0) = 0 (no off-diagonal)
    // The ILP finds any assignment; extracted solution gives value 5.
    let ilp_solution = ILPSolver::new()
        .solve(ilp)
        .expect("1x1 ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Min(Some(5)));
}

#[test]
fn test_minimum_matrix_cover_to_ilp_diagonal_matrix() {
    // Diagonal matrix: all off-diagonal entries are 0
    // Value is always Σ a_ii (constant), since f(i)²=1
    let problem = MinimumMatrixCover::new(vec![vec![2, 0, 0], vec![0, 3, 0], vec![0, 0, 1]]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("diagonal ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    // All configs give value 2+3+1 = 6
    assert_eq!(problem.evaluate(&extracted), Min(Some(6)));
}

#[test]
fn test_minimum_matrix_cover_to_ilp_asymmetric() {
    // Non-symmetric matrix
    let problem = MinimumMatrixCover::new(vec![vec![0, 5], vec![1, 0]]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let bf_value = BruteForce::new().solve(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(bf_value, ilp_value);
    // Different signs: -(5+1) = -6, same signs: +(5+1) = 6
    assert_eq!(ilp_value, Min(Some(-6)));
}

#[cfg(feature = "example-db")]
#[test]
fn test_minimum_matrix_cover_to_ilp_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "minimum_matrix_cover_to_ilp")
        .expect("missing canonical MinimumMatrixCover -> ILP example spec");
    let example = (spec.build)();

    assert_eq!(example.source.problem, "MinimumMatrixCover");
    assert_eq!(example.target.problem, "ILP");
    assert_eq!(
        example.source.instance["matrix"].as_array().unwrap().len(),
        2
    );
    assert_eq!(example.target.instance["num_vars"], 3);
}
