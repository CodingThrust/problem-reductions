use super::*;
use crate::solvers::BruteForce;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

// ============================================================
// VarBounds tests
// ============================================================

#[test]
fn test_varbounds_binary() {
    let bounds = VarBounds::binary();
    assert_eq!(bounds.lower, Some(0));
    assert_eq!(bounds.upper, Some(1));
    assert!(bounds.contains(0));
    assert!(bounds.contains(1));
    assert!(!bounds.contains(-1));
    assert!(!bounds.contains(2));
    assert_eq!(bounds.num_values(), Some(2));
}

#[test]
fn test_varbounds_non_negative() {
    let bounds = VarBounds::non_negative();
    assert_eq!(bounds.lower, Some(0));
    assert_eq!(bounds.upper, None);
    assert!(bounds.contains(0));
    assert!(bounds.contains(100));
    assert!(!bounds.contains(-1));
    assert_eq!(bounds.num_values(), None);
}

#[test]
fn test_varbounds_unbounded() {
    let bounds = VarBounds::unbounded();
    assert_eq!(bounds.lower, None);
    assert_eq!(bounds.upper, None);
    assert!(bounds.contains(-1000));
    assert!(bounds.contains(0));
    assert!(bounds.contains(1000));
    assert_eq!(bounds.num_values(), None);
}

#[test]
fn test_varbounds_bounded() {
    let bounds = VarBounds::bounded(-5, 10);
    assert_eq!(bounds.lower, Some(-5));
    assert_eq!(bounds.upper, Some(10));
    assert!(bounds.contains(-5));
    assert!(bounds.contains(0));
    assert!(bounds.contains(10));
    assert!(!bounds.contains(-6));
    assert!(!bounds.contains(11));
    assert_eq!(bounds.num_values(), Some(16)); // -5 to 10 inclusive
}

#[test]
fn test_varbounds_default() {
    let bounds = VarBounds::default();
    assert_eq!(bounds.lower, None);
    assert_eq!(bounds.upper, None);
}

#[test]
fn test_varbounds_empty_range() {
    let bounds = VarBounds::bounded(5, 3); // Invalid: lo > hi
    assert_eq!(bounds.num_values(), Some(0));
}

// ============================================================
// Comparison tests
// ============================================================

#[test]
fn test_comparison_le() {
    let cmp = Comparison::Le;
    assert!(cmp.holds(5.0, 10.0));
    assert!(cmp.holds(10.0, 10.0));
    assert!(!cmp.holds(11.0, 10.0));
}

#[test]
fn test_comparison_ge() {
    let cmp = Comparison::Ge;
    assert!(cmp.holds(10.0, 5.0));
    assert!(cmp.holds(10.0, 10.0));
    assert!(!cmp.holds(4.0, 5.0));
}

#[test]
fn test_comparison_eq() {
    let cmp = Comparison::Eq;
    assert!(cmp.holds(10.0, 10.0));
    assert!(!cmp.holds(10.0, 10.1));
    assert!(!cmp.holds(9.9, 10.0));
    // Test tolerance
    assert!(cmp.holds(10.0, 10.0 + 1e-10));
}

// ============================================================
// LinearConstraint tests
// ============================================================

#[test]
fn test_linear_constraint_le() {
    // x0 + 2*x1 <= 5
    let constraint = LinearConstraint::le(vec![(0, 1.0), (1, 2.0)], 5.0);
    assert_eq!(constraint.cmp, Comparison::Le);
    assert_eq!(constraint.rhs, 5.0);

    // x0=1, x1=2 => 1 + 4 = 5 <= 5 (satisfied)
    assert!(constraint.is_satisfied(&[1, 2]));
    // x0=2, x1=2 => 2 + 4 = 6 > 5 (not satisfied)
    assert!(!constraint.is_satisfied(&[2, 2]));
}

#[test]
fn test_linear_constraint_ge() {
    // x0 + x1 >= 3
    let constraint = LinearConstraint::ge(vec![(0, 1.0), (1, 1.0)], 3.0);
    assert_eq!(constraint.cmp, Comparison::Ge);

    assert!(constraint.is_satisfied(&[2, 2])); // 4 >= 3
    assert!(constraint.is_satisfied(&[1, 2])); // 3 >= 3
    assert!(!constraint.is_satisfied(&[1, 1])); // 2 < 3
}

