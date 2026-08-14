use super::*;
use crate::models::graph::IntegralFlowWithMultipliers;
use crate::models::misc::Partition;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;

#[test]
fn test_partition_to_integralflowwithmultipliers_closed_loop() {
    let source = Partition::new(vec![1, 2, 3]);
    let reduction = ReduceTo::<IntegralFlowWithMultipliers>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "Partition -> IntegralFlowWithMultipliers closed loop",
    );
}

#[test]
fn test_partition_to_integralflowwithmultipliers_structure_even_total() {
    let source = Partition::new(vec![2, 3, 4, 5, 6, 4]);
    let reduction = ReduceTo::<IntegralFlowWithMultipliers>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 9);
    assert_eq!(
        target.graph().arcs(),
        vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 5),
            (0, 6),
            (1, 7),
            (2, 7),
            (3, 7),
            (4, 7),
            (5, 7),
            (6, 7),
            (7, 8),
        ]
    );
    assert_eq!(
        target.capacities(),
        &[1, 1, 1, 1, 1, 1, 2, 3, 4, 5, 6, 4, 12]
    );
    assert_eq!(target.multipliers(), &[1, 2, 3, 4, 5, 6, 4, 1, 1]);
    assert_eq!(target.requirement(), 12);
}

#[test]
fn test_partition_to_integralflowwithmultipliers_even_no_instance_uses_bottleneck() {
    let source = Partition::new(vec![3, 5]);
    let reduction = ReduceTo::<IntegralFlowWithMultipliers>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.capacities(), &[1, 1, 3, 5, 4]);
    assert!(BruteForce::new().find_witness(target).is_none());
}

#[test]
fn test_partition_to_integralflowwithmultipliers_odd_total_is_fixed_no_instance() {
    let source = Partition::new(vec![1, 2]);
    let reduction = ReduceTo::<IntegralFlowWithMultipliers>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 3);
    assert_eq!(target.graph().arcs(), vec![(0, 1), (1, 2)]);
    assert_eq!(target.multipliers(), &[1, 2, 1]);
    assert_eq!(target.capacities(), &[1, 1]);
    assert_eq!(target.requirement(), 1);
    assert!(BruteForce::new().find_witness(target).is_none());
    assert_eq!(
        reduction.extract_solution(&[]).unwrap_err().to_string(),
        "the fixed infeasible target instance has no extractable witness"
    );
}

#[test]
fn test_partition_to_integralflowwithmultipliers_extract_solution() {
    let source = Partition::new(vec![2, 3, 4, 5, 6, 4]);
    let reduction = ReduceTo::<IntegralFlowWithMultipliers>::reduce_to(&source);

    assert_eq!(
        reduction
            .extract_solution(&[1, 0, 1, 0, 1, 0, 2, 0, 4, 0, 6, 0, 12])
            .unwrap(),
        vec![1, 0, 1, 0, 1, 0]
    );
}

#[cfg(feature = "example-db")]
#[test]
fn test_partition_to_integralflowwithmultipliers_canonical_example_spec() {
    let example = (canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "partition_to_integralflowwithmultipliers")
        .expect("canonical example spec should exist")
        .build)();

    assert_eq!(example.solutions.len(), 1);
    let solution = &example.solutions[0];
    assert_eq!(solution.source_config, vec![1, 0, 1, 0, 1, 0]);
    assert_eq!(
        solution.target_config,
        vec![1, 0, 1, 0, 1, 0, 2, 0, 4, 0, 6, 0, 12]
    );
}
