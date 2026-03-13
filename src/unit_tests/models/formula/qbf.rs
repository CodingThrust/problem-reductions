use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_quantifier_creation() {
    let q1 = Quantifier::Exists;
    let q2 = Quantifier::ForAll;
    assert_eq!(q1, Quantifier::Exists);
    assert_eq!(q2, Quantifier::ForAll);
    assert_ne!(q1, q2);
}

#[test]
fn test_qbf_creation() {
    let problem = QuantifiedBooleanFormulas::new(
        3,
        vec![Quantifier::Exists, Quantifier::ForAll, Quantifier::Exists],
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 3])],
    );
    assert_eq!(problem.num_vars(), 3);
    assert_eq!(problem.num_clauses(), 2);
    assert_eq!(problem.num_variables(), 3);
    assert_eq!(problem.quantifiers().len(), 3);
    assert_eq!(problem.clauses().len(), 2);
}

#[test]
#[should_panic(expected = "quantifiers length")]
fn test_qbf_creation_mismatch() {
    QuantifiedBooleanFormulas::new(
        3,
        vec![Quantifier::Exists, Quantifier::ForAll], // Only 2, need 3
        vec![],
    );
}

#[test]
fn test_qbf_evaluate() {
    // F = ∃u_1 ∀u_2 (u_1 ∨ u_2) ∧ (u_1 ∨ ¬u_2)
    let problem = QuantifiedBooleanFormulas::new(
        2,
        vec![Quantifier::Exists, Quantifier::ForAll],
        vec![
            CNFClause::new(vec![1, 2]),  // u_1 OR u_2
            CNFClause::new(vec![1, -2]), // u_1 OR NOT u_2
        ],
    );

    // evaluate() just checks if the CNF is satisfied under the given assignment
    assert!(problem.evaluate(&[1, 0])); // u_1=T, u_2=F: (T∨F)∧(T∨T) = T
    assert!(problem.evaluate(&[1, 1])); // u_1=T, u_2=T: (T∨T)∧(T∨F) = T
    assert!(!problem.evaluate(&[0, 0])); // u_1=F, u_2=F: (F∨F)∧(F∨T) = F
                                         // u_1=F, u_2=T: clause1=(F∨T)=T, clause2=(F∨F)=F → false
    assert!(!problem.evaluate(&[0, 1]));
}

#[test]
fn test_qbf_is_true_simple_true() {
    // F = ∃u_1 ∀u_2 (u_1 ∨ u_2) ∧ (u_1 ∨ ¬u_2)
    // Setting u_1=T satisfies both clauses regardless of u_2
    let problem = QuantifiedBooleanFormulas::new(
        2,
        vec![Quantifier::Exists, Quantifier::ForAll],
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![1, -2])],
    );
    assert!(problem.is_true());
}

#[test]
fn test_qbf_is_true_simple_false() {
    // F = ∀u_1 ∃u_2 (u_1) ∧ (¬u_1)
    // This is always false: no assignment can satisfy both u_1 and NOT u_1
    let problem = QuantifiedBooleanFormulas::new(
        2,
        vec![Quantifier::ForAll, Quantifier::Exists],
        vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])],
    );
    assert!(!problem.is_true());
}

#[test]
fn test_qbf_is_true_all_exists() {
    // When all quantifiers are Exists, QBF reduces to SAT
    // F = ∃u_1 ∃u_2 (u_1 ∨ u_2) ∧ (¬u_1 ∨ ¬u_2)
    let problem = QuantifiedBooleanFormulas::new(
        2,
        vec![Quantifier::Exists, Quantifier::Exists],
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, -2])],
    );
    // Satisfiable: u_1=T,u_2=F or u_1=F,u_2=T
    assert!(problem.is_true());
}

#[test]
fn test_qbf_is_true_all_forall() {
    // F = ∀u_1 ∀u_2 (u_1 ∨ u_2)
    // False: u_1=F, u_2=F fails the clause
    let problem = QuantifiedBooleanFormulas::new(
        2,
        vec![Quantifier::ForAll, Quantifier::ForAll],
        vec![CNFClause::new(vec![1, 2])],
    );
    assert!(!problem.is_true());
}

