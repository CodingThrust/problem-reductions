use super::*;
use crate::models::formula::{CNFClause, Maximum2Satisfiability};
use crate::models::graph::MaxCut;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Max;

fn make_issue_instance() -> Maximum2Satisfiability {
    Maximum2Satisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2]),
            CNFClause::new(vec![-1, 3]),
            CNFClause::new(vec![2, -3]),
            CNFClause::new(vec![-1, -2]),
            CNFClause::new(vec![1, 3]),
        ],
    )
}

#[test]
fn test_maximum2satisfiability_to_maxcut_closed_loop() {
    let source = make_issue_instance();
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&source);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "Maximum2Satisfiability -> MaxCut closed loop",
    );
}

#[test]
fn test_maximum2satisfiability_to_maxcut_structure() {
    let source = make_issue_instance();
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 4);
    assert_eq!(target.num_edges(), 4);
    assert_eq!(target.edge_weight(0, 1), None);
    assert_eq!(target.edge_weight(0, 2), Some(&-1));
    assert_eq!(target.edge_weight(0, 3), Some(&-1));
    assert_eq!(target.edge_weight(1, 2), Some(&2));
    assert_eq!(target.edge_weight(1, 3), None);
    assert_eq!(target.edge_weight(2, 3), Some(&-1));

    let source_solution = vec![0, 1, 1];
    let target_solution = vec![0, 1, 0, 0];
    assert_eq!(source.evaluate(&source_solution), Max(Some(5)));
    assert_eq!(target.evaluate(&target_solution), Max(Some(2)));
}

#[test]
fn test_maximum2satisfiability_to_maxcut_issue_affine_relation_on_all_partitions() {
    let source = make_issue_instance();
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // For this issue instance, every partition satisfies
    // 2 * satisfied_clauses = 8 + cut_weight.
    for mask in 0..(1usize << target.num_vertices()) {
        let target_solution: Vec<usize> = (0..target.num_vertices())
            .map(|bit| (mask >> bit) & 1)
            .collect();
        let source_solution = reduction.extract_solution(&target_solution);
        let satisfied = source.evaluate(&source_solution).unwrap() as i32;
        let cut_weight = target.evaluate(&target_solution).unwrap();

        assert_eq!(2 * satisfied, 8 + cut_weight, "target config {target_solution:?}");
    }
}

#[test]
fn test_maximum2satisfiability_to_maxcut_extract_solution_uses_reference_vertex() {
    let source = make_issue_instance();
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&source);

    assert_eq!(reduction.extract_solution(&[0, 1, 0, 0]), vec![0, 1, 1]);
    assert_eq!(reduction.extract_solution(&[1, 0, 1, 1]), vec![0, 1, 1]);
    assert_eq!(
        source.evaluate(&reduction.extract_solution(&[1, 0, 1, 1])),
        Max(Some(5))
    );
}

#[test]
fn test_maximum2satisfiability_to_maxcut_handles_duplicate_and_tautological_clauses() {
    let source = Maximum2Satisfiability::new(
        2,
        vec![
            CNFClause::new(vec![1, 1]),
            CNFClause::new(vec![1, -1]),
            CNFClause::new(vec![-2, -2]),
        ],
    );
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 3);
    assert_eq!(target.num_edges(), 2);
    assert_eq!(target.edge_weight(0, 1), Some(&-2));
    assert_eq!(target.edge_weight(0, 2), Some(&2));
    assert_eq!(target.edge_weight(1, 2), None);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "Maximum2Satisfiability -> MaxCut duplicate clauses",
    );
}

#[cfg(feature = "example-db")]
#[test]
fn test_maximum2satisfiability_to_maxcut_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "maximum2satisfiability_to_maxcut")
        .expect("missing canonical Maximum2Satisfiability -> MaxCut example spec");
    let example = (spec.build)();

    assert_eq!(example.source.problem, "Maximum2Satisfiability");
    assert_eq!(example.target.problem, "MaxCut");
    assert_eq!(example.source.instance["num_vars"], 3);
    assert_eq!(example.target.instance["graph"]["num_vertices"], 4);
    assert_eq!(
        example.target.instance["graph"]["edges"]
            .as_array()
            .unwrap()
            .len(),
        4
    );
    assert_eq!(
        example.target.instance["edge_weights"],
        serde_json::json!([-1, -1, 2, -1])
    );
    assert_eq!(
        example.solutions,
        vec![crate::export::SolutionPair {
            source_config: vec![0, 1, 1],
            target_config: vec![0, 1, 0, 0],
        }]
    );
}
