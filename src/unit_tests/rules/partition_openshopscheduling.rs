use super::*;
use crate::models::algebraic::ILP;
use crate::models::misc::{OpenShopScheduling, Partition};
use crate::solvers::ILPSolver;
use crate::traits::Problem;

fn solve_target(target: &OpenShopScheduling) -> Vec<usize> {
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(target).expect("ILP reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("open-shop target should be feasible");
    reduction.extract_solution(&ilp_solution).unwrap()
}

#[test]
fn test_partition_to_open_shop_scheduling_closed_loop() {
    let source = Partition::new(vec![1, 2, 3]).unwrap();
    let reduction = ReduceTo::<OpenShopScheduling>::reduce_to(&source).unwrap();
    let target_solution = solve_target(reduction.target_problem());
    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert!(source.evaluate(&extracted).unwrap());
}

#[test]
fn test_partition_to_open_shop_scheduling_structure() {
    let source = Partition::new(vec![1, 2, 3]).unwrap();
    let reduction =
        ReduceTo::<OpenShopScheduling>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_jobs(), 4);
    assert_eq!(target.num_machines(), 3);
    assert_eq!(
        target.processing_times(),
        &[vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3], vec![3, 3, 3]]
    );
}

#[test]
fn test_partition_to_open_shop_scheduling_extract_solution() {
    let source = Partition::new(vec![1, 2, 3]).unwrap();
    let reduction = ReduceTo::<OpenShopScheduling>::reduce_to(&source).unwrap();
    let target_solution = solve_target(reduction.target_problem());
    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(extracted.len(), 3);
    assert!(source.evaluate(&extracted).unwrap());
}

#[test]
fn test_partition_to_open_shop_scheduling_odd_total_is_not_satisfying() {
    let source = Partition::new(vec![2, 4, 5]).unwrap();
    let reduction = ReduceTo::<OpenShopScheduling>::reduce_to(&source).unwrap();
    let best = solve_target(reduction.target_problem());
    assert!(reduction.extract_solution(&best).is_err());
}

#[test]
fn test_partition_to_open_shop_scheduling_preserves_construction_overflow() {
    let source = Partition::new(vec![1_i64 << 61, 1_i64 << 61]).unwrap();
    let error = ReduceTo::<OpenShopScheduling>::reduce_to(&source).unwrap_err();
    assert!(matches!(
        error,
        crate::rules::ReductionError::Construction {
            source_problem: "Partition",
            target_problem: "OpenShopScheduling",
            cause: crate::registry::ConstructionError::IntegerOverflow(_),
        }
    ));
}
