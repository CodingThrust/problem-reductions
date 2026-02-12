use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_sat_to_3sat_exact_size() {
    // Clause already has 3 literals - should remain unchanged
    let sat = Satisfiability::<i32>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);

    let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    assert_eq!(ksat.num_vars(), 3);
    assert_eq!(ksat.num_clauses(), 1);
    assert_eq!(ksat.clauses()[0].literals, vec![1, 2, 3]);
}

#[test]
fn test_sat_to_3sat_padding() {
    // Clause has 2 literals - should be padded to 3
    // (a v b) becomes (a v b v x) AND (a v b v -x)
    let sat = Satisfiability::<i32>::new(2, vec![CNFClause::new(vec![1, 2])]);

    let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    // Should have 2 clauses (positive and negative ancilla)
    assert_eq!(ksat.num_clauses(), 2);
    // All clauses should have exactly 3 literals
    for clause in ksat.clauses() {
        assert_eq!(clause.len(), 3);
    }
}

#[test]
fn test_sat_to_3sat_splitting() {
    // Clause has 4 literals - should be split
    // (a v b v c v d) becomes (a v b v x) AND (-x v c v d)
    let sat = Satisfiability::<i32>::new(4, vec![CNFClause::new(vec![1, 2, 3, 4])]);

    let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    // Should have 2 clauses after splitting
    assert_eq!(ksat.num_clauses(), 2);
    // All clauses should have exactly 3 literals
    for clause in ksat.clauses() {
        assert_eq!(clause.len(), 3);
    }

    // Verify structure: first clause has positive ancilla, second has negative
    let c1 = &ksat.clauses()[0];
    let c2 = &ksat.clauses()[1];
    // First clause: [1, 2, 5] (ancilla is var 5)
    assert_eq!(c1.literals[0], 1);
    assert_eq!(c1.literals[1], 2);
    let ancilla = c1.literals[2];
    assert!(ancilla > 0);
    // Second clause: [-5, 3, 4]
    assert_eq!(c2.literals[0], -ancilla);
    assert_eq!(c2.literals[1], 3);
    assert_eq!(c2.literals[2], 4);
}

#[test]
fn test_sat_to_3sat_large_clause() {
    // Clause has 5 literals - requires multiple splits
    // (a v b v c v d v e) -> (a v b v x1) AND (-x1 v c v x2) AND (-x2 v d v e)
    let sat = Satisfiability::<i32>::new(5, vec![CNFClause::new(vec![1, 2, 3, 4, 5])]);

    let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    // Should have 3 clauses after splitting
    assert_eq!(ksat.num_clauses(), 3);
    // All clauses should have exactly 3 literals
    for clause in ksat.clauses() {
        assert_eq!(clause.len(), 3);
    }
}

#[test]
fn test_sat_to_3sat_single_literal() {
    // Single literal clause - needs padding twice
    // (a) becomes (a v x v y) where we pad twice
    let sat = Satisfiability::<i32>::new(1, vec![CNFClause::new(vec![1])]);

    let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    // With recursive padding: (a) -> (a v x) AND (a v -x)
    // Then each of those gets padded again
    // (a v x) -> (a v x v y) AND (a v x v -y)
    // (a v -x) -> (a v -x v z) AND (a v -x v -z)
    // Total: 4 clauses
    assert_eq!(ksat.num_clauses(), 4);
    for clause in ksat.clauses() {
        assert_eq!(clause.len(), 3);
    }
}

#[test]
fn test_sat_to_3sat_preserves_satisfiability() {
    // Create a SAT formula and verify the 3-SAT version is equisatisfiable
    let sat = Satisfiability::<i32>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2]),         // Needs padding
            CNFClause::new(vec![-1, 2, 3]),     // Already 3 literals
            CNFClause::new(vec![1, -2, 3, -3]), // Needs splitting (tautology for testing)
        ],
    );

    let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    // Solve both problems - use find_all_satisfying for satisfaction problems
    let solver = BruteForce::new();

    let sat_solutions = solver.find_all_satisfying(&sat);
    let ksat_solutions = solver.find_all_satisfying(ksat);

    // If SAT is satisfiable, K-SAT should be too
    let sat_satisfiable = !sat_solutions.is_empty();
    let ksat_satisfiable = !ksat_solutions.is_empty();

    assert_eq!(sat_satisfiable, ksat_satisfiable);

    // Extract solutions should map back correctly
    if ksat_satisfiable {
        for ksat_sol in &ksat_solutions {
            let sat_sol = reduction.extract_solution(ksat_sol);
            assert_eq!(sat_sol.len(), 3); // Original variable count
        }
    }
}

#[test]
fn test_sat_to_3sat_solution_extraction() {
    let sat = Satisfiability::<i32>::new(2, vec![CNFClause::new(vec![1, 2])]);

    let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    // Solve K-SAT - use find_all_satisfying for satisfaction problems
    let solver = BruteForce::new();
    let ksat_solutions = solver.find_all_satisfying(ksat);

    // Extract and verify solutions
    for ksat_sol in &ksat_solutions {
        let sat_sol = reduction.extract_solution(ksat_sol);
        // Should only have original 2 variables
        assert_eq!(sat_sol.len(), 2);
        // Should satisfy original problem
        assert!(sat.evaluate(&sat_sol));
    }
}

