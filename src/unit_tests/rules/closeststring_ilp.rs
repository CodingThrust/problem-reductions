use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::misc::ClosestString;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::traits::Problem;
use crate::types::Min;

/// Canonical issue #1032 instance: binary alphabet, four length-3 strings
/// 000, 011, 101, 110. Every binary length-3 center attains radius exactly 2;
/// no center attains radius 1.
fn issue_instance() -> ClosestString {
    ClosestString::new(
        2,
        vec![vec![0, 0, 0], vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]],
    )
}

#[test]
fn test_closeststring_to_ilp_structure() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // q = 2, m = 3 -> 2*3 + 1 = 7 variables.
    assert_eq!(ilp.num_vars, 7);
    // m = 3 assignment constraints + n = 4 radius constraints = 7.
    assert_eq!(ilp.constraints.len(), 7);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);

    // The objective puts weight 1 on the radius variable only.
    assert_eq!(ilp.objective.len(), 1);
    let (r_idx, r_coeff) = ilp.objective[0];
    assert_eq!(r_idx, 2 * 3);
    assert!((r_coeff - 1.0).abs() < 1e-9);

    // Each assignment constraint has q = 2 terms and rhs = 1.
    for c in ilp.constraints.iter().take(3) {
        assert_eq!(c.terms.len(), 2);
        assert!((c.rhs - 1.0).abs() < 1e-9);
    }

    // Each radius constraint has m + 1 = 4 terms (one per position + R) and
    // rhs = m = 3.
    for c in ilp.constraints.iter().skip(3) {
        assert_eq!(c.terms.len(), 4);
        assert!((c.rhs - 3.0).abs() < 1e-9);
    }
}

#[test]
fn test_closeststring_to_ilp_closed_loop() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");

    let bf_value = BruteForce::new().solve(&source).unwrap();
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let extracted_value = source.evaluate(&extracted).unwrap();

    // The extracted center must be syntactically valid and match the BF optimum.
    assert!(extracted_value.is_valid());
    assert_eq!(extracted_value, bf_value);
    // Sanity: the canonical instance has optimum radius 2.
    assert_eq!(extracted_value, Min(Some(2)));
}

#[test]
fn test_closeststring_to_ilp_bf_vs_ilp() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_closeststring_to_ilp_extract_known_center() {
    // Build the binary encoding of the center 000 by hand:
    // x_{0,0}=x_{1,0}=x_{2,0}=1, others 0, R = 2.
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");

    let mut target_solution = vec![0usize; reduction.target_problem().num_vars];
    target_solution[0] = 1; // x_{0,0}
    target_solution[2] = 1; // x_{1,0}
    target_solution[4] = 1; // x_{2,0}
    target_solution[6] = 2; // R = 2

    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(extracted, vec![0, 0, 0]);
    assert_eq!(source.evaluate(&extracted).unwrap(), Min(Some(2)));
}

#[test]
fn test_closeststring_to_ilp_rejects_missing_one_hot_symbol() {
    let source = ClosestString::new(2, vec![vec![0, 1]]);
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let target_solution = vec![0; reduction.target_problem().num_vars];

    assert_eq!(
        reduction
            .extract_solution(&target_solution)
            .unwrap_err()
            .to_string(),
        "center position 0 has no selected symbol"
    );
}

#[test]
fn test_closeststring_to_ilp_ternary_alphabet() {
    // q = 3, m = 2, three strings forcing a nonzero radius. The optimum
    // radius is 1 (any center matches at least one position of every string).
    let source = ClosestString::new(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]]);
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // q * m + 1 = 3 * 2 + 1 = 7 variables; m + n = 2 + 3 = 5 constraints.
    assert_eq!(ilp.num_vars, 7);
    assert_eq!(ilp.constraints.len(), 5);

    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_closeststring_to_ilp_single_string_zero_radius() {
    // A single input string: the center equals the input and the optimum
    // radius is 0. This guards against off-by-one errors in the radius
    // constraints.
    let source = ClosestString::new(2, vec![vec![1, 0, 1, 1]]);
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(extracted, vec![1, 0, 1, 1]);
    assert_eq!(source.evaluate(&extracted).unwrap(), Min(Some(0)));
}