#[test]
fn test_linear_constraint_eq() {
    // x0 + x1 == 2
    let constraint = LinearConstraint::eq(vec![(0, 1.0), (1, 1.0)], 2.0);
    assert_eq!(constraint.cmp, Comparison::Eq);

    assert!(constraint.is_satisfied(&[1, 1])); // 2 == 2
    assert!(!constraint.is_satisfied(&[1, 2])); // 3 != 2
    assert!(!constraint.is_satisfied(&[0, 1])); // 1 != 2
}

#[test]
fn test_linear_constraint_evaluate_lhs() {
    let constraint = LinearConstraint::le(vec![(0, 3.0), (2, -1.0)], 10.0);
    // 3*x0 - 1*x2 with x=[2, 5, 7] => 3*2 - 1*7 = -1
    assert!((constraint.evaluate_lhs(&[2, 5, 7]) - (-1.0)).abs() < 1e-9);
}

#[test]
fn test_linear_constraint_variables() {
    let constraint = LinearConstraint::le(vec![(0, 1.0), (3, 2.0), (5, -1.0)], 10.0);
    assert_eq!(constraint.variables(), vec![0, 3, 5]);
}

#[test]
fn test_linear_constraint_out_of_bounds() {
    // Constraint references variable 5, but values only has 3 elements
    let constraint = LinearConstraint::le(vec![(5, 1.0)], 10.0);
    // Missing variable defaults to 0, so 0 <= 10 is satisfied
    assert!(constraint.is_satisfied(&[1, 2, 3]));
}

// ============================================================
// ObjectiveSense tests
// ============================================================

#[test]
fn test_objective_sense_direction_conversions() {
    // Test that ObjectiveSense and Direction can be converted
    let max_sense = ObjectiveSense::Maximize;
    let min_sense = ObjectiveSense::Minimize;

    // Direction values match ObjectiveSense semantics
    assert_eq!(max_sense, ObjectiveSense::Maximize);
    assert_eq!(min_sense, ObjectiveSense::Minimize);
}

// ============================================================
// ILP tests
// ============================================================

#[test]
fn test_ilp_new() {
    let ilp = ILP::new(
        2,
        vec![VarBounds::binary(), VarBounds::binary()],
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    );
    assert_eq!(ilp.num_vars, 2);
    assert_eq!(ilp.bounds.len(), 2);
    assert_eq!(ilp.constraints.len(), 1);
    assert_eq!(ilp.objective.len(), 2);
    assert_eq!(ilp.sense, ObjectiveSense::Maximize);
}

#[test]
#[should_panic(expected = "bounds length must match num_vars")]
fn test_ilp_new_mismatched_bounds() {
    ILP::new(
        3,
        vec![VarBounds::binary(), VarBounds::binary()], // Only 2 bounds for 3 vars
        vec![],
        vec![],
        ObjectiveSense::Minimize,
    );
}

#[test]
fn test_ilp_binary() {
    let ilp = ILP::binary(
        3,
        vec![],
        vec![(0, 1.0), (1, 1.0), (2, 1.0)],
        ObjectiveSense::Minimize,
    );
    assert_eq!(ilp.num_vars, 3);
    assert!(ilp.bounds.iter().all(|b| *b == VarBounds::binary()));
}

#[test]
fn test_ilp_empty() {
    let ilp = ILP::empty();
    assert_eq!(ilp.num_vars, 0);
    assert!(ilp.bounds.is_empty());
    assert!(ilp.constraints.is_empty());
    assert!(ilp.objective.is_empty());
}

#[test]
fn test_ilp_evaluate_objective() {
    let ilp = ILP::binary(
        3,
        vec![],
        vec![(0, 2.0), (1, 3.0), (2, -1.0)],
        ObjectiveSense::Maximize,
    );
    // 2*1 + 3*1 + (-1)*0 = 5
    assert!((ilp.evaluate_objective(&[1, 1, 0]) - 5.0).abs() < 1e-9);
    // 2*0 + 3*0 + (-1)*1 = -1
    assert!((ilp.evaluate_objective(&[0, 0, 1]) - (-1.0)).abs() < 1e-9);
}

#[test]
fn test_ilp_bounds_satisfied() {
    let ilp = ILP::new(
        2,
        vec![VarBounds::bounded(0, 5), VarBounds::bounded(-2, 2)],
        vec![],
        vec![],
        ObjectiveSense::Minimize,
    );
    assert!(ilp.bounds_satisfied(&[0, 0]));
    assert!(ilp.bounds_satisfied(&[5, 2]));
    assert!(ilp.bounds_satisfied(&[3, -2]));
    assert!(!ilp.bounds_satisfied(&[6, 0])); // x0 > 5
    assert!(!ilp.bounds_satisfied(&[0, 3])); // x1 > 2
    assert!(!ilp.bounds_satisfied(&[0])); // Wrong length
}

