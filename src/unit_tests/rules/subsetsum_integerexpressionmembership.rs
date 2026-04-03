#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use crate::models::algebraic::IntegerExpressionMembership;
use crate::models::misc::SubsetSum;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::traits::ReductionResult;
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
#[cfg(feature = "example-db")]
use crate::traits::Problem;

fn issue_example_source() -> SubsetSum {
    SubsetSum::new(vec![1u32, 5, 6, 8], 11u32)
}

fn issue_example_source_config() -> Vec<usize> {
    vec![0, 1, 1, 0]
}

fn issue_example_target_config() -> Vec<usize> {
    vec![0, 1, 1, 0]
}

#[test]
fn test_subsetsum_to_integerexpressionmembership_closed_loop() {
    let source = issue_example_source();
    let reduction = ReduceTo::<IntegerExpressionMembership>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(
        target.choices(),
        &[vec![1, 2], vec![1, 6], vec![1, 7], vec![1, 9]]
    );
    assert_eq!(target.target(), 15);
    assert_eq!(target.num_positions(), source.num_elements());

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "SubsetSum -> IntegerExpressionMembership closed loop",
    );
}

#[test]
fn test_subsetsum_to_integerexpressionmembership_extract_solution_matches_choice_bits() {
    let source = issue_example_source();
    let reduction = ReduceTo::<IntegerExpressionMembership>::reduce_to(&source);

    assert_eq!(
        reduction.extract_solution(&issue_example_target_config()),
        issue_example_source_config()
    );
    assert_eq!(reduction.extract_solution(&[1, 0, 0, 1]), vec![1, 0, 0, 1]);
}

#[test]
fn test_subsetsum_to_integerexpressionmembership_unsatisfiable_instance_stays_unsatisfiable() {
    let source = SubsetSum::new(vec![2u32, 4, 6], 5u32);
    let reduction = ReduceTo::<IntegerExpressionMembership>::reduce_to(&source);

    assert!(BruteForce::new().find_witness(&source).is_none());
    assert!(BruteForce::new()
        .find_witness(reduction.target_problem())
        .is_none());
}

#[cfg(feature = "example-db")]
#[test]
fn test_subsetsum_to_integerexpressionmembership_canonical_example_spec() {
    let example = (canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "subsetsum_to_integerexpressionmembership")
        .expect("missing canonical SubsetSum -> IntegerExpressionMembership example spec")
        .build)();

    assert_eq!(example.source.problem, "SubsetSum");
    assert_eq!(example.target.problem, "IntegerExpressionMembership");
    assert_eq!(
        example.target.instance["choices"],
        serde_json::json!([[1, 2], [1, 6], [1, 7], [1, 9]])
    );
    assert_eq!(example.target.instance["target"], serde_json::json!(15));
    assert_eq!(example.solutions.len(), 1);
    assert_eq!(
        example.solutions[0].source_config,
        issue_example_source_config()
    );
    assert_eq!(
        example.solutions[0].target_config,
        issue_example_target_config()
    );

    let source: SubsetSum = serde_json::from_value(example.source.instance.clone())
        .expect("source example deserializes");
    let target: IntegerExpressionMembership =
        serde_json::from_value(example.target.instance.clone())
            .expect("target example deserializes");

    assert!(source
        .evaluate(&example.solutions[0].source_config)
        .is_valid());
    assert!(target
        .evaluate(&example.solutions[0].target_config)
        .is_valid());
}
