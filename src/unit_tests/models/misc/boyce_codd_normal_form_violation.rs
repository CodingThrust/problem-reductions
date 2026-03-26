use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

/// Build the canonical example: 6 attributes, 3 FDs, full target subset.
fn canonical_problem() -> BoyceCoddNormalFormViolation {
    BoyceCoddNormalFormViolation::new(
        6,
        vec![
            (vec![0, 1], vec![2]),
            (vec![2], vec![3]),
            (vec![3, 4], vec![5]),
        ],
        vec![0, 1, 2, 3, 4, 5],
    )
}

#[test]
fn test_bcnf_creation() {
    let problem = canonical_problem();
    assert_eq!(problem.num_attributes(), 6);
    assert_eq!(problem.num_functional_deps(), 3);
    assert_eq!(problem.num_target_attributes(), 6);
    assert_eq!(problem.num_variables(), 6);
    assert_eq!(problem.dims(), vec![2; 6]);
    assert_eq!(problem.target_subset(), &[0, 1, 2, 3, 4, 5]);
    assert_eq!(problem.functional_deps().len(), 3);
}

#[test]
fn test_bcnf_evaluate_violation() {
    let problem = canonical_problem();
    // X = {2}: closure = {2, 3}. In A' \ X = {0,1,3,4,5}: 3 ∈ closure, 0 ∉ closure → violation.
    assert!(problem.evaluate(&[0, 0, 1, 0, 0, 0]));
}

#[test]
fn test_bcnf_evaluate_no_violation_empty_x() {
    let problem = canonical_problem();
    // X = {} (all zeros): A' \ X = all attributes, closure of {} = {}.
    // Nothing in closure → no violation.
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0, 0]));
}

#[test]
fn test_bcnf_evaluate_no_violation_x_covers_all() {
    let problem = canonical_problem();
    // X = all attributes: A' \ X = {} → no attributes to test → no violation.
    assert!(!problem.evaluate(&[1, 1, 1, 1, 1, 1]));
}

#[test]
fn test_bcnf_evaluate_invalid_config_length() {
    let problem = canonical_problem();
    assert!(!problem.evaluate(&[0, 0, 1, 0, 0])); // too short
    assert!(!problem.evaluate(&[0, 0, 1, 0, 0, 0, 0])); // too long
}

#[test]
fn test_bcnf_evaluate_invalid_config_values() {
    let problem = canonical_problem();
    assert!(!problem.evaluate(&[0, 0, 2, 0, 0, 0])); // value > 1
}

#[test]
fn test_bcnf_solver_finds_violation() {
    let problem = canonical_problem();
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    // All returned solutions must evaluate to true.
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
    // The canonical witness must be among them.
    assert!(solutions.contains(&vec![0, 0, 1, 0, 0, 0]));
}

#[test]
fn test_bcnf_no_violation_when_fds_trivial() {
    // Only trivial FD: {0} → {0}. No non-trivial closure possible.
    let problem = BoyceCoddNormalFormViolation::new(3, vec![(vec![0], vec![0])], vec![0, 1, 2]);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(solutions.is_empty());
}

#[test]
fn test_bcnf_partial_target_subset() {
    // Only test a subset of attributes.
    // FD: {0} → {1}; target = {0, 1}.
    // X = {0}: closure = {0, 1}. A' \ X = {1}. 1 ∈ closure but nothing is outside → no violation.
    let problem = BoyceCoddNormalFormViolation::new(3, vec![(vec![0], vec![1])], vec![0, 1]);
    assert!(!problem.evaluate(&[1, 0])); // X={0}: all of A'\X = {1} ⊆ closure → no violation
    assert!(!problem.evaluate(&[0, 0])); // X={}: closure={}, nothing in closure → no violation
}

#[test]
fn test_bcnf_violation_with_three_attrs_in_target() {
    // Attrs 0,1,2. FD: {0} → {1}. Target = {0, 1, 2}.
    // X = {0}: closure = {0, 1}. A' \ X = {1, 2}. 1 ∈ closure, 2 ∉ closure → BCNF violation.
    let problem = BoyceCoddNormalFormViolation::new(3, vec![(vec![0], vec![1])], vec![0, 1, 2]);
    assert!(problem.evaluate(&[1, 0, 0])); // X = {0}
    assert!(!problem.evaluate(&[0, 1, 0])); // X = {1}: A'\X = {0,2}, closure of {1} = {1}, 0∉closure, 2∉closure → no violation
}

