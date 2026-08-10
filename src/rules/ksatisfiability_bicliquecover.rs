//! Reduction from KSatisfiability/K3 (3-SAT) to BicliqueCover.
//!
//! Implements the Chandran–Issac–Karrenbauer construction (IPEC 2016,
//! Theorem 6 and Section 3). The reduction has two stages:
//!
//! 1. **Normalize** the source 3-CNF formula:
//!    - For each original variable `x_i`, create two normalized variables
//!      `t_i` and `f_i`. Replace literal `x_i` by `t_i`, replace literal
//!      `¬x_i` by `f_i`. Add exactly-one clauses
//!      `(t_i ∨ f_i ∨ f_i)` and `(¬t_i ∨ ¬f_i ∨ ¬f_i)` so that any
//!      satisfying assignment has `t_i = ¬f_i`.
//!    - Pad with dummy variable pairs until the total number of
//!      normalized variables `n` is a power of two `2^ell`.
//!    - Every satisfying normalized assignment has exactly `n/2` true
//!      variables.
//!
//! 2. **Build the gadget** `G = (U, V, E)` with edges partitioned into
//!    *important* and *free* edges:
//!    - Crown graph `H_n` on `{h_i^u}` ∪ `{h_i^v}` with `n(n-1)` important
//!      edges (omit the diagonal `h_i^u h_i^v`).
//!    - Clause induced matchings `P_i` of size 3 (one important edge per
//!      literal slot).
//!    - Domino gadgets `S_j` (`j ∈ [ell]`) with 7 important edges each.
//!    - Guard induced matching `Q` of size 2.
//!    - Important `H-S` cross-edges `s_j2^u h_i^v` and `s_j2^v h_i^u`.
//!    - Free edges between `H-S` (extreme rows), `P-P`, `P-Q`, `H-P`
//!      (literal-aware omissions), `S_1-P`.
//!    - Forcing matching `Y` of `k_f` parallel edges, each `y_r^u y_r^v`
//!      made bisimplicial with one free-edge biclique `B_r^f`.
//!
//!    Set `k_f = 4·ell + 2·ceil(log2 m) + 6` and target rank
//!    `k = k_f + 2·ell + 2`.
//!
//! By Lemmas 16–19 of the paper, the BicliqueCover instance has rank `k`
//! iff the (normalized) formula is satisfiable. Solution extraction
//! identifies the biclique `B_1` covering `s_11^u s_11^v` and reads off
//! `x_i = true` iff `h_i^u ∈ B_1` (after mapping `t_i, f_i` back to the
//! source variables).
//!
//! See issue #1057 for the full construction; this file mirrors the
//! issue body section-by-section.

#[cfg(feature = "example-db")]
use crate::models::formula::CNFClause;
use crate::models::formula::KSatisfiability;
use crate::models::graph::BicliqueCover;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::BipartiteGraph;
use crate::variant::K3;
use std::collections::BTreeSet;

/// Result of reducing KSatisfiability/K3 to BicliqueCover.
///
/// Carries the normalization metadata needed for solution extraction:
/// the source variable count, the number of normalized variables, and
/// the offset of the `H` vertex block inside the bipartite gadget.
#[derive(Debug, Clone)]
pub struct ReductionKSatisfiabilityToBicliqueCover {
    target: BicliqueCover,
    /// Number of variables in the source 3-CNF formula.
    source_num_vars: usize,
    /// Number of normalized variables `n = 2^ell` (a power of two and
    /// at least `2 * source_num_vars`).
    normalized_n: usize,
    /// Bipartite-local offset of the `S_1` block on the left side.
    /// Used to locate vertex `s_11^u` for B_1 identification.
    s1_left_offset: usize,
    /// Bipartite-local offset of the `S_1` block on the right side.
    /// Used to locate vertex `s_11^v` for B_1 identification.
    s1_right_offset: usize,
    /// Bipartite-local offset of the `Y` block on the left side.
    /// Used to skip free-edge bicliques during extraction.
    y_left_offset: usize,
    /// Bipartite-local offset of the `Y` block on the right side.
    y_right_offset: usize,
    /// Number of free-edge bicliques `k_f`.
    k_f: usize,
}

