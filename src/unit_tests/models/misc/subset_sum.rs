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
fn test_subsetsum_basic() {
    let problem = SubsetSum::new(vec![3u32, 7, 1, 8, 2, 4], 11u32);
    assert_eq!(problem.num_elements(), 6);
    assert_eq!(problem.sizes(), buv(&[3, 7, 1, 8, 2, 4]).as_slice());
    assert_eq!(problem.target(), &bu(11));
    assert_eq!(problem.dimensions(), vec![2; 6]);
    assert_eq!(<SubsetSum as Problem>::NAME, "SubsetSum");
    assert_eq!(<SubsetSum as Problem>::variant(), vec![]);
}

#[test]
fn test_subsetsum_evaluate_satisfying() {
    let problem = SubsetSum::new(vec![3u32, 7, 1, 8, 2, 4], 11u32);
    // {3, 8} = 11
    assert!(problem
        .evaluate(&vec![true, false, false, true, false, false])
        .unwrap());
    // {7, 4} = 11
    assert!(problem
        .evaluate(&vec![false, true, false, false, false, true])
        .unwrap());
}

#[test]
fn test_subsetsum_evaluate_unsatisfying() {
    let problem = SubsetSum::new(vec![3u32, 7, 1, 8, 2, 4], 11u32);
    // {3, 7} = 10 ≠ 11
    assert!(!problem
        .evaluate(&vec![true, true, false, false, false, false])
        .unwrap());
    // empty = 0 ≠ 11
    assert!(!problem
        .evaluate(&vec![false, false, false, false, false, false])
        .unwrap());
    // all = 25 ≠ 11
    assert!(!problem
        .evaluate(&vec![true, true, true, true, true, true])
        .unwrap());
}

#[test]
fn test_subsetsum_evaluate_wrong_config_length() {
    let problem = SubsetSum::new(vec![3u32, 7, 1], 10u32);
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
fn test_subsetsum_evaluate_invalid_variable_value() {
    let problem = SubsetSum::new(vec![3u32, 7], 10u32);
    assert!(
        crate::registry::DynProblem::evaluate_dyn(&problem, &serde_json::json!([2, false]))
            .is_err()
    );
}

#[test]
fn test_subsetsum_empty_instance() {
    // Empty set, target 0: empty subset satisfies
    let problem = SubsetSum::new_unchecked(vec![], bu(0));
    assert_eq!(problem.num_elements(), 0);
    assert_eq!(problem.dimensions(), Vec::<usize>::new());
    assert!(problem.evaluate(&vec![]).unwrap());
}

#[test]
fn test_subsetsum_empty_instance_nonzero_target() {
    // Empty set, target 5: impossible
    let problem = SubsetSum::new_unchecked(vec![], bu(5));
    assert!(!problem.evaluate(&vec![]).unwrap());
}

#[test]
fn test_subsetsum_brute_force() {
    let problem = SubsetSum::new(vec![3u32, 7, 1, 8, 2, 4], 11u32);
    let solver = BruteForce::new();
    let solution = solver
        .solve(&problem)
        .unwrap()
        .expect("should find a solution");
    assert!(problem.evaluate(&solution).unwrap());
}

#[test]
fn test_subsetsum_brute_force_all() {
    let problem = SubsetSum::new(vec![3u32, 7, 1, 8, 2, 4], 11u32);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem).unwrap();
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol).unwrap());
    }
}

#[test]
fn test_subsetsum_unsatisfiable() {
    // Target 100 is unreachable
    let problem = SubsetSum::new(vec![1u32, 2, 3], 100u32);
    let solver = BruteForce::new();
    let solution = solver.solve(&problem).unwrap();
    assert!(solution.is_none());
}

#[test]
fn test_subsetsum_serialization() {
    let problem = SubsetSum::new(vec![3u32, 7, 1, 8, 2, 4], 11u32);
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "sizes": ["3", "7", "1", "8", "2", "4"],
            "target": "11",
        })
    );
    let restored: SubsetSum = serde_json::from_value(json).unwrap();
    assert_eq!(restored.sizes(), problem.sizes());
    assert_eq!(restored.target(), problem.target());
}

#[test]
fn test_subsetsum_deserialization_rejects_numeric_json() {
    let result = serde_json::from_value::<SubsetSum>(serde_json::json!({
        "sizes": [3, 7, 1, 8, 2, 4],
        "target": 11,
    }));
    assert!(result.is_err());
}

#[test]
fn test_subsetsum_single_element() {
    let problem = SubsetSum::new(vec![5u32], 5u32);
    assert!(problem.evaluate(&vec![true]).unwrap());
    assert!(!problem.evaluate(&vec![false]).unwrap());
}

#[test]
fn test_subsetsum_all_selected() {
    // Target equals sum of all elements
    let problem = SubsetSum::new(vec![1u32, 2, 3, 4], 10u32);
    assert!(problem.evaluate(&vec![true, true, true, true]).unwrap()); // 1+2+3+4 = 10
}

#[test]
fn test_subsetsum_target_zero() {
    // Target 0 with non-empty set: only empty subset works
    let problem = SubsetSum::new_unchecked(buv(&[1, 2, 3]), bu(0));
    assert!(problem.evaluate(&vec![false, false, false]).unwrap()); // empty subset sums to 0
    assert!(!problem.evaluate(&vec![true, false, false]).unwrap()); // 1 != 0
}

#[test]
#[should_panic(expected = "positive")]
fn test_subsetsum_negative_sizes_panic() {
    SubsetSum::new(vec![-1i64, 2, 3], 4u32);
}

#[test]
#[should_panic(expected = "positive")]
fn test_subsetsum_zero_size_panic() {
    SubsetSum::new(vec![0i64, 2, 3], 4u32);
}

#[test]
fn test_subsetsum_large_integer_input() {
    let problem = SubsetSum::new(vec![3i128, 7, 1, 8, 2, 4], 11i128);
    assert!(problem
        .evaluate(&vec![true, false, false, true, false, false])
        .unwrap()); // 3 + 8 = 11
    assert!(!problem
        .evaluate(&vec![true, true, false, false, false, false])
        .unwrap()); // 3 + 7 = 10
}
