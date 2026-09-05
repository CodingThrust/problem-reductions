use super::*;
use crate::models::algebraic::ILP;
use crate::models::misc::EnsembleComputation;
use crate::rules::ReduceTo;
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Min;

fn feasible_instance() -> EnsembleComputation {
    EnsembleComputation::new(4, vec![vec![0, 1], vec![0, 1, 2, 3]], 3)
}

#[test]
fn test_ensemblecomputation_to_ilp_structure() {
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&feasible_instance()).unwrap();
    assert_eq!(reduction.target_problem().num_vars(), 75);
    assert_eq!(reduction.target_problem().constraints().len(), 154);
}

#[test]
fn test_ensemblecomputation_to_ilp_closed_loop() {
    let source = feasible_instance();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).unwrap();
    let target_solution = ILPSolver::new().solve(reduction.target_problem()).unwrap();
    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(source.evaluate(&extracted).unwrap(), Min(Some(3)));
}

#[test]
fn test_ensemblecomputation_to_ilp_infeasible_budget() {
    let source = EnsembleComputation::new(3, vec![vec![0, 1, 2]], 1);
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).unwrap();
    assert!(ILPSolver::new().solve(reduction.target_problem()).is_err());
}

#[test]
fn test_ensemblecomputation_to_ilp_rejects_singleton_target() {
    let source = EnsembleComputation::new(3, vec![vec![0]], 2);
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).unwrap();
    assert!(ILPSolver::new().solve(reduction.target_problem()).is_err());
}

#[test]
fn test_ensemblecomputation_to_ilp_empty_family() {
    let source = EnsembleComputation::new(1, vec![], 2);
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).unwrap();
    let target_solution = ILPSolver::new().solve(reduction.target_problem()).unwrap();
    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(source.evaluate(&extracted).unwrap(), Min(Some(0)));
}