impl ReductionResult for ReductionKSatisfiabilityToBicliqueCover {
    type Source = KSatisfiability<K3>;
    type Target = BicliqueCover;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Recover a source assignment from a BicliqueCover witness.
    ///
    /// 1. Ignore bicliques that cover the `Y` matching edges — these are
    ///    the `k_f` free-edge bicliques fixed by Lemma 17.
    /// 2. Identify `B_1` as a biclique containing both `s_11^u` and
    ///    `s_11^v`, but no `y_r^u` or `y_r^v` (so it is an
    ///    important-edge biclique).
    /// 3. For each normalized variable `i`, set the normalized
    ///    `t_i = true` iff `h_i^u ∈ B_1`.
    /// 4. Map normalized variables back to source variables by reading
    ///    each original `t_i`.
    ///
    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        let n = self.normalized_n;
        let left_size = self.target.left_size();
        let k = self.target.k();
        // Unified-vertex helpers for the named gadget anchors.
        let s11_u = self.s1_left_offset; // s_{1,1}^u
        let s11_v = left_size + self.s1_right_offset; // s_{1,1}^v
        let h_left = |i: usize| -> usize { i }; // h_i^u (i in 0..n)
        let y_left = |r: usize| -> usize { self.y_left_offset + r };
        let y_right = |r: usize| -> usize { left_size + self.y_right_offset + r };

        // Find a biclique containing both s_11^u and s_11^v, but no
        // Y-matching vertex. By Lemma 17, free-edge bicliques touch the
        // Y matching; the important-edge biclique B_1 does not.
        let mut b1_index = None;
        for r in 0..k {
            let in_b1 = |vertex: usize| target_solution[vertex * k + r] == 1;
            if !in_b1(s11_u) || !in_b1(s11_v) {
                continue;
            }
            // Reject bicliques that touch the Y matching on either side.
            let touches_y = (0..self.k_f).any(|r_y| in_b1(y_left(r_y)) || in_b1(y_right(r_y)));
            if touches_y {
                continue;
            }
            b1_index = Some(r);
            break;
        }

        // Read off normalized assignment: t_i = (h_i^u in B_1) for i in 0..n.
        let b1_index = b1_index.ok_or_else(|| {
            crate::rules::ExtractionError::invalid(
                "target configuration has no important-edge biclique B_1",
            )
        })?;
        let mut normalized_assignment = vec![false; n];
        for (i, slot) in normalized_assignment.iter_mut().enumerate() {
            *slot = target_solution[h_left(i) * k + b1_index] == 1;
        }

        // Map normalized t_i back to the source: source x_s = t_s
        // (with s in 1..=source_num_vars). t_s sits at normalized index
        // 2 * (s - 1).
        let mut source_assignment = vec![0usize; self.source_num_vars];
        for (s, slot) in source_assignment.iter_mut().enumerate() {
            let t_idx = 2 * s;
            *slot = if normalized_assignment[t_idx] { 1 } else { 0 };
        }
        Ok(source_assignment)
    }
}

/// Smallest power of two greater than or equal to `n`. Returns at least
/// `1` (so `next_power_of_two(0) == 1`).
fn next_power_of_two_at_least(n: usize) -> usize {
    let mut p = 1usize;
    while p < n {
        p *= 2;
    }
    p
}

/// `ceil(log2(m))` with the convention `ceil_log2(0) = ceil_log2(1) = 0`.
fn ceil_log2(m: usize) -> usize {
    if m <= 1 {
        return 0;
    }
    let mut bits = 0usize;
    let mut x = m - 1;
    while x > 0 {
        bits += 1;
        x >>= 1;
    }
    bits
}

