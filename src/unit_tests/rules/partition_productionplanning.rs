use super::*;
use crate::models::misc::{Partition, ProductionPlanning};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;

#[test]
fn test_partition_to_productionplanning_closed_loop() {
    let source = Partition::new(vec![3, 5, 2, 4, 6]);
    let reduction = ReduceTo::<ProductionPlanning>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> ProductionPlanning closed loop",
    );
}

#[test]
fn test_partition_to_productionplanning_structure_even_total() {
    let source = Partition::new(vec![3, 5, 2, 4, 6]);
    let reduction = ReduceTo::<ProductionPlanning>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.demands(), &[0, 0, 0, 0, 0, 10]);
    assert_eq!(target.capacities(), &[3, 5, 2, 4, 6, 0]);
    assert_eq!(target.setup_costs(), &[3, 5, 2, 4, 6, 0]);
    assert_eq!(target.production_costs(), &[0, 0, 0, 0, 0, 0]);
    assert_eq!(target.inventory_costs(), &[0, 0, 0, 0, 0, 0]);
    assert_eq!(target.bound(), 10);
}

#[test]
fn test_partition_to_productionplanning_odd_total_is_infeasible() {
    let source = Partition::new(vec![2, 4, 5]);
    let reduction = ReduceTo::<ProductionPlanning>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.demands(), &[0, 0, 0, 6]);
    assert_eq!(target.capacities(), &[2, 4, 5, 0]);
    assert_eq!(target.setup_costs(), &[2, 4, 5, 0]);
    assert_eq!(target.bound(), 5);
    assert!(BruteForce::new().find_witness(target).is_none());
}

#[test]
fn test_partition_to_productionplanning_extract_solution() {
    let source = Partition::new(vec![3, 5, 2, 4, 6]);
    let reduction = ReduceTo::<ProductionPlanning>::reduce_to(&source);

    assert_eq!(
        reduction.extract_solution(&[0, 0, 0, 4, 6, 0]),
        vec![0, 0, 0, 1, 1]
    );
}
