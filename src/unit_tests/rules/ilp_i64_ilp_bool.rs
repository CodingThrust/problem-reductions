use crate::models::algebraic::{IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::ILPSolver;

fn integer_ilp(
    bounds: &[(i64, i64)],
    constraints: Vec<LinearConstraint>,
    objective: Vec<(usize, i64)>,
    sense: ObjectiveSense,
) -> ILP<i64> {
    ILP::with_variables(
        bounds
            .iter()
            .map(|&(lower, upper)| IntegerVariable::new(Some(lower), Some(upper)).unwrap())
            .collect(),
        constraints,
        objective,
        sense,
    )
    .unwrap()
}

fn solve_via_bool(source: &ILP<i64>) -> Option<(Vec<i64>, i64)> {
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(source).expect("reduction should succeed");
    let witness = ILPSolver::new().solve(reduction.target_problem()).ok()?;
    let source_solution = reduction.extract_solution(&witness).unwrap();
    let objective = source.evaluate_objective(&source_solution).unwrap();
    Some((source_solution, objective))
}

#[test]
fn test_ilp_i64_to_ilp_bool_closed_loop() {
    let source = integer_ilp(
        &[(0, 5), (0, 5)],
        vec![
            LinearConstraint::le(vec![(0, 1), (1, 1)], 5),
            LinearConstraint::le(vec![(0, 4), (1, 7)], 28),
        ],
        vec![(0, -5), (1, -6)],
        ObjectiveSense::Minimize,
    );
    let (solution, objective) = solve_via_bool(&source).unwrap();
    assert!(source.is_feasible(&solution).unwrap());
    assert_eq!(objective, -27);
}

#[test]
fn test_ilp_i64_to_ilp_bool_maximize() {
    let source = integer_ilp(
        &[(0, 4), (0, 3)],
        vec![LinearConstraint::le(vec![(0, 1), (1, 1)], 6)],
        vec![(0, 3), (1, 5)],
        ObjectiveSense::Maximize,
    );
    let (solution, objective) = solve_via_bool(&source).unwrap();
    assert!(source.is_feasible(&solution).unwrap());
    assert_eq!(objective, 24);
}

#[test]
fn test_ilp_i64_to_ilp_bool_empty() {
    let source = ILP::<i64>::empty();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    assert_eq!(reduction.target_problem().num_vars(), 0);
    assert!(reduction.target_problem().constraints().is_empty());
    assert!(reduction.target_problem().objective().is_empty());
}

#[test]
fn test_ilp_i64_to_ilp_bool_target_structure() {
    let source = integer_ilp(
        &[(0, 5), (0, 5)],
        vec![LinearConstraint::le(vec![(0, 1), (1, 1)], 5)],
        vec![(0, 1)],
        ObjectiveSense::Maximize,
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    assert_eq!(reduction.target_problem().num_vars(), 6);
    assert_eq!(reduction.target_problem().constraints().len(), 1);
    assert!(reduction
        .target_problem()
        .variables()
        .iter()
        .all(|variable| variable.lower_bound() == Some(0) && variable.upper_bound() == Some(1)));
}

#[test]
fn test_ilp_i64_to_ilp_bool_single_variable() {
    let source = integer_ilp(&[(0, 7)], vec![], vec![(0, 1)], ObjectiveSense::Maximize);
    assert_eq!(solve_via_bool(&source).unwrap(), (vec![7], 7));
}

#[test]
fn test_ilp_i64_to_ilp_bool_equality_constraint() {
    let source = integer_ilp(
        &[(0, 3), (0, 3)],
        vec![LinearConstraint::eq(vec![(0, 1), (1, 1)], 4)],
        vec![(0, 1)],
        ObjectiveSense::Minimize,
    );
    let (solution, objective) = solve_via_bool(&source).unwrap();
    assert!(source.is_feasible(&solution).unwrap());
    assert_eq!(objective, 1);
}

#[test]
fn test_ilp_i64_to_ilp_bool_ge_constraint() {
    let source = integer_ilp(
        &[(0, 5), (0, 5)],
        vec![
            LinearConstraint::ge(vec![(0, 1)], 2),
            LinearConstraint::ge(vec![(1, 1)], 1),
            LinearConstraint::le(vec![(0, 1), (1, 1)], 5),
        ],
        vec![(0, 1), (1, 1)],
        ObjectiveSense::Maximize,
    );
    assert_eq!(solve_via_bool(&source).unwrap().1, 5);
}

#[test]
fn test_ilp_i64_to_ilp_bool_infeasible() {
    let source = integer_ilp(
        &[(0, 3)],
        vec![
            LinearConstraint::ge(vec![(0, 1)], 3),
            LinearConstraint::le(vec![(0, 1)], 1),
        ],
        vec![],
        ObjectiveSense::Minimize,
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    assert!(ILPSolver::new().solve(reduction.target_problem()).is_err());
}

#[test]
fn test_ilp_i64_to_ilp_bool_variable_fixed_at_zero() {
    let source = integer_ilp(
        &[(0, 0), (0, 3)],
        vec![],
        vec![(1, 1)],
        ObjectiveSense::Maximize,
    );
    assert_eq!(solve_via_bool(&source).unwrap(), (vec![0, 3], 3));
}

#[test]
fn test_ilp_i64_to_ilp_bool_power_of_two_bound() {
    let source = integer_ilp(&[(0, 7)], vec![], vec![(0, 1)], ObjectiveSense::Maximize);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    assert_eq!(reduction.target_problem().num_vars(), 3);
}

#[test]
fn test_ilp_i64_to_ilp_bool_preserves_sense() {
    for sense in [ObjectiveSense::Minimize, ObjectiveSense::Maximize] {
        let source = integer_ilp(&[(0, 3)], vec![], vec![(0, 1)], sense);
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        assert_eq!(reduction.target_problem().sense(), sense);
    }
}
