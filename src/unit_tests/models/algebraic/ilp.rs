use super::*;
use crate::solvers::{ILPSolveError, ILPSolver};
use crate::traits::Problem;
use crate::types::Extremum;

fn binary_ilp(
    num_vars: usize,
    constraints: Vec<LinearConstraint>,
    objective: Vec<(usize, i64)>,
    sense: ObjectiveSense,
) -> ILP<bool> {
    ILP::new(num_vars, constraints, objective, sense).unwrap()
}

#[test]
fn ilp_variant_identifies_variable_domain() {
    assert_eq!(
        <ILP<bool> as Problem>::variant(),
        vec![("variable", "bool"), ("coefficient", "i64")]
    );
}

#[test]
fn ilp_variant_identifies_float_coefficients() {
    assert_eq!(
        <ILP<bool, f64> as Problem>::variant(),
        vec![("variable", "bool"), ("coefficient", "f64")]
    );
}

#[test]
fn integer_objective_keeps_adjacent_large_values_distinct() {
    let smaller = crate::types::MAX_EXACT_F64_INTEGER + 3;
    let larger = smaller + 1;
    let ilp = ILP::<bool>::new(
        2,
        vec![],
        vec![(0, smaller), (1, larger)],
        ObjectiveSense::Maximize,
    )
    .unwrap();

    assert_eq!(ilp.evaluate_objective(&[1, 0]).unwrap(), smaller);
    assert_eq!(ilp.evaluate_objective(&[0, 1]).unwrap(), larger);
}

#[test]
fn float_constraints_use_float_arithmetic() {
    let ilp = ILP::<bool, f64>::new(
        2,
        vec![LinearConstraint::eq(vec![(0, 0.1), (1, 0.2)], 0.3)],
        vec![(0, 0.5)],
        ObjectiveSense::Maximize,
    )
    .unwrap();

    assert!(ilp.is_feasible(&[1, 1]).unwrap());
    assert_eq!(ilp.evaluate_objective(&[1, 0]).unwrap(), 0.5);
}

#[test]
fn float_ilp_rejects_non_finite_coefficients() {
    assert!(matches!(
        ILP::<bool, f64>::new(
            1,
            vec![],
            vec![(0, f64::INFINITY)],
            ObjectiveSense::Minimize,
        ),
        Err(crate::registry::ConstructionError::NonFiniteFloat(_))
    ));
}

#[test]
fn test_linear_constraint_le() {
    let constraint = LinearConstraint::le(vec![(0, 1), (1, 2)], 5);
    assert_eq!(constraint.comparison(), Comparison::Le);
    assert_eq!(constraint.rhs(), 5);
    assert!(constraint.is_satisfied(&[1, 2]).unwrap());
    assert!(!constraint.is_satisfied(&[2, 2]).unwrap());
}

#[test]
fn test_linear_constraint_ge() {
    let constraint = LinearConstraint::ge(vec![(0, 1), (1, 1)], 3);
    assert_eq!(constraint.comparison(), Comparison::Ge);
    assert!(constraint.is_satisfied(&[2, 2]).unwrap());
    assert!(constraint.is_satisfied(&[1, 2]).unwrap());
    assert!(!constraint.is_satisfied(&[1, 1]).unwrap());
}

#[test]
fn test_linear_constraint_eq() {
    let constraint = LinearConstraint::eq(vec![(0, 1), (1, 1)], 2);
    assert_eq!(constraint.comparison(), Comparison::Eq);
    assert!(constraint.is_satisfied(&[1, 1]).unwrap());
    assert!(!constraint.is_satisfied(&[1, 2]).unwrap());
    assert!(!constraint.is_satisfied(&[0, 1]).unwrap());
}

#[test]
fn test_linear_constraint_evaluate_lhs() {
    let constraint = LinearConstraint::le(vec![(0, 3), (2, -1)], 10);
    assert_eq!(constraint.evaluate_lhs(&[2, 5, 7]).unwrap(), -1);
}

