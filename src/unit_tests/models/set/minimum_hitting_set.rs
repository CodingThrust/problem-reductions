use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::SolutionSize;
use std::collections::HashSet;

fn issue_example_problem() -> MinimumHittingSet {
    MinimumHittingSet::new(
        6,
        vec![
            vec![0, 1, 2],
            vec![0, 3, 4],
            vec![1, 3, 5],
            vec![2, 4, 5],
            vec![0, 1, 5],
            vec![2, 3],
            vec![1, 4],
        ],
    )
}

fn issue_example_config() -> Vec<usize> {
    vec![0, 1, 0, 1, 1, 0]
}

#[test]
fn test_minimum_hitting_set_creation_accessors_and_dimensions() {
    let problem = MinimumHittingSet::new(4, vec![vec![2, 1, 1], vec![3]]);

    assert_eq!(problem.universe_size(), 4);
    assert_eq!(problem.num_sets(), 2);
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(problem.dims(), vec![2; 4]);
    assert_eq!(problem.sets(), &[vec![1, 2], vec![3]]);
    assert_eq!(problem.get_set(0), Some(&vec![1, 2]));
    assert_eq!(problem.get_set(1), Some(&vec![3]));
    assert_eq!(problem.get_set(2), None);
}

#[test]
fn test_minimum_hitting_set_evaluate_valid_and_invalid() {
    let problem = MinimumHittingSet::new(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]]);

    assert_eq!(problem.selected_elements(&[0, 1, 0, 1]), Some(vec![1, 3]));
    assert_eq!(problem.selected_elements(&[0, 2, 0, 1]), None);
    assert_eq!(problem.evaluate(&[0, 1, 0, 1]), SolutionSize::Valid(2));
    assert_eq!(problem.evaluate(&[1, 0, 0, 0]), SolutionSize::Invalid);
    assert_eq!(problem.evaluate(&[0, 2, 0, 1]), SolutionSize::Invalid);
    assert!(problem.is_valid_solution(&[0, 1, 0, 1]));
    assert!(!problem.is_valid_solution(&[1, 0, 0, 0]));
    assert!(!problem.is_valid_solution(&[0, 2, 0, 1]));
}

#[test]
fn test_minimum_hitting_set_empty_set_is_always_invalid() {
    let problem = MinimumHittingSet::new(3, vec![vec![0, 1], vec![]]);

    assert_eq!(problem.evaluate(&[1, 1, 1]), SolutionSize::Invalid);
    assert_eq!(problem.evaluate(&[0, 0, 0]), SolutionSize::Invalid);
}

#[test]
fn test_minimum_hitting_set_constructor_normalizes_sets() {
    let problem = MinimumHittingSet::new(5, vec![vec![3, 1, 3, 2], vec![4, 0, 0], vec![]]);

    assert_eq!(problem.sets(), &[vec![1, 2, 3], vec![0, 4], vec![]]);
}

#[test]
#[should_panic(expected = "outside universe")]
fn test_minimum_hitting_set_rejects_out_of_range_elements() {
    MinimumHittingSet::new(3, vec![vec![0, 3]]);
}

#[test]
fn test_minimum_hitting_set_bruteforce_optimum_issue_example() {
    let problem = issue_example_problem();
    let solver = BruteForce::new();

    let best = solver.find_best(&problem).unwrap();
    assert_eq!(problem.evaluate(&best), SolutionSize::Valid(3));

    let best_solutions = solver.find_all_best(&problem);
    let best_solution_set: HashSet<Vec<usize>> = best_solutions.iter().cloned().collect();
    assert!(best_solution_set.contains(&issue_example_config()));
    assert!(
        best_solutions
            .iter()
            .all(|config| problem.evaluate(config) == SolutionSize::Valid(3))
    );
}

#[test]
fn test_minimum_hitting_set_serialization_round_trip() {
    let problem = MinimumHittingSet::new(4, vec![vec![2, 1, 1], vec![3, 0]]);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinimumHittingSet = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.universe_size(), problem.universe_size());
    assert_eq!(deserialized.num_sets(), problem.num_sets());
    assert_eq!(deserialized.sets(), problem.sets());
    assert_eq!(
        deserialized.evaluate(&[1, 1, 0, 0]),
        problem.evaluate(&[1, 1, 0, 0])
    );
}

#[test]
fn test_minimum_hitting_set_paper_example_consistency() {
    let problem = issue_example_problem();

    assert_eq!(problem.evaluate(&issue_example_config()), SolutionSize::Valid(3));
}

#[cfg(feature = "example-db")]
#[test]
fn test_minimum_hitting_set_canonical_example_spec() {
    let specs = canonical_model_example_specs();
    assert_eq!(specs.len(), 1);
    let spec = &specs[0];

    assert_eq!(spec.id, "minimum_hitting_set");
    assert_eq!(spec.optimal_config, issue_example_config());
    assert_eq!(spec.optimal_value, serde_json::json!({"Valid": 3}));

    let problem: MinimumHittingSet = serde_json::from_value(spec.instance.serialize_json()).unwrap();
    assert_eq!(problem.universe_size(), 6);
    assert_eq!(problem.sets().len(), 7);

    let solver = BruteForce::new();
    let best = solver.find_best(&problem).unwrap();
    assert_eq!(problem.evaluate(&best), SolutionSize::Valid(3));
}
