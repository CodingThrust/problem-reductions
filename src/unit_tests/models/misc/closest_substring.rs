use super::ClosestSubstring;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use crate::types::Min;

fn issue_instance() -> ClosestSubstring {
    // The #1033 canonical example: q = 2, ell = 3, three length-5 binary strings.
    ClosestSubstring::new(
        2,
        vec![
            vec![0, 0, 0, 1, 1],
            vec![1, 0, 1, 0, 0],
            vec![1, 1, 0, 0, 1],
        ],
        3,
    )
    .unwrap()
}

#[test]
fn test_closest_substring_creation() {
    let problem = issue_instance();
    assert_eq!(problem.alphabet_size(), 2);
    assert_eq!(problem.num_strings(), 3);
    assert_eq!(problem.substring_length(), 3);
    assert_eq!(problem.total_length(), 15);
    assert_eq!(problem.total_num_windows(), 9);
    assert_eq!(problem.num_window_choice_product(), 27);
    // dims: 3 center slots (each of size 2) + one window-position slot per
    // string (each of size W_i = 5 - 3 + 1 = 3).
    assert_eq!(problem.dimensions(), vec![2, 2, 2, 3, 3, 3]);
    assert_eq!(problem.num_variables(), 6);
    assert_eq!(<ClosestSubstring as Problem>::NAME, "ClosestSubstring");
    assert_eq!(<ClosestSubstring as Problem>::variant(), vec![]);
}

#[test]
fn test_closest_substring_evaluate_at_optimum() {
    let problem = issue_instance();
    // Center [0,1,0] with window picks (0, 1, 0):
    //   s_1[0..3] = [0,0,0], d_H([0,1,0], [0,0,0]) = 1
    //   s_2[1..4] = [0,1,0], d_H = 0
    //   s_3[0..3] = [1,1,0], d_H = 1
    // max = 1.
    assert_eq!(
        problem.evaluate(&vec![0, 1, 0, 0, 1, 0]).unwrap(),
        Min(Some(1))
    );
}

#[test]
fn test_closest_substring_evaluate_all_zero_windows() {
    let problem = issue_instance();
    // c = [0,0,0], windows (0, 0, 0):
    //   s_1[0..3] = [0,0,0]  d = 0
    //   s_2[0..3] = [1,0,1]  d = 2
    //   s_3[0..3] = [1,1,0]  d = 2
    // max = 2.
    assert_eq!(
        problem.evaluate(&vec![0, 0, 0, 0, 0, 0]).unwrap(),
        Min(Some(2))
    );
}

#[test]
fn test_closest_substring_evaluate_at_111_center() {
    let problem = issue_instance();
    // Any center [1,1,1] has Hamming distance >= 1 to every length-3 binary
    // string that contains at least one 0. All windows of s_1, s_2, s_3
    // contain at least one zero, so the radius is at least 1.
    let value = problem.evaluate(&vec![1, 1, 1, 0, 0, 0]).unwrap();
    if let Min(Some(d)) = value {
        assert!(d >= 1, "expected radius >= 1, got {d}");
    } else {
        panic!("expected feasible value, got {value:?}");
    }
}

#[test]
fn test_closest_substring_evaluate_invalid_length() {
    let problem = issue_instance();
    assert!(matches!(
        problem.evaluate(&vec![0, 0, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![0, 0, 0, 0, 0, 0, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_closest_substring_bruteforce_finds_optimum() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    // 8 centers * 27 window combinations = 216 configurations; optimum is 1.
    assert_eq!(
        problem
            .evaluate(&solver.solve(&problem).unwrap().unwrap())
            .unwrap(),
        Min(Some(1))
    );
    let witness = solver
        .solve(&problem)
        .unwrap()
        .expect("expected a witness for ClosestSubstring");
    assert_eq!(problem.evaluate(&witness).unwrap(), Min(Some(1)));
}

#[test]
fn test_closest_substring_specializes_to_closest_string() {
    // When substring_length == string_length, each input string has exactly
    // one window (W_i = 1) and the problem reduces to ClosestString on the
    // same instance. Use the #1032 canonical (4 binary strings of length 3),
    // whose optimum radius is 2.
    let problem = ClosestSubstring::new(
        2,
        vec![vec![0, 0, 0], vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]],
        3,
    )
    .unwrap();
    assert_eq!(problem.num_window_choice_product(), 1);
    assert_eq!(problem.dimensions(), vec![2, 2, 2, 1, 1, 1, 1]);
    let solver = BruteForce::new();
    assert_eq!(
        problem
            .evaluate(&solver.solve(&problem).unwrap().unwrap())
            .unwrap(),
        Min(Some(2))
    );
}

#[test]
fn test_closest_substring_rejects_empty_input_list() {
    assert!(matches!(
        ClosestSubstring::new(2, Vec::new(), 3).unwrap_err(),
        crate::registry::ConstructionError::Conversion(message)
            if message == "ClosestSubstring requires at least one input string"
    ));
}

#[test]
fn test_closest_substring_rejects_substring_too_long() {
    // s_2 has length 2 < substring_length 3.
    assert!(matches!(
        ClosestSubstring::new(2, vec![vec![0, 0, 0], vec![1, 1]], 3).unwrap_err(),
        crate::registry::ConstructionError::Conversion(message)
            if message == "substring_length must be <= |s_i| for every input string"
    ));
}

#[test]
fn test_closest_substring_rejects_out_of_alphabet_symbol() {
    assert!(matches!(
        ClosestSubstring::new(2, vec![vec![0, 1, 2]], 3).unwrap_err(),
        crate::registry::ConstructionError::Conversion(message)
            if message == "input symbols must be less than alphabet_size"
    ));
}

#[test]
fn test_closest_substring_serialization() {
    let problem = issue_instance();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: ClosestSubstring = serde_json::from_value(json).unwrap();
    assert_eq!(restored.alphabet_size(), problem.alphabet_size());
    assert_eq!(restored.strings(), problem.strings());
    assert_eq!(restored.substring_length(), problem.substring_length());
    assert_eq!(restored.dimensions(), problem.dimensions());
    assert_eq!(
        restored.evaluate(&vec![0, 1, 0, 0, 1, 0]).unwrap(),
        problem.evaluate(&vec![0, 1, 0, 0, 1, 0]).unwrap()
    );
}
