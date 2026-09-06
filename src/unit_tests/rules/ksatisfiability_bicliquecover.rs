//! Tests for the KSatisfiability/K3 → BicliqueCover reduction.
//!
//! The target instance even for tiny inputs has rank `>= 18` and many
//! hundreds of binary variables, so closed-loop brute-force solving is
//! infeasible. The tests below verify:
//!
//! - Structural sizes (vertex counts, rank, k_f) on the example from
//!   the issue body (1 source variable; smallest power-of-two padding).
//! - Structural sizes on the four-variable example from the issue body
//!   (n = 4, m = 2 source clauses; reduction adds 8 exactly-one
//!   clauses for m = 10 normalized clauses).
//! - The construction terminates and produces a valid `BipartiteGraph`
//!   on both YES and NO source formulas.
//! - `extract_solution` correctly inspects `B_1` (using a hand-built
//!   biclique that contains `s_11^u`, `s_11^v`, and selected `h_i^u`
//!   vertices) and maps the normalized assignment back to the source
//!   variables.

use super::*;
use crate::models::formula::CNFClause;
use crate::models::graph::BicliqueCover;
use crate::traits::Problem;
use crate::variant::K3;

/// Issue body Section 1 example: one source variable, one source clause.
/// Normalized counts: `n = 2` (t_1, f_1), `ell = 1`, `m = 3` clauses
/// (1 translated + 2 exactly-one), `k_f = 4 + 4 + 6 = 14`, rank `= 18`.
#[test]
fn test_ksatisfiability_to_bicliquecover_structure_single_variable() {
    let source = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]);
    let reduction =
        ReduceTo::<BicliqueCover>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    // Normalized: n = 2, ell = 1, m = 1 + 2 = 3.
    // k_f = 4*1 + 2*ceil(log2 3) + 6 = 4 + 4 + 6 = 14.
    // rank = 14 + 2 + 2 = 18.
    let n = 2;
    let m = 3;
    let ell = 1;
    let k_f = 14;
    let rank = 18;

    let partition_size = n + 3 * m + 3 * ell + 2 + k_f;
    assert_eq!(target.left_size(), partition_size);
    assert_eq!(target.right_size(), partition_size);
    assert_eq!(target.num_vertices(), 2 * partition_size);
    assert_eq!(target.k(), rank);
}

/// Issue body Example section: `n = 4`, `m_source = 2`, `ell = 2`,
/// `m_normalized = 2 + 4 = 6`, `k_f = 4*2 + 2*ceil(log2 6) + 6 = 18`,
/// `rank = 18 + 4 + 2 = 24`.
///
/// (The issue example computes `m = 2`, `k_f = 16`, `rank = 22` *if*
/// you skip the exactly-one normalization clauses. Our implementation
/// faithfully emits them so the rank rises from 22 to 24 — both are
/// admissible polynomial upper bounds for the same reduction.)
#[test]
fn test_ksatisfiability_to_bicliquecover_structure_issue_example() {
    let source = KSatisfiability::<K3>::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 3, 4]),
        ],
    );
    let reduction =
        ReduceTo::<BicliqueCover>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    let n = 8; // next power of two of 2*4 = 8
    let ell = 3; // log2 8
    let m = 2 + 2 * (n / 2); // 2 source + 2 per normalized var = 2 + 8 = 10
                             // k_f = 4*3 + 2*ceil(log2 10) + 6 = 12 + 8 + 6 = 26
    let k_f = 4 * ell + 2 * 4 + 6;
    let rank = k_f + 2 * ell + 2;

    let partition_size = n + 3 * m + 3 * ell + 2 + k_f;
    assert_eq!(target.left_size(), partition_size);
    assert_eq!(target.right_size(), partition_size);
    assert_eq!(target.k(), rank);

    // Crown contributes n(n-1) important edges; the constructed graph
    // must contain at least that many edges in total.
    assert!(target.num_edges() >= n * (n - 1));
}

