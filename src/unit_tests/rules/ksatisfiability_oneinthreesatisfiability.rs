use super::*;
use crate::models::formula::{CNFClause, OneInThreeSatisfiability};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::variant::K3;

#[test]
fn test_ksatisfiability_to_oneinthreesatisfiability_closed_loop() {
    let source = KSatisfiability::<K3>::new(
        4,
        vec![
            CNFClause::new(vec![1, -2, 3]),
            CNFClause::new(vec![2, -3, 4]),
        ],
    );

    let reduction =
        ReduceTo::<OneInThreeSatisfiability>::reduce_to(&source).expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "3SAT->1in3SAT closed loop",
    );
}

#[test]
fn test_ksatisfiability_to_oneinthreesatisfiability_structure_single_clause() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);

    let reduction =
        ReduceTo::<OneInThreeSatisfiability>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 11);
    assert_eq!(target.num_clauses(), 6);
    assert_eq!(
        target.clauses(),
        [
            CNFClause::new(vec![4, 4, 5]),
            CNFClause::new(vec![1, 6, 9]),
            CNFClause::new(vec![2, 7, 9]),
            CNFClause::new(vec![6, 7, 10]),
            CNFClause::new(vec![8, 9, 11]),
            CNFClause::new(vec![3, 8, 4]),
        ]
        .as_slice()
    );
}

#[test]
fn test_ksatisfiability_to_oneinthreesatisfiability_structure_negated_clause() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![-1, -2, -3])]);

    let reduction =
        ReduceTo::<OneInThreeSatisfiability>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 11);
    assert_eq!(target.num_clauses(), 6);
    assert_eq!(
        target.clauses(),
        [
            CNFClause::new(vec![4, 4, 5]),
            CNFClause::new(vec![-1, 6, 9]),
            CNFClause::new(vec![-2, 7, 9]),
            CNFClause::new(vec![6, 7, 10]),
            CNFClause::new(vec![8, 9, 11]),
            CNFClause::new(vec![-3, 8, 4]),
        ]
        .as_slice()
    );
}

#[test]
fn test_ksatisfiability_to_oneinthreesatisfiability_unsatisfiable() {
    let source = KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
        ],
    );

    let reduction =
        ReduceTo::<OneInThreeSatisfiability>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    assert!(solver.solve(&source).unwrap().is_none());
    assert!(solver.solve(target).unwrap().is_none());
}

#[test]
fn test_ksatisfiability_to_oneinthreesatisfiability_extract_solution() {
    let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction =
        ReduceTo::<OneInThreeSatisfiability>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    let target_solution = vec![
        false, false, true, false, true, false, false, false, true, true, false,
    ];
    assert!(target.evaluate(&target_solution).unwrap().0);

    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(extracted, vec![false, false, true]);
    assert!(source.evaluate(&extracted).unwrap().0);
}