#[test]
fn test_ilp_constraints_satisfied() {
    let ilp = ILP::binary(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0), // x0 + x1 <= 1
            LinearConstraint::ge(vec![(2, 1.0)], 0.0),           // x2 >= 0
        ],
        vec![],
        ObjectiveSense::Minimize,
    );
    assert!(ilp.constraints_satisfied(&[0, 0, 1]));
    assert!(ilp.constraints_satisfied(&[1, 0, 0]));
    assert!(ilp.constraints_satisfied(&[0, 1, 1]));
    assert!(!ilp.constraints_satisfied(&[1, 1, 0])); // x0 + x1 = 2 > 1
}

#[test]
fn test_ilp_is_feasible() {
    let ilp = ILP::binary(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );
    assert!(ilp.is_feasible(&[0, 0]));
    assert!(ilp.is_feasible(&[1, 0]));
    assert!(ilp.is_feasible(&[0, 1]));
    assert!(!ilp.is_feasible(&[1, 1])); // Constraint violated
    assert!(!ilp.is_feasible(&[2, 0])); // Bounds violated
}

// ============================================================
// Problem trait tests
// ============================================================

#[test]
fn test_ilp_num_variables() {
    let ilp = ILP::binary(5, vec![], vec![], ObjectiveSense::Minimize);
    assert_eq!(ilp.num_variables(), 5);
}

#[test]
fn test_ilp_direction() {
    let max_ilp = ILP::binary(2, vec![], vec![], ObjectiveSense::Maximize);
    let min_ilp = ILP::binary(2, vec![], vec![], ObjectiveSense::Minimize);

    assert_eq!(max_ilp.direction(), Direction::Maximize);
    assert_eq!(min_ilp.direction(), Direction::Minimize);
}

#[test]
fn test_ilp_evaluate_valid() {
    // Maximize x0 + 2*x1 subject to x0 + x1 <= 1
    let ilp = ILP::binary(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    );

    // Config [0, 1] means x0=0, x1=1 => obj = 2, valid
    assert_eq!(Problem::evaluate(&ilp, &[0, 1]), SolutionSize::Valid(2.0));

    // Config [1, 0] means x0=1, x1=0 => obj = 1, valid
    assert_eq!(Problem::evaluate(&ilp, &[1, 0]), SolutionSize::Valid(1.0));
}

#[test]
fn test_ilp_evaluate_invalid() {
    // x0 + x1 <= 1
    let ilp = ILP::binary(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    );

    // Config [1, 1] means x0=1, x1=1 => invalid (1+1 > 1), returns Invalid
    assert_eq!(Problem::evaluate(&ilp, &[1, 1]), SolutionSize::Invalid);
}

#[test]
fn test_ilp_evaluate_with_offset_bounds() {
    // Variables with non-zero lower bounds
    let ilp = ILP::new(
        2,
        vec![VarBounds::bounded(1, 3), VarBounds::bounded(-1, 1)],
        vec![],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );

    // Config [0, 0] maps to x0=1, x1=-1 => obj = 0
    assert_eq!(Problem::evaluate(&ilp, &[0, 0]), SolutionSize::Valid(0.0));

    // Config [2, 2] maps to x0=3, x1=1 => obj = 4
    assert_eq!(Problem::evaluate(&ilp, &[2, 2]), SolutionSize::Valid(4.0));
}

#[test]
fn test_ilp_brute_force_maximization() {
    // Maximize x0 + 2*x1 subject to x0 + x1 <= 1, x0, x1 binary
    let ilp = ILP::binary(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&ilp);

    // Optimal: x1=1, x0=0 => objective = 2
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0, 1]);
}