/// Build the normalized 3-CNF formula from a source formula.
///
/// Returns `(n, normalized_clauses)` where `n` is a power of two
/// (the normalized variable count). Normalized clauses use signed
/// integer literals with the convention:
///
/// - `t_i` is normalized variable `2*(i-1)` (0-indexed); 1-indexed literal `2i-1`.
/// - `f_i` is normalized variable `2*(i-1) + 1` (0-indexed); 1-indexed literal `2i`.
///
/// For each source variable `i` in `1..=source_num_vars` and each padded
/// dummy variable, two exactly-one clauses are appended.
fn normalize(source: &KSatisfiability<K3>) -> (usize, Vec<Vec<i32>>) {
    let s = source.num_vars();
    // Padded source-variable count `s_pad` so that `2 * s_pad` is a
    // power of two.
    let s_pad = next_power_of_two_at_least(s.max(1));
    let n = 2 * s_pad;

    let t_lit = |i_one_indexed: usize| -> i32 { (2 * i_one_indexed - 1) as i32 };
    let f_lit = |i_one_indexed: usize| -> i32 { (2 * i_one_indexed) as i32 };

    let mut clauses: Vec<Vec<i32>> = Vec::new();

    // 1. Translate source clauses: x_i -> t_i, ¬x_i -> f_i.
    //    Both replacements use positive normalized literals; the
    //    exactly-one clauses below tie t_i and f_i to opposite truth
    //    values in any satisfying assignment.
    for clause in source.clauses() {
        let mut translated: Vec<i32> = Vec::with_capacity(clause.literals.len());
        for &lit in &clause.literals {
            let var = lit.unsigned_abs() as usize; // 1-indexed source var
            if lit > 0 {
                translated.push(t_lit(var));
            } else {
                translated.push(f_lit(var));
            }
        }
        clauses.push(translated);
    }

    // 2. Exactly-one clauses for each (real or dummy) normalized pair.
    //    (t_i ∨ f_i ∨ f_i) and (¬t_i ∨ ¬f_i ∨ ¬f_i).
    for i in 1..=s_pad {
        let t = t_lit(i);
        let f = f_lit(i);
        clauses.push(vec![t, f, f]);
        clauses.push(vec![-t, -f, -f]);
    }

    (n, clauses)
}

/// Compute `k_f = 4*ell + 2*ceil(log2 m) + 6` for the normalized formula.
fn free_edge_budget(ell: usize, m: usize) -> usize {
    4 * ell + 2 * ceil_log2(m) + 6
}

// Size expressions are upper bounds in terms of source counts.
// After normalization, `n ≤ 4·num_vars` (next power of two of `2·num_vars`)
// and `m ≤ num_clauses + n ≤ num_clauses + 4·num_vars`. With
// `ell = log2 n ≤ 2 + log2(num_vars)` we use the coarser bound
// `ell ≤ num_vars` and `ceil(log2 m) ≤ num_clauses + 4·num_vars`,
// giving the polynomial bounds below. Edges are bounded by
// `partition_size^2` which is `O((num_vars + num_clauses)^2)`.
#[reduction(
    exact = {
        num_vertices = "32 * num_vars + 24 * num_clauses + 100",
        num_edges = "(32 * num_vars + 24 * num_clauses + 100) * (32 * num_vars + 24 * num_clauses + 100)",
        rank = "10 * num_vars + 4 * num_clauses + 20",
    }
)]
impl ReduceTo<BicliqueCover> for KSatisfiability<K3> {
    type Result = ReductionKSatisfiabilityToBicliqueCover;

