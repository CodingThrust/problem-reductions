use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use crate::types::Min;
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

#[test]
fn test_minimum_hitting_set_create_spec_uses_subsets_input() {
    assert_eq!(MinimumHittingSetCreateSpec::FIELDS[1].name, "subsets");
    let problem = MinimumHittingSet::try_from(MinimumHittingSetCreateSpec {
        universe_size: 3,
        subsets: vec![vec![0, 2]],
    })
    .unwrap();
    assert_eq!(problem.sets(), &[vec![0, 2]]);
}

fn issue_example_config() -> Vec<bool> {
    vec![false, true, false, true, true, false]
}

#[test]
fn test_minimum_hitting_set_creation_accessors_and_dimensions() {
    let problem = MinimumHittingSet::new(4, vec![vec![2, 1, 1], vec![3]]);

    assert_eq!(problem.universe_size(), 4);
    assert_eq!(problem.num_sets(), 2);
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(problem.dimensions(), vec![2; 4]);
    assert_eq!(problem.sets(), &[vec![1, 2], vec![3]]);
    assert_eq!(problem.get_set(0), Some(&vec![1, 2]));
    assert_eq!(problem.get_set(1), Some(&vec![3]));
    assert_eq!(problem.get_set(2), None);
}

#[test]
fn test_minimum_hitting_set_evaluate_valid_and_invalid() {
    let problem = MinimumHittingSet::new(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]]);

    assert_eq!(
        problem.selected_elements(&[false, true, false, true]),
        Some(vec![1, 3])
    );
    assert_eq!(
        problem.evaluate(&vec![false, true, false, true]).unwrap(),
        Min(Some(2))
    );
    assert_eq!(
        problem.evaluate(&vec![true, false, false, false]).unwrap(),
        Min(None)
    );
    assert!(crate::registry::DynProblem::evaluate_dyn(
        &problem,
        &serde_json::json!([false, 2, false, true])
    )
    .is_err());
    assert!(problem.is_valid_solution(&[false, true, false, true]));
    assert!(!problem.is_valid_solution(&[true, false, false, false]));
}

#[test]
fn test_minimum_hitting_set_empty_set_is_always_invalid() {
    let problem = MinimumHittingSet::new(3, vec![vec![0, 1], vec![]]);

    assert_eq!(
        problem.evaluate(&vec![true, true, true]).unwrap(),
        Min(None)
    );
    assert_eq!(
        problem.evaluate(&vec![false, false, false]).unwrap(),
        Min(None)
    );
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

    let best = solver.solve(&problem).unwrap().unwrap();
    assert_eq!(problem.evaluate(&best).unwrap(), Min(Some(3)));

    let best_solutions = solver.find_all_witnesses(&problem).unwrap();
    let best_solution_set: HashSet<Vec<bool>> = best_solutions.iter().cloned().collect();
    assert!(best_solution_set.contains(&issue_example_config()));
    assert!(best_solutions
        .iter()
        .all(|config| problem.evaluate(config).unwrap() == Min(Some(3))));
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
        deserialized
            .evaluate(&vec![true, true, false, false])
            .unwrap(),
        problem.evaluate(&vec![true, true, false, false]).unwrap()
    );
}

#[test]
fn test_minimum_hitting_set_paper_example_consistency() {
    let problem = issue_example_problem();

    assert_eq!(
        problem.evaluate(&issue_example_config()).unwrap(),
        Min(Some(3))
    );
}

#[test]
fn test_minimum_hitting_set_declares_problem_parameters() {
    let fields: HashSet<&'static str> = MinimumHittingSet::parameter_names()
        .iter()
        .copied()
        .collect();
    assert_eq!(fields, HashSet::from(["num_sets", "universe_size"]),);
}

#[cfg(feature = "example-db")]
#[test]
fn test_minimum_hitting_set_canonical_example_spec() {
    let specs = canonical_model_example_specs();
    assert_eq!(specs.len(), 1);
    let spec = &specs[0];

    assert_eq!(spec.id, "minimum_hitting_set");
    assert_eq!(
        spec.optimal_config,
        serde_json::json!(issue_example_config())
    );
    assert_eq!(spec.optimal_value, serde_json::json!(3));

    let problem: MinimumHittingSet =
        serde_json::from_value(spec.instance.serialize_json()).unwrap();
    assert_eq!(problem.universe_size(), 6);
    assert_eq!(problem.sets().len(), 7);

    let solver = BruteForce::new();
    let best = solver.solve(&problem).unwrap().unwrap();
    assert_eq!(problem.evaluate(&best).unwrap(), Min(Some(3)));
}
