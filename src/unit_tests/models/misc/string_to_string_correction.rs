use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;

#[test]
fn test_string_to_string_correction_creation() {
    let problem = StringToStringCorrection::new(4, vec![0, 1, 2, 3, 1, 0], vec![0, 1, 3, 2, 1], 2);
    assert_eq!(problem.alphabet_size(), 4);
    assert_eq!(problem.source(), &[0, 1, 2, 3, 1, 0]);
    assert_eq!(problem.target(), &[0, 1, 3, 2, 1]);
    assert_eq!(problem.bound(), 2);
    assert_eq!(problem.source_length(), 6);
    assert_eq!(problem.target_length(), 5);
    // domain = 2*6+1 = 13, bound = 2
    assert_eq!(problem.dimensions(), vec![13; 2]);
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
    assert!(problem.evaluate(&vec![8, 5]).unwrap());
    // All no-ops should not produce target (source != target)
    assert!(!problem.evaluate(&vec![12, 12]).unwrap());
}

#[test]
fn test_string_to_string_correction_invalid_operations() {
    let problem = StringToStringCorrection::new(4, vec![0, 1, 2, 3, 1, 0], vec![0, 1, 3, 2, 1], 2);
    // out-of-domain values
    assert!(matches!(
        problem.evaluate(&vec![13, 5]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![8, 13]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    // wrong length config
    assert!(matches!(
        problem.evaluate(&vec![8]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![8, 5, 12]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_string_to_string_correction_invalid_after_deletion() {
    // After a deletion, some swap indices become invalid
    let problem = StringToStringCorrection::new(2, vec![0, 1, 0], vec![1], 2);
    // source len = 3, domain = 7, noop = 6
    // op=0: delete index 0 → [1, 0], current_len=2
    // op=5: 5 >= 2, swap_pos = 5-2=3, need 3+1<2 → false → invalid
    assert!(!problem.evaluate(&vec![0, 5]).unwrap());
}

#[test]
fn test_string_to_string_correction_serialization() {
    let problem = StringToStringCorrection::new(4, vec![0, 1, 2, 3, 1, 0], vec![0, 1, 3, 2, 1], 2);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: StringToStringCorrection = serde_json::from_value(json).unwrap();
    assert_eq!(restored.alphabet_size(), problem.alphabet_size());
    assert_eq!(restored.source(), problem.source());
    assert_eq!(restored.target(), problem.target());
    assert_eq!(restored.bound(), problem.bound());
}

#[test]
fn test_string_to_string_correction_solver() {
    // Small instance: source [0,1], target [1,0], bound 1
    // Need a single swap: swap_pos=0, value = current_len + 0 = 2
    let problem = StringToStringCorrection::new(2, vec![0, 1], vec![1, 0], 1);
    let solver = BruteForce::new();
    let solution = solver
        .solve(&problem)
        .unwrap()
        .expect("should find a solution");
    assert!(problem.evaluate(&solution).unwrap());
}

#[test]
fn test_string_to_string_correction_paper_example() {
    // Paper example: source [0,1,2,3,1,0], target [0,1,3,2,1], bound 2
    let problem = StringToStringCorrection::new(4, vec![0, 1, 2, 3, 1, 0], vec![0, 1, 3, 2, 1], 2);
    // Verify the known solution
    assert!(problem.evaluate(&vec![8, 5]).unwrap());

    // Verify all solutions with brute force
    let solver = BruteForce::new();
    let all_solutions = solver.find_all_witnesses(&problem).unwrap();
    assert!(!all_solutions.is_empty());
    // The known solution must be among them
    assert!(all_solutions.contains(&vec![8, 5]));
    for sol in &all_solutions {
        assert!(problem.evaluate(sol).unwrap());
    }
}

#[test]
fn test_string_to_string_correction_unsatisfiable() {
    // bound=0, source != target → impossible
    let problem = StringToStringCorrection::new(2, vec![0, 1], vec![1, 0], 0);
    assert_eq!(problem.dimensions(), Vec::<usize>::new());
    assert!(!problem.evaluate(&vec![]).unwrap());

    let solver = BruteForce::new();
    assert!(solver.solve(&problem).unwrap().is_none());
}

#[test]
fn test_string_to_string_correction_identity() {
    // source == target, bound_k=0 → satisfied with empty config
    let problem = StringToStringCorrection::new(2, vec![0, 1], vec![0, 1], 0);
    assert!(problem.evaluate(&vec![]).unwrap());
}

#[test]
fn test_string_to_string_correction_empty_strings() {
    // Both empty, bound_k=0 → trivially satisfied
    let problem = StringToStringCorrection::new(0, vec![], vec![], 0);
    assert!(problem.evaluate(&vec![]).unwrap());
}

#[test]
fn test_string_to_string_correction_delete_only() {
    // source [0,1,2], target [0,2], bound 1
    // Delete index 1: op=1, current_len=3, 1<3 → delete → [0,2] = target
    let problem = StringToStringCorrection::new(3, vec![0, 1, 2], vec![0, 2], 1);
    assert!(problem.evaluate(&vec![1]).unwrap());

    let solver = BruteForce::new();
    let solution = solver
        .solve(&problem)
        .unwrap()
        .expect("should find a solution");
    assert!(problem.evaluate(&solution).unwrap());
}

#[test]
fn test_string_to_string_correction_rejects_target_longer_than_source() {
    let problem = StringToStringCorrection::new(3, vec![0, 1], vec![0, 1, 2], 1);
    assert!(!problem.evaluate(&vec![4]).unwrap());
}

#[test]
fn test_string_to_string_correction_rejects_excessive_deletions_requirement() {
    let problem = StringToStringCorrection::new(4, vec![0, 1, 2, 3], vec![0], 2);
    assert!(!problem.evaluate(&vec![8, 8]).unwrap());
}

#[test]
fn test_string_to_string_correction_is_available_in_prelude() {
    let problem = crate::prelude::StringToStringCorrection::new(2, vec![0], vec![0], 0);
    assert!(problem.evaluate(&vec![]).unwrap());
}

#[test]
fn test_string_to_string_correction_create_spec_derives_alphabet() {
    let problem = StringToStringCorrection::try_from(StringToStringCorrectionCreateSpec {
        alphabet_size: None,
        source_string: vec![0, 3],
        target_string: vec![3],
        bound: 1,
    })
    .unwrap();

    assert_eq!(problem.alphabet_size(), 4);
    assert_eq!(problem.source(), &[0, 3]);
    assert_eq!(problem.target(), &[3]);
    assert_eq!(
        StringToStringCorrectionCreateSpec::FIELDS
            .iter()
            .map(|field| field.name)
            .collect::<Vec<_>>(),
        ["alphabet_size", "source_string", "target_string", "bound"]
    );
}

#[test]
fn test_string_to_string_correction_create_spec_rejects_small_alphabet() {
    let result = StringToStringCorrection::try_from(StringToStringCorrectionCreateSpec {
        alphabet_size: Some(2),
        source_string: vec![2],
        target_string: vec![],
        bound: 1,
    });

    assert!(result.is_err());
}