    fn reduce_to(&self) -> Self::Result {
        // ---------------- Stage 1: normalize ----------------
        let source_num_vars = self.num_vars();
        let (n, normalized_clauses) = normalize(self);
        let ell = ceil_log2(n).max(1); // n = 2^ell; ell >= 1
        let m = normalized_clauses.len();
        let k_f = free_edge_budget(ell, m);
        let rank = k_f + 2 * ell + 2;

        // ---------------- Stage 2: assemble vertex layout ----------------
        // Bipartite-local block offsets (same on left and right partitions).
        let h_offset = 0usize;
        let p_offset = h_offset + n;
        let s_offset = p_offset + 3 * m;
        let q_offset = s_offset + 3 * ell;
        let y_offset = q_offset + 2;
        let partition_size = y_offset + k_f;

        // Coordinate helpers (bipartite-local).
        let h_left = |i: usize| -> usize { h_offset + i };
        let h_right = |i: usize| -> usize { h_offset + i };
        // P_i has rows a in {0,1,2}; i in 0..m.
        let p_left = |i: usize, a: usize| -> usize { p_offset + 3 * i + a };
        let p_right = |i: usize, a: usize| -> usize { p_offset + 3 * i + a };
        // S_j has rows a in {0,1,2}; j in 0..ell.
        let s_left = |j: usize, a: usize| -> usize { s_offset + 3 * j + a };
        let s_right = |j: usize, a: usize| -> usize { s_offset + 3 * j + a };
        // Q has rows t in {0,1}.
        let q_left = |t: usize| -> usize { q_offset + t };
        let q_right = |t: usize| -> usize { q_offset + t };
        // Y has rows r in 0..k_f.
        let y_left = |r: usize| -> usize { y_offset + r };
        let y_right = |r: usize| -> usize { y_offset + r };

        // Edge list (bipartite-local).
        let mut edges: BTreeSet<(usize, usize)> = BTreeSet::new();
        let mut add_edge = |u: usize, v: usize| {
            edges.insert((u, v));
        };

        // ---------------- Important edges ----------------
        // 3. Crown H_n: h_i^u h_j^v for all i != j.
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    add_edge(h_left(i), h_right(j));
                }
            }
        }

        // 4. Clause matchings P_i: p_{i,a}^u p_{i,a}^v for a in {0,1,2}.
        for i in 0..m {
            for a in 0..3 {
                add_edge(p_left(i, a), p_right(i, a));
            }
        }

        // 5. Domino gadgets S_j: 7 important edges per domino.
        //    (s1,s1), (s1,s2), (s2,s1), (s2,s2), (s2,s3), (s3,s2), (s3,s3)
        //    using 0-indexed rows.
        let domino_pattern = [(0, 0), (0, 1), (1, 0), (1, 1), (1, 2), (2, 1), (2, 2)];
        for j in 0..ell {
            for &(a, b) in &domino_pattern {
                add_edge(s_left(j, a), s_right(j, b));
            }
        }

        // 6. Guard Q: q_t^u q_t^v for t in {0,1}.
        for t in 0..2 {
            add_edge(q_left(t), q_right(t));
        }

        // 7. Important H-S cross edges: s_{j,1}^u h_i^v and s_{j,1}^v h_i^u
        //    for all j in [ell], i in [n] (using 0-indexed rows; row 1 is s_j2).
        for j in 0..ell {
            for i in 0..n {
                add_edge(s_left(j, 1), h_right(i));
                add_edge(h_left(i), s_right(j, 1));
            }
        }

        // ---------------- Free edges ----------------
        // 8. Free H-S: s_{j,0}^u h_i^v, s_{j,2}^u h_i^v, h_i^u s_{j,0}^v,
        //    h_i^u s_{j,2}^v.
        for j in 0..ell {
            for i in 0..n {
                add_edge(s_left(j, 0), h_right(i));
                add_edge(s_left(j, 2), h_right(i));
                add_edge(h_left(i), s_right(j, 0));
                add_edge(h_left(i), s_right(j, 2));
            }
        }

        // 9. Free P-P: U(P_i) x V(P_j) for all i != j.
        for i in 0..m {
            for j in 0..m {
                if i == j {
                    continue;
                }
                for a in 0..3 {
                    for b in 0..3 {
                        add_edge(p_left(i, a), p_right(j, b));
                    }
                }
            }
        }

        // 10. Free P-Q: U(Q) x V(P_i) and U(P_i) x V(Q) for all i.
        for i in 0..m {
            for a in 0..3 {
                for t in 0..2 {
                    add_edge(q_left(t), p_right(i, a));
                    add_edge(p_left(i, a), q_right(t));
                }
            }
        }

        // 11. Free H-P: for each literal edge in P_i:
        //     - add p_{i,a}^u h_j^v unless C_i^a is positive literal x_j
        //     - add p_{i,a}^v h_j^u unless C_i^a is negative literal ¬x_j
        //     (1-indexed literal lit -> normalized var index var = |lit|;
        //     0-indexed var_idx = var - 1.)
        for (i, clause) in normalized_clauses.iter().enumerate() {
            for (a, &lit) in clause.iter().enumerate() {
                let var_one_indexed = lit.unsigned_abs() as usize;
                let var_zero_indexed = var_one_indexed - 1;
                let is_positive = lit > 0;
                for j in 0..n {
                    // p_{i,a}^u → h_j^v unless literal is +x_{j+1}.
                    if !is_positive || j != var_zero_indexed {
                        add_edge(p_left(i, a), h_right(j));
                    }
                    // p_{i,a}^v ← h_j^u unless literal is -x_{j+1}.
                    if is_positive || j != var_zero_indexed {
                        add_edge(h_left(j), p_right(i, a));
                    }
                }
            }
        }

        // 12. Free S_1-P: {s_{1,0}^u, s_{1,1}^u} connect to all V(P_i),
        //     {s_{1,0}^v, s_{1,1}^v} connect to all U(P_i).
        for i in 0..m {
            for a in 0..3 {
                add_edge(s_left(0, 0), p_right(i, a));
                add_edge(s_left(0, 1), p_right(i, a));
                add_edge(p_left(i, a), s_right(0, 0));
                add_edge(p_left(i, a), s_right(0, 1));
            }
        }

        // 13. Y matching edges and bisimplicial connections.
        //
        //     Lemma 16 lists `k_f` free-edge bicliques. To make the
        //     paper's accounting check at the construction site, we
        //     enumerate one canonical set of free-edge bicliques B_r^f
        //     and add bisimplicial edges (y_r^u, V(B_r^f)) and
        //     (U(B_r^f), y_r^v). The exact membership of each B_r^f is
        //     irrelevant for soundness — any choice of `k_f` bicliques
        //     that jointly cover the non-Y free edges works.
        let free_bicliques = enumerate_free_bicliques(
            n,
            m,
            ell,
            &normalized_clauses,
            &h_left,
            &h_right,
            &p_left,
            &p_right,
            &s_left,
            &s_right,
            &q_left,
            &q_right,
        );
        debug_assert_eq!(
            free_bicliques.len(),
            k_f,
            "free-edge biclique enumeration must match k_f"
        );
        for (r, biclique) in free_bicliques.iter().enumerate() {
            let yu = y_left(r);
            let yv = y_right(r);
            add_edge(yu, yv);
            for &v_right in &biclique.right {
                add_edge(yu, v_right);
            }
            for &u_left in &biclique.left {
                add_edge(u_left, yv);
            }
        }

        // ---------------- Assemble target ----------------
        let edges_vec: Vec<(usize, usize)> = edges.into_iter().collect();
        let bipartite = BipartiteGraph::new(partition_size, partition_size, edges_vec);
        let target = BicliqueCover::new(bipartite, rank);

        ReductionKSatisfiabilityToBicliqueCover {
            target,
            source_num_vars,
            normalized_n: n,
            s1_left_offset: s_offset,
            s1_right_offset: s_offset,
            y_left_offset: y_offset,
            y_right_offset: y_offset,
            k_f,
        }
    }
}

