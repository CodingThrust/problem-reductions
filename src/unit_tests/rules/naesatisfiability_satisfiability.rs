use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn reduce(source: &NAESatisfiability) -> ReductionNAESATToSAT {
    ReduceTo::<Satisfiability>::reduce_to(source).expect("reduction should succeed")
}

fn assignments(num_vars: usize) -> impl Iterator<Item = Vec<bool>> {
    (0..1usize << num_vars).map(move |mask| (0..num_vars).map(|i| (mask >> i) & 1 == 1).collect())
}

/// Checks that source and target agree on every assignment, that satisfying
/// assignments extract unchanged, and that all others are rejected.
fn assert_equivalent_on_all_assignments(source: &NAESatisfiability) {
    let reduction = reduce(source);
    let target = reduction.target_problem();
    assert_eq!(target.num_vars(), source.num_vars());
    for assignment in assignments(source.num_vars()) {
        let source_ok = source.evaluate(&assignment).unwrap().0;
        assert_eq!(
            source_ok,
            target.evaluate(&assignment).unwrap().0,
            "disagreement on {assignment:?}"
        );
        match reduction.extract_solution(&assignment) {
            Ok(extracted) => {
                assert!(source_ok, "accepted non-witness {assignment:?}");
                assert_eq!(extracted, assignment);
            }
            Err(_) => assert!(!source_ok, "rejected witness {assignment:?}"),
        }
    }
}

fn canonical_source() -> NAESatisfiability {
    NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 2]),
            CNFClause::new(vec![1, -2, -3]),
        ],
    )
}

#[test]
fn test_naesatisfiability_to_satisfiability_closed_loop() {
    let source = canonical_source();
    let reduction = reduce(&source);
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "NAE-SAT->SAT closed loop",
    );
}

#[test]
fn test_naesatisfiability_to_satisfiability_structure() {
    let source = canonical_source();
    let reduction = reduce(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 3);
    assert_eq!(target.num_clauses(), 2 * source.num_clauses());
    assert_eq!(target.num_literals(), 2 * source.num_literals());
    let literals: Vec<Vec<i64>> = target
        .clauses()
        .iter()
        .map(|clause| clause.literals.clone())
        .collect();
    assert_eq!(
        literals,
        vec![
            vec![1, 2, 3],
            vec![-1, -2, -3],
            vec![-1, 2],
            vec![1, -2],
            vec![1, -2, -3],
            vec![-1, 2, 3],
        ]
    );
}

#[test]
fn test_naesatisfiability_to_satisfiability_canonical_all_assignments() {
    let source = canonical_source();
    assert_equivalent_on_all_assignments(&source);

    let witnesses: Vec<Vec<bool>> = assignments(3)
        .filter(|assignment| source.evaluate(assignment).unwrap().0)
        .collect();
    assert_eq!(
        witnesses,
        vec![vec![true, true, false], vec![false, false, true]]
    );
}

#[test]
fn test_naesatisfiability_to_satisfiability_witness_and_near_witness() {
    let reduction = reduce(&canonical_source());

    assert_eq!(
        reduction.extract_solution(&vec![false, false, true]).unwrap(),
        vec![false, false, true]
    );
    // [0,1,0] makes (¬x1, x2) all true, so the excluded all-true CNF clause fails.
    let error = reduction
        .extract_solution(&vec![false, true, false])
        .unwrap_err();
    assert!(error
        .to_string()
        .contains("target assignment does not satisfy CNF clauses"));
}

#[test]
fn test_naesatisfiability_to_satisfiability_rejects_malformed_assignment() {
    let reduction = reduce(&canonical_source());
    assert!(reduction.extract_solution(&vec![false, false]).is_err());
    assert!(reduction
        .extract_solution(&vec![false, false, true, false])
        .is_err());
}

#[test]
fn test_naesatisfiability_to_satisfiability_empty_conjunction() {
    for num_vars in [0, 2] {
        let source = NAESatisfiability::new(num_vars, vec![]);
        let reduction = reduce(&source);
        let target = reduction.target_problem();
        assert_eq!(target.num_vars(), num_vars);
        assert_eq!(target.num_clauses(), 0);
        assert_eq!(target.num_literals(), 0);
        assert_equivalent_on_all_assignments(&source);
    }
}

#[test]
fn test_naesatisfiability_to_satisfiability_unsatisfiable_repeated_variable() {
    let source = NAESatisfiability::new(1, vec![CNFClause::new(vec![1, 1])]);
    let reduction = reduce(&source);
    let literals: Vec<Vec<i64>> = reduction
        .target_problem()
        .clauses()
        .iter()
        .map(|clause| clause.literals.clone())
        .collect();
    assert_eq!(literals, vec![vec![1, 1], vec![-1, -1]]);

    let solver = BruteForce::new();
    assert!(solver.solve(&source).unwrap().is_none());
    assert!(solver.solve(reduction.target_problem()).unwrap().is_none());
    assert_equivalent_on_all_assignments(&source);
}

#[test]
fn test_naesatisfiability_to_satisfiability_repeated_literals() {
    // NAE(x1, x1, x2) holds exactly when x1 != x2.
    let source = NAESatisfiability::new(2, vec![CNFClause::new(vec![1, 1, 2])]);
    assert_equivalent_on_all_assignments(&source);
}

#[test]
fn test_naesatisfiability_to_satisfiability_tautological_clauses() {
    // (x1, ¬x1) always has one true and one false literal.
    let source = NAESatisfiability::new(
        2,
        vec![CNFClause::new(vec![1, -1]), CNFClause::new(vec![2, -2, 1])],
    );
    let reduction = reduce(&source);
    assert_eq!(
        reduction.target_problem().clauses()[1].literals,
        vec![-1, 1]
    );
    assert_equivalent_on_all_assignments(&source);
}

#[test]
fn test_naesatisfiability_to_satisfiability_arbitrary_clause_lengths() {
    let source = NAESatisfiability::new(
        5,
        vec![
            CNFClause::new(vec![1, -2]),
            CNFClause::new(vec![2, 3, -4]),
            CNFClause::new(vec![-1, 3, 4, 5]),
            CNFClause::new(vec![1, 2, -3, -4, 5]),
        ],
    );
    let reduction = reduce(&source);
    let target = reduction.target_problem();
    assert_eq!(target.num_clauses(), 8);
    assert_eq!(target.num_literals(), 2 * 14);
    assert_equivalent_on_all_assignments(&source);
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "NAE-SAT->SAT mixed clause lengths",
    );
}
