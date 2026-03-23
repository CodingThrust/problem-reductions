use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn issue_yes_instance() -> GroupingBySwapping {
    GroupingBySwapping::new(3, vec![0, 1, 2, 0, 1, 2], 5)
}

fn issue_minimum_three_swaps_instance() -> GroupingBySwapping {
    GroupingBySwapping::new(3, vec![0, 1, 2, 0, 1, 2], 3)
}

fn issue_two_swap_instance() -> GroupingBySwapping {
    GroupingBySwapping::new(3, vec![0, 1, 2, 0, 1, 2], 2)
}

#[test]
fn test_grouping_by_swapping_basic() {
    let problem = issue_yes_instance();
    assert_eq!(problem.alphabet_size(), 3);
    assert_eq!(problem.string(), &[0, 1, 2, 0, 1, 2]);
    assert_eq!(problem.budget(), 5);
    assert_eq!(problem.string_len(), 6);
    assert_eq!(problem.num_variables(), 5);
    assert_eq!(problem.dims(), vec![6; 5]);
    assert_eq!(<GroupingBySwapping as Problem>::NAME, "GroupingBySwapping");
    assert_eq!(<GroupingBySwapping as Problem>::variant(), vec![]);

    let _: crate::models::misc::GroupingBySwapping = problem.clone();
    let _: crate::models::GroupingBySwapping = problem.clone();
    let _: crate::prelude::GroupingBySwapping = problem;
}

#[test]
fn test_grouping_by_swapping_evaluate_issue_yes() {
    let problem = issue_yes_instance();
    assert!(problem.evaluate(&[2, 1, 3, 5, 5]));
    assert_eq!(
        problem.apply_swap_program(&[2, 1, 3, 5, 5]),
        Some(vec![0, 0, 1, 1, 2, 2])
    );
    assert!(!problem.evaluate(&[0, 1, 2, 3, 4]));
    assert!(!problem.is_grouped(&[0, 1, 0]));
    assert!(problem.is_grouped(&[0, 0, 1, 1, 2, 2]));
}

#[test]
fn test_grouping_by_swapping_rejects_wrong_length_and_out_of_range_swaps() {
    let problem = issue_yes_instance();
    assert!(!problem.evaluate(&[2, 1, 3, 5]));
    assert!(!problem.evaluate(&[2, 1, 3, 5, 6]));
    assert_eq!(problem.apply_swap_program(&[2, 1, 3, 5]), None);
    assert_eq!(problem.apply_swap_program(&[2, 1, 3, 5, 6]), None);
}

#[test]
fn test_grouping_by_swapping_bruteforce_yes_and_no() {
    let yes_problem = issue_minimum_three_swaps_instance();
    let no_problem = issue_two_swap_instance();
    let solver = BruteForce::new();

    let satisfying = solver
        .find_witness(&yes_problem)
        .expect("expected a satisfying 3-swap sequence");
    assert!(yes_problem.evaluate(&satisfying));
    assert!(solver
        .find_all_witnesses(&yes_problem)
        .iter()
        .any(|config| config == &vec![2, 1, 3]));

    assert!(solver.find_witness(&no_problem).is_none());
    assert!(solver.find_all_witnesses(&no_problem).is_empty());
}

#[test]
fn test_grouping_by_swapping_paper_example() {
    let problem = issue_yes_instance();
    assert!(problem.evaluate(&[2, 1, 3, 5, 5]));

    let solver = BruteForce::new();
    assert!(solver
        .find_all_witnesses(&problem)
        .iter()
        .any(|config| config == &vec![2, 1, 3, 5, 5]));
    assert!(solver.find_witness(&issue_two_swap_instance()).is_none());
}

#[test]
fn test_grouping_by_swapping_serialization() {
    let problem = issue_yes_instance();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: GroupingBySwapping = serde_json::from_value(json).unwrap();
    assert_eq!(restored.alphabet_size(), 3);
    assert_eq!(restored.string(), &[0, 1, 2, 0, 1, 2]);
    assert_eq!(restored.budget(), 5);
}

#[test]
#[should_panic(expected = "input symbols must be less than alphabet_size")]
fn test_grouping_by_swapping_symbol_out_of_range_panics() {
    GroupingBySwapping::new(3, vec![0, 1, 3], 1);
}

#[test]
#[should_panic(expected = "budget must be 0 when string is empty")]
fn test_grouping_by_swapping_empty_string_requires_zero_budget() {
    GroupingBySwapping::new(0, vec![], 1);
}