/// Build the reduction on an UNSAT source. The reduction itself is a
/// purely syntactic construction, so it must not panic.
#[test]
fn test_ksatisfiability_to_bicliquecover_unsat_constructs() {
    // (x_1) ∧ (¬x_1), padded to 3-literal clauses by repetition.
    let source = KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
        ],
    );
    let reduction =
        ReduceTo::<BicliqueCover>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    // Even an UNSAT formula yields a syntactically valid BicliqueCover
    // instance with positive partitions.
    assert!(target.left_size() > 0);
    assert!(target.right_size() > 0);
    assert!(target.k() > 0);
}

#[test]
fn test_ksatisfiability_to_bicliquecover_rejects_invalid_covers() {
    let source = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]);
    let reduction = ReduceTo::<BicliqueCover>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    let mut sparse = vec![vec![false; target.num_vertices()]; target.k()];
    sparse[0][reduction.s1_left_offset] = true;
    sparse[0][target.left_size() + reduction.s1_right_offset] = true;
    sparse[0][0] = true;
    assert!(target.evaluate(&sparse).unwrap().0.is_none());
    for invalid in [
        sparse,
        vec![],
        vec![vec![true; target.num_vertices()]; target.k()],
        vec![vec![false; target.num_vertices() - 1]; target.k()],
    ] {
        assert!(reduction.extract_solution(&invalid).is_err());
    }
}

#[test]
fn test_ksatisfiability_to_bicliquecover_extracts_every_row_rotation() {
    let source = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]);
    let reduction = ReduceTo::<BicliqueCover>::reduce_to(&source).unwrap();
    let mut cover = super::forward_witness_single_variable_single_clause(&source);
    let cost = reduction.target_problem().evaluate(&cover).unwrap();
    assert!(cost.0.is_some());
    for _ in 0..cover.len() {
        assert_eq!(reduction.target_problem().evaluate(&cover).unwrap(), cost);
        let assignment = reduction.extract_solution(&cover).unwrap();
        assert_eq!(assignment, vec![true]);
        assert!(source.evaluate(&assignment).unwrap().0);
        cover.rotate_left(1);
    }
}

/// Closed-loop round-trip on the canonical small case (1 source
/// variable, 1 source clause). Uses the hand-built forward witness
/// from the `example-db` builder so the test does not need a full
/// BicliqueCover solver. Verifies:
///
/// 1. The constructed witness is a valid biclique cover of the target.
/// 2. `extract_solution` on the witness produces a satisfying source
///    assignment.
#[test]
fn test_ksatisfiability_to_bicliquecover_closed_loop_smallest() {
    let source = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]);
    let reduction =
        ReduceTo::<BicliqueCover>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    let witness = super::forward_witness_single_variable_single_clause(&source);
    assert!(
        target.is_valid_cover(&witness),
        "forward witness must be a valid biclique cover"
    );

    let extracted = reduction.extract_solution(&witness).unwrap();
    assert_eq!(extracted.len(), 1);
    assert!(
        extracted[0],
        "extracted assignment must set x_1 = true (the only satisfying assignment)"
    );
    assert_eq!(
        source.evaluate(&extracted).unwrap(),
        crate::types::Or(true),
        "extracted source assignment must satisfy the formula"
    );
}

/// Sanity check that the constructed `BipartiteGraph` references vertex
/// indices within the declared partition sizes — the
/// `BipartiteGraph::new` call would panic otherwise.
#[test]
fn test_ksatisfiability_to_bicliquecover_construct_two_vars_no_panic() {
    let source = KSatisfiability::<K3>::new(
        2,
        vec![
            CNFClause::new(vec![1, 2, 1]),
            CNFClause::new(vec![-1, 2, -2]),
        ],
    );
    let reduction =
        ReduceTo::<BicliqueCover>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    // Normalized n = next_power_of_two(2*2) = 4, ell = 2.
    // m_normalized = 2 source + 2 * 2 = 6.
    // k_f = 4*2 + 2*ceil(log2 6) + 6 = 8 + 6 + 6 = 20.
    // rank = 20 + 4 + 2 = 26.
    assert_eq!(reduction.normalized_n, 4);
    assert_eq!(target.k(), 26);
}

