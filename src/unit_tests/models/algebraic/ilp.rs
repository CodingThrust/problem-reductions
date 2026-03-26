use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Extremum;

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

// ============================================================
// ILP tests
// ============================================================

#[test]
fn test_ilp_new() {
    let ilp = ILP::<bool>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    );
    assert_eq!(ilp.num_vars, 2);
    assert_eq!(ilp.constraints.len(), 1);
    assert_eq!(ilp.objective.len(), 2);
    assert_eq!(ilp.sense, ObjectiveSense::Maximize);
}

#[test]
fn test_ilp_empty() {
    let ilp = ILP::<bool>::empty();
    assert_eq!(ilp.num_vars, 0);
    assert!(ilp.constraints.is_empty());
    assert!(ilp.objective.is_empty());
}

#[test]
fn test_ilp_evaluate_objective() {
    let ilp = ILP::<bool>::new(
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
fn test_ilp_constraints_satisfied() {
    let ilp = ILP::<bool>::new(
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
    let ilp = ILP::<bool>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );
    assert!(ilp.is_feasible(&[0, 0]));
    assert!(ilp.is_feasible(&[1, 0]));
    assert!(ilp.is_feasible(&[0, 1]));
    assert!(!ilp.is_feasible(&[1, 1])); // Constraint violated
}

// ============================================================
// Problem trait tests
// ============================================================

#[test]
fn test_ilp_num_variables() {
    let ilp = ILP::<bool>::new(5, vec![], vec![], ObjectiveSense::Minimize);
    assert_eq!(ilp.num_variables(), 5);
}

#[test]
fn test_ilp_evaluate_valid() {
    // Maximize x0 + 2*x1 subject to x0 + x1 <= 1
    let ilp = ILP::<bool>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    );

    // Config [0, 1] means x0=0, x1=1 => obj = 2, valid
    assert_eq!(
        Problem::evaluate(&ilp, &[0, 1]),
        Extremum::maximize(Some(2.0))
    );

    // Config [1, 0] means x0=1, x1=0 => obj = 1, valid
    assert_eq!(
        Problem::evaluate(&ilp, &[1, 0]),
        Extremum::maximize(Some(1.0))
    );
}

#[test]
fn test_ilp_evaluate_invalid() {
    // x0 + x1 <= 1
    let ilp = ILP::<bool>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    );

    // Config [1, 1] means x0=1, x1=1 => invalid (1+1 > 1), returns Invalid
    assert_eq!(Problem::evaluate(&ilp, &[1, 1]), Extremum::maximize(None));
}

#[test]
fn test_ilp_brute_force_maximization() {
    // Maximize x0 + 2*x1 subject to x0 + x1 <= 1, x0, x1 binary
    let ilp = ILP::<bool>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&ilp);

    // Optimal: x1=1, x0=0 => objective = 2
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0, 1]);
}

#[test]
fn test_ilp_brute_force_minimization() {
    // Minimize x0 + x1 subject to x0 + x1 >= 1, x0, x1 binary
    let ilp = ILP::<bool>::new(
        2,
        vec![LinearConstraint::ge(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Minimize,
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&ilp);

    // Optimal: x0=1,x1=0 or x0=0,x1=1 => objective = 1
    assert_eq!(solutions.len(), 2);
    for sol in &solutions {
        assert_eq!(Problem::evaluate(&ilp, sol), Extremum::minimize(Some(1.0)));
    }
}

#[test]
fn test_ilp_brute_force_no_feasible() {
    // x0 >= 1 AND x0 <= 0 (infeasible)
    let ilp = ILP::<bool>::new(
        1,
        vec![
            LinearConstraint::ge(vec![(0, 1.0)], 1.0),
            LinearConstraint::le(vec![(0, 1.0)], 0.0),
        ],
        vec![(0, 1.0)],
        ObjectiveSense::Minimize,
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&ilp);

    // All solutions are infeasible - BruteForce should return empty list
    assert!(
        solutions.is_empty(),
        "Expected no solutions for infeasible ILP"
    );

    // Verify all configs are indeed infeasible
    for config in &[[0], [1]] {
        assert_eq!(Problem::evaluate(&ilp, config), Extremum::minimize(None));
        let values = ilp.config_to_values(config);
        assert!(!ilp.is_feasible(&values));
    }
}

#[test]
fn test_ilp_unconstrained() {
    // Maximize x0 + x1, no constraints, binary vars
    let ilp = ILP::<bool>::new(
        2,
        vec![],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Maximize,
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&ilp);

    // Optimal: both = 1
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 1]);
}

