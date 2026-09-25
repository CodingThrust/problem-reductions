use super::*;
use crate::models::algebraic::{IntegerVariable, LinearConstraint};

#[test]
fn backend_statuses_preserve_termination_causes() {
    assert_eq!(accept_backend_status(HighsModelStatus::Optimal), Ok(()));
    assert_eq!(
        accept_backend_status(HighsModelStatus::Infeasible),
        Err(ILPSolveError::Infeasible)
    );
    assert_eq!(
        accept_backend_status(HighsModelStatus::Unbounded),
        Err(ILPSolveError::Unbounded)
    );
    assert_eq!(
        accept_backend_status(HighsModelStatus::ReachedTimeLimit),
        Err(ILPSolveError::Timeout)
    );
    for status in [
        HighsModelStatus::UnboundedOrInfeasible,
        HighsModelStatus::SolveError,
        HighsModelStatus::ObjectiveBound,
        HighsModelStatus::ObjectiveTarget,
        HighsModelStatus::ReachedIterationLimit,
        HighsModelStatus::ReachedMemoryLimit,
        HighsModelStatus::ReachedSolutionLimit,
        HighsModelStatus::ReachedInterrupt,
    ] {
        assert!(matches!(accept_backend_status(status),
            Err(ILPSolveError::BackendFailure(message)) if message.contains(&format!("{status:?}"))));
    }
}

#[test]
fn native_terminals_return_the_input_ilp_solution_format() {
    let adapter = HighsAdapter::new(None);
    let boolean_integer = ILP::<bool, i64>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1), (1, 1)], 1)],
        vec![(0, 1), (1, 2)],
        ObjectiveSense::Maximize,
    )
    .unwrap();
    let boolean_float = ILP::<bool, f64>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    )
    .unwrap();
    let integer_integer = ILP::<i64, i64>::with_variables(
        vec![IntegerVariable::new(Some(-2), Some(3)).unwrap()],
        vec![],
        vec![(0, 1)],
        ObjectiveSense::Minimize,
    )
    .unwrap();
    let integer_float = ILP::<i64, f64>::with_variables(
        vec![IntegerVariable::new(Some(-2), Some(3)).unwrap()],
        vec![],
        vec![(0, 1.0)],
        ObjectiveSense::Minimize,
    )
    .unwrap();
    let values: [Vec<i64>; 4] = [
        adapter.solve(&boolean_integer).unwrap(),
        adapter.solve(&boolean_float).unwrap(),
        adapter.solve(&integer_integer).unwrap(),
        adapter.solve(&integer_float).unwrap(),
    ];
    assert_eq!(values, [vec![0, 1], vec![0, 1], vec![-2], vec![-2]]);
}

#[test]
fn decoding_checks_shape_integrality_range_and_original_constraints() {
    let ilp = ILP::<bool>::new(
        2,
        vec![LinearConstraint::eq(vec![(0, 1), (1, 1)], 1)],
        vec![(0, 1)],
        ObjectiveSense::Maximize,
    )
    .unwrap();
    assert_eq!(
        decode_and_validate(&ilp, [1.00000001, 0.0]).unwrap(),
        vec![1, 0]
    );
    for raw in [
        vec![],
        vec![1.0],
        vec![1.0, 0.0, 0.0],
        vec![1.0, 1.0],
        vec![2.0, -1.0],
        vec![0.5, 0.5],
        vec![f64::NAN, 0.0],
        vec![f64::INFINITY, 0.0],
        vec![f64::NEG_INFINITY, 0.0],
        vec![i64::MAX as f64, 0.0],
        vec![i64::MIN as f64, 0.0],
    ] {
        assert!(matches!(
            decode_and_validate(&ilp, raw),
            Err(ILPSolveError::InvalidSolution(_))
        ));
    }
}

