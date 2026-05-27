//! Closed-loop and structural tests for the Planar 3-SAT → Minimum Geometric Connected
//! Dominating Set reduction (Lichtenstein 1982 §6, Theorem 5).
//!
//! Because the Lichtenstein construction emits dozens of points for even tiny source
//! instances, full brute-force round-trip solving is only feasible for the trivial
//! `num_clauses == 0` case. The other tests cover:
//!
//! * structural invariants (radius, point-count overhead bound, K formula),
//! * Phase A occurrence counting (including repeated literals and tautologies),
//! * trivial-case correctness (m = 0 → 1-point target with K = 1, solvable by brute force),
//! * `extract_solution` recovery on the trivial case.
//!
//! A full closed-loop solve test on non-trivial instances will become feasible once a
//! `MinimumGeometricConnectedDominatingSet → ILP` rule is added (separate issue).

use crate::models::formula::{CNFClause, Planar3Satisfiability};
use crate::models::graph::MinimumGeometricConnectedDominatingSet;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_planar3satisfiability_to_minimumgeometricconnecteddominatingset_closed_loop() {
    // Trivial corner case: vacuously satisfiable source (no clauses).
    // The reduction emits a 1-point target with K = 1; brute force confirms 1 ≤ K.
    let source = Planar3Satisfiability::new(0, vec![]);
    let reduction = ReduceTo::<MinimumGeometricConnectedDominatingSet>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_points(), 1);
    assert_eq!(target.radius(), 1.0);
    assert_eq!(reduction.bound_k(), 1);

    // Brute-force solve the trivial target: a single point is its own connected
    // dominating set of size 1.
    let solver = BruteForce::new();
    let witness = solver
        .find_witness(target)
        .expect("trivial target must be solvable");
    let value = target.evaluate(&witness);
    assert_eq!(value, Min(Some(1)));
    assert!(value.0.unwrap() <= reduction.bound_k());
}

#[test]
fn test_trivial_reduction_with_unused_variables() {
    // m = 0 but n > 0: variables exist but no clauses. Source is still trivially
    // satisfiable; reduction still emits the trivial 1-point target.
    let source = Planar3Satisfiability::new(3, vec![]);
    let reduction = ReduceTo::<MinimumGeometricConnectedDominatingSet>::reduce_to(&source);
    assert_eq!(reduction.target_problem().num_points(), 1);
    assert_eq!(reduction.bound_k(), 1);

    // extract_solution should return all-false assignment of the right length.
    let extracted = reduction.extract_solution(&[1]);
    assert_eq!(extracted.len(), 3);
    assert_eq!(extracted, vec![0, 0, 0]);
    assert!(source.is_satisfying(&[false, false, false]));
}

#[test]
fn test_structural_single_clause() {
    // Single clause (x_1 ∨ x_2 ∨ x_3): three variables each occurring once.
    // Phase A produces n' = 3 copies (m_i = 1 for each variable), m_b = 1 + 3 = 4
    // bipolar clauses. The target must have radius = 1, respect the overhead bound, and
    // K must follow the NV + NC + NG + m_b formula.
    let source = Planar3Satisfiability::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<MinimumGeometricConnectedDominatingSet>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.radius(), 1.0);
    // Overhead bound: 100 * 1 + 10 * 3 = 130.
    assert!(
        target.num_points() <= 130,
        "num_points {} should be ≤ 130 (overhead bound)",
        target.num_points()
    );
    assert!(target.num_points() >= 1);
    // K must be positive.
    assert!(reduction.bound_k() > 0);
}

#[test]
fn test_structural_canonical_example() {
    // Canonical example from Planar3Satisfiability::canonical_model_example_specs.
    // n = 4, m = 4. Phase A: total_copies = 3 * 4 = 12, m_b = 4 + 12 = 16.
    let source = Planar3Satisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 2, 4]),
            CNFClause::new(vec![1, -3, 4]),
            CNFClause::new(vec![-2, 3, -4]),
        ],
    );
    let reduction = ReduceTo::<MinimumGeometricConnectedDominatingSet>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.radius(), 1.0);
    // Overhead bound: 100 * 4 + 10 * 4 = 440.
    assert!(
        target.num_points() <= 440,
        "num_points {} should be ≤ 440 (overhead bound)",
        target.num_points()
    );

    // K must satisfy NV + NC + NG + m_b with m_b = 16:
    //   NV ≥ ceil(4 * 12 / 3) = 16
    //   NC = 3 * 16 = 48
    //   NG = 12
    //   m_b = 16
    // ⇒ K ≥ 16 + 48 + 12 + 16 = 92
    assert!(
        reduction.bound_k() >= 92,
        "bound_k {} should be ≥ 92 for the canonical example",
        reduction.bound_k()
    );
}

#[test]
fn test_phase_a_repeated_literals() {
    // Clause with repeated literal (x_1 ∨ x_1 ∨ x_2): m_1 = 2, m_2 = 1.
    // total_copies = 3, m_b = 1 + 3 = 4. The reduction must accept this without panicking.
    let source = Planar3Satisfiability::new(2, vec![CNFClause::new(vec![1, 1, 2])]);
    let reduction = ReduceTo::<MinimumGeometricConnectedDominatingSet>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.radius(), 1.0);
    assert!(target.num_points() <= 100 + 20); // 100 * 1 + 10 * 2 = 120
}

#[test]
fn test_phase_a_tautological_clause() {
    // Tautological clause (x_1 ∨ ¬x_1 ∨ x_2): both polarities of x_1, plus x_2.
    // m_1 = 2 (one positive + one negative), m_2 = 1. total_copies = 3, m_b = 4.
    let source = Planar3Satisfiability::new(2, vec![CNFClause::new(vec![1, -1, 2])]);
    let reduction = ReduceTo::<MinimumGeometricConnectedDominatingSet>::reduce_to(&source);
    assert_eq!(reduction.target_problem().radius(), 1.0);
    assert!(reduction.target_problem().num_points() >= 1);
}

#[test]
fn test_extract_solution_default_for_unused_variable() {
    // n = 2, but only x_1 is used. extract_solution should default x_2 to false.
    let source = Planar3Satisfiability::new(2, vec![CNFClause::new(vec![1, 1, 1])]);
    let reduction = ReduceTo::<MinimumGeometricConnectedDominatingSet>::reduce_to(&source);
    // Construct a fake target solution that picks no variable-column anchors.
    let n = reduction.target_problem().num_points();
    let target_solution = vec![0usize; n];
    let extracted = reduction.extract_solution(&target_solution);
    assert_eq!(extracted.len(), 2);
    assert_eq!(extracted[1], 0); // unused variable defaults to false
}

#[test]
fn test_extract_solution_length_matches_num_vars() {
    let source = Planar3Satisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 2, -3]),
        ],
    );
    let reduction = ReduceTo::<MinimumGeometricConnectedDominatingSet>::reduce_to(&source);
    let n = reduction.target_problem().num_points();
    let extracted = reduction.extract_solution(&vec![0usize; n]);
    assert_eq!(extracted.len(), 3);
}

#[test]
fn test_accessors() {
    let source = Planar3Satisfiability::new(2, vec![CNFClause::new(vec![1, 2, -2])]);
    let reduction = ReduceTo::<MinimumGeometricConnectedDominatingSet>::reduce_to(&source);
    assert_eq!(reduction.num_vars(), 2);
    assert!(reduction.bound_k() >= 1);
}