#[test]
fn test_bcnf_serialization() {
    let problem = canonical_problem();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: BoyceCoddNormalFormViolation = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.num_attributes(), problem.num_attributes());
    assert_eq!(
        deserialized.num_functional_deps(),
        problem.num_functional_deps()
    );
    assert_eq!(deserialized.target_subset(), problem.target_subset());
    assert_eq!(deserialized.functional_deps(), problem.functional_deps());
}

#[test]
#[should_panic(expected = "target_subset must be non-empty")]
fn test_bcnf_rejects_empty_target_subset() {
    BoyceCoddNormalFormViolation::new(3, vec![], vec![]);
}

#[test]
#[should_panic(expected = "empty LHS")]
fn test_bcnf_rejects_empty_lhs_fd() {
    BoyceCoddNormalFormViolation::new(3, vec![(vec![], vec![1])], vec![0, 1]);
}

#[test]
#[should_panic(expected = "out of range")]
fn test_bcnf_rejects_out_of_range_fd_attr() {
    BoyceCoddNormalFormViolation::new(3, vec![(vec![0], vec![5])], vec![0, 1]);
}

#[test]
#[should_panic(expected = "out of range")]
fn test_bcnf_rejects_out_of_range_target_attr() {
    BoyceCoddNormalFormViolation::new(3, vec![], vec![0, 5]);
}

#[test]
fn test_bcnf_deduplicates_fd_attrs() {
    // LHS with duplicates should be deduped without panic.
    let problem =
        BoyceCoddNormalFormViolation::new(3, vec![(vec![0, 0], vec![1, 1])], vec![0, 1, 2]);
    assert_eq!(problem.functional_deps()[0].0, vec![0]);
    assert_eq!(problem.functional_deps()[0].1, vec![1]);
}

#[test]
fn test_bcnf_deduplicates_target_subset() {
    let problem = BoyceCoddNormalFormViolation::new(3, vec![(vec![0], vec![1])], vec![0, 1, 0, 2]);
    assert_eq!(problem.target_subset(), &[0, 1, 2]);
    assert_eq!(problem.num_target_attributes(), 3);
}

#[test]
fn test_bcnf_fds_outside_target_subset() {
    // FDs reference attributes outside A'. X={0} triggers {0}→{3}→{4} but 3,4 ∉ A'.
    // A' \ X = {1, 2}: neither 1 nor 2 is in closure → no violation.
    let problem = BoyceCoddNormalFormViolation::new(
        5,
        vec![(vec![0], vec![3]), (vec![3], vec![4])],
        vec![0, 1, 2],
    );
    assert!(!problem.evaluate(&[1, 0, 0])); // X={0}: closure reaches {0,3,4} but A'\X={1,2} untouched
}

#[test]
fn test_bcnf_cyclic_keys_no_violation() {
    // Issue example: 4 attributes, cyclic keys — every non-trivial subset is a superkey.
    // FDs: {0,1}→{2,3}, {2,3}→{0,1}, {0,2}→{1,3}, {1,3}→{0,2}.
    // All 2-element subsets containing a key pair have full closure → no BCNF violation.
    let problem = BoyceCoddNormalFormViolation::new(
        4,
        vec![
            (vec![0, 1], vec![2, 3]),
            (vec![2, 3], vec![0, 1]),
            (vec![0, 2], vec![1, 3]),
            (vec![1, 3], vec![0, 2]),
        ],
        vec![0, 1, 2, 3],
    );
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(
        solutions.is_empty(),
        "Cyclic-key instance should have no BCNF violation"
    );
}

#[test]
fn test_bcnf_multi_step_transitive_closure() {
    // X={0,1}: {0,1}→{2} then {2}→{3} (two-step chain).
    // A' \ X = {2,3,4,5}. closure = {0,1,2,3}. 2∈closure, 4∉closure → violation.
    let problem = canonical_problem();
    assert!(problem.evaluate(&[1, 1, 0, 0, 0, 0]));
}
