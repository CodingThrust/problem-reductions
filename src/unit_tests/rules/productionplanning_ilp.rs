use super::*;
use crate::{
    solvers::{BruteForce, ILPSolveError, ILPSolver},
    Problem,
};

#[test]
fn inventory_and_setup_formulation_matches_exhaustive_production_plans() {
    for capacities in [vec![2, 0, 1], vec![0, 0, 0], vec![2, 2, 2]] {
        for demands in [vec![1, 1, 1], vec![0, 0, 0], vec![0, 1, 2]] {
            for budget in [0, 3, 7, 8] {
                let source = ProductionPlanning::new(
                    3,
                    demands.clone(),
                    capacities.clone(),
                    vec![2; 3],
                    vec![1; 3],
                    vec![1; 3],
                    budget,
                );
                let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
                crate::rules::test_helpers::assert_parameter_predictions(&source, &reduction);
                match (
                    BruteForce::new().solve(&source).unwrap(),
                    ILPSolver::new().solve(reduction.target_problem()),
                ) {
                    (Some(_), Ok(witness)) => assert!(
                        source
                            .evaluate(&reduction.extract_solution(&witness).unwrap())
                            .unwrap()
                            .0
                    ),
                    (None, Err(ILPSolveError::Infeasible)) => {}
                    other => panic!("planning disagreement: {other:?}"),
                }
                assert!(reduction.extract_solution(&vec![]).is_err());
                assert!(reduction.extract_solution(&vec![-1; 9]).is_err());
            }
        }
    }
}

#[test]
fn unrepresentable_planning_arithmetic_is_an_error_not_infeasibility() {
    for source in [
        ProductionPlanning::new(
            2,
            vec![i64::MAX, 1],
            vec![0; 2],
            vec![0; 2],
            vec![0; 2],
            vec![0; 2],
            0,
        ),
        ProductionPlanning::new(
            2,
            vec![0; 2],
            vec![i64::MAX, 1],
            vec![0; 2],
            vec![0; 2],
            vec![0; 2],
            0,
        ),
        ProductionPlanning::new(1, vec![0], vec![2], vec![0], vec![i64::MAX], vec![0], 0),
    ] {
        assert!(matches!(
            ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source),
            Err(crate::rules::ReductionError::IntegerOverflow { .. })
        ));
    }
}
