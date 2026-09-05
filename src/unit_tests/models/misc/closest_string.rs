use super::ClosestString;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use crate::types::Min;

fn issue_instance() -> ClosestString {
    ClosestString::new(
        2,
        vec![vec![0, 0, 0], vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]],
    )
}

#[test]
fn test_closest_string_creation() {
    let problem = issue_instance();
    assert_eq!(problem.alphabet_size(), 2);
    assert_eq!(problem.num_strings(), 4);
    assert_eq!(problem.string_length(), 3);
    assert_eq!(problem.total_length(), 12);
    assert_eq!(problem.dimensions(), vec![2, 2, 2]);
    assert_eq!(problem.num_variables(), 3);
    assert_eq!(<ClosestString as Problem>::NAME, "ClosestString");
    assert_eq!(<ClosestString as Problem>::variant(), vec![]);
}

#[test]
fn test_closest_string_evaluate_at_optimum() {
    let problem = issue_instance();
    // c = 000: d(000,000)=0, d(000,011)=2, d(000,101)=2, d(000,110)=2.
    assert_eq!(problem.evaluate(&vec![0, 0, 0]).unwrap(), Min(Some(2)));
}

#[test]
fn test_closest_string_evaluate_at_100() {
    let problem = issue_instance();
    // c = 100: d(100,000)=1, d(100,011)=3, d(100,101)=1, d(100,110)=1.
    assert_eq!(problem.evaluate(&vec![1, 0, 0]).unwrap(), Min(Some(3)));
}

#[test]
fn test_closest_string_evaluate_at_111() {
    let problem = issue_instance();
    // c = 111: d(111,000)=3, d(111,011)=1, d(111,101)=1, d(111,110)=1.
    assert_eq!(problem.evaluate(&vec![1, 1, 1]).unwrap(), Min(Some(3)));
}

#[test]
fn test_closest_string_evaluate_invalid_length() {
    let problem = issue_instance();
    assert!(matches!(
        problem.evaluate(&vec![0, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![0, 0, 0, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_closest_string_bruteforce_finds_optimum() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    // The minimum achievable radius over all 8 binary length-3 centers is 2.
    assert_eq!(
        problem
            .evaluate(&solver.solve(&problem).unwrap().unwrap())
            .unwrap(),
        Min(Some(2))
    );
    let witness = solver
        .solve(&problem)
        .unwrap()
        .expect("expected a witness for ClosestString");
    assert_eq!(problem.evaluate(&witness).unwrap(), Min(Some(2)));
}

#[test]
#[should_panic(expected = "ClosestString requires at least one input string")]
fn test_closest_string_panics_on_empty_input_list() {
    let _ = ClosestString::new(2, Vec::new());
}

#[test]
#[should_panic(expected = "all input strings must have the same length")]
fn test_closest_string_panics_on_length_mismatch() {
    let _ = ClosestString::new(2, vec![vec![0, 1, 0], vec![1, 0]]);
}

#[test]
#[should_panic(expected = "input symbols must be less than alphabet_size")]
fn test_closest_string_panics_on_out_of_alphabet_symbol() {
    let _ = ClosestString::new(2, vec![vec![0, 1, 2]]);
}

#[test]
fn test_closest_string_larger_alphabet_smoke() {
    // q = 3, length = 2, 3 strings; brute force enumerates 9 centers (<= 27).
    // Inputs (01, 12, 20) are pairwise at Hamming distance 2, so any center
    // must have radius at least 2; e.g., c = 00 achieves d(00,01)=1,
    // d(00,12)=2, d(00,20)=1, giving a max of 2.
    let problem = ClosestString::new(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]]);
    assert_eq!(problem.dimensions(), vec![3, 3]);
    assert_eq!(problem.num_strings(), 3);
    assert_eq!(problem.string_length(), 2);
    let solver = BruteForce::new();
    assert_eq!(
        problem
            .evaluate(&solver.solve(&problem).unwrap().unwrap())
            .unwrap(),
        Min(Some(2))
    );
}

#[test]
fn test_closest_string_serialization() {
    let problem = issue_instance();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: ClosestString = serde_json::from_value(json).unwrap();
    assert_eq!(restored.alphabet_size(), problem.alphabet_size());
    assert_eq!(restored.strings(), problem.strings());
    assert_eq!(restored.dimensions(), problem.dimensions());
    assert_eq!(
        restored.evaluate(&vec![0, 0, 0]).unwrap(),
        problem.evaluate(&vec![0, 0, 0]).unwrap()
    );
}
