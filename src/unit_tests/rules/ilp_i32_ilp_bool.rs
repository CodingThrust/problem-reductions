use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::traits::Problem;

/// Helper: brute-force solve a small ILP<bool>, extract solution back to ILP<i32>,
/// and return (source_config, source_obj).
fn solve_via_bool(source: &ILP<i32>) -> Option<(Vec<usize>, f64)> {
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(source);
    let target = reduction.target_problem();
    let solver = BruteForce::new();
    let witness = solver.find_witness(target)?;
    let source_config = reduction.extract_solution(&witness);
    let values: Vec<i64> = source_config.iter().map(|&c| c as i64).collect();
    let obj = source.evaluate_objective(&values);
    Some((source_config, obj))
}

#[test]
fn test_ilp_i32_to_ilp_bool_closed_loop() {
    // Minimize -5x0 - 6x1, s.t. x0 + x1 <= 5, 4x0 + 7x1 <= 28
    let source = ILP::<i32>::new(
        2,
        vec![
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 5.0),
            LinearConstraint::le(vec![(0, 4.0), (1, 7.0)], 28.0),
        ],
        vec![(0, -5.0), (1, -6.0)],
        ObjectiveSense::Minimize,
    );

    let (config, obj) = solve_via_bool(&source).expect("should find optimal");
    // Optimal: x0=3, x1=2, obj=-27
    let values: Vec<i64> = config.iter().map(|&c| c as i64).collect();
    assert!(
        source.is_feasible(&values),
        "extracted solution must be feasible"
    );
    assert!(
        (obj - (-27.0)).abs() < 1e-9,
        "optimal objective should be -27, got {obj}"
    );
}

#[test]
fn test_ilp_i32_to_ilp_bool_maximize() {
    // Maximize 3x0 + 5x1, s.t. x0 <= 4, x1 <= 3, x0 + x1 <= 6
    let source = ILP::<i32>::new(
        2,
        vec![
            LinearConstraint::le(vec![(0, 1.0)], 4.0),
            LinearConstraint::le(vec![(1, 1.0)], 3.0),
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 6.0),
        ],
        vec![(0, 3.0), (1, 5.0)],
        ObjectiveSense::Maximize,
    );

    let (config, obj) = solve_via_bool(&source).expect("should find optimal");
    let values: Vec<i64> = config.iter().map(|&c| c as i64).collect();
    assert!(source.is_feasible(&values));
    // Optimal: x0=3, x1=3, obj=24
    assert!(
        (obj - 24.0).abs() < 1e-9,
        "optimal objective should be 24, got {obj}"
    );
}

#[test]
fn test_ilp_i32_to_ilp_bool_empty() {
    let source = ILP::<i32>::empty();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let target = reduction.target_problem();
    assert_eq!(target.num_vars, 0);
    assert!(target.constraints.is_empty());
    assert!(target.objective.is_empty());
}

#[test]
fn test_ilp_i32_to_ilp_bool_target_structure() {
    // x0 + x1 <= 5, with bounds => U=[5, 5], K=[3, 3], total=6 bool vars
    let source = ILP::<i32>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 5.0)],
        vec![(0, 1.0)],
        ObjectiveSense::Maximize,
    );

    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Both variables bounded to 5: K=3 each, total 6
    assert_eq!(target.num_vars, 6);
    // Same number of constraints
    assert_eq!(target.constraints.len(), 1);
    // All dims are 2 (binary)
    assert!(target.dims().iter().all(|&d| d == 2));
}

#[test]
fn test_ilp_i32_to_ilp_bool_single_variable() {
    // Maximize x0, s.t. x0 <= 7
    let source = ILP::<i32>::new(
        1,
        vec![LinearConstraint::le(vec![(0, 1.0)], 7.0)],
        vec![(0, 1.0)],
        ObjectiveSense::Maximize,
    );

    let (config, obj) = solve_via_bool(&source).expect("should find optimal");
    assert_eq!(config, vec![7]);
    assert!((obj - 7.0).abs() < 1e-9);
}

