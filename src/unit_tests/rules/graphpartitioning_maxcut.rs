use crate::models::graph::{GraphPartitioning, MaxCut};
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

fn issue_example() -> GraphPartitioning<SimpleGraph> {
    super::issue_example()
}

#[test]
fn test_graphpartitioning_to_maxcut_closed_loop() {
    let source = issue_example();
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&source);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "GraphPartitioning->MaxCut closed loop",
    );
}

#[test]
fn test_graphpartitioning_to_maxcut_target_structure() {
    let source = issue_example();
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();
    let num_vertices = source.num_vertices();
    let penalty = i32::try_from(source.num_edges()).unwrap() + 1;

    assert_eq!(target.num_vertices(), num_vertices);
    assert_eq!(target.num_edges(), num_vertices * (num_vertices - 1) / 2);

    for u in 0..num_vertices {
        for v in (u + 1)..num_vertices {
            let expected_weight = if source.graph().has_edge(u, v) {
                penalty - 1
            } else {
                penalty
            };
            assert_eq!(
                target.edge_weight(u, v),
                Some(&expected_weight),
                "unexpected weight on edge ({u}, {v})"
            );
        }
    }
}

#[test]
fn test_graphpartitioning_to_maxcut_extract_solution_identity() {
    let source = issue_example();
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&source);
    let target_solution = super::ISSUE_EXAMPLE_WITNESS.to_vec();

    assert_eq!(reduction.extract_solution(&target_solution), target_solution);
}

#[test]
fn test_graphpartitioning_to_maxcut_penalty_overflow_panics() {
    let result = std::panic::catch_unwind(|| super::penalty_weight(i32::MAX as usize));
    assert!(result.is_err());
}
