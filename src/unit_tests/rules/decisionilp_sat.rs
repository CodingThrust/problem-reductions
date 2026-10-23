use crate::models::algebraic::{Comparison, LinearConstraint, ObjectiveSense};
use crate::models::{Decision, Satisfiability, ILP};
use crate::rules::{AggregateReductionResult, ReduceTo, ReductionResult};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_decisionilp_to_sat_closed_loop() {
    for (sense, bound, expected) in [
        (ObjectiveSense::Minimize, 1, Some(vec![0, 0, 1])),
        (ObjectiveSense::Minimize, 0, None),
        (ObjectiveSense::Maximize, 3, Some(vec![1, 0, 0])),
        (ObjectiveSense::Maximize, 4, None),
    ] {
        let source = Decision::new(
            ILP::<bool>::new(
                3,
                vec![LinearConstraint::eq(vec![(0, 1), (1, 1), (2, 1)], 1)],
                vec![(0, 3), (1, 2), (2, 1)],
                sense,
            )
            .unwrap(),
            bound,
        );
        let reduction = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap();
        let sat = ReductionResult::target_problem(&reduction);
        let result = ILPSolver::new().solve(sat);
        if let Some(expected) = expected {
            let witness = result.unwrap();
            assert_eq!(reduction.extract_solution(&witness).unwrap(), expected);
            assert_eq!(
                reduction.extract_value(sat.evaluate(&witness).unwrap()),
                Or(true)
            );
            assert!(reduction
                .extract_solution(&witness[..witness.len() - 1].to_vec())
                .is_err());
            let invalid = vec![false; sat.num_vars()];
            assert!(reduction.extract_solution(&invalid).is_err());
        } else {
            assert!(matches!(
                result,
                Err(crate::solvers::ILPSolveError::Infeasible)
            ));
            assert_eq!(reduction.extract_value(Or(false)), Or(false));
        }
    }
}

fn solve_with_source_assignment(sat: &Satisfiability, source: &[i64]) -> Option<Vec<bool>> {
    let mut clauses = sat.clauses().to_vec();
    clauses.extend(source.iter().enumerate().map(|(index, &bit)| {
        let variable = i64::try_from(index + 1).unwrap();
        crate::models::CNFClause::new(vec![if bit == 1 { variable } else { -variable }])
    }));
    let pinned = Satisfiability::new(sat.num_vars(), clauses);
    match ILPSolver::new().solve(&pinned) {
        Err(crate::solvers::ILPSolveError::Infeasible) => None,
        result => Some(result.unwrap()),
    }
}

#[test]
fn test_signed_rows_and_objectives_match_all_binary_assignments() {
    for comparison in [Comparison::Le, Comparison::Eq, Comparison::Ge] {
        for rhs in -4..=4 {
            for sense in [ObjectiveSense::Minimize, ObjectiveSense::Maximize] {
                let row = row(vec![(0, -3), (1, 2), (2, 1)], comparison, rhs);
                let source = Decision::new(
                    ILP::<bool>::new(3, vec![row], vec![(0, -2), (1, 3)], sense).unwrap(),
                    0,
                );
                let reduction = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap();
                for mask in 0..8 {
                    let x: Vec<i64> = (0..3).map(|i| (mask >> i) & 1).collect();
                    let witness = solve_with_source_assignment(
                        ReductionResult::target_problem(&reduction),
                        &x,
                    );
                    assert_eq!(witness.is_some(), source.evaluate(&x).unwrap().0);
                    if let Some(witness) = witness {
                        assert_eq!(reduction.extract_solution(&witness).unwrap(), x);
                    }
                }
            }
        }
    }
}

#[test]
fn test_empty_constant_and_full_width_rows() {
    for comparison in [Comparison::Le, Comparison::Eq, Comparison::Ge] {
        for rhs in [-1, 0, 1] {
            let source = Decision::new(
                ILP::<bool>::new(
                    0,
                    vec![row(vec![], comparison, rhs)],
                    vec![],
                    ObjectiveSense::Minimize,
                )
                .unwrap(),
                0,
            );
            let reduction = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap();
            let witnesses = BruteForce::new()
                .find_all_witnesses(ReductionResult::target_problem(&reduction))
                .unwrap();
            assert_eq!(!witnesses.is_empty(), source.evaluate(&vec![]).unwrap().0);
            for witness in witnesses {
                assert_eq!(
                    reduction.extract_solution(&witness).unwrap(),
                    Vec::<i64>::new()
                );
            }
        }
    }
    for coefficient in [i64::MIN, i64::MAX] {
        for comparison in [Comparison::Le, Comparison::Eq, Comparison::Ge] {
            let source = Decision::new(
                ILP::<bool>::new(
                    1,
                    vec![row(vec![(0, coefficient)], comparison, coefficient)],
                    vec![(0, coefficient)],
                    ObjectiveSense::Minimize,
                )
                .unwrap(),
                coefficient,
            );
            let reduction = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap();
            for x in [vec![0], vec![1]] {
                assert_eq!(
                    solve_with_source_assignment(ReductionResult::target_problem(&reduction), &x)
                        .is_some(),
                    source.evaluate(&x).unwrap().0
                );
            }
        }
    }
}

#[test]
fn test_unrepresentable_source_arithmetic_is_rejected() {
    for terms in [vec![(0, i64::MAX), (1, 1)], vec![(0, i64::MIN), (1, -1)]] {
        for objective in [false, true] {
            let (rows, cost) = if objective {
                (vec![], terms.clone())
            } else {
                (vec![LinearConstraint::le(terms.clone(), 0)], vec![])
            };
            let source = Decision::new(
                ILP::<bool>::new(2, rows, cost, ObjectiveSense::Minimize).unwrap(),
                0,
            );
            assert!(matches!(
                ReduceTo::<Satisfiability>::reduce_to(&source),
                Err(crate::rules::ReductionError::IntegerOverflow { .. })
            ));
        }
    }
}

fn row(terms: Vec<(usize, i64)>, comparison: Comparison, rhs: i64) -> LinearConstraint {
    match comparison {
        Comparison::Le => LinearConstraint::le(terms, rhs),
        Comparison::Eq => LinearConstraint::eq(terms, rhs),
        Comparison::Ge => LinearConstraint::ge(terms, rhs),
    }
}
