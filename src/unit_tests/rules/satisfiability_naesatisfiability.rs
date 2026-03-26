use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_satisfiability_to_naesatisfiability_closed_loop() {
    // Formula: (x1 ∨ x2) ∧ (¬x1 ∨ x3) ∧ (¬x2 ∨ ¬x3)
    let sat = Satisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2]),
            CNFClause::new(vec![-1, 3]),
            CNFClause::new(vec![-2, -3]),
        ],
    );

    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&sat);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &sat,
        &reduction,
        "SAT->NAE-SAT closed loop",
    );
}

#[test]
fn test_reduction_structure() {
    let sat = Satisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2]),
            CNFClause::new(vec![-1, 3]),
            CNFClause::new(vec![-2, -3]),
        ],
    );

    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&sat);
    let naesat = reduction.target_problem();

    // Should have n+1 variables (3 original + 1 sentinel)
    assert_eq!(naesat.num_vars(), 4);
    // Same number of clauses
    assert_eq!(naesat.num_clauses(), 3);

    // Each clause should have one extra literal (the sentinel)
    for (i, clause) in naesat.clauses().iter().enumerate() {
        assert_eq!(
            clause.len(),
            sat.clauses()[i].len() + 1,
            "clause {} should have one extra literal",
            i
        );
        // Last literal should be the sentinel (positive literal for variable 3, i.e., literal 4)
        assert_eq!(*clause.literals.last().unwrap(), 4);
    }
}

#[test]
fn test_solution_extraction_sentinel_false() {
    // When sentinel is false, return original variables as-is
    let sat = Satisfiability::new(3, vec![CNFClause::new(vec![1, 2])]);

    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&sat);

    // target_solution: [1, 0, 1, 0] means x1=true, x2=false, x3=true, sentinel=false
    let extracted = reduction.extract_solution(&[1, 0, 1, 0]);
    assert_eq!(extracted, vec![1, 0, 1]);
}

#[test]
fn test_solution_extraction_sentinel_true() {
    // When sentinel is true, return complement of original variables
    let sat = Satisfiability::new(3, vec![CNFClause::new(vec![1, 2])]);

    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&sat);

    // target_solution: [0, 1, 0, 1] means x1=false, x2=true, x3=false, sentinel=true
    // Complement: x1=true, x2=false, x3=true
    let extracted = reduction.extract_solution(&[0, 1, 0, 1]);
    assert_eq!(extracted, vec![1, 0, 1]);
}

#[test]
fn test_unsatisfiable_formula() {
    // (x1) ∧ (¬x1) is unsatisfiable
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])]);

    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&sat);
    let naesat = reduction.target_problem();

    let solver = BruteForce::new();
    let sat_solutions = solver.find_all_witnesses(&sat);
    let naesat_solutions = solver.find_all_witnesses(naesat);

    // Both should be unsatisfiable
    assert!(sat_solutions.is_empty());
    assert!(naesat_solutions.is_empty());
}

#[test]
fn test_single_clause() {
    let sat = Satisfiability::new(2, vec![CNFClause::new(vec![1, -2])]);

    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&sat);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &sat,
        &reduction,
        "SAT->NAE-SAT single clause",
    );
}

#[test]
fn test_empty_formula() {
    // Empty formula is trivially satisfiable
    let sat = Satisfiability::new(2, vec![]);

    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&sat);
    let naesat = reduction.target_problem();

    assert_eq!(naesat.num_vars(), 3);
    assert_eq!(naesat.num_clauses(), 0);

    // Both should be satisfiable (any assignment works)
    let solver = BruteForce::new();
    assert!(!solver.find_all_witnesses(&sat).is_empty());
    assert!(!solver.find_all_witnesses(naesat).is_empty());
}

#[test]
fn test_larger_instance() {
    // A larger SAT instance to stress the reduction
    let sat = Satisfiability::new(
        5,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 4]),
            CNFClause::new(vec![2, -3, 5]),
            CNFClause::new(vec![-4, -5]),
            CNFClause::new(vec![1, 3, -5]),
        ],
    );

    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&sat);
    let naesat = reduction.target_problem();

    assert_eq!(naesat.num_vars(), 6);
    assert_eq!(naesat.num_clauses(), 5);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &sat,
        &reduction,
        "SAT->NAE-SAT larger instance",
    );
}

#[test]
fn test_all_satisfying_assignments_map_back() {
    // Small instance: verify every NAE-SAT solution maps to a valid SAT solution
    let sat = Satisfiability::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, -2])],
    );

    let reduction = ReduceTo::<NAESatisfiability>::reduce_to(&sat);
    let naesat = reduction.target_problem();

    let solver = BruteForce::new();
    let nae_solutions = solver.find_all_witnesses(naesat);

    for nae_sol in &nae_solutions {
        let sat_sol = reduction.extract_solution(nae_sol);
        assert_eq!(sat_sol.len(), 2);
        assert!(
            sat.evaluate(&sat_sol).0,
            "Extracted solution {:?} from NAE solution {:?} does not satisfy SAT",
            sat_sol,
            nae_sol
        );
    }
}