#[test]
fn test_linear_constraint_rejects_short_assignment() {
    let constraint = LinearConstraint::le(vec![(1, 1)], 1);
    assert!(matches!(
        constraint.evaluate_lhs(&[0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_linear_constraint_variables() {
    let constraint = LinearConstraint::le(vec![(0, 1), (3, 2), (5, -1)], 10);
    assert_eq!(constraint.variables().collect::<Vec<_>>(), vec![0, 3, 5]);
}

#[test]
fn test_ilp_normalizes_exact_integer_constraint_terms() {
    let ilp = binary_ilp(
        2,
        vec![LinearConstraint::le(vec![(1, 2), (0, 3), (1, -2)], 4)],
        vec![],
        ObjectiveSense::Minimize,
    );
    assert_eq!(ilp.constraints()[0].terms(), &[(0, 3)]);
}

#[test]
fn test_ilp_rejects_constraint_normalization_overflow() {
    let error = ILP::<bool>::new(
        1,
        vec![LinearConstraint::le(vec![(0, i64::MAX), (0, 1)], 0)],
        vec![],
        ObjectiveSense::Minimize,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        crate::registry::ConstructionError::IntegerOverflow(_)
    ));
}

#[test]
fn test_linear_constraint_reports_exact_evaluation_overflow() {
    let constraint = LinearConstraint::le(vec![(0, i64::MAX)], 0);
    assert!(matches!(
        constraint.evaluate_lhs(&[2]),
        Err(crate::traits::EvaluationError::IntegerOverflow(_))
    ));
}

#[test]
fn test_linear_constraint_out_of_bounds() {
    assert!(ILP::<bool>::new(
        3,
        vec![LinearConstraint::le(vec![(5, 1)], 10)],
        vec![],
        ObjectiveSense::Minimize,
    )
    .is_err());
}

#[test]
fn test_ilp_new() {
    let ilp = binary_ilp(
        2,
        vec![LinearConstraint::le(vec![(0, 1), (1, 1)], 1)],
        vec![(0, 1), (1, 2)],
        ObjectiveSense::Maximize,
    );
    assert_eq!(ilp.num_vars(), 2);
    assert_eq!(ilp.constraints().len(), 1);
    assert_eq!(ilp.objective().len(), 2);
    assert_eq!(ilp.sense(), ObjectiveSense::Maximize);
}

#[test]
fn test_ilp_empty() {
    let ilp = ILP::<bool>::empty();
    assert_eq!(ilp.num_vars(), 0);
    assert!(ilp.constraints().is_empty());
    assert!(ilp.objective().is_empty());
}

#[test]
fn test_ilp_evaluate_objective() {
    let ilp = binary_ilp(
        3,
        vec![],
        vec![(0, 2), (1, 3), (2, -1)],
        ObjectiveSense::Maximize,
    );
    assert_eq!(ilp.evaluate_objective(&[1, 1, 0]).unwrap(), 5);
    assert_eq!(ilp.evaluate_objective(&[0, 0, 1]).unwrap(), -1);
}

#[test]
fn test_ilp_objective_rejects_short_assignment() {
    let ilp = binary_ilp(2, vec![], vec![(1, 1)], ObjectiveSense::Maximize);
    assert!(matches!(
        ilp.evaluate_objective(&[0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_ilp_constraints_satisfied() {
    let ilp = binary_ilp(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1), (1, 1)], 1),
            LinearConstraint::ge(vec![(2, 1)], 0),
        ],
        vec![],
        ObjectiveSense::Minimize,
    );
    assert!(ilp.is_feasible(&[0, 0, 1]).unwrap());
    assert!(ilp.is_feasible(&[1, 0, 0]).unwrap());
    assert!(ilp.is_feasible(&[0, 1, 1]).unwrap());
    assert!(!ilp.is_feasible(&[1, 1, 0]).unwrap());
}

#[test]
fn test_ilp_is_feasible() {
    let ilp = binary_ilp(
        2,
        vec![LinearConstraint::le(vec![(0, 1), (1, 1)], 1)],
        vec![],
        ObjectiveSense::Maximize,
    );
    assert!(ilp.is_feasible(&[0, 0]).unwrap());
    assert!(ilp.is_feasible(&[1, 0]).unwrap());
    assert!(ilp.is_feasible(&[0, 1]).unwrap());
    assert!(!ilp.is_feasible(&[1, 1]).unwrap());
}

#[test]
fn test_ilp_num_variables() {
    let ilp = binary_ilp(5, vec![], vec![], ObjectiveSense::Minimize);
    assert_eq!(ilp.num_variables(), 5);
}

#[test]
fn test_ilp_evaluate_valid() {
    let ilp = binary_ilp(
        2,
        vec![LinearConstraint::le(vec![(0, 1), (1, 1)], 1)],
        vec![(0, 1), (1, 2)],
        ObjectiveSense::Maximize,
    );
    assert_eq!(
        ilp.evaluate(&vec![0, 1]).unwrap(),
        Extremum::maximize(Some(2))
    );
    assert_eq!(
        ilp.evaluate(&vec![1, 0]).unwrap(),
        Extremum::maximize(Some(1))
    );
}

#[test]
fn test_ilp_evaluate_infeasible() {
    let ilp = binary_ilp(
        2,
        vec![LinearConstraint::le(vec![(0, 1), (1, 1)], 1)],
        vec![],
        ObjectiveSense::Maximize,
    );
    assert_eq!(ilp.evaluate(&vec![1, 1]).unwrap(), Extremum::maximize(None));
}

#[test]
fn test_ilp_solver_maximization() {
    let ilp = binary_ilp(
        2,
        vec![LinearConstraint::le(vec![(0, 1), (1, 1)], 1)],
        vec![(0, 1), (1, 2)],
        ObjectiveSense::Maximize,
    );
    assert_eq!(ILPSolver::new().solve(&ilp).unwrap(), vec![0, 1]);
}

#[test]
fn test_ilp_solver_minimization() {
    let ilp = binary_ilp(
        2,
        vec![LinearConstraint::ge(vec![(0, 1), (1, 1)], 1)],
        vec![(0, 1), (1, 1)],
        ObjectiveSense::Minimize,
    );
    let solution = ILPSolver::new().solve(&ilp).unwrap();
    assert!(solution == vec![1, 0] || solution == vec![0, 1]);
}

#[test]
fn test_ilp_solver_infeasible() {
    let ilp = binary_ilp(
        1,
        vec![
            LinearConstraint::ge(vec![(0, 1)], 1),
            LinearConstraint::le(vec![(0, 1)], 0),
        ],
        vec![],
        ObjectiveSense::Minimize,
    );
    assert_eq!(ILPSolver::new().solve(&ilp), Err(ILPSolveError::Infeasible));
    assert!(!ilp.is_feasible(&[0]).unwrap());
    assert!(!ilp.is_feasible(&[1]).unwrap());
}

#[test]
fn test_ilp_unconstrained() {
    let ilp = binary_ilp(2, vec![], vec![(0, 1), (1, 1)], ObjectiveSense::Maximize);
    assert_eq!(ILPSolver::new().solve(&ilp).unwrap(), vec![1, 1]);
}

#[test]
fn test_ilp_equality_constraint() {
    let ilp = binary_ilp(
        2,
        vec![LinearConstraint::eq(vec![(0, 1), (1, 1)], 1)],
        vec![(0, 1)],
        ObjectiveSense::Minimize,
    );
    assert_eq!(ILPSolver::new().solve(&ilp).unwrap(), vec![0, 1]);
}

#[test]
fn test_ilp_multiple_constraints() {
    let ilp = binary_ilp(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1), (1, 1)], 1),
            LinearConstraint::le(vec![(1, 1), (2, 1)], 1),
        ],
        vec![(0, 1), (1, 1), (2, 1)],
        ObjectiveSense::Maximize,
    );
    assert_eq!(ILPSolver::new().solve(&ilp).unwrap(), vec![1, 0, 1]);
}

#[test]
fn test_binary_ilp_enforces_variable_domain() {
    let ilp = binary_ilp(3, vec![], vec![], ObjectiveSense::Minimize);
    assert!(ilp.is_feasible(&[0, 0, 0]).unwrap());
    assert!(ilp.is_feasible(&[1, 0, 1]).unwrap());
    assert!(!ilp.is_feasible(&[2, 0, 1]).unwrap());
}

#[test]
fn test_ilp_problem() {
    let ilp = binary_ilp(
        2,
        vec![LinearConstraint::le(vec![(0, 1), (1, 1)], 1)],
        vec![(0, 1), (1, 2)],
        ObjectiveSense::Maximize,
    );
    assert_eq!(
        ilp.evaluate(&vec![0, 0]).unwrap(),
        Extremum::maximize(Some(0))
    );
    assert_eq!(
        ilp.evaluate(&vec![0, 1]).unwrap(),
        Extremum::maximize(Some(2))
    );
    assert_eq!(
        ilp.evaluate(&vec![1, 0]).unwrap(),
        Extremum::maximize(Some(1))
    );
    assert_eq!(ilp.evaluate(&vec![1, 1]).unwrap(), Extremum::maximize(None));
}

#[test]
fn test_ilp_problem_minimize() {
    let ilp = binary_ilp(2, vec![], vec![(0, 1), (1, 1)], ObjectiveSense::Minimize);
    assert_eq!(
        ilp.evaluate(&vec![0, 0]).unwrap(),
        Extremum::minimize(Some(0))
    );
    assert_eq!(
        ilp.evaluate(&vec![1, 1]).unwrap(),
        Extremum::minimize(Some(2))
    );
}

#[test]
fn test_parameter_getters() {
    let ilp = binary_ilp(
        2,
        vec![
            LinearConstraint::le(vec![(0, 1), (1, 1)], 3),
            LinearConstraint::le(vec![(0, 1)], 2),
        ],
        vec![(0, 1), (1, 2)],
        ObjectiveSense::Maximize,
    );
    assert_eq!(ilp.num_vars(), 2);
    assert_eq!(ilp.num_constraints(), 2);
    assert_eq!(ilp.num_nonzeros(), 3);
}

#[test]
fn test_ilp_i64_defaults_to_nonnegative_variables() {
    let ilp = ILP::<i64>::new(3, vec![], vec![], ObjectiveSense::Minimize).unwrap();
    assert_eq!(
        ilp.variables(),
        vec![IntegerVariable::new(Some(0), None).unwrap(); 3],
    );
}

#[test]
fn test_ilp_explicit_finite_one_sided_and_free_bounds() {
    let ilp = ILP::<i64>::with_variables(
        vec![
            IntegerVariable::new(Some(-2), Some(3)).unwrap(),
            IntegerVariable::new(Some(1), None).unwrap(),
            IntegerVariable::new(None, Some(4)).unwrap(),
            IntegerVariable::free(),
        ],
        vec![],
        vec![],
        ObjectiveSense::Minimize,
    )
    .unwrap();

    assert!(ilp.is_feasible(&[-2, 1, 4, -100]).unwrap());
    assert!(!ilp.is_feasible(&[-3, 1, 4, 0]).unwrap());
    assert!(!ilp.is_feasible(&[-2, 0, 4, 0]).unwrap());
    assert!(!ilp.is_feasible(&[-2, 1, 5, 0]).unwrap());
}

#[test]
fn test_integer_variable_deserialization_enforces_bound_order() {
    let invalid = serde_json::json!({
        "lower_bound": 3,
        "upper_bound": 2,
    });

    assert!(serde_json::from_value::<IntegerVariable>(invalid).is_err());
}

#[test]
fn test_ilp_evaluates_constraint_infeasibility_without_a_solver() {
    let ilp = ILP::<i64>::with_variables(
        vec![IntegerVariable::new(Some(0), Some(1)).unwrap()],
        vec![
            LinearConstraint::le(vec![(0, 1)], 0),
            LinearConstraint::ge(vec![(0, 1)], 1),
        ],
        vec![],
        ObjectiveSense::Minimize,
    )
    .unwrap();

    assert_eq!(ilp.evaluate(&vec![0]).unwrap(), Extremum::minimize(None));
    assert_eq!(ilp.evaluate(&vec![1]).unwrap(), Extremum::minimize(None));
}

#[test]
fn test_ilp_paper_example() {
    let ilp = ILP::<i64>::new(
        2,
        vec![
            LinearConstraint::le(vec![(0, 1), (1, 1)], 5),
            LinearConstraint::le(vec![(0, 4), (1, 7)], 28),
        ],
        vec![(0, -5), (1, -6)],
        ObjectiveSense::Minimize,
    )
    .unwrap();
    assert_eq!(
        ilp.evaluate(&vec![3, 2]).unwrap(),
        Extremum::minimize(Some(-27))
    );
    assert!(ilp.is_feasible(&[3, 2]).unwrap());
    assert!(!ilp.is_feasible(&[4, 4]).unwrap());
    assert_eq!(
        ilp.evaluate(&vec![0, 4]).unwrap(),
        Extremum::minimize(Some(-24))
    );
}
