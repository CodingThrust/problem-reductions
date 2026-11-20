use super::*;
use crate::models::formula::CNFClause;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::rules::{ExtractionError, ReductionResult};
use crate::solvers::{ILPSolveError, ILPSolver};
use crate::traits::{EvaluationError, Problem};

#[test]
fn test_satisfiability_to_ilp_closed_loop() {
    let source = Satisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 2]),
            CNFClause::new(vec![-1, 3]),
            CNFClause::new(vec![-2, -3]),
            CNFClause::new(vec![1, -1]),
        ],
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn clause_rows_preserve_empty_repeated_and_opposite_literals() {
    let cases = [
        (vec![], LinearConstraint::ge(vec![], 1)),
        (
            vec![1, 1, -2, -2],
            LinearConstraint::ge(vec![(0, 2), (1, -2)], -1),
        ),
        (vec![1, -1], LinearConstraint::ge(vec![], 0)),
        (vec![1, -1, -2], LinearConstraint::ge(vec![(1, -1)], -1)),
    ];
    for (literals, row) in cases {
        let source = Satisfiability::new(2, vec![CNFClause::new(literals)]);
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        let target = reduction.target_problem();
        assert_eq!(target.num_vars(), 2);
        assert!(target.objective().is_empty());
        assert_eq!(target.constraints(), &[row]);
        for bits in [[false, false], [false, true], [true, false], [true, true]] {
            let solution: Vec<_> = bits.into_iter().map(i64::from).collect();
            assert_eq!(
                target.is_feasible(&solution).unwrap(),
                source.evaluate(&bits.to_vec()).unwrap().0
            );
        }
    }
}

#[test]
fn an_empty_clause_makes_the_ilp_infeasible() {
    for num_vars in [0, 2] {
        let source = Satisfiability::new(num_vars, vec![CNFClause::new(vec![])]);
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        assert!(matches!(
            ILPSolver::new().solve(reduction.target_problem()),
            Err(ILPSolveError::Infeasible)
        ));
    }
}

#[test]
fn extraction_rejects_malformed_or_infeasible_assignments() {
    let source = Satisfiability::new(2, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    assert!(matches!(
        reduction.extract_solution(&vec![1]),
        Err(ExtractionError::Evaluation(
            EvaluationError::InvalidConfiguration(_)
        ))
    ));
    for invalid in [vec![1, 2], vec![0, 0]] {
        assert!(matches!(
            reduction.extract_solution(&invalid),
            Err(ExtractionError::InvalidTargetSolution(_))
        ));
    }
}

#[test]
fn ilp_feasibility_maps_to_the_sat_value() {
    use crate::rules::AggregateReductionResult;
    use crate::solvers::BruteForce;
    use crate::types::{Extremum, Or};

    let solver = BruteForce::new();
    for clauses in [
        vec![],
        vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])],
    ] {
        let source = Satisfiability::new(1, clauses);
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        let target = ReductionResult::target_problem(&reduction);
        let value = match ILPSolver::new().solve(target) {
            Ok(solution) => target.evaluate(&solution).unwrap(),
            Err(ILPSolveError::Infeasible) => Extremum::minimize(None),
            Err(error) => panic!("unexpected ILP solver error: {error}"),
        };
        assert_eq!(
            reduction.extract_value(value).unwrap(),
            Or(solver.solve(&source).unwrap().is_some())
        );
    }
}