#[test]
fn test_ilp_i32_to_ilp_bool_equality_constraint() {
    // Minimize x0, s.t. x0 + x1 = 4, x0 <= 3, x1 <= 3
    let source = ILP::<i32>::new(
        2,
        vec![
            LinearConstraint::eq(vec![(0, 1.0), (1, 1.0)], 4.0),
            LinearConstraint::le(vec![(0, 1.0)], 3.0),
            LinearConstraint::le(vec![(1, 1.0)], 3.0),
        ],
        vec![(0, 1.0)],
        ObjectiveSense::Minimize,
    );

    let (config, obj) = solve_via_bool(&source).expect("should find optimal");
    let values: Vec<i64> = config.iter().map(|&c| c as i64).collect();
    assert!(source.is_feasible(&values));
    // x0=1, x1=3, obj=1
    assert!((obj - 1.0).abs() < 1e-9, "optimal should be 1, got {obj}");
}

#[test]
fn test_ilp_i32_to_ilp_bool_ge_constraint() {
    // Maximize x0 + x1, s.t. x0 >= 2, x1 >= 1, x0 + x1 <= 5
    let source = ILP::<i32>::new(
        2,
        vec![
            LinearConstraint::ge(vec![(0, 1.0)], 2.0),
            LinearConstraint::ge(vec![(1, 1.0)], 1.0),
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 5.0),
        ],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );

    let (config, obj) = solve_via_bool(&source).expect("should find optimal");
    let values: Vec<i64> = config.iter().map(|&c| c as i64).collect();
    assert!(source.is_feasible(&values));
    assert!((obj - 5.0).abs() < 1e-9, "optimal should be 5, got {obj}");
}

#[test]
fn test_ilp_i32_to_ilp_bool_infeasible() {
    // x0 >= 3 AND x0 <= 1 => infeasible
    let source = ILP::<i32>::new(
        1,
        vec![
            LinearConstraint::ge(vec![(0, 1.0)], 3.0),
            LinearConstraint::le(vec![(0, 1.0)], 1.0),
        ],
        vec![(0, 1.0)],
        ObjectiveSense::Minimize,
    );

    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let target = reduction.target_problem();
    let solver = BruteForce::new();
    // Should have no feasible solution
    assert!(solver.find_witness(target).is_none());
}

#[test]
fn test_ilp_i32_to_ilp_bool_variable_fixed_at_zero() {
    // x0 <= 0 means x0 is always 0 => 0 binary variables for x0
    // Maximize x1, s.t. x0 <= 0, x1 <= 3
    let source = ILP::<i32>::new(
        2,
        vec![
            LinearConstraint::le(vec![(0, 1.0)], 0.0),
            LinearConstraint::le(vec![(1, 1.0)], 3.0),
        ],
        vec![(1, 1.0)],
        ObjectiveSense::Maximize,
    );

    let (config, obj) = solve_via_bool(&source).expect("should find optimal");
    assert_eq!(config[0], 0, "x0 should be fixed at 0");
    assert_eq!(config[1], 3, "x1 should be 3");
    assert!((obj - 3.0).abs() < 1e-9);
}

#[test]
fn test_ilp_i32_to_ilp_bool_power_of_two_bound() {
    // x0 <= 7 (= 2^3 - 1): standard binary, weights = [1, 2, 4]
    let source = ILP::<i32>::new(
        1,
        vec![LinearConstraint::le(vec![(0, 1.0)], 7.0)],
        vec![(0, 1.0)],
        ObjectiveSense::Maximize,
    );

    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let target = reduction.target_problem();
    // 7 = 2^3 - 1, so K=3 bits
    assert_eq!(target.num_vars, 3);
}

#[test]
fn test_ilp_i32_to_ilp_bool_preserves_sense() {
    for sense in [ObjectiveSense::Minimize, ObjectiveSense::Maximize] {
        let source = ILP::<i32>::new(
            1,
            vec![LinearConstraint::le(vec![(0, 1.0)], 3.0)],
            vec![(0, 1.0)],
            sense,
        );
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
        assert_eq!(reduction.target_problem().sense, sense);
    }
}
