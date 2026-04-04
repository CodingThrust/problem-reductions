use super::*;
use crate::models::misc::{OpenShopScheduling, Partition};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_partition_to_open_shop_scheduling_closed_loop() {
    let source = Partition::new(vec![1, 2, 3]);
    let reduction = ReduceTo::<OpenShopScheduling>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "Partition -> OpenShopScheduling closed loop",
    );
}

#[test]
fn test_partition_to_open_shop_scheduling_structure() {
    let source = Partition::new(vec![1, 2, 3]);
    let reduction = ReduceTo::<OpenShopScheduling>::reduce_to(&source);
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
    let source = Partition::new(vec![1, 2, 3]);
    let reduction = ReduceTo::<OpenShopScheduling>::reduce_to(&source);

    assert_eq!(
        reduction.extract_solution(&[0, 0, 0, 0, 2, 2, 0, 0, 3, 0, 0, 0]),
        vec![0, 0, 1]
    );
}

#[test]
fn test_partition_to_open_shop_scheduling_odd_total_is_not_satisfying() {
    let source = Partition::new(vec![2, 4, 5]);
    let reduction = ReduceTo::<OpenShopScheduling>::reduce_to(&source);
    let target = reduction.target_problem();
    let best = BruteForce::new()
        .find_witness(target)
        .expect("open-shop target should always have an optimal solution");

    assert_eq!(target.evaluate(&best), Min(Some(16)));
    assert!(!source.evaluate(&reduction.extract_solution(&best)));
}
