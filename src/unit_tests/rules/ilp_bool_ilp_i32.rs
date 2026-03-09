use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;

#[test]
fn test_ilp_bool_to_ilp_i32_closed_loop() {
    // Binary ILP: maximize x0 + 2*x1, s.t. x0 + x1 <= 1
    let source = ILP::<bool>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    );

    let result = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let target = result.target_problem();

    // Target should have same number of variables
    assert_eq!(target.num_vars, 2);
    // Target should have original constraint + 2 binary bound constraints
    assert_eq!(target.constraints.len(), 3);
    // Dims should be i32::MAX per variable
    assert_eq!(target.dims(), vec![i32::MAX as usize; 2]);

    // Solution [0, 1] should be optimal (x1=1 gives objective 2)
    let target_solution = vec![0, 1];
    let source_solution = result.extract_solution(&target_solution);
    assert_eq!(source_solution, vec![0, 1]);

    // Verify the source evaluates this correctly
    use crate::types::SolutionSize;
    assert_eq!(source.evaluate(&source_solution), SolutionSize::Valid(2.0));
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
    // Two constraints: x0 + x1 <= 1, x0 >= 0
    let source = ILP::<bool>::new(
        2,
        vec![
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0),
            LinearConstraint::ge(vec![(0, 1.0)], 0.0),
        ],
        vec![(0, 1.0)],
        ObjectiveSense::Maximize,
    );

    let result = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let target = result.target_problem();

    // Original 2 constraints + 2 binary bound constraints (x0 <= 1, x1 <= 1)
    assert_eq!(target.constraints.len(), 4);
}
