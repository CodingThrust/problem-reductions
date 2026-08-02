#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionGraph, ReductionResult};
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_naesatisfiability_to_satisfiability_closed_loop() {
    let source = canonical_source();
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "NAESatisfiability -> Satisfiability closed loop",
    );
    assert_eq!(reduction.extract_solution(&[0, 0, 1]), vec![0, 0, 1]);
    assert!(source.evaluate(&[0, 0, 1]).0);
    assert!(reduction.target_problem().evaluate(&[0, 0, 1]).0);
}

#[test]
fn test_naesatisfiability_to_satisfiability_repeated_literal_infeasible() {
    let source = NAESatisfiability::new(1, vec![CNFClause::new(vec![1, 1])]);
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);

    assert_eq!(BruteForce::new().find_witness(&source), None);
    assert_eq!(
        BruteForce::new().find_witness(reduction.target_problem()),
        None
    );
}

#[test]
fn test_naesatisfiability_to_satisfiability_structure_and_overhead() {
    let source = NAESatisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, -2]),
            CNFClause::new(vec![2, 3, -4, 1]),
            CNFClause::new(vec![-1, -1, 4]),
        ],
    );
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), source.num_vars());
    assert_eq!(target.num_clauses(), 2 * source.num_clauses());
    assert_eq!(target.num_literals(), 2 * source.num_literals());
    assert_eq!(
        target.clauses(),
        &[
            CNFClause::new(vec![1, -2]),
            CNFClause::new(vec![-1, 2]),
            CNFClause::new(vec![2, 3, -4, 1]),
            CNFClause::new(vec![-2, -3, 4, -1]),
            CNFClause::new(vec![-1, -1, 4]),
            CNFClause::new(vec![1, 1, -4]),
        ]
    );

    let entry = inventory::iter::<crate::rules::ReductionEntry>()
        .find(|entry| {
            entry.source_name == "NAESatisfiability" && entry.target_name == "Satisfiability"
        })
        .expect("NAESatisfiability -> Satisfiability reduction should be registered");
    let overhead = (entry.overhead_eval_fn)(&source as &dyn std::any::Any);
    assert_eq!(overhead.get("num_vars"), Some(target.num_vars()));
    assert_eq!(overhead.get("num_clauses"), Some(target.num_clauses()));
    assert_eq!(overhead.get("num_literals"), Some(target.num_literals()));
}

#[test]
fn test_naesatisfiability_to_satisfiability_edge_case_semantics() {
    let formulas = [
        NAESatisfiability::new(0, vec![]),
        NAESatisfiability::new(5, vec![CNFClause::new(vec![1, 2, -3, 4, -5])]),
        NAESatisfiability::new(2, vec![CNFClause::new(vec![1, 1, -2])]),
        NAESatisfiability::new(2, vec![CNFClause::new(vec![1, -1, 2])]),
    ];

    for source in formulas {
        let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);
        for mask in 0..(1usize << source.num_vars()) {
            let config = (0..source.num_vars())
                .map(|bit| (mask >> bit) & 1)
                .collect::<Vec<_>>();
            assert_eq!(
                source.evaluate(&config),
                reduction.target_problem().evaluate(&config),
                "evaluation differs for config {config:?}",
            );
            assert_eq!(reduction.extract_solution(&config), config);
        }
    }
}

#[test]
fn test_reduction_graph_registers_naesatisfiability_to_satisfiability() {
    assert!(
        ReductionGraph::new().has_direct_reduction_by_name("NAESatisfiability", "Satisfiability")
    );
}

#[cfg(feature = "example-db")]
#[test]
fn test_naesatisfiability_to_satisfiability_canonical_example_spec() {
    let example = (canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "naesatisfiability_to_satisfiability")
        .expect("missing canonical NAESatisfiability -> Satisfiability example spec")
        .build)();

    assert_eq!(example.source.problem, "NAESatisfiability");
    assert_eq!(example.target.problem, "Satisfiability");
    assert_eq!(example.source.instance["num_vars"], serde_json::json!(3));
    assert_eq!(
        example.target.instance["clauses"].as_array().unwrap().len(),
        6
    );
    assert_eq!(example.solutions.len(), 1);
    assert_eq!(example.solutions[0].source_config, vec![0, 0, 1]);
    assert_eq!(example.solutions[0].target_config, vec![0, 0, 1]);

    let source: NAESatisfiability = serde_json::from_value(example.source.instance.clone())
        .expect("source example deserializes");
    let target: Satisfiability = serde_json::from_value(example.target.instance.clone())
        .expect("target example deserializes");
    assert!(source.evaluate(&example.solutions[0].source_config).0);
    assert!(target.evaluate(&example.solutions[0].target_config).0);
}
