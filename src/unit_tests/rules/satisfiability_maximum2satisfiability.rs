use super::*;
use crate::models::decision::Decision;
use crate::models::formula::{CNFClause, Maximum2Satisfiability, Satisfiability};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::traits::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::solvers::SolveOutcome;
use crate::traits::EvaluationError::InvalidConfiguration;
use crate::traits::Problem;
use crate::types::OptimizationValue;

#[test]
fn test_satisfiability_to_maximum2satisfiability_structure() {
    let source = Satisfiability::new(
        3,
        vec![CNFClause::new(vec![1, -2, 3]), CNFClause::new(vec![-1, 2])],
    );

    let reduction = ReduceTo::<Decision<Maximum2Satisfiability>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    let aggregate_target = crate::rules::ReductionResult::target_problem(&reduction);
    assert!(std::ptr::eq(target, aggregate_target));
    assert_eq!(aggregate_target.inner().num_clauses(), 30);
    assert_eq!(target.inner().num_vars(), 7);
    assert_eq!(target.inner().num_clauses(), 30);
    assert_eq!(target.inner().clauses()[0].literals, vec![1, 1]);
    assert_eq!(target.inner().clauses()[4].literals, vec![-1, 2]);
    assert_eq!(target.inner().clauses()[10].literals, vec![-1, -1]);
    assert_eq!(target.inner().clauses()[20].literals, vec![-1, -1]);
}

#[test]
fn test_satisfiability_to_maximum2satisfiability_closed_loop() {
    let source = Satisfiability::new(
        3,
        vec![CNFClause::new(vec![1, -2, 3]), CNFClause::new(vec![-1, 2])],
    );

    let reduction = ReduceTo::<Decision<Maximum2Satisfiability>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "SAT -> Maximum2Satisfiability closed loop",
    );

    assert_eq!(
        target
            .inner()
            .evaluate(&BruteForce::new().solve(target).unwrap().unwrap())
            .unwrap()
            .0,
        Some(21)
    );
}

#[test]
fn test_satisfiability_to_maximum2satisfiability_unsatisfiable_gap() {
    let source = Satisfiability::new(1, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])]);

    let reduction = ReduceTo::<Decision<Maximum2Satisfiability>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    assert!(BruteForce::new().solve(target).unwrap().is_none());

    assert_eq!(
        target
            .inner()
            .evaluate(&BruteForce::new().solve(target.inner()).unwrap().unwrap())
            .unwrap()
            .0,
        Some(55)
    );

    let target_solution = BruteForce::new()
        .solve(target.inner())
        .unwrap()
        .expect("MAX-2-SAT target should always have a witness");
    assert!(!ReductionResult::target_problem(&reduction)
        .evaluate(&target_solution)
        .unwrap()
        .is_valid());
    assert_eq!(
        crate::types::Or(OptimizationValue::meets_bound(
            &(crate::types::Max(Some(55))),
            crate::rules::ReductionResult::target_problem(&reduction).bound()
        )),
        crate::types::Or(false)
    );
}

#[test]
fn test_satisfiability_to_maximum2satisfiability_empty_clause() {
    let source = Satisfiability::new(1, vec![CNFClause::new(vec![])]);

    let reduction = ReduceTo::<Decision<Maximum2Satisfiability>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    assert!(BruteForce::new().solve(target).unwrap().is_none());

    assert_eq!(target.inner().num_vars(), 4);
    assert_eq!(target.inner().num_clauses(), 20);
    assert_eq!(
        target
            .inner()
            .evaluate(&BruteForce::new().solve(target.inner()).unwrap().unwrap())
            .unwrap()
            .0,
        Some(13)
    );
}

#[test]
fn test_satisfiability_to_maximum2satisfiability_all_gadget_assignments() {
    let mut clauses = Vec::new();
    add_gjs_gadget(&CNFClause::new(vec![1, 2, 3]), 4, &mut clauses);
    let target = Maximum2Satisfiability::try_new(4, clauses).unwrap();
    for abc in 0..8 {
        let mut best = 0;
        for w in [false, true] {
            let assignment = vec![abc & 1 != 0, abc & 2 != 0, abc & 4 != 0, w];
            let score = target.evaluate(&assignment).unwrap().0.unwrap();
            assert!(score <= 7);
            if score == 7 {
                assert!(assignment[..3].iter().any(|&v| v));
            }
            best = best.max(score);
        }
        assert_eq!(best, if abc == 0 { 6 } else { 7 });
    }
}

#[test]
fn test_satisfiability_to_maximum2satisfiability_every_target_witness() {
    let mut sources = vec![
        Satisfiability::new(0, vec![]),
        Satisfiability::new(0, vec![CNFClause::new(vec![])]),
    ];
    for n in 1..=3 {
        for len in 0..=5 {
            for pattern in 0..3 {
                let literals = (0..len)
                    .map(|i| {
                        let variable = (i % n + 1) as i64;
                        if (i + pattern) % 3 == 0 {
                            -variable
                        } else {
                            variable
                        }
                    })
                    .collect();
                sources.push(Satisfiability::new(n, vec![CNFClause::new(literals)]));
            }
        }
    }
    for source in sources {
        let reduction = ReduceTo::<Decision<Maximum2Satisfiability>>::reduce_to(&source).unwrap();
        let target = reduction.target_problem();
        let threshold = (target.inner().num_clauses() / 10 * 7) as i64;
        let mut best = 0;
        for bits in 0usize..(1 << target.inner().num_vars()) {
            let assignment = (0..target.inner().num_vars())
                .map(|i| bits & (1 << i) != 0)
                .collect();
            let value = target.inner().evaluate(&assignment).unwrap();
            best = best.max(value.0.unwrap());
            let expected = value == crate::types::Max(Some(threshold));
            assert_eq!(
                crate::types::Or(OptimizationValue::meets_bound(
                    &(value),
                    crate::rules::ReductionResult::target_problem(&reduction).bound()
                )),
                crate::types::Or(expected)
            );
            if expected {
                let decoded = reduction
                    .recover_result(
                        &source,
                        SolveOutcome::optimal(reduction.target_problem(), assignment.clone())
                            .unwrap(),
                    )
                    .unwrap()
                    .into_solution()
                    .expect("qualifying target result must recover a source solution");
                assert!(source.evaluate(&decoded).unwrap().0);
            } else {
                assert!(!ReductionResult::target_problem(&reduction)
                    .evaluate(&assignment)
                    .unwrap()
                    .is_valid());
            }
        }
        let source_yes = BruteForce::new().solve(&source).unwrap().is_some();
        assert_eq!(best == threshold, source_yes);
        assert!(matches!(
            ReductionResult::target_problem(&reduction)
                .inner()
                .evaluate(&vec![false; target.inner().num_vars() + 1]),
            Err(InvalidConfiguration(_))
        ));
        assert_eq!(
            crate::types::Or(OptimizationValue::meets_bound(
                &(crate::types::Max(None)),
                crate::rules::ReductionResult::target_problem(&reduction).bound()
            )),
            crate::types::Or(false)
        );
    }
}