#[test]
fn test_qbf_is_true_all_forall_tautology() {
    // F = ∀u_1 (u_1 ∨ ¬u_1)
    // Always true (tautology)
    let problem = QuantifiedBooleanFormulas::new(
        1,
        vec![Quantifier::ForAll],
        vec![CNFClause::new(vec![1, -1])],
    );
    assert!(problem.is_true());
}

#[test]
fn test_qbf_empty_formula() {
    // Empty CNF is trivially true
    let problem =
        QuantifiedBooleanFormulas::new(2, vec![Quantifier::Exists, Quantifier::ForAll], vec![]);
    assert!(problem.evaluate(&[0, 0]));
    assert!(problem.is_true());
}

#[test]
fn test_qbf_zero_vars() {
    // Zero variables, empty clauses
    let problem = QuantifiedBooleanFormulas::new(0, vec![], vec![]);
    assert!(problem.evaluate(&[]));
    assert!(problem.is_true());
    assert_eq!(problem.dims(), Vec::<usize>::new());
}

#[test]
fn test_qbf_zero_vars_unsat() {
    // Zero variables, but a clause that refers to var 1 (unsatisfiable)
    let problem = QuantifiedBooleanFormulas::new(0, vec![], vec![CNFClause::new(vec![1])]);
    assert!(!problem.evaluate(&[]));
    assert!(!problem.is_true());
}

#[test]
fn test_qbf_solver() {
    // F = ∃u_1 ∀u_2 (u_1 ∨ u_2) ∧ (u_1 ∨ ¬u_2)
    let problem = QuantifiedBooleanFormulas::new(
        2,
        vec![Quantifier::Exists, Quantifier::ForAll],
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![1, -2])],
    );

    let solver = BruteForce::new();
    // find_satisfying finds any config where evaluate() returns true
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert!(problem.evaluate(&sol));
}

#[test]
fn test_qbf_solver_all_satisfying() {
    // F = ∃u_1 ∀u_2 (u_1 ∨ u_2) ∧ (u_1 ∨ ¬u_2)
    let problem = QuantifiedBooleanFormulas::new(
        2,
        vec![Quantifier::Exists, Quantifier::ForAll],
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![1, -2])],
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    // u_1=T makes both clauses satisfied regardless of u_2
    // So configs [1,0] and [1,1] should satisfy
    assert_eq!(solutions.len(), 2);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_qbf_serialization() {
    let problem = QuantifiedBooleanFormulas::new(
        2,
        vec![Quantifier::Exists, Quantifier::ForAll],
        vec![CNFClause::new(vec![1, -2])],
    );

    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: QuantifiedBooleanFormulas = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.num_vars(), problem.num_vars());
    assert_eq!(deserialized.num_clauses(), problem.num_clauses());
    assert_eq!(deserialized.quantifiers(), problem.quantifiers());
    assert_eq!(deserialized.dims(), problem.dims());
}

#[test]
fn test_qbf_three_vars() {
    // F = ∃u_1 ∀u_2 ∃u_3 (u_1 ∨ u_2 ∨ u_3) ∧ (¬u_1 ∨ ¬u_2 ∨ u_3)
    // Strategy: set u_1=T. Then for any u_2:
    //   Clause 1 is satisfied (u_1=T).
    //   Set u_3=T: Clause 2 = (F ∨ ¬u_2 ∨ T) = T.
    // So this is true.
    let problem = QuantifiedBooleanFormulas::new(
        3,
        vec![Quantifier::Exists, Quantifier::ForAll, Quantifier::Exists],
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    assert!(problem.is_true());
}

#[test]
fn test_qbf_dims() {
    let problem = QuantifiedBooleanFormulas::new(
        4,
        vec![
            Quantifier::Exists,
            Quantifier::ForAll,
            Quantifier::Exists,
            Quantifier::ForAll,
        ],
        vec![CNFClause::new(vec![1, 2, 3, 4])],
    );
    assert_eq!(problem.dims(), vec![2, 2, 2, 2]);
}

#[test]
fn test_qbf_variant() {
    assert_eq!(QuantifiedBooleanFormulas::variant(), vec![]);
}

#[test]
fn test_qbf_quantifier_debug() {
    let q = Quantifier::Exists;
    let debug = format!("{:?}", q);
    assert!(debug.contains("Exists"));
}
