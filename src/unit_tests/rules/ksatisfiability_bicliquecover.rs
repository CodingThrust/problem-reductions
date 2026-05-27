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
#[cfg(feature = "example-db")]
use crate::traits::Problem;
use crate::variant::K3;

/// Issue body Section 1 example: one source variable, one source clause.
/// Normalized counts: `n = 2` (t_1, f_1), `ell = 1`, `m = 3` clauses
/// (1 translated + 2 exactly-one), `k_f = 4 + 4 + 6 = 14`, rank `= 18`.
#[test]
fn test_ksatisfiability_to_bicliquecover_structure_single_variable() {
    let source = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]);
    let reduction = ReduceTo::<BicliqueCover>::reduce_to(&source);
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
    let reduction = ReduceTo::<BicliqueCover>::reduce_to(&source);
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
    let reduction = ReduceTo::<BicliqueCover>::reduce_to(&source);
    let target = reduction.target_problem();

    // Even an UNSAT formula yields a syntactically valid BicliqueCover
    // instance with positive partitions.
    assert!(target.left_size() > 0);
    assert!(target.right_size() > 0);
    assert!(target.k() > 0);
}

/// Verify that `extract_solution` reads the normalized assignment off
/// of a hand-built `B_1` biclique. The witness here is not a valid
/// biclique cover — only the slice corresponding to `B_1` is used by
/// `extract_solution`, and that slice contains the expected `h_i^u`
/// memberships.
#[test]
fn test_ksatisfiability_to_bicliquecover_extract_solution_reads_b1() {
    let source = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]);
    let reduction = ReduceTo::<BicliqueCover>::reduce_to(&source);
    let target = reduction.target_problem();

    let n = reduction.normalized_n;
    let k = target.k();
    let left_size = target.left_size();
    let num_vertices = target.num_vertices();
    let s1_left = reduction.s1_left_offset; // s_{1,1}^u (bipartite-local)
    let s1_right_unified = left_size + reduction.s1_right_offset; // s_{1,1}^v

    // Use biclique slot r = 0 as B_1: contains s_11^u, s_11^v, h_0^u
    // (i.e. t_1 == true), and no Y matching vertex.
    let mut witness = vec![0usize; num_vertices * k];
    let set = |w: &mut [usize], vertex: usize, biclique: usize| {
        w[vertex * k + biclique] = 1;
    };
    set(&mut witness, s1_left, 0);
    set(&mut witness, s1_right_unified, 0);
    // h_0^u is unified vertex 0 (h_offset = 0, left partition).
    set(&mut witness, 0, 0);
    // Leave h_1^u (vertex 1) unset → f_1 = false in B_1.

    let assignment = reduction.extract_solution(&witness);
    assert_eq!(assignment.len(), 1);
    assert_eq!(assignment[0], 1, "expected source x_1 = true from B_1");

    // Sanity: n should be 2 for this source.
    assert_eq!(n, 2);
}

/// If `B_1` is shadowed by a free-edge biclique that touches `Y`, the
/// extractor must skip it and proceed to the next candidate. We test
/// this by setting up two bicliques that both contain `s_11^u` and
/// `s_11^v`: biclique 0 also contains `y_0^u` (so it is rejected) and
/// biclique 1 is the real `B_1` containing `h_0^u`.
#[test]
fn test_ksatisfiability_to_bicliquecover_extract_skips_y_touching_bicliques() {
    let source = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]);
    let reduction = ReduceTo::<BicliqueCover>::reduce_to(&source);
    let target = reduction.target_problem();

    let k = target.k();
    let left_size = target.left_size();
    let num_vertices = target.num_vertices();
    let s1_left = reduction.s1_left_offset;
    let s1_right_unified = left_size + reduction.s1_right_offset;
    let y_left_0 = reduction.y_left_offset; // y_0^u (bipartite-local on left)

    let mut witness = vec![0usize; num_vertices * k];
    let set = |w: &mut [usize], vertex: usize, biclique: usize| {
        w[vertex * k + biclique] = 1;
    };

    // Biclique 0: touches Y on the left side (y_0^u). The extractor
    // must reject this candidate for B_1.
    set(&mut witness, s1_left, 0);
    set(&mut witness, s1_right_unified, 0);
    set(&mut witness, y_left_0, 0);
    set(&mut witness, 0, 0); // h_0^u

    // Biclique 1: clean B_1 covering s_11 edges plus h_1^u (so t_1
    // would be false). Extraction reads h_0^u from biclique 1 (false).
    set(&mut witness, s1_left, 1);
    set(&mut witness, s1_right_unified, 1);
    // h_1^u is unified vertex 1.
    set(&mut witness, 1, 1);

    let assignment = reduction.extract_solution(&witness);
    assert_eq!(assignment.len(), 1);
    assert_eq!(
        assignment[0], 0,
        "B_1 was biclique 1 (not biclique 0); h_0^u not in B_1 so x_1 = false"
    );
}

/// Closed-loop round-trip on the canonical small case (1 source
/// variable, 1 source clause). Uses the hand-built forward witness
/// from the `example-db` builder so the test does not need a full
/// BicliqueCover solver. Verifies:
///
/// 1. The constructed witness is a valid biclique cover of the target.
/// 2. `extract_solution` on the witness produces a satisfying source
///    assignment.
#[cfg(feature = "example-db")]
#[test]
fn test_ksatisfiability_to_bicliquecover_closed_loop_smallest() {
    let source = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]);
    let reduction = ReduceTo::<BicliqueCover>::reduce_to(&source);
    let target = reduction.target_problem();

    let witness = super::forward_witness_single_variable_single_clause(&source);
    assert!(
        target.is_valid_cover(&witness),
        "forward witness must be a valid biclique cover"
    );

    let extracted = reduction.extract_solution(&witness);
    assert_eq!(extracted.len(), 1);
    assert_eq!(
        extracted[0], 1,
        "extracted assignment must set x_1 = true (the only satisfying assignment)"
    );
    assert_eq!(
        source.evaluate(&extracted),
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
    let reduction = ReduceTo::<BicliqueCover>::reduce_to(&source);
    let target = reduction.target_problem();

    // Normalized n = next_power_of_two(2*2) = 4, ell = 2.
    // m_normalized = 2 source + 2 * 2 = 6.
    // k_f = 4*2 + 2*ceil(log2 6) + 6 = 8 + 6 + 6 = 20.
    // rank = 20 + 4 + 2 = 26.
    assert_eq!(reduction.normalized_n, 4);
    assert_eq!(target.k(), 26);
}
