use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;

/// Helper: build the canonical 6-element, K=2 instance from the issue.
fn issue_instance() -> ThreeMatroidIntersection {
    ThreeMatroidIntersection::new(
        6,
        vec![
            vec![vec![0, 1, 2], vec![3, 4, 5]],       // M1
            vec![vec![0, 3], vec![1, 4], vec![2, 5]], // M2
            vec![vec![0, 4], vec![1, 5], vec![2, 3]], // M3
        ],
        2,
    )
}

#[test]
fn test_three_matroid_intersection_creation() {
    let problem = issue_instance();
    assert_eq!(problem.ground_set_size(), 6);
    assert_eq!(problem.bound(), 2);
    assert_eq!(problem.partitions().len(), 3);
    assert_eq!(problem.num_groups(), 8); // 2 + 3 + 3
    assert_eq!(problem.num_variables(), 6);
    assert_eq!(problem.dimensions(), vec![2; 6]);
}

#[test]
fn test_three_matroid_intersection_evaluate_valid() {
    let problem = issue_instance();
    // {0, 5} is a valid common independent set of size 2
    // M1: 0 in {0,1,2}, 5 in {3,4,5} -> at most 1 per group
    // M2: 0 in {0,3}, 5 in {2,5} -> at most 1 per group
    // M3: 0 in {0,4}, 5 in {1,5} -> at most 1 per group
    assert!(problem
        .evaluate(&vec![true, false, false, false, false, true])
        .unwrap());

    // {1, 3} is also valid
    assert!(problem
        .evaluate(&vec![false, true, false, true, false, false])
        .unwrap());

    // {2, 4} is also valid
    assert!(problem
        .evaluate(&vec![false, false, true, false, true, false])
        .unwrap());
}

#[test]
fn test_three_matroid_intersection_evaluate_invalid() {
    let problem = issue_instance();

    // {0, 3} fails M2: both in group {0, 3}
    assert!(!problem
        .evaluate(&vec![true, false, false, true, false, false])
        .unwrap());

    // {0, 4} fails M3: both in group {0, 4}
    assert!(!problem
        .evaluate(&vec![true, false, false, false, true, false])
        .unwrap());

    // {1, 2} fails M1: both in group {0, 1, 2}
    assert!(!problem
        .evaluate(&vec![false, true, true, false, false, false])
        .unwrap());

    // Wrong size: only 1 element selected
    assert!(!problem
        .evaluate(&vec![true, false, false, false, false, false])
        .unwrap());

    // Wrong size: 3 elements selected
    assert!(!problem
        .evaluate(&vec![true, false, false, false, true, true])
        .unwrap());

    // All zeros
    assert!(!problem
        .evaluate(&vec![false, false, false, false, false, false])
        .unwrap());
}

#[test]
fn test_three_matroid_intersection_solver() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem).unwrap();

    // Exactly 3 valid solutions: {0,5}, {1,3}, {2,4}
    assert_eq!(solutions.len(), 3);
    for sol in &solutions {
        assert!(problem.evaluate(sol).unwrap());
    }
    assert!(solutions.contains(&vec![true, false, false, false, false, true]));
    assert!(solutions.contains(&vec![false, true, false, true, false, false]));
    assert!(solutions.contains(&vec![false, false, true, false, true, false]));
}

#[test]
fn test_three_matroid_intersection_no_solution() {
    // Same instance but K=3: M1 has only 2 groups, so independent sets have size ≤ 2
    let problem = ThreeMatroidIntersection::new(
        6,
        vec![
            vec![vec![0, 1, 2], vec![3, 4, 5]],
            vec![vec![0, 3], vec![1, 4], vec![2, 5]],
            vec![vec![0, 4], vec![1, 5], vec![2, 3]],
        ],
        3,
    );
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem).unwrap();
    assert!(solutions.is_empty());
}

#[test]
fn test_three_matroid_intersection_serialization() {
    let problem = issue_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: ThreeMatroidIntersection = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.ground_set_size(), problem.ground_set_size());
    assert_eq!(deserialized.bound(), problem.bound());
    assert_eq!(deserialized.partitions(), problem.partitions());
}

#[test]
fn test_three_matroid_intersection_rejects_wrong_config_length() {
    let problem = issue_instance();
    assert!(matches!(
        problem.evaluate(&vec![true, false, false]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_three_matroid_intersection_rejects_non_binary_config() {
    let problem = issue_instance();
    assert!(crate::registry::DynProblem::evaluate_dyn(
        &problem,
        &serde_json::json!([2, false, false, false, false, false])
    )
    .is_err());
}

#[test]
#[should_panic(expected = "Expected exactly 3")]
fn test_three_matroid_intersection_wrong_matroid_count() {
    ThreeMatroidIntersection::new(4, vec![vec![vec![0, 1]], vec![vec![2, 3]]], 1);
}

#[test]
#[should_panic(expected = "outside 0..")]
fn test_three_matroid_intersection_element_out_of_range() {
    ThreeMatroidIntersection::new(
        3,
        vec![
            vec![vec![0, 1, 2]],
            vec![vec![0, 1, 2]],
            vec![vec![0, 1, 5]], // 5 >= 3
        ],
        1,
    );
}

#[test]
#[should_panic(expected = "Bound 4 exceeds")]
fn test_three_matroid_intersection_bound_exceeds_ground_set() {
    ThreeMatroidIntersection::new(
        3,
        vec![
            vec![vec![0, 1, 2]],
            vec![vec![0, 1, 2]],
            vec![vec![0, 1, 2]],
        ],
        4,
    );
}

#[test]
fn test_three_matroid_intersection_paper_example() {
    // Issue's canonical 6-element example, K=2
    let problem = issue_instance();

    // Valid: {0, 5}
    assert!(problem
        .evaluate(&vec![true, false, false, false, false, true])
        .unwrap());

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem).unwrap();
    // Exactly 3 valid common independent sets of size 2
    assert_eq!(solutions.len(), 3);
    assert!(solutions.contains(&vec![true, false, false, false, false, true])); // {0, 5}
    assert!(solutions.contains(&vec![false, true, false, true, false, false])); // {1, 3}
    assert!(solutions.contains(&vec![false, false, true, false, true, false])); // {2, 4}

    // Negative modification from issue: K=3 is infeasible (M1 has only 2 groups)
    let problem_k3 = ThreeMatroidIntersection::new(
        6,
        vec![
            vec![vec![0, 1, 2], vec![3, 4, 5]],
            vec![vec![0, 3], vec![1, 4], vec![2, 5]],
            vec![vec![0, 4], vec![1, 5], vec![2, 3]],
        ],
        3,
    );
    let solutions_k3 = solver.find_all_witnesses(&problem_k3).unwrap();
    assert!(solutions_k3.is_empty());
}