/// A free-edge biclique listed by Lemma 16 of the paper. Vertices are
/// expressed in bipartite-local indices.
#[derive(Debug, Default)]
struct FreeBiclique {
    left: Vec<usize>,
    right: Vec<usize>,
}

/// Enumerate the `k_f = 4*ell + 2*ceil(log2 m) + 6` free-edge bicliques
/// from Lemma 16, in the following order:
///
/// - 2 H–S bicliques.
/// - `2*ceil(log2 m)` P–P bicliques (binary-encoded clause indices).
/// - 2 P–Q bicliques.
/// - `4*ell` H–P bicliques (bit-wise selection over variable indices).
/// - 2 S_1–P bicliques.
///
/// The exact biclique sets are unused for solution extraction — only
/// the count matters at construction time. Membership is provided so
/// the Y bisimplicial wiring is well-defined.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn enumerate_free_bicliques(
    n: usize,
    m: usize,
    ell: usize,
    normalized_clauses: &[Vec<i32>],
    h_left: &dyn Fn(usize) -> usize,
    h_right: &dyn Fn(usize) -> usize,
    p_left: &dyn Fn(usize, usize) -> usize,
    p_right: &dyn Fn(usize, usize) -> usize,
    s_left: &dyn Fn(usize, usize) -> usize,
    s_right: &dyn Fn(usize, usize) -> usize,
    q_left: &dyn Fn(usize) -> usize,
    q_right: &dyn Fn(usize) -> usize,
) -> Vec<FreeBiclique> {
    let mut out: Vec<FreeBiclique> = Vec::new();

    // (a) H–S: 2 bicliques.
    //  B1 = (∪_j {s_{j,0}^u, s_{j,2}^u}, {h_i^v : i in [n]})
    //  B2 = ({h_i^u : i in [n]}, ∪_j {s_{j,0}^v, s_{j,2}^v})
    {
        let mut b = FreeBiclique::default();
        for j in 0..ell {
            b.left.push(s_left(j, 0));
            b.left.push(s_left(j, 2));
        }
        for i in 0..n {
            b.right.push(h_right(i));
        }
        out.push(b);
    }
    {
        let mut b = FreeBiclique::default();
        for i in 0..n {
            b.left.push(h_left(i));
        }
        for j in 0..ell {
            b.right.push(s_right(j, 0));
            b.right.push(s_right(j, 2));
        }
        out.push(b);
    }

    // (b) P–P: 2 * ceil(log2 m) bicliques.
    //   For each bit b in 0..ceil_log2_m:
    //     B_b^+ = (∪_{i : bit_b(i)=1} U(P_i), ∪_{j : bit_b(j)=0} V(P_j))
    //     B_b^- = (∪_{i : bit_b(i)=0} U(P_i), ∪_{j : bit_b(j)=1} V(P_j))
    //   Each pair covers all (U(P_i), V(P_j)) with i != j.
    let bits_m = ceil_log2(m);
    for bit in 0..bits_m {
        for invert in [false, true] {
            let mut b = FreeBiclique::default();
            for i in 0..m {
                let has_bit = (i >> bit) & 1 == 1;
                if has_bit != invert {
                    for a in 0..3 {
                        b.left.push(p_left(i, a));
                    }
                }
            }
            for j in 0..m {
                let has_bit = (j >> bit) & 1 == 1;
                if has_bit == invert {
                    for a in 0..3 {
                        b.right.push(p_right(j, a));
                    }
                }
            }
            out.push(b);
        }
    }

    // (c) P–Q: 2 bicliques.
    {
        let mut b = FreeBiclique::default();
        for t in 0..2 {
            b.left.push(q_left(t));
        }
        for i in 0..m {
            for a in 0..3 {
                b.right.push(p_right(i, a));
            }
        }
        out.push(b);
    }
    {
        let mut b = FreeBiclique::default();
        for i in 0..m {
            for a in 0..3 {
                b.left.push(p_left(i, a));
            }
        }
        for t in 0..2 {
            b.right.push(q_right(t));
        }
        out.push(b);
    }

    // (d) H–P: 4 * ell bicliques. For each bit b in 0..ell and each
    //     invert in {false, true}, two bicliques (one "left to right",
    //     one "right to left"). Each covers all (p^u, h^v) and
    //     (h^u, p^v) pairs whose variable index differs at bit b from
    //     the literal's omitted variable.
    //
    //     B_b^{u, invert} = ({p_{i,a}^u : positive lit -> var_idx has bit b != invert,
    //                                or negative lit (no constraint here, always include)},
    //                        {h_j^v : bit_b(j) = invert})
    //     B_b^{v, invert} = ({h_j^u : bit_b(j) = invert},
    //                        {p_{i,a}^v : negative lit -> var_idx has bit b != invert,
    //                                or positive lit (no constraint)})
    //
    //     Because we cannot include (p_{i,a}^u, h_{var_idx}^v) for a
    //     positive literal, the biclique excludes that p vertex on
    //     the matching bit. The union over the 2*ell bicliques (one
    //     per (bit, invert)) covers all required (p^u, h^v) free edges.
    for bit in 0..ell {
        for invert in [false, true] {
            // B_b^{u, invert}
            let mut b_u = FreeBiclique::default();
            for j in 0..n {
                let has_bit = (j >> bit) & 1 == 1;
                if has_bit == invert {
                    b_u.right.push(h_right(j));
                }
            }
            for (i, clause) in normalized_clauses.iter().enumerate() {
                for (a, &lit) in clause.iter().enumerate() {
                    let var_idx = lit.unsigned_abs() as usize - 1;
                    let is_positive = lit > 0;
                    // p_{i,a}^u h_j^v omitted only when positive literal
                    // hits j == var_idx. Include p_{i,a}^u in B_u iff
                    // for every j in this biclique's right side
                    // (bit_bit(j) == invert), edge exists. That happens
                    // iff *not* (is_positive && bit_bit(var_idx) == invert).
                    let var_bit_matches = ((var_idx >> bit) & 1 == 1) == invert;
                    let include = !(is_positive && var_bit_matches);
                    if include {
                        b_u.left.push(p_left(i, a));
                    }
                }
            }
            out.push(b_u);

            // B_b^{v, invert}
            let mut b_v = FreeBiclique::default();
            for j in 0..n {
                let has_bit = (j >> bit) & 1 == 1;
                if has_bit == invert {
                    b_v.left.push(h_left(j));
                }
            }
            for (i, clause) in normalized_clauses.iter().enumerate() {
                for (a, &lit) in clause.iter().enumerate() {
                    let var_idx = lit.unsigned_abs() as usize - 1;
                    let is_positive = lit > 0;
                    let var_bit_matches = ((var_idx >> bit) & 1 == 1) == invert;
                    let include = is_positive || !var_bit_matches;
                    if include {
                        b_v.right.push(p_right(i, a));
                    }
                }
            }
            out.push(b_v);
        }
    }

    // (e) S_1–P: 2 bicliques.
    {
        let mut b = FreeBiclique::default();
        b.left.push(s_left(0, 0));
        b.left.push(s_left(0, 1));
        for i in 0..m {
            for a in 0..3 {
                b.right.push(p_right(i, a));
            }
        }
        out.push(b);
    }
    {
        let mut b = FreeBiclique::default();
        for i in 0..m {
            for a in 0..3 {
                b.left.push(p_left(i, a));
            }
        }
        b.right.push(s_right(0, 0));
        b.right.push(s_right(0, 1));
        out.push(b);
    }

    out
}

