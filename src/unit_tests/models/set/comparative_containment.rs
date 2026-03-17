use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::One;

fn yes_instance() -> ComparativeContainment<i32> {
    ComparativeContainment::with_weights(
        4,
        vec![vec![0, 1, 2, 3], vec![0, 1]],
        vec![vec![0, 1, 2, 3], vec![2, 3]],
        vec![2, 5],
        vec![3, 6],
    )
}

fn no_instance() -> ComparativeContainment<i32> {
    ComparativeContainment::with_weights(
        2,
        vec![vec![0], vec![1]],
        vec![vec![0, 1]],
        vec![1, 1],
        vec![3],
    )
}

#[test]
fn test_comparative_containment_creation() {
    let problem = yes_instance();
    assert_eq!(problem.universe_size(), 4);
    assert_eq!(problem.num_r_sets(), 2);
    assert_eq!(problem.num_s_sets(), 2);
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2]);
}

#[test]
fn test_comparative_containment_unit_weights() {
    let problem =
        ComparativeContainment::<One>::new(3, vec![vec![0, 1], vec![1, 2]], vec![vec![0]]);
    assert_eq!(problem.r_weights(), &[One, One]);
    assert_eq!(problem.s_weights(), &[One]);
}

#[test]
fn test_comparative_containment_evaluation_yes_and_no_examples() {
    let yes = yes_instance();
    assert!(yes.evaluate(&[1, 0, 0, 0]));
    assert!(!yes.evaluate(&[0, 0, 1, 0]));
    assert!(!yes.evaluate(&[0, 0, 0, 0]));

    let no = no_instance();
    assert!(!no.evaluate(&[0, 0]));
    assert!(!no.evaluate(&[1, 0]));
    assert!(!no.evaluate(&[0, 1]));
    assert!(!no.evaluate(&[1, 1]));
}

#[test]
fn test_comparative_containment_rejects_invalid_configs() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[1, 0, 0]));
    assert!(!problem.evaluate(&[1, 0, 0, 2]));
}

#[test]
fn test_comparative_containment_contains_selected_subset_requires_valid_config() {
    let problem = yes_instance();
    assert!(problem.contains_selected_subset(&[1, 0, 0, 0], &[0, 1, 2, 3]));
    assert!(!problem.contains_selected_subset(&[0, 0, 1, 0], &[0, 1]));
    assert!(!problem.contains_selected_subset(&[1, 0, 0], &[0, 1, 2, 3]));
    assert!(!problem.contains_selected_subset(&[1, 0, 0, 2], &[0, 1, 2, 3]));
}

#[test]
fn test_comparative_containment_solver() {
    let solver = BruteForce::new();

    let yes_solutions = solver.find_all_satisfying(&yes_instance());
    assert!(yes_solutions.contains(&vec![1, 0, 0, 0]));
    assert!(!yes_solutions.is_empty());

    let no_solutions = solver.find_all_satisfying(&no_instance());
    assert!(no_solutions.is_empty());
}

#[test]
fn test_comparative_containment_serialization() {
    let problem = yes_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let restored: ComparativeContainment<i32> = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.universe_size(), problem.universe_size());
    assert_eq!(restored.r_sets(), problem.r_sets());
    assert_eq!(restored.s_sets(), problem.s_sets());
    assert_eq!(restored.r_weights(), problem.r_weights());
    assert_eq!(restored.s_weights(), problem.s_weights());
}

#[test]
fn test_comparative_containment_paper_example() {
    let problem = yes_instance();
    let config = vec![1, 0, 0, 0];
    assert!(problem.evaluate(&config));

    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert_eq!(solutions.len(), 3);
    assert!(solutions.contains(&config));
}

#[test]
fn test_comparative_containment_weight_sums() {
    let problem = yes_instance();
    // Y = {0}: R1={0,1,2,3} contains {0} (w=2), R2={0,1} contains {0} (w=5) → 7
    assert_eq!(problem.r_weight_sum(&[1, 0, 0, 0]), Some(7));
    // Y = {0}: S1={0,1,2,3} contains {0} (w=3), S2={2,3} does not → 3
    assert_eq!(problem.s_weight_sum(&[1, 0, 0, 0]), Some(3));
    // Invalid config returns None
    assert_eq!(problem.r_weight_sum(&[1, 0, 0]), None);
    assert_eq!(problem.s_weight_sum(&[1, 0, 0, 2]), None);
}

#[test]
#[should_panic(expected = "number of R sets and R weights must match")]
fn test_comparative_containment_rejects_mismatched_r_weights() {
    ComparativeContainment::with_weights(2, vec![vec![0]], vec![vec![0]], vec![1, 2], vec![1]);
}

#[test]
#[should_panic(expected = "number of S sets and S weights must match")]
fn test_comparative_containment_rejects_mismatched_s_weights() {
    ComparativeContainment::with_weights(2, vec![vec![0]], vec![vec![0]], vec![1], vec![1, 2]);
}

#[test]
#[should_panic(expected = "R weights must be finite and positive")]
fn test_comparative_containment_rejects_nonpositive_i32_weights() {
    ComparativeContainment::with_weights(2, vec![vec![0]], vec![vec![0]], vec![0], vec![1]);
}

#[test]
#[should_panic(expected = "S weights must be finite and positive")]
fn test_comparative_containment_rejects_nonpositive_i32_s_weights() {
    ComparativeContainment::with_weights(2, vec![vec![0]], vec![vec![0]], vec![1], vec![0]);
}

#[test]
#[should_panic(expected = "R weights must be finite and positive")]
fn test_comparative_containment_rejects_non_finite_f64_weights() {
    ComparativeContainment::with_weights(
        2,
        vec![vec![0]],
        vec![vec![0]],
        vec![f64::NAN],
        vec![1.0],
    );
}

#[test]
#[should_panic(expected = "S weights must be finite and positive")]
fn test_comparative_containment_rejects_nonpositive_f64_weights() {
    ComparativeContainment::with_weights(2, vec![vec![0]], vec![vec![0]], vec![1.0], vec![0.0]);
}

#[test]
#[should_panic(expected = "contains element")]
fn test_comparative_containment_rejects_out_of_range_elements() {
    ComparativeContainment::<i32>::new(2, vec![vec![0, 2]], vec![vec![0]]);
}
