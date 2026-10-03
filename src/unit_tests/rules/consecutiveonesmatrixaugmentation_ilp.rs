use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_coma_to_ilp_structure() {
    let problem = ConsecutiveOnesMatrixAugmentation::new(
        vec![vec![true, false, true], vec![false, true, true]],
        1,
    );
    let reduction: ReductionCOMAToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars(), 13);
    assert_eq!(ilp.num_constraints(), 15);
    assert_eq!(ilp.num_nonzeros(), 46);
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);
}

#[test]
fn test_coma_to_ilp_closed_loop() {
    let problem = ConsecutiveOnesMatrixAugmentation::new(
        vec![vec![true, false, true], vec![false, true, true]],
        1,
    );
    let reduction: ReductionCOMAToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&problem).expect("reduction should succeed");

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(problem.evaluate(&extracted).unwrap(), Or(true));

    // Also verify that brute-force on the source agrees
    let bf = BruteForce::new();
    let bf_witness = bf.solve(&problem).unwrap().expect("should be feasible");
    assert_eq!(problem.evaluate(&bf_witness).unwrap(), Or(true));
}

#[test]
fn test_coma_to_ilp_bf_vs_ilp() {
    let problem = ConsecutiveOnesMatrixAugmentation::new(
        vec![vec![true, false, true], vec![false, true, true]],
        1,
    );
    let reduction: ReductionCOMAToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&problem).expect("reduction should succeed");

    let bf = BruteForce::new();
    let bf_witness = bf.solve(&problem).unwrap().expect("should be feasible");
    assert_eq!(problem.evaluate(&bf_witness).unwrap(), Or(true));

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(problem.evaluate(&extracted).unwrap(), Or(true));
}

#[test]
fn test_coma_to_ilp_trivial() {
    // 1x1 matrix, bound 0 — already consecutive
    let problem = ConsecutiveOnesMatrixAugmentation::new(vec![vec![true]], 0);
    let reduction: ReductionCOMAToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars(), 3);
}

#[test]
fn test_augmentation_threshold_normalization() {
    for bound in [0, 1, i64::MAX] {
        let source = ConsecutiveOnesMatrixAugmentation::new(vec![vec![true]], bound);
        let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
        assert_eq!(
            reduction.target_problem().max_constraint_magnitude_bits(),
            1
        );
        let solution = ILPSolver::new().solve(reduction.target_problem()).unwrap();
        assert_eq!(
            source
                .evaluate(&reduction.extract_solution(&solution).unwrap())
                .unwrap(),
            Or(true)
        );
    }
}

#[test]
fn interval_endpoints_preserve_zero_rows_and_exact_budget() {
    for (matrix, bound, feasible) in [
        (vec![], 0, true),
        (vec![vec![], vec![]], 0, true),
        (vec![vec![false; 3]], 0, true),
        (vec![vec![true; 3]], 0, true),
        (
            vec![
                vec![true, true, false],
                vec![true, false, true],
                vec![false, true, true],
            ],
            0,
            false,
        ),
        (
            vec![
                vec![true, true, false],
                vec![true, false, true],
                vec![false, true, true],
            ],
            1,
            true,
        ),
    ] {
        let source = ConsecutiveOnesMatrixAugmentation::new(matrix, bound);
        let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
        match ILPSolver::new().solve(reduction.target_problem()) {
            Ok(solution) => {
                assert!(feasible);
                assert_eq!(
                    source
                        .evaluate(&reduction.extract_solution(&solution).unwrap())
                        .unwrap(),
                    Or(true)
                );
            }
            Err(crate::solvers::ILPSolveError::Infeasible) => assert!(!feasible),
            Err(error) => panic!("unexpected solver failure: {error}"),
        }
    }
}

#[test]
fn loose_intervals_spend_budget_and_infeasible_targets_are_rejected() {
    let solution = vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 2];
    for bound in [1, 2] {
        let source = ConsecutiveOnesMatrixAugmentation::new(vec![vec![false, true, false]], bound);
        let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
        if bound == 2 {
            assert_eq!(
                reduction.extract_solution(&solution).unwrap(),
                vec![0, 1, 2]
            );
        } else {
            assert!(reduction.extract_solution(&solution).is_err());
        }
    }
}