#[test]
fn test_ilp_equality_constraint() {
    // Minimize x0 subject to x0 + x1 == 1, binary vars
    let ilp = ILP::<bool>::new(
        2,
        vec![LinearConstraint::eq(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0)],
        ObjectiveSense::Minimize,
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&ilp);

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
    let ilp = ILP::<bool>::new(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0),
            LinearConstraint::le(vec![(1, 1.0), (2, 1.0)], 1.0),
        ],
        vec![(0, 1.0), (1, 1.0), (2, 1.0)],
        ObjectiveSense::Maximize,
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&ilp);

    // Optimal: x0=1, x1=0, x2=1 => objective = 2
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 0, 1]);
}

#[test]
fn test_ilp_config_to_values() {
    let ilp = ILP::<bool>::new(3, vec![], vec![], ObjectiveSense::Minimize);

    // For binary ILP, config maps directly: config[i] -> value[i] as i64
    assert_eq!(ilp.config_to_values(&[0, 0, 0]), vec![0, 0, 0]);
    assert_eq!(ilp.config_to_values(&[1, 1, 1]), vec![1, 1, 1]);
    assert_eq!(ilp.config_to_values(&[1, 0, 1]), vec![1, 0, 1]);
}

#[test]
fn test_ilp_problem() {
    // Maximize x0 + 2*x1, s.t. x0 + x1 <= 1, binary
    let ilp = ILP::<bool>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    );
    assert_eq!(ilp.dims(), vec![2, 2]);

    // [0, 0] -> feasible, obj = 0
    assert_eq!(
        Problem::evaluate(&ilp, &[0, 0]),
        Extremum::maximize(Some(0.0))
    );
    // [0, 1] -> feasible, obj = 2
    assert_eq!(
        Problem::evaluate(&ilp, &[0, 1]),
        Extremum::maximize(Some(2.0))
    );
    // [1, 0] -> feasible, obj = 1
    assert_eq!(
        Problem::evaluate(&ilp, &[1, 0]),
        Extremum::maximize(Some(1.0))
    );
    // [1, 1] -> infeasible
    assert_eq!(Problem::evaluate(&ilp, &[1, 1]), Extremum::maximize(None));
}

#[test]
fn test_ilp_problem_minimize() {
    // Minimize x0 + x1, no constraints, binary
    let ilp = ILP::<bool>::new(
        2,
        vec![],
        vec![(0, 1.0), (1, 1.0)],
        ObjectiveSense::Minimize,
    );
    assert_eq!(
        Problem::evaluate(&ilp, &[0, 0]),
        Extremum::minimize(Some(0.0))
    );
    assert_eq!(
        Problem::evaluate(&ilp, &[1, 1]),
        Extremum::minimize(Some(2.0))
    );
}

#[test]
fn test_size_getters() {
    let ilp = ILP::<bool>::new(
        2,
        vec![
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 3.0),
            LinearConstraint::le(vec![(0, 1.0)], 2.0),
        ],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    );
    assert_eq!(ilp.num_vars(), 2);
    assert_eq!(ilp.num_variables(), 2);
    assert_eq!(ilp.num_constraints(), 2);
}

#[test]
fn test_ilp_i32_dims() {
    let ilp = ILP::<i32>::new(3, vec![], vec![], ObjectiveSense::Minimize);
    assert_eq!(ilp.dims(), vec![(i32::MAX as usize) + 1; 3]);
}

#[test]
fn test_ilp_paper_example() {
    // Paper: minimize -5x₁ - 6x₂
    // s.t. x₁ + x₂ ≤ 5, 4x₁ + 7x₂ ≤ 28, x₁, x₂ ≥ 0, x ∈ Z²
    // Optimal: x* = (3, 2), objective = -27
    let ilp = ILP::<i32>::new(
        2,
        vec![
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 5.0),
            LinearConstraint::le(vec![(0, 4.0), (1, 7.0)], 28.0),
        ],
        vec![(0, -5.0), (1, -6.0)],
        ObjectiveSense::Minimize,
    );

    // Verify optimal solution x* = (3, 2) → config [3, 2]
    let result = Problem::evaluate(&ilp, &[3, 2]);
    assert_eq!(result, Extremum::minimize(Some(-27.0)));

    // Verify feasibility: 3+2=5≤5, 4*3+7*2=26≤28
    assert!(ilp.is_feasible(&[3, 2]));

    // Verify infeasible point: 4+4=8>5
    assert!(!ilp.is_feasible(&[4, 4]));

    // Verify suboptimal feasible point: -5*0 - 6*4 = -24 > -27
    let result2 = Problem::evaluate(&ilp, &[0, 4]);
    assert_eq!(result2, Extremum::minimize(Some(-24.0)));
}
