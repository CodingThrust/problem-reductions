use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_string_to_string_correction_creation() {
    let problem = StringToStringCorrection::new(4, vec![0, 1, 2, 3, 1, 0], vec![0, 1, 3, 2, 1], 2);
    assert_eq!(problem.alphabet_size(), 4);
    assert_eq!(problem.source(), &[0, 1, 2, 3, 1, 0]);
    assert_eq!(problem.target(), &[0, 1, 3, 2, 1]);
    assert_eq!(problem.bound_k(), 2);
    assert_eq!(problem.source_length(), 6);
    assert_eq!(problem.target_length(), 5);
    // domain = 2*6+1 = 13, bound_k = 2
    assert_eq!(problem.dims(), vec![13; 2]);
    assert_eq!(
        <StringToStringCorrection as Problem>::NAME,
        "StringToStringCorrection"
    );
    assert_eq!(<StringToStringCorrection as Problem>::variant(), vec![]);
}

#[test]
fn test_string_to_string_correction_evaluation() {
    let problem = StringToStringCorrection::new(4, vec![0, 1, 2, 3, 1, 0], vec![0, 1, 3, 2, 1], 2);
    // Known solution: swap positions 2&3 (value=8), then delete index 5 (value=5)
    // Step 1: current_len=6, op=8 >= 6, swap_pos = 8-6=2, swap(2,3) → [0,1,3,2,1,0]
    // Step 2: current_len=6, op=5 < 6, delete(5) → [0,1,3,2,1] = target
    assert!(problem.evaluate(&[8, 5]));
    // All no-ops should not produce target (source != target)
    assert!(!problem.evaluate(&[12, 12]));
}

#[test]
fn test_string_to_string_correction_invalid_operations() {
    let problem = StringToStringCorrection::new(4, vec![0, 1, 2, 3, 1, 0], vec![0, 1, 3, 2, 1], 2);
    // out-of-domain values
    assert!(!problem.evaluate(&[13, 5]));
    assert!(!problem.evaluate(&[8, 13]));
    // wrong length config
    assert!(!problem.evaluate(&[8]));
    assert!(!problem.evaluate(&[8, 5, 12]));
}

#[test]
fn test_string_to_string_correction_invalid_after_deletion() {
    // After a deletion, some swap indices become invalid
    let problem = StringToStringCorrection::new(2, vec![0, 1, 0], vec![1], 2);
    // source len = 3, domain = 7, noop = 6
    // op=0: delete index 0 → [1, 0], current_len=2
    // op=5: 5 >= 2, swap_pos = 5-2=3, need 3+1<2 → false → invalid
    assert!(!problem.evaluate(&[0, 5]));
}

#[test]
fn test_string_to_string_correction_serialization() {
    let problem = StringToStringCorrection::new(4, vec![0, 1, 2, 3, 1, 0], vec![0, 1, 3, 2, 1], 2);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: StringToStringCorrection = serde_json::from_value(json).unwrap();
    assert_eq!(restored.alphabet_size(), problem.alphabet_size());
    assert_eq!(restored.source(), problem.source());
    assert_eq!(restored.target(), problem.target());
    assert_eq!(restored.bound_k(), problem.bound_k());
}

#[test]
fn test_string_to_string_correction_solver() {
    // Small instance: source [0,1], target [1,0], bound 1
    // Need a single swap: swap_pos=0, value = current_len + 0 = 2
    let problem = StringToStringCorrection::new(2, vec![0, 1], vec![1, 0], 1);
    let solver = BruteForce::new();
    let solution = solver
        .find_satisfying(&problem)
        .expect("should find a solution");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_string_to_string_correction_paper_example() {
    // Paper example: source [0,1,2,3,1,0], target [0,1,3,2,1], bound 2
    let problem = StringToStringCorrection::new(4, vec![0, 1, 2, 3, 1, 0], vec![0, 1, 3, 2, 1], 2);
    // Verify the known solution
    assert!(problem.evaluate(&[8, 5]));

    // Verify all solutions with brute force
    let solver = BruteForce::new();
    let all_solutions = solver.find_all_satisfying(&problem);
    assert!(!all_solutions.is_empty());
    // The known solution must be among them
    assert!(all_solutions.contains(&vec![8, 5]));
    for sol in &all_solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_string_to_string_correction_unsatisfiable() {
    // bound_k=0, source != target → impossible
    let problem = StringToStringCorrection::new(2, vec![0, 1], vec![1, 0], 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(!problem.evaluate(&[]));

    let solver = BruteForce::new();
    assert!(solver.find_satisfying(&problem).is_none());
}

#[test]
fn test_string_to_string_correction_identity() {
    // source == target, bound_k=0 → satisfied with empty config
    let problem = StringToStringCorrection::new(2, vec![0, 1], vec![0, 1], 0);
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_string_to_string_correction_empty_strings() {
    // Both empty, bound_k=0 → trivially satisfied
    let problem = StringToStringCorrection::new(0, vec![], vec![], 0);
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_string_to_string_correction_delete_only() {
    // source [0,1,2], target [0,2], bound 1
    // Delete index 1: op=1, current_len=3, 1<3 → delete → [0,2] = target
    let problem = StringToStringCorrection::new(3, vec![0, 1, 2], vec![0, 2], 1);
    assert!(problem.evaluate(&[1]));

    let solver = BruteForce::new();
    let solution = solver
        .find_satisfying(&problem)
        .expect("should find a solution");
    assert!(problem.evaluate(&solution));
}