#[test]
fn test_3sat_to_sat() {
    let ksat = KSatisfiability::<3, i32>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );

    let reduction = ReduceTo::<Satisfiability<i32>>::reduce_to(&ksat);
    let sat = reduction.target_problem();

    assert_eq!(sat.num_vars(), 3);
    assert_eq!(sat.num_clauses(), 2);

    // Verify clauses are preserved
    assert_eq!(sat.clauses()[0].literals, vec![1, 2, 3]);
    assert_eq!(sat.clauses()[1].literals, vec![-1, -2, 3]);
}

#[test]
fn test_3sat_to_sat_solution_extraction() {
    let ksat = KSatisfiability::<3, i32>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);

    let reduction = ReduceTo::<Satisfiability<i32>>::reduce_to(&ksat);

    let sol = vec![1, 0, 1];
    let extracted = reduction.extract_solution(&sol);
    assert_eq!(extracted, vec![1, 0, 1]);
}

#[test]
fn test_roundtrip_sat_3sat_sat() {
    // SAT -> 3-SAT -> SAT roundtrip
    let original_sat = Satisfiability::<i32>::new(
        3,
        vec![CNFClause::new(vec![1, -2]), CNFClause::new(vec![2, 3])],
    );

    // SAT -> 3-SAT
    let to_ksat = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&original_sat);
    let ksat = to_ksat.target_problem();

    // 3-SAT -> SAT
    let to_sat = ReduceTo::<Satisfiability<i32>>::reduce_to(ksat);
    let final_sat = to_sat.target_problem();

    // Solve all three - use find_all_satisfying for satisfaction problems
    let solver = BruteForce::new();

    let orig_solutions = solver.find_all_satisfying(&original_sat);
    let ksat_solutions = solver.find_all_satisfying(ksat);
    let final_solutions = solver.find_all_satisfying(final_sat);

    // All should be satisfiable (have at least one solution)
    assert!(!orig_solutions.is_empty());
    assert!(!ksat_solutions.is_empty());
    assert!(!final_solutions.is_empty());
}

#[test]
fn test_sat_to_4sat() {
    let sat = Satisfiability::<i32>::new(
        4,
        vec![
            CNFClause::new(vec![1, 2]),           // Needs padding
            CNFClause::new(vec![1, 2, 3, 4]),     // Exact
            CNFClause::new(vec![1, 2, 3, 4, -1]), // Needs splitting
        ],
    );

    let reduction = ReduceTo::<KSatisfiability<4, i32>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    // All clauses should have exactly 4 literals
    for clause in ksat.clauses() {
        assert_eq!(clause.len(), 4);
    }
}

#[test]
fn test_ksat_structure() {
    let sat = Satisfiability::<i32>::new(3, vec![CNFClause::new(vec![1, 2, 3, 4])]);

    let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    // K-SAT should preserve original variables plus auxiliary vars
    // A 4-literal clause requires 1 auxiliary variable for Tseitin
    assert_eq!(ksat.num_vars(), 3 + 1); // Original vars + 1 auxiliary for Tseitin
}

#[test]
fn test_empty_sat_to_3sat() {
    let sat = Satisfiability::<i32>::new(3, vec![]);

    let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    assert_eq!(ksat.num_clauses(), 0);
    assert_eq!(ksat.num_vars(), 3);
}

#[test]
fn test_mixed_clause_sizes() {
    let sat = Satisfiability::<i32>::new(
        5,
        vec![
            CNFClause::new(vec![1]),             // 1 literal
            CNFClause::new(vec![2, 3]),          // 2 literals
            CNFClause::new(vec![1, 2, 3]),       // 3 literals
            CNFClause::new(vec![1, 2, 3, 4]),    // 4 literals
            CNFClause::new(vec![1, 2, 3, 4, 5]), // 5 literals
        ],
    );

    let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    // All clauses should have exactly 3 literals
    for clause in ksat.clauses() {
        assert_eq!(clause.len(), 3);
    }

    // Verify satisfiability is preserved - use find_all_satisfying for satisfaction problems
    let solver = BruteForce::new();
    let sat_solutions = solver.find_all_satisfying(&sat);
    let ksat_solutions = solver.find_all_satisfying(ksat);

    let sat_satisfiable = !sat_solutions.is_empty();
    let ksat_satisfiable = !ksat_solutions.is_empty();
    assert_eq!(sat_satisfiable, ksat_satisfiable);
}

#[test]
fn test_unsatisfiable_formula() {
    // (x) AND (-x) is unsatisfiable
    let sat =
        Satisfiability::<i32>::new(1, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])]);

    let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    let solver = BruteForce::new();

    // Both should be unsatisfiable - use find_all_satisfying for satisfaction problems
    let sat_solutions = solver.find_all_satisfying(&sat);
    let ksat_solutions = solver.find_all_satisfying(ksat);

    let sat_satisfiable = !sat_solutions.is_empty();
    let ksat_satisfiable = !ksat_solutions.is_empty();

    assert!(!sat_satisfiable);
    assert!(!ksat_satisfiable);
}
