use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use num_bigint::BigUint;

fn bu(n: u32) -> BigUint {
    BigUint::from(n)
}

fn buv(values: &[u32]) -> Vec<BigUint> {
    values.iter().copied().map(BigUint::from).collect()
}

#[test]
fn test_subsetproduct_basic() {
    let problem = SubsetProduct::new(vec![2u32, 3, 5, 7, 6, 10], 210u32);
    assert_eq!(problem.num_elements(), 6);
    assert_eq!(problem.sizes(), buv(&[2, 3, 5, 7, 6, 10]).as_slice());
    assert_eq!(problem.target(), &bu(210));
    assert_eq!(problem.dimensions(), vec![2; 6]);
    assert_eq!(<SubsetProduct as Problem>::NAME, "SubsetProduct");
    assert_eq!(<SubsetProduct as Problem>::variant(), vec![]);
}

#[test]
fn test_subsetproduct_evaluate_satisfying() {
    let problem = SubsetProduct::new(vec![2u32, 3, 5, 7, 6, 10], 210u32);
    // {2, 3, 5, 7} = 210
    assert!(problem
        .evaluate(&vec![true, true, true, true, false, false])
        .unwrap());
    // {3, 7, 10} = 210
    assert!(problem
        .evaluate(&vec![false, true, false, true, false, true])
        .unwrap());
}

#[test]
fn test_subsetproduct_evaluate_unsatisfying() {
    let problem = SubsetProduct::new(vec![2u32, 3, 5, 7, 6, 10], 210u32);
    // {2, 3} = 6 != 210
    assert!(!problem
        .evaluate(&vec![true, true, false, false, false, false])
        .unwrap());
    // empty = 1 != 210
    assert!(!problem
        .evaluate(&vec![false, false, false, false, false, false])
        .unwrap());
    // all = 2*3*5*7*6*10 = 12600 != 210
    assert!(!problem
        .evaluate(&vec![true, true, true, true, true, true])
        .unwrap());
}

#[test]
fn test_subsetproduct_evaluate_wrong_config_length() {
    let problem = SubsetProduct::new(vec![2u32, 3, 5], 30u32);
    assert!(matches!(
        problem.evaluate(&vec![true, false]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![true, false, false, false]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_subsetproduct_evaluate_invalid_variable_value() {
    let problem = SubsetProduct::new(vec![2u32, 3], 6u32);
    assert!(
        crate::registry::DynProblem::evaluate_dyn(&problem, &serde_json::json!([2, false]))
            .is_err()
    );
}

#[test]
fn test_subsetproduct_empty_instance() {
    // Empty set, target 1: empty subset product = 1 satisfies
    let problem = SubsetProduct::new_unchecked(vec![], bu(1));
    assert_eq!(problem.num_elements(), 0);
    assert_eq!(problem.dimensions(), Vec::<usize>::new());
    assert!(problem.evaluate(&vec![]).unwrap());
}

#[test]
fn test_subsetproduct_empty_instance_nonunit_target() {
    // Empty set, target 5: impossible (empty product = 1)
    let problem = SubsetProduct::new_unchecked(vec![], bu(5));
    assert!(!problem.evaluate(&vec![]).unwrap());
}

#[test]
fn test_subsetproduct_brute_force() {
    let problem = SubsetProduct::new(vec![2u32, 3, 5, 7, 6, 10], 210u32);
    let solver = BruteForce::new();
    let solution = solver
        .solve(&problem)
        .unwrap()
        .expect("should find a solution");
    assert!(problem.evaluate(&solution).unwrap());
}

#[test]
fn test_subsetproduct_brute_force_all() {
    let problem = SubsetProduct::new(vec![2u32, 3, 5, 7, 6, 10], 210u32);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem).unwrap();
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol).unwrap());
    }
}

#[test]
fn test_subsetproduct_unsatisfiable() {
    // Target 1000 is unreachable with these sizes
    let problem = SubsetProduct::new(vec![2u32, 3, 5], 1000u32);
    let solver = BruteForce::new();
    let solution = solver.solve(&problem).unwrap();
    assert!(solution.is_none());
}

#[test]
fn test_subsetproduct_serialization() {
    let problem = SubsetProduct::new(vec![2u32, 3, 5, 7, 6, 10], 210u32);
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "sizes": ["2", "3", "5", "7", "6", "10"],
            "target": "210",
        })
    );
    let restored: SubsetProduct = serde_json::from_value(json).unwrap();
    assert_eq!(restored.sizes(), problem.sizes());
    assert_eq!(restored.target(), problem.target());
}

#[test]
fn test_subsetproduct_deserialization_rejects_numeric_json() {
    let result = serde_json::from_value::<SubsetProduct>(serde_json::json!({
        "sizes": [2, 3, 5, 7, 6, 10],
        "target": 210,
    }));
    assert!(result.is_err());
}

#[test]
fn test_subsetproduct_single_element() {
    let problem = SubsetProduct::new(vec![5u32], 5u32);
    assert!(problem.evaluate(&vec![true]).unwrap());
    assert!(!problem.evaluate(&vec![false]).unwrap());
}

#[test]
fn test_subsetproduct_all_selected() {
    // Target equals product of all elements
    let problem = SubsetProduct::new(vec![2u32, 3, 5], 30u32);
    assert!(problem.evaluate(&vec![true, true, true]).unwrap()); // 2*3*5 = 30
}

#[test]
fn test_subsetproduct_target_one() {
    // Target 1 with non-empty set: only empty subset works (product = 1)
    let problem = SubsetProduct::new(vec![2u32, 3, 5], 1u32);
    assert!(problem.evaluate(&vec![false, false, false]).unwrap()); // empty subset product = 1
    assert!(!problem.evaluate(&vec![true, false, false]).unwrap()); // 2 != 1
}

#[test]
#[should_panic(expected = "positive")]
fn test_subsetproduct_negative_sizes_panic() {
    SubsetProduct::new(vec![-1i64, 2, 3], 4u32);
}

#[test]
#[should_panic(expected = "positive")]
fn test_subsetproduct_zero_size_panic() {
    SubsetProduct::new(vec![0i64, 2, 3], 4u32);
}

#[test]
#[should_panic(expected = "positive")]
fn test_subsetproduct_zero_target_panic() {
    SubsetProduct::new(vec![2u32, 3], 0u32);
}

#[test]
fn test_subsetproduct_large_integer_input() {
    let problem = SubsetProduct::new(vec![2i128, 3, 5, 7, 6, 10], 210i128);
    assert!(problem
        .evaluate(&vec![true, true, true, true, false, false])
        .unwrap()); // 2*3*5*7 = 210
    assert!(!problem
        .evaluate(&vec![true, true, false, false, false, false])
        .unwrap()); // 2*3 = 6
}
