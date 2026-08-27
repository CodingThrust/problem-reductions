#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use crate::models::misc::IntegerExpressionMembership;
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

fn issue_example_source_config() -> Vec<bool> {
    vec![false, true, true, false]
}

fn issue_example_target_config() -> Vec<bool> {
    vec![false, true, true, false]
}

#[test]
fn test_subsetsum_to_integerexpressionmembership_closed_loop() {
    let source = issue_example_source();
    let reduction = ReduceTo::<IntegerExpressionMembership>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    // 4 items -> 4 union nodes
    assert_eq!(target.num_union_nodes(), 4);
    // Shifted target: 11 + 4 = 15
    assert_eq!(target.target(), 15);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "SubsetSum -> IntegerExpressionMembership closed loop",
    );
}

#[test]
fn test_subsetsum_to_integerexpressionmembership_extract_solution_matches_choice_bits() {
    let source = issue_example_source();
    let reduction = ReduceTo::<IntegerExpressionMembership>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_eq!(
        reduction
            .extract_solution(&issue_example_target_config())
            .unwrap(),
        issue_example_source_config()
    );
    assert_eq!(
        reduction
            .extract_solution(&vec![true, false, false, true])
            .unwrap(),
        vec![true, false, false, true]
    );
}

#[test]
fn test_subsetsum_to_integerexpressionmembership_unsatisfiable_instance_stays_unsatisfiable() {
    let source = SubsetSum::new(vec![2u32, 4, 6], 5u32);
    let reduction = ReduceTo::<IntegerExpressionMembership>::reduce_to(&source)
        .expect("reduction should succeed");

    assert!(BruteForce::new().solve(&source).unwrap().is_none());
    assert!(BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
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
    assert_eq!(example.target.instance["target"], serde_json::json!(15));
    assert!(!example.solutions.is_empty());
    assert_eq!(
        example.solutions[0].source_config,
        serde_json::json!(issue_example_source_config())
    );
    assert_eq!(
        example.solutions[0].target_config,
        serde_json::json!(issue_example_target_config())
    );

    let source: SubsetSum = serde_json::from_value(example.source.instance.clone())
        .expect("source example deserializes");
    let target: IntegerExpressionMembership =
        serde_json::from_value(example.target.instance.clone())
            .expect("target example deserializes");

    let source_config: Vec<bool> =
        serde_json::from_value(example.solutions[0].source_config.clone()).unwrap();
    let target_config: Vec<bool> =
        serde_json::from_value(example.solutions[0].target_config.clone()).unwrap();
    assert!(source.evaluate(&source_config).unwrap().is_valid());
    assert!(target.evaluate(&target_config).unwrap().is_valid());
}
