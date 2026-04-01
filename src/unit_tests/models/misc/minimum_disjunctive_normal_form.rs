use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Min;

fn issue_instance() -> MinimumDisjunctiveNormalForm {
    // f(x1,x2,x3) = 1 when exactly 1 or 2 variables are true
    MinimumDisjunctiveNormalForm::new(3, vec![false, true, true, true, true, true, true, false])
}

#[test]
fn test_minimum_dnf_creation() {
    let problem = issue_instance();
    assert_eq!(problem.num_variables(), 3);
    assert_eq!(problem.minterms().len(), 6);
    assert_eq!(problem.num_prime_implicants(), 6);
    assert_eq!(problem.dims(), vec![2; 6]);
}

#[test]
fn test_minimum_dnf_prime_implicants() {
    let problem = issue_instance();
    let pis = problem.prime_implicants();

    // Each prime implicant should cover exactly 2 of the 6 minterms
    for pi in pis {
        let covered: Vec<usize> = problem
            .minterms()
            .iter()
            .filter(|&&mt| pi.covers(mt))
            .copied()
            .collect();
        assert_eq!(covered.len(), 2, "PI {:?} covers {:?}", pi.pattern, covered);
    }
}

#[test]
fn test_minimum_dnf_evaluate_all_selected() {
    let problem = issue_instance();
    // Select all prime implicants — valid but not minimal
    let config = vec![1; 6];
    assert_eq!(problem.evaluate(&config), Min(Some(6)));
}

#[test]
fn test_minimum_dnf_evaluate_none_selected() {
    let problem = issue_instance();
    let config = vec![0; 6];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimum_dnf_evaluate_insufficient() {
    let problem = issue_instance();
    // Select only the first prime implicant — covers at most 2 minterms, not all 6
    let config = vec![1, 0, 0, 0, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimum_dnf_solver() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    let value = solver.solve(&problem);
    assert_eq!(value, Min(Some(3)));
}

#[test]
fn test_minimum_dnf_all_witnesses() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    let witnesses = solver.find_all_witnesses(&problem);
    // Should be exactly 2 optimal covers of size 3
    assert_eq!(witnesses.len(), 2);
    for w in &witnesses {
        assert_eq!(problem.evaluate(w), Min(Some(3)));
    }
}

#[test]
fn test_minimum_dnf_serialization() {
    let problem = issue_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let restored: MinimumDisjunctiveNormalForm = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.num_variables(), 3);
    assert_eq!(restored.num_prime_implicants(), 6);
}

#[test]
fn test_minimum_dnf_two_variables() {
    // f(x1,x2) = x1 XOR x2 = {01, 10}
    let problem = MinimumDisjunctiveNormalForm::new(2, vec![false, true, true, false]);
    assert_eq!(problem.minterms(), &[1, 2]);
    // Prime implicants: ¬x1∧x2 covers {01}, x1∧¬x2 covers {10}
    assert_eq!(problem.num_prime_implicants(), 2);

    let solver = BruteForce::new();
    let value = solver.solve(&problem);
    assert_eq!(value, Min(Some(2))); // Both PIs needed
}

#[test]
fn test_minimum_dnf_single_minterm() {
    // f(x1,x2) = x1 AND x2 = {11}
    let problem = MinimumDisjunctiveNormalForm::new(2, vec![false, false, false, true]);
    assert_eq!(problem.minterms(), &[3]);
    assert_eq!(problem.num_prime_implicants(), 1); // x1∧x2
    let solver = BruteForce::new();
    assert_eq!(solver.solve(&problem), Min(Some(1)));
}

#[test]
fn test_minimum_dnf_tautology_minus_one() {
    // f = all true except 000 and 111 (same as issue example)
    let problem = issue_instance();
    let solver = BruteForce::new();
    assert_eq!(solver.solve(&problem), Min(Some(3)));
}

#[test]
fn test_minimum_dnf_wrong_config_length() {
    let problem = issue_instance();
    assert_eq!(problem.evaluate(&[1, 0, 1]), Min(None));
}

#[test]
#[should_panic(expected = "at least one minterm")]
fn test_minimum_dnf_all_false() {
    MinimumDisjunctiveNormalForm::new(2, vec![false, false, false, false]);
}
