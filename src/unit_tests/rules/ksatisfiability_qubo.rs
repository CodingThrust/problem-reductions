use super::*;
use crate::models::satisfiability::CNFClause;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_ksatisfiability_to_qubo_closed_loop() {
    // 3 vars, 4 clauses (matches ground truth):
    // (x1 ∨ x2), (¬x1 ∨ x3), (x2 ∨ ¬x3), (¬x2 ∨ ¬x3)
    let ksat = KSatisfiability::<2, i32>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2]),   // x1 ∨ x2
            CNFClause::new(vec![-1, 3]),  // ¬x1 ∨ x3
            CNFClause::new(vec![2, -3]),  // x2 ∨ ¬x3
            CNFClause::new(vec![-2, -3]), // ¬x2 ∨ ¬x3
        ],
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&ksat);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    // Verify all solutions satisfy all clauses
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(ksat.solution_size(&extracted).is_valid);
    }
}

#[test]
fn test_ksatisfiability_to_qubo_simple() {
    // 2 vars, 1 clause: (x1 ∨ x2) → 3 satisfying assignments
    let ksat = KSatisfiability::<2, i32>::new(2, vec![CNFClause::new(vec![1, 2])]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&ksat);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(ksat.solution_size(&extracted).is_valid);
    }
}

#[test]
fn test_ksatisfiability_to_qubo_contradiction() {
    // 1 var, 2 clauses: (x1 ∨ x1) and (¬x1 ∨ ¬x1) — can't satisfy both
    // Actually, this is (x1) and (¬x1), which is a contradiction
    // Max-2-SAT will satisfy 1 of 2 clauses
    let ksat = KSatisfiability::<2, i32>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1]),   // x1 ∨ x1 = x1
            CNFClause::new(vec![-1, -1]), // ¬x1 ∨ ¬x1 = ¬x1
        ],
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&ksat);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    // Both x=0 and x=1 satisfy exactly 1 clause
    assert_eq!(qubo_solutions.len(), 2);
}

#[test]
fn test_ksatisfiability_to_qubo_reversed_vars() {
    // Clause (3, -1) has var_i=2 > var_j=0, triggering the swap branch (line 71).
    // 3 vars, clauses: (x3 ∨ ¬x1), (x1 ∨ x2)
    let ksat = KSatisfiability::<2, i32>::new(
        3,
        vec![
            CNFClause::new(vec![3, -1]), // var 2 > var 0 → swap
            CNFClause::new(vec![1, 2]),
        ],
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&ksat);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(ksat.solution_size(&extracted).is_valid);
    }
}

#[test]
fn test_ksatisfiability_to_qubo_sizes() {
    let ksat = KSatisfiability::<2, i32>::new(
        3,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 3])],
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&ksat);

    let source_size = reduction.source_size();
    let target_size = reduction.target_size();
    assert!(!source_size.components.is_empty());
    assert!(!target_size.components.is_empty());
}
