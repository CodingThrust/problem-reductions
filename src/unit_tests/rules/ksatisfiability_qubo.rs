use super::*;
use crate::models::satisfiability::CNFClause;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

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
        assert!(ksat.evaluate(&extracted));
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
        assert!(ksat.evaluate(&extracted));
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
        assert!(ksat.evaluate(&extracted));
    }
}

#[test]
fn test_ksatisfiability_to_qubo_structure() {
    let ksat = KSatisfiability::<2, i32>::new(
        3,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 3])],
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&ksat);
    let qubo = reduction.target_problem();

    // QUBO should have at least the original variables
    assert!(qubo.num_variables() >= ksat.num_vars());
}

#[test]
fn test_k3satisfiability_to_qubo_closed_loop() {
    // 3-SAT: 5 vars, 7 clauses
    let ksat = KSatisfiability::<3, i32>::new(
        5,
        vec![
            CNFClause::new(vec![1, 2, -3]),  // x1 ∨ x2 ∨ ¬x3
            CNFClause::new(vec![-1, 3, 4]),  // ¬x1 ∨ x3 ∨ x4
            CNFClause::new(vec![2, -4, 5]),  // x2 ∨ ¬x4 ∨ x5
            CNFClause::new(vec![-2, 3, -5]), // ¬x2 ∨ x3 ∨ ¬x5
            CNFClause::new(vec![1, -3, 5]),  // x1 ∨ ¬x3 ∨ x5
            CNFClause::new(vec![-1, -2, 4]), // ¬x1 ∨ ¬x2 ∨ x4
            CNFClause::new(vec![3, -4, -5]), // x3 ∨ ¬x4 ∨ ¬x5
        ],
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&ksat);
    let qubo = reduction.target_problem();

    // QUBO should have 5 + 7 = 12 variables
    assert_eq!(qubo.num_variables(), 12);

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    // Verify all extracted solutions maximize satisfied clauses
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert_eq!(extracted.len(), 5);
        let assignment: Vec<bool> = extracted.iter().map(|&v| v == 1).collect();
        let satisfied = ksat.count_satisfied(&assignment);
        assert_eq!(satisfied, 7, "Expected all 7 clauses satisfied");
    }
}

#[test]
fn test_k3satisfiability_to_qubo_single_clause() {
    // Single 3-SAT clause: (x1 ∨ x2 ∨ x3) — 7 satisfying assignments
    let ksat = KSatisfiability::<3, i32>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&ksat);
    let qubo = reduction.target_problem();

    // 3 vars + 1 auxiliary = 4 total
    assert_eq!(qubo.num_variables(), 4);

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    // All solutions should satisfy the single clause
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert_eq!(extracted.len(), 3);
        assert!(ksat.evaluate(&extracted));
    }
    // 7 out of 8 assignments satisfy (x1 ∨ x2 ∨ x3)
    assert_eq!(qubo_solutions.len(), 7);
}

#[test]
fn test_k3satisfiability_to_qubo_all_negated() {
    // All negated: (¬x1 ∨ ¬x2 ∨ ¬x3) — 7 satisfying assignments
    let ksat = KSatisfiability::<3, i32>::new(3, vec![CNFClause::new(vec![-1, -2, -3])]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&ksat);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(ksat.evaluate(&extracted));
    }
    // 7 out of 8 assignments satisfy (¬x1 ∨ ¬x2 ∨ ¬x3)
    assert_eq!(qubo_solutions.len(), 7);
}
