use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::misc::ClosestSubstring;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::traits::Problem;
use crate::types::Min;

/// Canonical issue #1033 instance: binary alphabet, length-3 windows on three
/// length-5 strings. Optimum radius is 1.
fn issue_instance() -> ClosestSubstring {
    ClosestSubstring::new(
        2,
        vec![
            vec![0, 0, 0, 1, 1],
            vec![1, 0, 1, 0, 0],
            vec![1, 1, 0, 0, 1],
        ],
        3,
    )
    .unwrap()
}

#[test]
fn test_closestsubstring_to_ilp_structure() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // q = 2, ell = 3, total windows W = 3 + 3 + 3 = 9.
    // num_vars = q*ell + W + 1 = 6 + 9 + 1 = 16.
    assert_eq!(ilp.num_vars, 16);
    // num_constraints = ell + 1 (radius upper bound) + n + W
    //                 = 3 + 1 + 3 + 9 = 16.
    assert_eq!(ilp.constraints.len(), 16);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);

    // The objective puts weight 1 on the radius variable only, at the very
    // last index.
    assert_eq!(ilp.objective.len(), 1);
    let (r_idx, r_coeff) = ilp.objective[0];
    assert_eq!(r_idx, ilp.num_vars - 1);
    assert!((r_coeff - 1.0).abs() < 1e-9);

    // First ell = 3 constraints are assignment constraints (q = 2 terms, rhs = 1).
    for c in ilp.constraints.iter().take(3) {
        assert_eq!(c.terms.len(), 2);
        assert!((c.rhs - 1.0).abs() < 1e-9);
    }

    // Constraint index ell = 3 is the radius upper bound R <= ell.
    let r_bound = &ilp.constraints[3];
    assert_eq!(r_bound.terms.len(), 1);
    assert_eq!(r_bound.terms[0].0, r_idx);
    assert!((r_bound.terms[0].1 - 1.0).abs() < 1e-9);
    assert!((r_bound.rhs - 3.0).abs() < 1e-9);

    // Next n = 3 constraints are window-choice constraints (W_i = 3 terms,
    // rhs = 1).
    for c in ilp.constraints.iter().skip(4).take(3) {
        assert_eq!(c.terms.len(), 3);
        assert!((c.rhs - 1.0).abs() < 1e-9);
    }

    // Remaining W = 9 constraints are conditional radius constraints. Each
    // has ell + 2 = 5 terms (R, the ell center-position match terms, and
    // -ell * y_{i, p}) and rhs = 0.
    for c in ilp.constraints.iter().skip(7) {
        assert_eq!(c.terms.len(), 5);
        assert!(c.rhs.abs() < 1e-9);
    }
}

#[test]
fn test_closestsubstring_to_ilp_rejects_missing_one_hot_symbol() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let target_solution = vec![0; reduction.target_problem().num_vars];

    assert_eq!(
        reduction
            .extract_solution(&target_solution)
            .unwrap_err()
            .to_string(),
        "center position 0 has no selected value"
    );
}

#[test]
fn test_closestsubstring_to_ilp_closed_loop() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");

    let bf_value = BruteForce::new().solve(&source).unwrap();
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    // Extracted config must be syntactically valid (length ell + n = 6) and
    // match the brute-force optimum.
    assert_eq!(extracted.len(), 6);
    let extracted_value = source.evaluate(&extracted).unwrap();
    assert!(extracted_value.is_valid());
    assert_eq!(extracted_value, bf_value);
    // Sanity: the canonical instance has optimum radius 1.
    assert_eq!(extracted_value, Min(Some(1)));
}

#[test]
fn test_closestsubstring_to_ilp_bf_vs_ilp() {
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_closestsubstring_to_ilp_zero_radius_when_common_substring_exists() {
    // Each string contains 010 as a substring; the optimum radius is 0 with
    // center 010. Also exercises non-first window choices in some strings.
    let source = ClosestSubstring::new(
        2,
        vec![vec![0, 1, 0, 0], vec![1, 0, 1, 0], vec![0, 0, 1, 0]],
        3,
    )
    .unwrap();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    let extracted_value = source.evaluate(&extracted).unwrap();
    assert!(extracted_value.is_valid());
    assert_eq!(extracted_value, Min(Some(0)));
}

#[test]
fn test_closestsubstring_to_ilp_ternary_alphabet() {
    // q = 3, ell = 2, three length-3 strings. Brute-force optimum is small
    // enough to cross-check via the closed loop.
    let source =
        ClosestSubstring::new(3, vec![vec![0, 1, 2], vec![1, 2, 0], vec![2, 0, 1]], 2).unwrap();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // q*ell + W + 1 with W = 2 + 2 + 2 = 6: num_vars = 6 + 6 + 1 = 13.
    // num_constraints = ell + 1 + n + W = 2 + 1 + 3 + 6 = 12.
    assert_eq!(ilp.num_vars, 13);
    assert_eq!(ilp.constraints.len(), 12);

    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_closestsubstring_to_ilp_extract_known_solution() {
    // Build the ILP encoding of center 010 with chosen windows (0, 1, 0) on
    // the canonical issue instance, by hand: x_{0,0}=x_{1,1}=x_{2,0}=1,
    // y_{1,0}=y_{2,1}=y_{3,0}=1, R = 1. Then verify the extracted source
    // config matches and gives radius 1.
    let source = issue_instance();
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let mut target_solution = vec![0usize; ilp.num_vars];
    target_solution[0] = 1; // x_{0,0}
    target_solution[3] = 1; // x_{1,1}
    target_solution[4] = 1; // x_{2,0}

    // y_{i, p} live at indices q*ell + window_offsets[i] + p. With q*ell = 6
    // and W_i = 3 for each string, the y-block starts at 6.
    target_solution[6] = 1; // y_{1, 0}
    target_solution[6 + 3 + 1] = 1; // y_{2, 1}
    target_solution[6 + 6] = 1; // y_{3, 0}
    target_solution[ilp.num_vars - 1] = 1; // R = 1

    let extracted = reduction.extract_solution(&target_solution).unwrap();
    assert_eq!(extracted, vec![0, 1, 0, 0, 1, 0]);
    assert_eq!(source.evaluate(&extracted).unwrap(), Min(Some(1)));
}
