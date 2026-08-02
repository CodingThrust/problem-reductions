use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn reduce(source: &KColoring<KN, SimpleGraph>) -> ReductionKColoringToSatisfiability {
    ReduceTo::<Satisfiability>::reduce_to(source)
}

#[test]
fn test_kcoloring_to_satisfiability_closed_loop() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::cycle(5), 3);
    let reduction = reduce(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "KColoring->Satisfiability closed loop",
    );
}

#[test]
fn test_kcoloring_to_satisfiability_clause_families_and_sizes() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(2, vec![(0, 1)]), 3);
    let reduction = reduce(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 6);
    assert_eq!(target.num_clauses(), 11);
    assert_eq!(target.num_literals(), 24);
    assert_eq!(
        target.clauses(),
        &[
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![4, 5, 6]),
            CNFClause::new(vec![-1, -2]),
            CNFClause::new(vec![-1, -3]),
            CNFClause::new(vec![-2, -3]),
            CNFClause::new(vec![-4, -5]),
            CNFClause::new(vec![-4, -6]),
            CNFClause::new(vec![-5, -6]),
            CNFClause::new(vec![-1, -4]),
            CNFClause::new(vec![-2, -5]),
            CNFClause::new(vec![-3, -6]),
        ]
    );
}

#[test]
fn test_kcoloring_to_satisfiability_extracts_canonical_coloring() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::cycle(5), 3);
    let reduction = reduce(&source);
    let assignment = vec![1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1];

    assert!(reduction.target_problem().evaluate(&assignment).0);
    assert_eq!(reduction.extract_solution(&assignment), vec![0, 1, 0, 1, 2]);
}

#[test]
fn test_kcoloring_to_satisfiability_five_cycle_feasibility() {
    let solver = BruteForce::new();

    let feasible = KColoring::<KN, _>::with_k(SimpleGraph::cycle(5), 3);
    let feasible_reduction = reduce(&feasible);
    assert_eq!(feasible_reduction.target_problem().num_vars(), 15);
    assert_eq!(feasible_reduction.target_problem().num_clauses(), 35);
    assert_eq!(feasible_reduction.target_problem().num_literals(), 75);
    assert!(solver
        .find_witness(feasible_reduction.target_problem())
        .is_some());

    let infeasible = KColoring::<KN, _>::with_k(SimpleGraph::cycle(5), 2);
    let infeasible_reduction = reduce(&infeasible);
    assert_eq!(infeasible_reduction.target_problem().num_vars(), 10);
    assert_eq!(infeasible_reduction.target_problem().num_clauses(), 20);
    assert_eq!(infeasible_reduction.target_problem().num_literals(), 40);
    assert!(solver
        .find_witness(infeasible_reduction.target_problem())
        .is_none());
}

#[test]
fn test_kcoloring_to_satisfiability_empty_zero_colors_is_satisfiable() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::empty(0), 0);
    let reduction = reduce(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 0);
    assert!(target.clauses().is_empty());
    assert_eq!(BruteForce::new().find_witness(target), Some(vec![]));
    assert_eq!(reduction.extract_solution(&[]), Vec::<usize>::new());
}

#[test]
fn test_kcoloring_to_satisfiability_nonempty_zero_colors_is_infeasible() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::empty(2), 0);
    let reduction = reduce(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 0);
    assert_eq!(
        target.clauses(),
        &[CNFClause::new(vec![]), CNFClause::new(vec![])]
    );
    assert!(BruteForce::new().find_witness(target).is_none());
}

#[test]
fn test_kcoloring_to_satisfiability_isolated_and_disconnected_vertices() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(5, vec![(0, 1), (2, 3)]), 2);
    let reduction = reduce(&source);
    let target_solution = BruteForce::new()
        .find_witness(reduction.target_problem())
        .expect("disconnected bipartite graph must be colorable");
    let source_solution = reduction.extract_solution(&target_solution);

    assert!(source.evaluate(&source_solution).0);
    assert_eq!(source_solution.len(), 5);
}

#[test]
fn test_kcoloring_to_satisfiability_self_loop_is_infeasible() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(1, vec![(0, 0)]), 1);
    let reduction = reduce(&source);

    assert_eq!(
        reduction.target_problem().clauses(),
        &[CNFClause::new(vec![1]), CNFClause::new(vec![-1, -1])]
    );
    assert!(BruteForce::new()
        .find_witness(reduction.target_problem())
        .is_none());
}

#[test]
fn test_kcoloring_to_satisfiability_parallel_edges_duplicate_conflicts() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(2, vec![(0, 1), (0, 1)]), 2);
    let reduction = reduce(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_clauses(), 8);
    assert_eq!(
        target
            .clauses()
            .iter()
            .filter(|clause| clause.literals == vec![-1, -3])
            .count(),
        2
    );
    assert_eq!(
        target
            .clauses()
            .iter()
            .filter(|clause| clause.literals == vec![-2, -4])
            .count(),
        2
    );
}

#[test]
fn test_kcoloring_to_satisfiability_more_colors_than_vertices() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(2, vec![(0, 1)]), 3);
    let reduction = reduce(&source);
    let target_solution = BruteForce::new()
        .find_witness(reduction.target_problem())
        .expect("an edge is colorable with three colors");

    assert!(
        source
            .evaluate(&reduction.extract_solution(&target_solution))
            .0
    );
}