/// Build a forward witness for the smallest canonical case: 1 source
/// variable, 1 source clause. After normalization the formula has
/// `n = 2`, `ell = 1`, `m = 3` clauses, `k_f = 14`, and rank `= 18`.
///
/// The witness is a vertex-major BicliqueCover configuration with
/// `4` important-edge bicliques (`B_1`, `B̄_1`, `B_1^g`, `B_2^g`)
/// followed by `14` free-edge bicliques `B_r^f ∪ {y_r^u, y_r^v}`
/// that each absorb the matching edge `y_r^u y_r^v` and the
/// bisimplicial wiring around it.
///
/// The construction follows the paper section by section:
///
/// - Assignment `t_1 = true` (`h_0^u ∈ B_1`, `h_1^v ∈ B_1`);
///   `f_1 = false` (`h_1^u ∈ B̄_1`, `h_0^v ∈ B̄_1`).
/// - Selected satisfied literal per clause: slot 0 for `C_0`, slot 0
///   for `C_1`, slot 1 for `C_2`. The remaining two literal edges
///   per clause are absorbed into the two guard bicliques.
/// - The single domino `S_0` is covered by the duplex pair
///   `B_1 = ({s_{0,0}^u, s_{0,1}^u}, {s_{0,0}^v, s_{0,1}^v})` and
///   `B̄_1 = ({s_{0,1}^u, s_{0,2}^u}, {s_{0,1}^v, s_{0,2}^v})`.
/// - `B_1` additionally absorbs the selected literal edges and the
///   crown edges of the satisfying assignment.
/// - The two guard bicliques each cover one `Q` edge and one of the
///   two non-selected literal edges per clause; cross-pairs are P-P
///   and P-Q free edges.
#[cfg(feature = "example-db")]
fn forward_witness_single_variable_single_clause(source: &KSatisfiability<K3>) -> Vec<usize> {
    use crate::traits::Problem;

    let reduction = ReduceTo::<BicliqueCover>::reduce_to(source);
    let target = reduction.target_problem();
    let k = target.k();
    let left_size = target.left_size();
    let num_vertices = target.num_vertices();
    let mut config = vec![0usize; num_vertices * k];
    let _ = target.dims(); // ensure dims matches num_vertices * k

    // Bipartite-local helpers, mirroring `reduce_to`.
    let n = reduction.normalized_n;
    let ell = ceil_log2(n).max(1);
    let m = 3usize; // hard-coded for the canonical case
    let k_f = free_edge_budget(ell, m);
    let h_offset = 0usize;
    let p_offset = h_offset + n;
    let s_offset = p_offset + 3 * m;
    let q_offset = s_offset + 3 * ell;
    let y_offset = q_offset + 2;

    // Unified-vertex coordinates.
    let h_left = |i: usize| h_offset + i;
    let h_right_u = |i: usize| left_size + h_offset + i;
    let p_left = |i: usize, a: usize| p_offset + 3 * i + a;
    let p_right_u = |i: usize, a: usize| left_size + p_offset + 3 * i + a;
    let s_left = |j: usize, a: usize| s_offset + 3 * j + a;
    let s_right_u = |j: usize, a: usize| left_size + s_offset + 3 * j + a;
    let q_left = |t: usize| q_offset + t;
    let q_right_u = |t: usize| left_size + q_offset + t;
    let y_left = |r: usize| y_offset + r;
    let y_right_u = |r: usize| left_size + y_offset + r;

    let mark = |cfg: &mut [usize], vertex: usize, biclique: usize| {
        cfg[vertex * k + biclique] = 1;
    };

    // Biclique 0: B_1 — important.
    // Left: {h_0^u, s_{0,0}^u, s_{0,1}^u, p_{0,0}^u, p_{1,0}^u, p_{2,1}^u}.
    // Right: {h_1^v, s_{0,0}^v, s_{0,1}^v, p_{0,0}^v, p_{1,0}^v, p_{2,1}^v}.
    for v in [
        h_left(0),
        s_left(0, 0),
        s_left(0, 1),
        p_left(0, 0),
        p_left(1, 0),
        p_left(2, 1),
    ] {
        mark(&mut config, v, 0);
    }
    for v in [
        h_right_u(1),
        s_right_u(0, 0),
        s_right_u(0, 1),
        p_right_u(0, 0),
        p_right_u(1, 0),
        p_right_u(2, 1),
    ] {
        mark(&mut config, v, 0);
    }

    // Biclique 1: B̄_1 — important.
    // Left: {h_1^u, s_{0,1}^u, s_{0,2}^u}; Right: {h_0^v, s_{0,1}^v, s_{0,2}^v}.
    for v in [h_left(1), s_left(0, 1), s_left(0, 2)] {
        mark(&mut config, v, 1);
    }
    for v in [h_right_u(0), s_right_u(0, 1), s_right_u(0, 2)] {
        mark(&mut config, v, 1);
    }

    // Biclique 2: B_1^g — guard #1. Covers q_0 + non-selected slot for
    // each clause.
    //   C_0 non-selected slot for B_1^g: 1; C_1: 1; C_2: 0.
    for v in [q_left(0), p_left(0, 1), p_left(1, 1), p_left(2, 0)] {
        mark(&mut config, v, 2);
    }
    for v in [
        q_right_u(0),
        p_right_u(0, 1),
        p_right_u(1, 1),
        p_right_u(2, 0),
    ] {
        mark(&mut config, v, 2);
    }

    // Biclique 3: B_2^g — guard #2. Covers q_1 + last non-selected slot.
    //   C_0 leftover slot: 2; C_1: 2; C_2: 2.
    for v in [q_left(1), p_left(0, 2), p_left(1, 2), p_left(2, 2)] {
        mark(&mut config, v, 3);
    }
    for v in [
        q_right_u(1),
        p_right_u(0, 2),
        p_right_u(1, 2),
        p_right_u(2, 2),
    ] {
        mark(&mut config, v, 3);
    }

    // Bicliques 4..(4+k_f): free-edge bicliques B_r^f ∪ {y_r^u, y_r^v}.
    let (_, normalized_clauses) = normalize(source);
    let free = enumerate_free_bicliques(
        n,
        m,
        ell,
        &normalized_clauses,
        &|i| h_offset + i,
        &|i| h_offset + i,
        &|i, a| p_offset + 3 * i + a,
        &|i, a| p_offset + 3 * i + a,
        &|j, a| s_offset + 3 * j + a,
        &|j, a| s_offset + 3 * j + a,
        &|t| q_offset + t,
        &|t| q_offset + t,
    );
    assert_eq!(free.len(), k_f);
    for (r, biclique) in free.iter().enumerate() {
        let slot = 4 + r;
        // Left: U(B_r^f) ∪ {y_r^u}.
        for &lv in &biclique.left {
            mark(&mut config, lv, slot);
        }
        mark(&mut config, y_left(r), slot);
        // Right: V(B_r^f) ∪ {y_r^v}.
        for &rv in &biclique.right {
            mark(&mut config, left_size + rv, slot);
        }
        mark(&mut config, y_right_u(r), slot);
    }

    config
}

/// Canonical example for the KSatisfiability/K3 → BicliqueCover rule.
///
/// Uses the smallest possible source: one variable `x_1` and one clause
/// `(x_1 ∨ x_1 ∨ x_1)`. After normalization the gadget has rank `18`
/// and ~`1188` binary variables, so solving the target by brute force
/// is out of reach. The witness is constructed by hand, following the
/// paper's Lemma 16/17 free-edge decomposition and a direct
/// `(B_1, B̄_1)` duplex on the single domino `S_0`.
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_bicliquecover",
        build: || {
            let source = KSatisfiability::<K3>::new(1, vec![CNFClause::new(vec![1, 1, 1])]);
            let target_config = forward_witness_single_variable_single_clause(&source);
            crate::example_db::specs::rule_example_with_witness::<_, BicliqueCover>(
                source,
                SolutionPair {
                    source_config: vec![1usize], // x_1 = true
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_bicliquecover.rs"]
mod tests;
