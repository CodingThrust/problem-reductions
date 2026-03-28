use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

/// Instance 1: 6 attributes, cyclic FDs, 3 known keys.
fn instance1() -> AdditionalKey {
    AdditionalKey::new(
        6,
        vec![
            (vec![0, 1], vec![2, 3]),
            (vec![2, 3], vec![4, 5]),
            (vec![4, 5], vec![0, 1]),
            (vec![0, 2], vec![3]),
            (vec![3, 5], vec![1]),
        ],
        vec![0, 1, 2, 3, 4, 5],
        vec![vec![0, 1], vec![2, 3], vec![4, 5]],
    )
}

/// Instance 2: 3 attributes, single FD {0}->{1,2}, known key [{0}].
fn instance2() -> AdditionalKey {
    AdditionalKey::new(3, vec![(vec![0], vec![1, 2])], vec![0, 1, 2], vec![vec![0]])
}

#[test]
fn test_additional_key_creation() {
    let problem = instance1();
    assert_eq!(problem.num_attributes(), 6);
    assert_eq!(problem.num_dependencies(), 5);
    assert_eq!(problem.num_relation_attrs(), 6);
    assert_eq!(problem.num_known_keys(), 3);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2, 2, 2]);
    assert_eq!(<AdditionalKey as Problem>::NAME, "AdditionalKey");
    assert_eq!(<AdditionalKey as Problem>::variant(), vec![]);
    // Data getters
    assert_eq!(problem.dependencies().len(), 5);
    assert_eq!(problem.dependencies()[0], (vec![0, 1], vec![2, 3]));
    assert_eq!(problem.relation_attrs(), &[0, 1, 2, 3, 4, 5]);
    assert_eq!(problem.known_keys().len(), 3);
    assert_eq!(problem.known_keys()[0], vec![0, 1]);
}

#[test]
fn test_additional_key_evaluate_satisfying() {
    let problem = instance1();
    // Config [1,0,1,0,0,0] selects attrs {0,2}.
    // Closure of {0,2}: {0,2} -> apply (0,2)->3 => {0,2,3} -> apply (2,3)->(4,5) => {0,2,3,4,5}
    //   -> apply (4,5)->(0,1) => {0,1,2,3,4,5}. Covers all.
    // Minimality: remove 0 => {2}, closure of {2} = {2} => does not cover all. OK.
    //             remove 2 => {0}, closure of {0} = {0} => does not cover all. OK.
    // {0,2} sorted is [0,2], not in known_keys [{0,1},{2,3},{4,5}].
    assert!(problem.evaluate(&[1, 0, 1, 0, 0, 0]));
}

#[test]
fn test_additional_key_evaluate_known_key() {
    let problem = instance1();
    // Config [1,1,0,0,0,0] selects attrs {0,1} which IS in known_keys.
    assert!(!problem.evaluate(&[1, 1, 0, 0, 0, 0]));
}

#[test]
fn test_additional_key_evaluate_not_a_key() {
    let problem = instance1();
    // Config [0,0,0,0,0,1] selects {5}. Closure of {5} = {5}.
    // Does not cover all attrs.
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0, 1]));
}

#[test]
fn test_additional_key_evaluate_non_minimal() {
    let problem = instance1();
    // Config [1,1,1,0,0,0] selects {0,1,2}.
    // {0,1} alone determines all attrs (known key), so {0,1,2} is NOT minimal.
    assert!(!problem.evaluate(&[1, 1, 1, 0, 0, 0]));
}

#[test]
fn test_additional_key_no_additional_key() {
    let problem = instance2();
    // Only candidate key is {0}, which is already known.
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_none());
}

#[test]
fn test_additional_key_wrong_config_length() {
    let problem = instance1();
    assert!(!problem.evaluate(&[1, 0]));
    assert!(!problem.evaluate(&[1, 0, 0, 0, 0, 0, 0]));
}

#[test]
fn test_additional_key_invalid_variable_value() {
    let problem = instance1();
    assert!(!problem.evaluate(&[2, 0, 0, 0, 0, 0]));
}

#[test]
fn test_additional_key_brute_force() {
    let problem = instance1();
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_additional_key_brute_force_all() {
    let problem = instance1();
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    // Exactly 2 additional keys: {0,2} and {0,3,5}
    assert_eq!(solutions.len(), 2);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_additional_key_serialization() {
    let problem = instance1();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: AdditionalKey = serde_json::from_value(json.clone()).unwrap();
    assert_eq!(restored.num_attributes(), problem.num_attributes());
    assert_eq!(restored.num_dependencies(), problem.num_dependencies());
    assert_eq!(restored.num_relation_attrs(), problem.num_relation_attrs());
    assert_eq!(restored.num_known_keys(), problem.num_known_keys());
    // Verify round-trip produces same evaluation
    assert_eq!(
        problem.evaluate(&[1, 0, 1, 0, 0, 0]),
        restored.evaluate(&[1, 0, 1, 0, 0, 0])
    );
}

#[test]
fn test_additional_key_empty_selection() {
    let problem = instance1();
    // All zeros = no attributes selected = not a key
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0, 0]));
}

#[test]
#[should_panic(expected = "relation_attrs element")]
fn test_additional_key_panic_relation_attrs_out_of_bounds() {
    AdditionalKey::new(3, vec![], vec![0, 1, 5], vec![]);
}

#[test]
#[should_panic(expected = "relation_attrs contains duplicates")]
fn test_additional_key_panic_relation_attrs_duplicates() {
    AdditionalKey::new(3, vec![], vec![0, 1, 1], vec![]);
}

#[test]
#[should_panic(expected = "dependency lhs attribute")]
fn test_additional_key_panic_dependency_lhs_out_of_bounds() {
    AdditionalKey::new(3, vec![(vec![5], vec![0])], vec![0, 1, 2], vec![]);
}

#[test]
#[should_panic(expected = "dependency rhs attribute")]
fn test_additional_key_panic_dependency_rhs_out_of_bounds() {
    AdditionalKey::new(3, vec![(vec![0], vec![5])], vec![0, 1, 2], vec![]);
}

#[test]
#[should_panic(expected = "known_keys attribute")]
fn test_additional_key_panic_known_keys_out_of_bounds() {
    AdditionalKey::new(3, vec![], vec![0, 1, 2], vec![vec![5]]);
}
