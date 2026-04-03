use super::*;
use crate::registry::declared_size_fields;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use std::collections::HashSet;

fn issue_example_problem() -> IntegerExpressionMembership {
    IntegerExpressionMembership::new(vec![vec![1, 2], vec![1, 6], vec![1, 7], vec![1, 9]], 15)
}

fn issue_example_config() -> Vec<usize> {
    vec![0, 1, 1, 0]
}

#[test]
fn test_integer_expression_membership_creation_accessors_and_dimensions() {
    let problem = issue_example_problem();

    assert_eq!(
        problem.choices(),
        &[vec![1, 2], vec![1, 6], vec![1, 7], vec![1, 9]]
    );
    assert_eq!(problem.target(), 15);
    assert_eq!(problem.num_positions(), 4);
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2]);
    assert_eq!(
        <IntegerExpressionMembership as Problem>::NAME,
        "IntegerExpressionMembership"
    );
    assert_eq!(
        <IntegerExpressionMembership as Problem>::variant(),
        Vec::<(&'static str, &'static str)>::new()
    );
}

#[test]
fn test_integer_expression_membership_evaluate_valid_and_invalid_configs() {
    let problem = issue_example_problem();

    assert!(problem.evaluate(&issue_example_config()));
    assert!(!problem.evaluate(&[1, 0, 1, 0]));
    assert!(!problem.evaluate(&[0, 1, 1]));
    assert!(!problem.evaluate(&[0, 1, 1, 2]));
}

#[test]
fn test_integer_expression_membership_multiway_choices() {
    let problem = IntegerExpressionMembership::new(vec![vec![0, 2, 5], vec![1, 4]], 9);

    assert_eq!(problem.dims(), vec![3, 2]);
    assert!(problem.evaluate(&[2, 1]));
    assert!(!problem.evaluate(&[1, 1]));
}

#[test]
fn test_integer_expression_membership_bruteforce_issue_example() {
    let problem = issue_example_problem();
    let solver = BruteForce::new();

    let best = solver
        .find_witness(&problem)
        .expect("should find a witness");
    assert_eq!(best, issue_example_config());
    assert!(problem.evaluate(&best));
}

#[test]
fn test_integer_expression_membership_serialization_round_trip() {
    let problem = issue_example_problem();
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "choices": [[1, 2], [1, 6], [1, 7], [1, 9]],
            "target": 15,
        })
    );

    let restored: IntegerExpressionMembership = serde_json::from_value(json).unwrap();
    assert_eq!(restored.choices(), problem.choices());
    assert_eq!(restored.target(), problem.target());
}

#[test]
fn test_integer_expression_membership_declares_problem_size_fields() {
    let fields: HashSet<&'static str> = declared_size_fields("IntegerExpressionMembership")
        .into_iter()
        .collect();
    assert_eq!(fields, HashSet::from(["num_positions"]));
}

#[test]
fn test_integer_expression_membership_paper_example() {
    let problem = issue_example_problem();

    assert!(problem.evaluate(&issue_example_config()));
    assert_eq!(
        BruteForce::new().find_all_witnesses(&problem),
        vec![issue_example_config()]
    );
}

#[cfg(feature = "example-db")]
#[test]
fn test_integer_expression_membership_canonical_example_spec() {
    let specs = canonical_model_example_specs();
    assert_eq!(specs.len(), 1);
    let spec = &specs[0];

    assert_eq!(spec.id, "integer_expression_membership");
    assert_eq!(spec.optimal_config, issue_example_config());
    assert_eq!(spec.optimal_value, serde_json::json!(true));

    let problem: IntegerExpressionMembership =
        serde_json::from_value(spec.instance.serialize_json()).unwrap();
    assert_eq!(problem.choices(), issue_example_problem().choices());
    assert_eq!(problem.target(), issue_example_problem().target());
    assert!(problem.evaluate(&issue_example_config()));
}
