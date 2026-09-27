use super::*;
use crate::models::algebraic::{IntegerVariable, LinearConstraint, ObjectiveSense};
use crate::rules::AggregateReductionResult;
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Extremum;

#[test]
fn bounded_ilp_embedding_preserves_domains_optima_and_values() {
    for (sense, expected) in [
        (ObjectiveSense::Minimize, -6),
        (ObjectiveSense::Maximize, 3),
    ] {
        let source = ILP::<i64, i64, Bounded>::with_variables(
            vec![IntegerVariable::new(Some(-2), Some(4)).unwrap()],
            vec![LinearConstraint::le(vec![(0, 1)], 1)],
            vec![(0, 3)],
            sense,
        )
        .unwrap();
        let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).unwrap();
        let target = ReductionResult::target_problem(&reduction);
        assert_eq!(target.variables(), source.variables());
        assert_eq!(target.parameters(), source.parameters());
        let solution = ILPSolver::new().solve(target).unwrap();
        let recovered = reduction.extract_solution(&solution).unwrap();
        let value = match sense {
            ObjectiveSense::Minimize => Extremum::minimize(Some(expected)),
            ObjectiveSense::Maximize => Extremum::maximize(Some(expected)),
        };
        assert_eq!(source.evaluate(&recovered).unwrap(), value);
        assert_eq!(
            reduction.extract_value(target.evaluate(&solution).unwrap()),
            value
        );
        assert!(reduction.extract_solution(&vec![-3]).is_err());
        assert!(reduction.extract_solution(&vec![2]).is_err());
        assert!(reduction.extract_solution(&vec![]).is_err());
    }
}
