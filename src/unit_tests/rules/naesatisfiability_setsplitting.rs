use crate::models::formula::{CNFClause, NAESatisfiability};
use crate::models::set::SetSplitting;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn rule_example_problem() -> NAESatisfiability {
    NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, -2, 3]),
            CNFClause::new(vec![-1, 2, -3]),
        ],
    )
}

#[test]
fn test_naesatisfiability_to_setsplitting_closed_loop() {
    let source = rule_example_problem();
    let reduction = ReduceTo::<SetSplitting>::reduce_to(&source).expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "NAE-SAT -> SetSplitting",
    );
}

#[test]
fn test_naesatisfiability_to_setsplitting_structure() {
    let source = rule_example_problem();
    let reduction = ReduceTo::<SetSplitting>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.universe_size(), 6);
    assert_eq!(target.num_subsets(), 5);
    assert_eq!(
        target.subsets(),
        &[
            vec![0, 3],
            vec![1, 4],
            vec![2, 5],
            vec![0, 4, 2],
            vec![3, 1, 5],
        ],
    );
}

#[test]
fn test_naesatisfiability_to_setsplitting_extract_solution_uses_positive_literals() {
    let source = rule_example_problem();
    let reduction = ReduceTo::<SetSplitting>::reduce_to(&source).expect("reduction should succeed");

    assert_eq!(
        reduction
            .extract_solution(&vec![true, false, true, false, true, false])
            .unwrap(),
        vec![true, false, true]
    );
}

#[test]
fn test_naesatisfiability_to_setsplitting_target_witness_extracts_to_satisfying_assignment() {
    let source = rule_example_problem();
    let reduction = ReduceTo::<SetSplitting>::reduce_to(&source).expect("reduction should succeed");
    let solver = BruteForce::new();

    let target_solution = solver.solve(reduction.target_problem()).unwrap().unwrap();
    let source_solution = reduction.extract_solution(&target_solution).unwrap();

    assert!(source.evaluate(&source_solution).unwrap());
}

#[cfg(feature = "example-db")]
#[test]
fn test_naesatisfiability_to_setsplitting_canonical_example_spec() {
    let specs = crate::rules::naesatisfiability_setsplitting::canonical_rule_example_specs();
    assert_eq!(specs.len(), 1);

    let example = (specs[0].build)();
    assert_eq!(example.source.problem, "NAESatisfiability");
    assert_eq!(example.target.problem, "SetSplitting");
    assert_eq!(example.solutions.len(), 1);

    let pair = &example.solutions[0];
    assert_eq!(pair.source_config, serde_json::json!([true, true, true]));
    assert_eq!(
        pair.target_config,
        serde_json::json!([true, true, true, false, false, false])
    );
}