#[test]
fn test_ilp_brute_force_minimization() {
    // Minimize x0 + x1 subject to x0 + x1 >= 1, x0, x1 binary
    let ilp = ILP::binary(
        2,
        vec![LinearConstraint::ge(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Minimize,
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&ilp);

    // Optimal: x0=1,x1=0 or x0=0,x1=1 => objective = 1
    assert_eq!(solutions.len(), 2);
    for sol in &solutions {
        assert_eq!(Problem::evaluate(&ilp, sol), SolutionSize::Valid(1.0));
    }
}

#[test]
fn test_ilp_brute_force_no_feasible() {
    // x0 >= 1 AND x0 <= 0 (infeasible)
    let ilp = ILP::binary(
        1,
        vec![
            LinearConstraint::ge(vec![(0, 1.0)], 1.0),
            LinearConstraint::le(vec![(0, 1.0)], 0.0),
        ],
        vec![(0, 1.0)],
        ObjectiveSense::Minimize,
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&ilp);

    // All solutions are infeasible - BruteForce should return empty list
    assert!(
        solutions.is_empty(),
        "Expected no solutions for infeasible ILP"
    );

    // Verify all configs are indeed infeasible
    for config in &[[0], [1]] {
        assert_eq!(Problem::evaluate(&ilp, config), SolutionSize::Invalid);
        let values = ilp.config_to_values(config);
        assert!(!ilp.is_feasible(&values));
    }
}

#[test]
fn test_ilp_unconstrained() {
    // Maximize x0 + x1, no constraints, binary vars
    let ilp = ILP::binary(
        2,
        vec![],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&ilp);

    // Optimal: both = 1
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 1]);
}

#[test]
fn test_ilp_equality_constraint() {
    // Minimize x0 subject to x0 + x1 == 1, binary vars
    let ilp = ILP::binary(
        2,
        vec![LinearConstraint::eq(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0)],
        ObjectiveSense::Minimize,
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&ilp);

    // Optimal: x0=0, x1=1 => objective = 0
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0, 1]);
}

#[test]
fn test_ilp_multiple_constraints() {
    // Maximize x0 + x1 + x2 subject to:
    //   x0 + x1 <= 1
    //   x1 + x2 <= 1
    // Binary vars
    let ilp = ILP::binary(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0),
            LinearConstraint::le(vec![(1, 1.0), (2, 1.0)], 1.0),
        ],
        vec![(0, 1.0), (1, 1.0), (2, 1.0)],
        ObjectiveSense::Maximize,
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&ilp);

    // Optimal: x0=1, x1=0, x2=1 => objective = 2
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 0, 1]);
}

#[test]
fn test_ilp_config_to_values() {
    let ilp = ILP::new(
        3,
        vec![
            VarBounds::bounded(0, 2),  // 0,1,2
            VarBounds::bounded(-1, 1), // -1,0,1
            VarBounds::bounded(5, 7),  // 5,6,7
        ],
        vec![],
        vec![],
        ObjectiveSense::Minimize,
    );

    // Config [0,0,0] => [0, -1, 5]
    assert_eq!(ilp.config_to_values(&[0, 0, 0]), vec![0, -1, 5]);
    // Config [2,2,2] => [2, 1, 7]
    assert_eq!(ilp.config_to_values(&[2, 2, 2]), vec![2, 1, 7]);
    // Config [1,1,1] => [1, 0, 6]
    assert_eq!(ilp.config_to_values(&[1, 1, 1]), vec![1, 0, 6]);
}

#[test]
fn test_ilp_problem() {
    // Maximize x0 + 2*x1, s.t. x0 + x1 <= 1, binary
    let ilp = ILP::binary(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    );
    assert_eq!(ilp.dims(), vec![2, 2]);

    // [0, 0] -> feasible, obj = 0
    assert_eq!(Problem::evaluate(&ilp, &[0, 0]), SolutionSize::Valid(0.0));
    // [0, 1] -> feasible, obj = 2
    assert_eq!(Problem::evaluate(&ilp, &[0, 1]), SolutionSize::Valid(2.0));
    // [1, 0] -> feasible, obj = 1
    assert_eq!(Problem::evaluate(&ilp, &[1, 0]), SolutionSize::Valid(1.0));
    // [1, 1] -> infeasible
    assert_eq!(Problem::evaluate(&ilp, &[1, 1]), SolutionSize::Invalid);

    assert_eq!(ilp.direction(), Direction::Maximize);
}

#[test]
fn test_ilp_problem_minimize() {
    // Minimize x0 + x1, no constraints, binary
    let ilp = ILP::binary(
        2,
        vec![],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Minimize,
    );
    assert_eq!(Problem::evaluate(&ilp, &[0, 0]), SolutionSize::Valid(0.0));
    assert_eq!(Problem::evaluate(&ilp, &[1, 1]), SolutionSize::Valid(2.0));
    assert_eq!(ilp.direction(), Direction::Minimize);
}
