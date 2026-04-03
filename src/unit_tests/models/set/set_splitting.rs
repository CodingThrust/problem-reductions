use crate::models::set::SetSplitting;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;
use std::collections::HashSet;

fn issue_example_problem() -> SetSplitting {
    SetSplitting::new(
        6,
        vec![vec![0, 1, 2], vec![2, 3, 4], vec![0, 4, 5], vec![1, 3, 5]],
    )
}

fn issue_example_config() -> Vec<usize> {
    vec![0, 1, 0, 1, 1, 0]
}

#[test]
fn test_set_splitting_creation_accessors_and_dimensions() {
    let problem = SetSplitting::new(4, vec![vec![2, 1, 1], vec![3], vec![]]);

    assert_eq!(problem.universe_size(), 4);
    assert_eq!(problem.num_subsets(), 3);
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(problem.dims(), vec![2; 4]);
    assert_eq!(problem.subsets(), &[vec![1, 2], vec![3], vec![]]);
}

#[test]
fn test_set_splitting_evaluate_split_monochromatic_and_invalid_configs() {
    let problem = SetSplitting::new(4, vec![vec![0, 1, 2], vec![1, 3], vec![2, 3]]);

    assert_eq!(problem.evaluate(&[0, 1, 1, 0]), Or(true));
    assert_eq!(problem.evaluate(&[1, 1, 0, 0]), Or(false));
    assert_eq!(problem.evaluate(&[0, 2, 0, 1]), Or(false));
    assert_eq!(problem.evaluate(&[0, 1, 0]), Or(false));
}

#[test]
fn test_set_splitting_empty_or_singleton_subset_is_unsplittable() {
    let problem = SetSplitting::new(3, vec![vec![], vec![1]]);

    assert_eq!(problem.evaluate(&[0, 1, 0]), Or(false));
    assert_eq!(problem.evaluate(&[1, 0, 1]), Or(false));
}

#[test]
fn test_set_splitting_bruteforce_issue_example() {
    let problem = issue_example_problem();
    let solver = BruteForce::new();

    let solutions = solver.find_all_witnesses(&problem);
    let set: HashSet<Vec<usize>> = solutions.into_iter().collect();

    assert_eq!(set.len(), 18);
    assert!(set.contains(&issue_example_config()));
    assert!(set
        .iter()
        .all(|config| problem.evaluate(config) == Or(true)));
}

#[test]
fn test_set_splitting_serialization_round_trip() {
    let problem = SetSplitting::new(4, vec![vec![0, 1], vec![1, 2, 2], vec![3]]);
    let json = serde_json::to_string(&problem).unwrap();
    let round_trip: SetSplitting = serde_json::from_str(&json).unwrap();

    assert_eq!(round_trip.universe_size(), problem.universe_size());
    assert_eq!(round_trip.num_subsets(), problem.num_subsets());
    assert_eq!(round_trip.subsets(), problem.subsets());
    assert_eq!(
        round_trip.evaluate(&[0, 1, 0, 1]),
        problem.evaluate(&[0, 1, 0, 1])
    );
}

#[test]
fn test_set_splitting_paper_example_consistency() {
    let problem = issue_example_problem();

    assert_eq!(problem.evaluate(&issue_example_config()), Or(true));
}

#[cfg(feature = "example-db")]
#[test]
fn test_set_splitting_canonical_example_spec() {
    let specs = crate::models::set::set_splitting::canonical_model_example_specs();
    assert_eq!(specs.len(), 1);

    let spec = &specs[0];
    assert_eq!(spec.id, "set_splitting");
    assert_eq!(spec.optimal_config, issue_example_config());
    assert_eq!(spec.optimal_value, serde_json::json!(true));

    let problem: SetSplitting = serde_json::from_value(spec.instance.serialize_json()).unwrap();
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);

    assert_eq!(problem.universe_size(), 6);
    assert_eq!(problem.num_subsets(), 4);
    assert_eq!(solutions.len(), 18);
    assert!(solutions.contains(&issue_example_config()));
}
