use super::*;
use crate::solvers::BruteForce;
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
    assert_eq!(problem.num_variables(), 0);
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
fn test_qbf_evaluate_true() {
    // F = ∃u_1 ∀u_2 (u_1 ∨ u_2) ∧ (u_1 ∨ ¬u_2)
    // Setting u_1=T satisfies both clauses regardless of u_2
    let problem = QuantifiedBooleanFormulas::new(
        2,
        vec![Quantifier::Exists, Quantifier::ForAll],
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![1, -2])],
    );

    // dims() is empty; evaluate([]) runs the game-tree search
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[]));
    assert!(problem.is_true());
}

#[test]
fn test_qbf_evaluate_false() {
    // F = ∀u_1 ∃u_2 (u_1) ∧ (¬u_1)
    // Always false: no assignment can satisfy both u_1 and NOT u_1
    let problem = QuantifiedBooleanFormulas::new(
        2,
        vec![Quantifier::ForAll, Quantifier::Exists],
        vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])],
    );

    assert!(!problem.evaluate(&[]));
    assert!(!problem.is_true());
}

#[test]
fn test_qbf_evaluate_nonempty_config_returns_false() {
    // Non-empty config is always false (no external variables)
    let problem = QuantifiedBooleanFormulas::new(
        2,
        vec![Quantifier::Exists, Quantifier::ForAll],
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![1, -2])],
    );
    assert!(!problem.evaluate(&[1, 0]));
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
    assert!(problem.evaluate(&[]));
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
    // F = ∃u_1 ∀u_2 (u_1 ∨ u_2) ∧ (u_1 ∨ ¬u_2) — TRUE
    let problem = QuantifiedBooleanFormulas::new(
        2,
        vec![Quantifier::Exists, Quantifier::ForAll],
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![1, -2])],
    );

    let solver = BruteForce::new();
    // With dims()=[], there is exactly one config: []. evaluate([]) = is_true() = true
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert_eq!(sol, Vec::<usize>::new());
    assert!(problem.evaluate(&sol));
}

#[test]
fn test_qbf_solver_false() {
    // F = ∀u_1 ∃u_2 (u_1) ∧ (¬u_1) — FALSE
    let problem = QuantifiedBooleanFormulas::new(
        2,
        vec![Quantifier::ForAll, Quantifier::Exists],
        vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])],
    );

    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_none());
}

#[test]
fn test_qbf_solver_all_satisfying() {
    // F = ∃u_1 ∀u_2 (u_1 ∨ u_2) ∧ (u_1 ∨ ¬u_2) — TRUE
    let problem = QuantifiedBooleanFormulas::new(
        2,
        vec![Quantifier::Exists, Quantifier::ForAll],
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![1, -2])],
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    // Only one config exists (the empty config []), and it satisfies
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], Vec::<usize>::new());
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
    // dims() is always empty — QBF has no external config variables
    assert_eq!(problem.dims(), Vec::<usize>::new());
}

#[test]
fn test_qbf_variant() {
    assert_eq!(QuantifiedBooleanFormulas::variant(), vec![]);
}

#[test]
fn test_qbf_quantifier_clone() {
    let q = Quantifier::Exists;
    let q2 = q;
    assert_eq!(q, q2);
    let q3 = Quantifier::ForAll;
    assert_ne!(q, q3);
}

#[test]
fn test_qbf_empty_clause() {
    // An empty clause (disjunction of zero literals) is always false
    let problem =
        QuantifiedBooleanFormulas::new(1, vec![Quantifier::Exists], vec![CNFClause::new(vec![])]);
    assert!(!problem.is_true());
}