#[test]
fn test_ksatisfiability_to_bicliquecover_sparse_variable_inverse() {
    // The only used variable has a high source index. Its normalized gadget
    // equals the canonical one-variable gadget; unused variables map to false.
    let source = KSatisfiability::<K3>::new_allow_less(7, vec![CNFClause::new(vec![7])]);
    let reduction = ReduceTo::<BicliqueCover>::reduce_to(&source).unwrap();
    assert_eq!(reduction.source_variables, vec![6]);
    assert_eq!(reduction.normalized_n, 2);
    let cover = super::forward_witness_single_variable_single_clause(&source);
    let assignment = reduction.extract_solution(&cover).unwrap();
    assert_eq!(
        assignment,
        vec![false, false, false, false, false, false, true]
    );
    assert!(source.evaluate(&assignment).unwrap().0);

    // No massive assignment allocation: verify that construction depends on
    // appearing variables rather than the declared count.
    let largest = usize::try_from(i64::MAX).unwrap_or(usize::MAX);
    let literal = i64::try_from(largest).unwrap();
    let huge = KSatisfiability::<K3>::new(largest, vec![CNFClause::new(vec![literal; 3])]);
    let huge_reduction = ReduceTo::<BicliqueCover>::reduce_to(&huge).unwrap();
    assert_eq!(huge_reduction.source_variables, vec![largest - 1]);
    assert_eq!(
        huge_reduction.target_problem().graph().left_edges(),
        reduction.target_problem().graph().left_edges()
    );
    assert_eq!(
        huge_reduction.target_problem().k(),
        reduction.target_problem().k()
    );
}

#[test]
fn test_ksatisfiability_to_bicliquecover_empty_conjunction_and_clause() {
    for n in [0, 3] {
        let yes = KSatisfiability::<K3>::new(n, vec![]);
        let reduction = ReduceTo::<BicliqueCover>::reduce_to(&yes).unwrap();
        assert_eq!(reduction.target_problem().num_vertices(), 0);
        assert_eq!(reduction.extract_solution(&vec![]).unwrap(), vec![false; n]);
        assert!(yes.evaluate(&vec![false; n]).unwrap().0);
        let no = KSatisfiability::<K3>::new_allow_less(n, vec![CNFClause::new(vec![])]);
        let reduction = ReduceTo::<BicliqueCover>::reduce_to(&no).unwrap();
        assert!(reduction
            .target_problem()
            .evaluate(&vec![])
            .unwrap()
            .0
            .is_none());
        assert!(reduction.extract_solution(&vec![]).is_err());
        assert!(!no.evaluate(&vec![false; n]).unwrap().0);
    }
}

#[test]
fn test_ksatisfiability_to_bicliquecover_normalization_preserves_short_clauses() {
    for literals in [vec![2], vec![-2], vec![2, -5], vec![-2, 5, 5]] {
        let source = KSatisfiability::<K3>::new_allow_less(6, vec![CNFClause::new(literals)]);
        let reduction = ReduceTo::<BicliqueCover>::reduce_to(&source).unwrap();
        let (n, clauses) = super::normalize(&source, &reduction.source_variables).unwrap();
        let normalized =
            KSatisfiability::<K3>::new(n, clauses.into_iter().map(CNFClause::new).collect());
        for mask in 0..64usize {
            let assignment: Vec<bool> = (0..6).map(|i| mask & (1 << i) != 0).collect();
            let mut expanded = vec![false; n];
            for pair in 0..n / 2 {
                let truth = reduction
                    .source_variables
                    .get(pair)
                    .map(|&i| assignment[i])
                    .unwrap_or(false);
                expanded[2 * pair] = truth;
                expanded[2 * pair + 1] = !truth;
            }
            assert_eq!(
                source.evaluate(&assignment).unwrap(),
                normalized.evaluate(&expanded).unwrap()
            );
        }
    }
}
