use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::ILPSolver;
use crate::traits::Problem;

#[test]
fn test_ilp_bool_to_ilp_i64_closed_loop() {
    // Binary ILP: maximize x0 + 2*x1 + 3*x2, s.t. x0 + x1 + x2 <= 2, x1 + x2 <= 1
    let source = ILP::<bool>::new(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1), (1, 1), (2, 1)], 2),
            LinearConstraint::le(vec![(1, 1), (2, 1)], 1),
        ],
        vec![(0, 1), (1, 2), (2, 3)],
        ObjectiveSense::Maximize,
    )
    .unwrap();

    let source_best = ILPSolver::new().solve(&source).unwrap();
    let source_obj = source.evaluate(&source_best).unwrap();

    let result = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let target = result.target_problem();

    // Target should have same number of variables
    assert_eq!(target.num_vars(), 3);
    assert_eq!(target.constraints(), source.constraints());
    assert_eq!(target.variables(), source.variables());

    // Extract solution back to source and verify optimality
    let target_solution = ILPSolver::new().solve(target).unwrap();
    let source_solution = result.extract_solution(&target_solution).unwrap();
    assert_eq!(source.evaluate(&source_solution).unwrap(), source_obj);
}

#[test]
fn test_ilp_bool_to_ilp_i64_empty() {
    let source = ILP::<bool>::empty();
    let result = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let target = result.target_problem();
    assert_eq!(target.num_vars(), 0);
    assert!(target.constraints().is_empty());
}

#[test]
fn test_ilp_bool_to_ilp_i64_preserves_constraints() {
    // Three constraints on 3 variables
    let source = ILP::<bool>::new(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1), (1, 1)], 1),
            LinearConstraint::ge(vec![(0, 1)], 0),
            LinearConstraint::eq(vec![(2, 1)], 1),
        ],
        vec![(0, 1)],
        ObjectiveSense::Maximize,
    )
    .unwrap();

    let result = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let target = result.target_problem();

    assert_eq!(target.constraints(), source.constraints());
    assert_eq!(target.objective(), source.objective());
    assert_eq!(target.sense(), source.sense());
    assert_eq!(target.variables(), source.variables());
}