#[test]
fn validation_rejects_constraint_violations_in_both_coefficient_domains() {
    let integer = ILP::<bool>::new(
        1,
        vec![LinearConstraint::le(vec![(0, 2)], 1)],
        vec![],
        ObjectiveSense::Minimize,
    )
    .unwrap();
    assert!(matches!(
        decode_and_validate(&integer, [1.0]),
        Err(ILPSolveError::InvalidSolution(_))
    ));
    let float = ILP::<bool, f64>::new(
        1,
        vec![LinearConstraint::le(vec![(0, 2.0)], 1.0)],
        vec![],
        ObjectiveSense::Minimize,
    )
    .unwrap();
    assert!(!float.is_feasible(&[1]).unwrap());
    assert!(matches!(
        decode_and_validate(&float, [1.0]),
        Err(ILPSolveError::InvalidSolution(_))
    ));
}

#[test]
fn validation_propagates_constraint_and_objective_overflow() {
    let objective = ILP::<bool>::new(
        2,
        vec![],
        vec![(0, i64::MAX), (1, 1)],
        ObjectiveSense::Maximize,
    )
    .unwrap();
    let constraint = ILP::<bool>::new(
        2,
        vec![LinearConstraint::le(vec![(0, i64::MAX), (1, 1)], 0)],
        vec![],
        ObjectiveSense::Maximize,
    )
    .unwrap();
    for ilp in [objective, constraint] {
        assert!(matches!(
            decode_and_validate(&ilp, [1.0, 1.0]),
            Err(ILPSolveError::InvalidSolution(_))
        ));
    }
}

#[test]
fn coefficient_encoding_enforces_supported_transport_range() {
    assert_eq!(BackendCoefficient::to_backend_number(17_i64).unwrap(), 17.0);
    assert_eq!(BackendCoefficient::to_backend_number(0.5_f64).unwrap(), 0.5);
    let value = MAX_EXACT_F64_INTEGER + 1;
    for ilp in [
        ILP::<bool>::new(1, vec![], vec![(0, value)], ObjectiveSense::Maximize).unwrap(),
        ILP::<bool>::new(
            1,
            vec![LinearConstraint::le(vec![(0, value)], 1)],
            vec![],
            ObjectiveSense::Maximize,
        )
        .unwrap(),
        ILP::<bool>::new(
            1,
            vec![LinearConstraint::le(vec![(0, 1)], value)],
            vec![],
            ObjectiveSense::Maximize,
        )
        .unwrap(),
    ] {
        assert!(matches!(
            HighsAdapter::new(None).solve(&ilp),
            Err(ILPSolveError::InexactTransport(_))
        ));
    }
}

#[test]
fn invalid_time_limits_are_errors_instead_of_backend_panics() {
    for time in [-1.0, f64::NAN, f64::INFINITY] {
        assert!(matches!(
            HighsAdapter::new(Some(time)).solve(&ILP::<bool>::empty()),
            Err(ILPSolveError::BackendFailure(_))
        ));
    }
}

#[test]
fn adapter_accepts_an_ilp_domain_without_any_registry_entry() {
    #[derive(Clone, Debug)]
    struct UnregisteredDomain;
    impl VariableDomain for UnregisteredDomain {
        const NAME: &'static str = "UnregisteredDomain";
        fn default_variable() -> IntegerVariable {
            <i64 as VariableDomain>::default_variable()
        }
        fn validate_variables(
            variables: &[IntegerVariable],
        ) -> Result<(), crate::registry::ConstructionError> {
            <i64 as VariableDomain>::validate_variables(variables)
        }
    }
    let ilp = ILP::<UnregisteredDomain>::with_variables(
        vec![IntegerVariable::new(Some(0), Some(2)).unwrap()],
        vec![],
        vec![(0, 1)],
        ObjectiveSense::Maximize,
    )
    .unwrap();
    assert_eq!(HighsAdapter::new(None).solve(&ilp).unwrap(), vec![2]);
}

#[test]
fn backend_model_loading_failure_is_an_explicit_error() {
    let ilp = ILP::<bool, f64>::new(
        1,
        vec![LinearConstraint::le(vec![(0, 1e30)], 1.0)],
        vec![],
        ObjectiveSense::Minimize,
    )
    .unwrap();
    assert!(matches!(
        HighsAdapter::new(None).solve(&ilp),
        Err(ILPSolveError::BackendFailure(message)) if message.contains("loading HiGHS model")
    ));
}
