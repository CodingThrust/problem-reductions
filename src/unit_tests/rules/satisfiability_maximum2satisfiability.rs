use super::*;
use crate::models::formula::{CNFClause, Maximum2Satisfiability, Satisfiability};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::rules::traits::ReduceTo;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_satisfiability_to_maximum2satisfiability_structure() {
    let source = Satisfiability::new(
        3,
        vec![CNFClause::new(vec![1, -2, 3]), CNFClause::new(vec![-1, 2])],
    );

    let reduction =
        ReduceTo::<Maximum2Satisfiability>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    let aggregate_target = crate::rules::AggregateReductionResult::target_problem(&reduction);
    assert!(std::ptr::eq(target, aggregate_target));
    assert_eq!(aggregate_target.num_clauses(), 30);
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

    let reduction =
        ReduceTo::<Maximum2Satisfiability>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "SAT -> Maximum2Satisfiability closed loop",
    );

    assert_eq!(
        target
            .evaluate(&BruteForce::new().solve(target).unwrap().unwrap())
            .unwrap()
            .0,
        Some(21)
    );
}

#[test]
fn test_satisfiability_to_maximum2satisfiability_unsatisfiable_gap() {
    let source = Satisfiability::new(1, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])]);

    let reduction =
        ReduceTo::<Maximum2Satisfiability>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(
        target
            .evaluate(&BruteForce::new().solve(target).unwrap().unwrap())
            .unwrap()
            .0,
        Some(55)
    );

    let target_solution = BruteForce::new()
        .solve(target)
        .unwrap()
        .expect("MAX-2-SAT target should always have a witness");
    assert!(reduction.extract_solution(&target_solution).is_err());
    assert_eq!(
        crate::rules::AggregateReductionResult::extract_value(&reduction, Max(Some(55))),
        Or(false)
    );
}

#[test]
fn test_satisfiability_to_maximum2satisfiability_empty_clause() {
    let source = Satisfiability::new(1, vec![CNFClause::new(vec![])]);

    let reduction =
        ReduceTo::<Maximum2Satisfiability>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 4);
    assert_eq!(target.num_clauses(), 20);
    assert_eq!(
        target
            .evaluate(&BruteForce::new().solve(target).unwrap().unwrap())
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
        let reduction = ReduceTo::<Maximum2Satisfiability>::reduce_to(&source).unwrap();
        let target = reduction.target_problem();
        let threshold = (target.num_clauses() / 10 * 7) as i64;
        let mut best = 0;
        for bits in 0usize..(1 << target.num_vars()) {
            let assignment = (0..target.num_vars())
                .map(|i| bits & (1 << i) != 0)
                .collect();
            let value = target.evaluate(&assignment).unwrap();
            best = best.max(value.0.unwrap());
            let expected = value == Max(Some(threshold));
            assert_eq!(
                crate::rules::AggregateReductionResult::extract_value(&reduction, value),
                Or(expected)
            );
            if expected {
                let decoded = reduction.extract_solution(&assignment).unwrap();
                assert!(source.evaluate(&decoded).unwrap().0);
            } else {
                assert!(reduction.extract_solution(&assignment).is_err());
            }
        }
        let source_yes = BruteForce::new().solve(&source).unwrap().is_some();
        assert_eq!(best == threshold, source_yes);
        assert!(reduction
            .extract_solution(&vec![false; target.num_vars() + 1])
            .is_err());
        assert_eq!(
            crate::rules::AggregateReductionResult::extract_value(&reduction, Max(None)),
            Or(false)
        );
    }
}
