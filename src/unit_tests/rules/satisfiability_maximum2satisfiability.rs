use super::*;
use crate::models::formula::{CNFClause, Maximum2Satisfiability, Satisfiability};
use crate::rules::test_helpers::{
    assert_satisfaction_round_trip_from_optimization_target, solve_optimization_problem,
};
use crate::rules::traits::ReduceTo;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_satisfiability_to_maximum2satisfiability_structure() {
    let source = Satisfiability::new(
        3,
        vec![CNFClause::new(vec![1, -2, 3]), CNFClause::new(vec![-1, 2])],
    );

    let reduction = ReduceTo::<Maximum2Satisfiability>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 7);
    assert_eq!(target.num_clauses(), 30);
    assert_eq!(target.clauses()[0].literals, vec![1, 1]);
    assert_eq!(target.clauses()[4].literals, vec![-1, 2]);
    assert_eq!(target.clauses()[10].literals, vec![-1, -1]);
    assert_eq!(target.clauses()[20].literals, vec![-1, -1]);
}

#[test]
fn test_satisfiability_to_maximum2satisfiability_closed_loop() {
    let source = Satisfiability::new(
        3,
        vec![CNFClause::new(vec![1, -2, 3]), CNFClause::new(vec![-1, 2])],
    );

    let reduction = ReduceTo::<Maximum2Satisfiability>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "SAT -> Maximum2Satisfiability closed loop",
    );

    assert_eq!(BruteForce::new().solve(target).0, Some(21));
}

#[test]
fn test_satisfiability_to_maximum2satisfiability_unsatisfiable_gap() {
    let source = Satisfiability::new(1, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])]);

    let reduction = ReduceTo::<Maximum2Satisfiability>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(BruteForce::new().solve(target).0, Some(55));

    let target_solution =
        solve_optimization_problem(target).expect("MAX-2-SAT target should always have a witness");
    let extracted = reduction.extract_solution(&target_solution);
    assert!(!source.evaluate(&extracted).0);
}

#[test]
fn test_satisfiability_to_maximum2satisfiability_empty_clause() {
    let source = Satisfiability::new(1, vec![CNFClause::new(vec![])]);

    let reduction = ReduceTo::<Maximum2Satisfiability>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 4);
    assert_eq!(target.num_clauses(), 20);
    assert_eq!(BruteForce::new().solve(target).0, Some(13));
}
