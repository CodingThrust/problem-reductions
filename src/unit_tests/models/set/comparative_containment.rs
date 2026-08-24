use super::*;

#[test]
fn create_spec_defaults_weights_and_validates_sets() {
    let problem = ComparativeContainment::<i64>::try_from(ComparativeContainmentI64CreateSpec {
        universe_size: 2,
        r_sets: vec![vec![0]],
        s_sets: vec![vec![1]],
        r_weights: None,
        s_weights: None,
    })
    .unwrap();
    assert_eq!(problem.r_weights(), &[1]);
    assert!(
        ComparativeContainment::<i64>::try_from(ComparativeContainmentI64CreateSpec {
            universe_size: 1,
            r_sets: vec![vec![1]],
            s_sets: vec![],
            r_weights: None,
            s_weights: None
        })
        .is_err()
    );
}
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::One;

fn yes_instance() -> ComparativeContainment<i64> {
    ComparativeContainment::with_weights(
        4,
        vec![vec![0, 1, 2, 3], vec![0, 1]],
        vec![vec![0, 1, 2, 3], vec![2, 3]],
        vec![2, 5],
        vec![3, 6],
    )
    .unwrap()
}

fn no_instance() -> ComparativeContainment<i64> {
    ComparativeContainment::with_weights(
        2,
        vec![vec![0], vec![1]],
        vec![vec![0, 1]],
        vec![1, 1],
        vec![3],
    )
    .unwrap()
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
        ComparativeContainment::<One>::new(3, vec![vec![0, 1], vec![1, 2]], vec![vec![0]]).unwrap();
    assert_eq!(problem.r_weights(), &[One, One]);
    assert_eq!(problem.s_weights(), &[One]);
}

#[test]
fn test_comparative_containment_evaluation_yes_and_no_examples() {
    let yes = yes_instance();
    assert!(yes.evaluate(&[1, 0, 0, 0]).unwrap());
    assert!(!yes.evaluate(&[0, 0, 1, 0]).unwrap());
    assert!(!yes.evaluate(&[0, 0, 0, 0]).unwrap());

    let no = no_instance();
    assert!(!no.evaluate(&[0, 0]).unwrap());
    assert!(!no.evaluate(&[1, 0]).unwrap());
    assert!(!no.evaluate(&[0, 1]).unwrap());
    assert!(!no.evaluate(&[1, 1]).unwrap());
}

#[test]
fn test_comparative_containment_rejects_invalid_configs() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[1, 0, 0]).unwrap());
    assert!(!problem.evaluate(&[1, 0, 0, 2]).unwrap());
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

    let yes_solutions = solver.find_all_witnesses(&yes_instance()).unwrap();
    assert!(yes_solutions.contains(&vec![1, 0, 0, 0]));
    assert!(!yes_solutions.is_empty());

    let no_solutions = solver.find_all_witnesses(&no_instance()).unwrap();
    assert!(no_solutions.is_empty());
}

#[test]
fn test_comparative_containment_serialization() {
    let problem = yes_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let restored: ComparativeContainment<i64> = serde_json::from_str(&json).unwrap();
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
    assert!(problem.evaluate(&config).unwrap());

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem).unwrap();
    assert_eq!(solutions.len(), 3);
    assert!(solutions.contains(&config));
}

#[test]
fn test_comparative_containment_weight_sums() {
    let problem = yes_instance();
    // Y = {0}: R1={0,1,2,3} contains {0} (w=2), R2={0,1} contains {0} (w=5) → 7
    assert_eq!(problem.r_weight_sum(&[1, 0, 0, 0]).unwrap(), Some(7));
    // Y = {0}: S1={0,1,2,3} contains {0} (w=3), S2={2,3} does not → 3
    assert_eq!(problem.s_weight_sum(&[1, 0, 0, 0]).unwrap(), Some(3));
    // Invalid config returns None
    assert_eq!(problem.r_weight_sum(&[1, 0, 0]).unwrap(), None);
    assert_eq!(problem.s_weight_sum(&[1, 0, 0, 2]).unwrap(), None);
}

#[test]
fn test_comparative_containment_rejects_mismatched_r_weights() {
    assert!(ComparativeContainment::with_weights(
        2,
        vec![vec![0]],
        vec![vec![0]],
        vec![1, 2],
        vec![1]
    )
    .is_err());
}

#[test]
fn test_comparative_containment_rejects_mismatched_s_weights() {
    assert!(ComparativeContainment::with_weights(
        2,
        vec![vec![0]],
        vec![vec![0]],
        vec![1],
        vec![1, 2]
    )
    .is_err());
}

#[test]
fn test_comparative_containment_rejects_nonpositive_i64_weights() {
    assert!(ComparativeContainment::with_weights(
        2,
        vec![vec![0]],
        vec![vec![0]],
        vec![0],
        vec![1]
    )
    .is_err());
}

#[test]
fn test_comparative_containment_rejects_nonpositive_i64_s_weights() {
    assert!(ComparativeContainment::with_weights(
        2,
        vec![vec![0]],
        vec![vec![0]],
        vec![1],
        vec![0]
    )
    .is_err());
}

#[test]
fn test_comparative_containment_rejects_non_finite_f64_weights() {
    assert!(ComparativeContainment::with_weights(
        2,
        vec![vec![0]],
        vec![vec![0]],
        vec![f64::NAN],
        vec![1.0],
    )
    .is_err());
}

#[test]
fn test_comparative_containment_rejects_nonpositive_f64_weights() {
    assert!(ComparativeContainment::with_weights(
        2,
        vec![vec![0]],
        vec![vec![0]],
        vec![1.0],
        vec![0.0]
    )
    .is_err());
}

#[test]
fn test_comparative_containment_rejects_out_of_range_elements() {
    assert!(ComparativeContainment::<i64>::new(2, vec![vec![0, 2]], vec![vec![0]]).is_err());
}

#[test]
fn test_comparative_containment_deserialization_validates_fields() {
    let json = r#"{
        "universe_size": 1,
        "r_sets": [[1]],
        "s_sets": [],
        "r_weights": [1.0],
        "s_weights": []
    }"#;
    assert!(serde_json::from_str::<ComparativeContainment<f64>>(json).is_err());
}
