use crate::models::misc::Betweenness;
use crate::models::set::SetSplitting;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;

fn small_yes_instance() -> SetSplitting {
    SetSplitting::new(3, vec![vec![0, 1, 2]])
}

fn issue_yes_instance() -> SetSplitting {
    SetSplitting::new(
        5,
        vec![vec![0, 1, 2], vec![2, 3, 4], vec![0, 3, 4], vec![1, 2, 3]],
    )
}

fn issue_no_instance() -> SetSplitting {
    SetSplitting::new(3, vec![vec![0, 1], vec![1, 2], vec![0, 2], vec![0, 1, 2]])
}

#[test]
fn test_setsplitting_to_betweenness_closed_loop() {
    let source = small_yes_instance();
    let reduction = ReduceTo::<Betweenness>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "SetSplitting -> Betweenness",
    );
}

#[test]
fn test_setsplitting_to_betweenness_issue_yes_instance_structure() {
    let source = issue_yes_instance();
    let reduction = ReduceTo::<Betweenness>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_elements(), 10);
    assert_eq!(
        target.triples(),
        &[
            (0, 6, 1),
            (6, 5, 2),
            (2, 7, 3),
            (7, 5, 4),
            (0, 8, 3),
            (8, 5, 4),
            (1, 9, 2),
            (9, 5, 3),
        ],
    );
    assert_eq!(
        reduction.extract_solution(&[8, 2, 9, 0, 1, 4, 3, 6, 7, 5]),
        vec![1, 0, 1, 0, 0]
    );
}

#[test]
fn test_setsplitting_to_betweenness_normalizes_large_subsets() {
    let source = SetSplitting::new(4, vec![vec![0, 1, 2, 3]]);
    let reduction = ReduceTo::<Betweenness>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_elements(), 9);
    assert_eq!(
        target.triples(),
        &[(0, 7, 1), (7, 6, 4), (4, 6, 5), (5, 8, 2), (8, 6, 3),],
    );
}

#[test]
fn test_setsplitting_to_betweenness_issue_no_instance_is_unsat() {
    let source = issue_no_instance();
    let reduction = ReduceTo::<Betweenness>::reduce_to(&source);

    assert!(BruteForce::new()
        .find_witness(reduction.target_problem())
        .is_none());
}

#[cfg(feature = "example-db")]
#[test]
fn test_setsplitting_to_betweenness_canonical_example_spec() {
    let specs = crate::rules::setsplitting_betweenness::canonical_rule_example_specs();
    assert_eq!(specs.len(), 1);

    let example = (specs[0].build)();
    assert_eq!(example.source.problem, "SetSplitting");
    assert_eq!(example.target.problem, "Betweenness");
    assert_eq!(example.solutions.len(), 1);

    let pair = &example.solutions[0];
    assert_eq!(pair.source_config, vec![1, 0, 1, 0, 0]);
    assert_eq!(pair.target_config, vec![8, 2, 9, 0, 1, 4, 3, 6, 7, 5]);
}
