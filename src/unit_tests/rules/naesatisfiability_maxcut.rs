use super::*;
use crate::models::formula::{CNFClause, NAESatisfiability};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Max;

#[test]
fn test_naesatisfiability_to_maxcut_closed_loop() {
    let source = super::issue_example();
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "NAE-SAT -> MaxCut",
    );
}

#[test]
fn test_naesatisfiability_to_maxcut_target_structure() {
    let source = super::issue_example();
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 6);
    assert_eq!(target.num_edges(), 8);

    assert_eq!(target.edge_weight(0, 1), Some(&5));
    assert_eq!(target.edge_weight(2, 3), Some(&5));
    assert_eq!(target.edge_weight(4, 5), Some(&5));
    assert_eq!(target.edge_weight(0, 2), Some(&2));
    assert_eq!(target.edge_weight(2, 4), Some(&1));
    assert_eq!(target.edge_weight(0, 4), Some(&1));
    assert_eq!(target.edge_weight(2, 5), Some(&1));
    assert_eq!(target.edge_weight(0, 5), Some(&1));

    assert_eq!(
        target.evaluate(&super::ISSUE_EXAMPLE_TARGET_CONFIG),
        Max(Some(19))
    );
}

#[test]
fn test_naesatisfiability_to_maxcut_extract_solution_reads_positive_literal_vertices() {
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&super::issue_example());

    assert_eq!(
        reduction.extract_solution(&super::ISSUE_EXAMPLE_TARGET_CONFIG),
        super::ISSUE_EXAMPLE_SOURCE_CONFIG,
    );
}

#[test]
#[should_panic(expected = "requires every clause to have exactly 3 literals")]
fn test_naesatisfiability_to_maxcut_rejects_non_3sat_instances() {
    let source = NAESatisfiability::new(2, vec![CNFClause::new(vec![1, 2])]);

    let _ = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&source);
}

#[test]
fn test_naesatisfiability_to_maxcut_penalty_overflow_panics() {
    let result =
        std::panic::catch_unwind(|| super::variable_gadget_weight((i32::MAX as usize) / 2 + 1));

    assert!(result.is_err());
}

#[cfg(feature = "example-db")]
#[test]
fn test_naesatisfiability_to_maxcut_canonical_example_spec() {
    let specs = crate::rules::naesatisfiability_maxcut::canonical_rule_example_specs();
    assert_eq!(specs.len(), 1);

    let example = (specs[0].build)();
    assert_eq!(example.source.problem, "NAESatisfiability");
    assert_eq!(example.target.problem, "MaxCut");
    assert_eq!(example.solutions.len(), 1);

    let pair = &example.solutions[0];
    assert_eq!(pair.source_config, super::ISSUE_EXAMPLE_SOURCE_CONFIG);
    assert_eq!(pair.target_config, super::ISSUE_EXAMPLE_TARGET_CONFIG);
}
