use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

/// Helper: build the canonical 8-sentence example from the issue.
fn canonical_instance() -> MinimumAxiomSet {
    MinimumAxiomSet::new(
        8,
        vec![0, 1, 2, 3, 4, 5, 6, 7],
        vec![
            (vec![0], 2),
            (vec![0], 3),
            (vec![1], 4),
            (vec![1], 5),
            (vec![2, 4], 6),
            (vec![3, 5], 7),
            (vec![6, 7], 0),
            (vec![6, 7], 1),
        ],
    )
}

#[test]
fn test_minimum_axiom_set_creation() {
    let problem = canonical_instance();
    assert_eq!(problem.num_sentences(), 8);
    assert_eq!(problem.num_true_sentences(), 8);
    assert_eq!(problem.num_implications(), 8);
    assert_eq!(problem.true_sentences(), &[0, 1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(problem.dims(), vec![2; 8]);
    assert_eq!(problem.num_variables(), 8);
}

#[test]
fn test_minimum_axiom_set_evaluate_optimal() {
    let problem = canonical_instance();
    // Select a and b (indices 0, 1): closure = all 8 sentences
    let result = problem.evaluate(&[1, 1, 0, 0, 0, 0, 0, 0]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 2);
}

#[test]
fn test_minimum_axiom_set_evaluate_insufficient() {
    let problem = canonical_instance();
    // Select only a (index 0): closure = {a, c, d} — missing b, e, f, g, h
    let result = problem.evaluate(&[1, 0, 0, 0, 0, 0, 0, 0]);
    assert!(!result.is_valid());
}

#[test]
fn test_minimum_axiom_set_evaluate_all_selected() {
    let problem = canonical_instance();
    // Select all 8 sentences: closure = all 8 trivially
    let result = problem.evaluate(&[1, 1, 1, 1, 1, 1, 1, 1]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 8);
}

#[test]
fn test_minimum_axiom_set_evaluate_none_selected() {
    let problem = canonical_instance();
    // Select nothing: closure = empty
    let result = problem.evaluate(&[0, 0, 0, 0, 0, 0, 0, 0]);
    assert!(!result.is_valid());
}

#[test]
fn test_minimum_axiom_set_evaluate_wrong_length() {
    let problem = canonical_instance();
    let result = problem.evaluate(&[1, 0]);
    assert!(!result.is_valid());
}

#[test]
fn test_minimum_axiom_set_evaluate_out_of_range() {
    let problem = canonical_instance();
    let result = problem.evaluate(&[2, 0, 0, 0, 0, 0, 0, 0]);
    assert!(!result.is_valid());
}

#[test]
fn test_minimum_axiom_set_solver() {
    let problem = canonical_instance();
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 2);
}

#[test]
fn test_minimum_axiom_set_serialization() {
    let problem = canonical_instance();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: MinimumAxiomSet = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_sentences(), problem.num_sentences());
    assert_eq!(restored.true_sentences(), problem.true_sentences());
    assert_eq!(restored.implications(), problem.implications());
}

#[test]
fn test_minimum_axiom_set_partial_true_sentences() {
    // Only sentences 0,1,2 are true; implications: ({0}, 1), ({1}, 2)
    // Optimal: select {0} → closure {0,1,2} = T
    let problem = MinimumAxiomSet::new(5, vec![0, 1, 2], vec![(vec![0], 1), (vec![1], 2)]);
    assert_eq!(problem.num_sentences(), 5);
    assert_eq!(problem.num_true_sentences(), 3);
    assert_eq!(problem.dims(), vec![2; 3]);

    // Select sentence 0 only
    let result = problem.evaluate(&[1, 0, 0]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 1);

    // Select sentence 2 only — cannot derive 0 or 1
    let result = problem.evaluate(&[0, 0, 1]);
    assert!(!result.is_valid());
}

#[test]
fn test_minimum_axiom_set_no_implications() {
    // 3 sentences, all true, no implications
    // Only way to cover T is to select all of them
    let problem = MinimumAxiomSet::new(3, vec![0, 1, 2], vec![]);
    let result = problem.evaluate(&[1, 1, 1]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 3);

    // Selecting only 2 leaves one uncovered
    let result = problem.evaluate(&[1, 1, 0]);
    assert!(!result.is_valid());
}

#[test]
fn test_minimum_axiom_set_paper_example() {
    // The canonical 8-sentence example
    let problem = canonical_instance();

    // Verify the issue's expected outcome: config [1,1,0,0,0,0,0,0] → Min(2)
    let result = problem.evaluate(&[1, 1, 0, 0, 0, 0, 0, 0]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 2);

    // Confirm with brute force that 2 is optimal
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 2);
}
