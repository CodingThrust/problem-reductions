use crate::models::formula::NAESatisfiability;
use crate::models::set::SetSplitting;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;

fn issue_example() -> SetSplitting {
    SetSplitting::new(4, vec![vec![0, 1], vec![1, 2, 3], vec![0, 2, 3]])
}

#[test]
fn test_setsplitting_to_naesatisfiability_closed_loop() {
    let source = issue_example();
    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "SetSplitting -> NAE-SAT",
    );
    assert_eq!(reduction.extract_solution(&[0, 1, 0, 1]), vec![0, 1, 0, 1]);
}

#[test]
fn test_setsplitting_to_naesatisfiability_structure_and_overhead() {
    let source = issue_example();
    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 4);
    assert_eq!(target.num_clauses(), 3);
    assert_eq!(target.num_literals(), 8);
    assert_eq!(
        target
            .clauses()
            .iter()
            .map(|clause| clause.literals.clone())
            .collect::<Vec<_>>(),
        vec![vec![1, 2], vec![2, 3, 4], vec![1, 3, 4]],
    );

    let entry = inventory::iter::<crate::rules::ReductionEntry>()
        .find(|entry| {
            entry.source_name == "SetSplitting" && entry.target_name == "NAESatisfiability"
        })
        .expect("SetSplitting -> NAESatisfiability reduction should be registered");
    let overhead = (entry.overhead_eval_fn)(&source as &dyn std::any::Any);

    assert_eq!(overhead.get("num_vars"), Some(target.num_vars()));
    assert_eq!(overhead.get("num_clauses"), Some(target.num_clauses()));
    assert_eq!(overhead.get("num_literals"), Some(15));
    assert!(target.num_literals() <= overhead.get("num_literals").unwrap());
}

#[test]
fn test_setsplitting_to_naesatisfiability_deduplicates_in_first_occurrence_order() {
    let source = SetSplitting::new(4, vec![vec![2, 0, 2, 1, 0]]);
    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&source);

    assert_eq!(
        reduction.target_problem().clauses()[0].literals,
        vec![3, 1, 2]
    );
}

#[test]
fn test_setsplitting_to_naesatisfiability_all_repeated_subset_is_infeasible() {
    let source = SetSplitting::new(1, vec![vec![0, 0]]);
    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&source);

    assert_eq!(reduction.target_problem().clauses()[0].literals, vec![1, 1]);
    assert!(BruteForce::new().find_witness(&source).is_none());
    assert!(BruteForce::new()
        .find_witness(reduction.target_problem())
        .is_none());
}

#[test]
fn test_setsplitting_to_naesatisfiability_empty_family_and_unused_element() {
    let empty_source = SetSplitting::new(3, vec![]);
    let empty_reduction = ReduceTo::<NAESatisfiability>::reduce_to(&empty_source);
    assert_eq!(empty_reduction.target_problem().num_vars(), 3);
    assert!(empty_reduction.target_problem().clauses().is_empty());

    let source = SetSplitting::new(5, vec![vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3, 1]]);
    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&source);
    assert_eq!(reduction.target_problem().num_vars(), 5);
    assert_eq!(
        reduction
            .target_problem()
            .clauses()
            .iter()
            .map(|clause| clause.literals.clone())
            .collect::<Vec<_>>(),
        vec![vec![1, 2], vec![1, 2, 3], vec![1, 2, 3, 4]],
    );
}

#[cfg(feature = "example-db")]
#[test]
fn test_setsplitting_to_naesatisfiability_canonical_example_spec() {
    let specs = crate::rules::setsplitting_naesatisfiability::canonical_rule_example_specs();
    assert_eq!(specs.len(), 1);

    let example = (specs[0].build)();
    assert_eq!(example.source.problem, "SetSplitting");
    assert_eq!(example.target.problem, "NAESatisfiability");
    assert_eq!(example.source.instance["universe_size"], 4);
    assert_eq!(
        example.target.instance["clauses"],
        serde_json::json!([
            { "literals": [1, 2] },
            { "literals": [2, 3, 4] },
            { "literals": [1, 3, 4] },
        ]),
    );

    let pair = &example.solutions[0];
    assert_eq!(pair.source_config, vec![0, 1, 0, 1]);
    assert_eq!(pair.target_config, vec![0, 1, 0, 1]);
}
