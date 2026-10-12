use crate::models::formula::NAESatisfiability;
use crate::models::set::SetSplitting;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn rule_example_problem() -> SetSplitting {
    SetSplitting::new(4, vec![vec![0, 1], vec![1, 2, 3], vec![0, 2, 3]])
}

#[test]
fn test_setsplitting_to_naesatisfiability_closed_loop() {
    let source = rule_example_problem();
    let reduction =
        ReduceTo::<NAESatisfiability>::reduce_to(&source).expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "SetSplitting -> NAE-SAT",
    );
}

#[test]
fn test_setsplitting_to_naesatisfiability_structure() {
    let source = rule_example_problem();
    let reduction =
        ReduceTo::<NAESatisfiability>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 4);
    assert_eq!(target.num_clauses(), 3);
    assert_eq!(
        target
            .clauses()
            .iter()
            .map(|clause| clause.literals.clone())
            .collect::<Vec<_>>(),
        vec![vec![1, 2], vec![2, 3, 4], vec![1, 3, 4]],
    );
}

#[test]
fn test_setsplitting_to_naesatisfiability_repeated_members() {
    let source = SetSplitting::new(1, vec![vec![0, 0]]);
    let reduction =
        ReduceTo::<NAESatisfiability>::reduce_to(&source).expect("reduction should succeed");

    assert_eq!(reduction.target_problem().clauses()[0].literals, [1, 1]);
    assert!(BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
        .is_none());
}

#[test]
fn test_setsplitting_to_naesatisfiability_extract_solution() {
    let source = rule_example_problem();
    let reduction =
        ReduceTo::<NAESatisfiability>::reduce_to(&source).expect("reduction should succeed");

    assert_eq!(
        reduction
            .extract_solution(&vec![true, false, true, false])
            .unwrap(),
        vec![true, false, true, false]
    );
}

#[test]
fn test_setsplitting_to_naesatisfiability_target_witness_extracts_to_satisfying_assignment() {
    let source = rule_example_problem();
    let reduction =
        ReduceTo::<NAESatisfiability>::reduce_to(&source).expect("reduction should succeed");
    let solver = BruteForce::new();

    let target_solution = solver.solve(reduction.target_problem()).unwrap().unwrap();
    let source_solution = reduction.extract_solution(&target_solution).unwrap();

    assert!(source.evaluate(&source_solution).unwrap());
}

#[cfg(feature = "example-db")]
#[test]
fn test_setsplitting_to_naesatisfiability_canonical_example_spec() {
    let specs = crate::rules::setsplitting_naesatisfiability::canonical_rule_example_specs();
    assert_eq!(specs.len(), 1);

    let example = (specs[0].build)();
    assert_eq!(example.source.problem, "SetSplitting");
    assert_eq!(example.target.problem, "NAESatisfiability");
    assert_eq!(example.solutions.len(), 1);

    let pair = &example.solutions[0];
    assert_eq!(
        pair.source_config,
        serde_json::json!([true, false, true, false])
    );
    assert_eq!(
        pair.target_config,
        serde_json::json!([true, false, true, false])
    );
}
