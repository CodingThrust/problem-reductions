use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_ilp_bool_to_ilp_i32_closed_loop() {
    // Binary ILP: maximize x0 + 2*x1 + 3*x2, s.t. x0 + x1 + x2 <= 2, x1 + x2 <= 1
    let source = ILP::<bool>::new(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0), (2, 1.0)], 2.0),
            LinearConstraint::le(vec![(1, 1.0), (2, 1.0)], 1.0),
        ],
        vec![(0, 1.0), (1, 2.0), (2, 3.0)],
        ObjectiveSense::Maximize,
    );

    // Find optimal on source via brute force
    let solver = BruteForce::new();
    let source_best = solver
        .find_best(&source)
        .expect("source should have optimal");
    let source_obj = source.evaluate(&source_best);

    let result = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let target = result.target_problem();

    // Target should have same number of variables
    assert_eq!(target.num_vars, 3);
    // Target should have original 2 constraints + 3 binary bound constraints
    assert_eq!(target.constraints.len(), 5);
    // Dims should be (i32::MAX + 1) per variable
    assert_eq!(target.dims(), vec![(i32::MAX as usize) + 1; 3]);

    // Extract solution back to source and verify optimality
    let source_solution = result.extract_solution(&source_best);
    assert_eq!(source.evaluate(&source_solution), source_obj);
}

#[test]
fn test_ilp_bool_to_ilp_i32_empty() {
    let source = ILP::<bool>::empty();
    let result = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let target = result.target_problem();
    assert_eq!(target.num_vars, 0);
    assert!(target.constraints.is_empty());
}

#[test]
fn test_ilp_bool_to_ilp_i32_preserves_constraints() {
    // Three constraints on 3 variables
    let source = ILP::<bool>::new(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0),
            LinearConstraint::ge(vec![(0, 1.0)], 0.0),
            LinearConstraint::eq(vec![(2, 1.0)], 1.0),
        ],
        vec![(0, 1.0)],
        ObjectiveSense::Maximize,
    );

    let result = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let target = result.target_problem();

    // Original 3 constraints + 3 binary bound constraints (x_i <= 1)
    assert_eq!(target.constraints.len(), 6);

    // Verify bound constraints are the last 3
    for i in 0..3 {
        let c = &target.constraints[3 + i];
        assert_eq!(c.terms, vec![(i, 1.0)]);
        assert_eq!(c.cmp, crate::models::algebraic::Comparison::Le);
        assert_eq!(c.rhs, 1.0);
    }
}
