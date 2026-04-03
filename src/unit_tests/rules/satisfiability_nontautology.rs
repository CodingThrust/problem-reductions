use crate::models::formula::{CNFClause, NonTautology, Satisfiability};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionGraph, ReductionResult};
use crate::solvers::BruteForce;

#[test]
fn test_satisfiability_to_non_tautology_structure() {
    let source = Satisfiability::new(
        3,
        vec![CNFClause::new(vec![1, -2]), CNFClause::new(vec![-1, 3])],
    );

    let reduction = ReduceTo::<NonTautology>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 3);
    assert_eq!(target.num_disjuncts(), 2);
    assert_eq!(target.disjuncts(), &[vec![-1, 2], vec![1, -3]]);
}

#[test]
fn test_satisfiability_to_non_tautology_closed_loop() {
    let source = Satisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2]),
            CNFClause::new(vec![-1, 3]),
            CNFClause::new(vec![-2, -3]),
        ],
    );

    let reduction = ReduceTo::<NonTautology>::reduce_to(&source);
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "SAT->NonTautology closed loop",
    );
}

#[test]
fn test_satisfiability_to_non_tautology_unsatisfiable_source_has_no_target_witness() {
    let source = Satisfiability::new(1, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])]);

    let reduction = ReduceTo::<NonTautology>::reduce_to(&source);
    let solver = BruteForce::new();

    assert_eq!(solver.find_witness(reduction.target_problem()), None);
}

#[test]
fn test_satisfiability_to_non_tautology_extract_solution_is_identity() {
    let source = Satisfiability::new(2, vec![CNFClause::new(vec![1]), CNFClause::new(vec![2])]);

    let reduction = ReduceTo::<NonTautology>::reduce_to(&source);
    let target_solution = BruteForce::new()
        .find_witness(reduction.target_problem())
        .expect("target should have a witness");

    assert_eq!(
        reduction.extract_solution(&target_solution),
        target_solution
    );
}

#[test]
fn test_reduction_graph_registers_satisfiability_to_non_tautology() {
    let graph = ReductionGraph::new();
    assert!(graph.has_direct_reduction_by_name("Satisfiability", "NonTautology"));
}
