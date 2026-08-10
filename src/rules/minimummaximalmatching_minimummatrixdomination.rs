//! Reduction from MinimumMaximalMatching (on a bipartite graph) to
//! MinimumMatrixDomination.
//!
//! Classical reduction of Yannakakis and Gavril (1980) establishing
//! NP-completeness of MATRIX DOMINATION (Garey & Johnson MS12). For a bipartite
//! graph `B = (L, R, F)` with `|L| = m` and `|R| = n`, construct the `N x N`
//! binary matrix `M` (with `N = m + n`) whose upper-right `m x n` block is the
//! biadjacency matrix `B*` of `B` and whose remaining entries are zero. The
//! 1-entries of `M` are in bijection with the edges of `B`, and two 1-entries
//! share a row or column iff the corresponding edges share an endpoint. Hence a
//! dominating set of 1-entries in `M` corresponds to an edge dominating set of
//! `B`, and by Yannakakis and Gavril (1980), the minimum edge dominating set
//! size equals the minimum maximal matching size.
//!
//! ## Witness extraction
//!
//! Solving Minimum Matrix Domination on the constructed instance yields a
//! minimum edge dominating set of `B`, which is in general NOT a matching.
//! Yannakakis and Gavril (1980) prove that any edge dominating set `D` can be
//! transformed in polynomial time into an independent edge dominating set
//! (a maximal matching) `M` of the same or smaller size. We implement this
//! polynomial transformation directly: repeatedly resolve adjacent pairs in
//! `D` by either dropping a redundant edge (when its endpoint is already
//! dominated by `D \ {e}`) or swapping it for an edge whose new endpoint lies
//! outside the current vertex cover. The procedure runs in `O(|F|^3)` worst
//! case and never enumerates configurations.
//!
//! ## Source variant
//!
//! The reduction requires the bipartite (`BipartiteGraph`) variant of
//! `MinimumMaximalMatching`. The biadjacency matrix faithfully represents the
//! edge structure of a bipartite graph (each edge -> exactly one 1-entry),
//! whereas an undirected adjacency matrix would produce two symmetric 1-entries
//! per edge that do not preserve the row/column sharing pattern.

use crate::models::algebraic::MinimumMatrixDomination;
use crate::models::graph::MinimumMaximalMatching;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{BipartiteGraph, Graph};

/// Result of reducing `MinimumMaximalMatching<BipartiteGraph>` to
/// `MinimumMatrixDomination`.
///
/// Holds the constructed target matrix-domination instance together with a copy
/// of the source bipartite-matching problem. The source copy is used by
/// `extract_solution` to perform the Yannakakis-Gavril conversion from an edge
/// dominating set to an equally-sized maximal matching.
#[derive(Debug, Clone)]
pub struct ReductionMMMToMatrixDomination {
    target: MinimumMatrixDomination,
    source: MinimumMaximalMatching<BipartiteGraph>,
}

impl ReductionResult for ReductionMMMToMatrixDomination {
    type Source = MinimumMaximalMatching<BipartiteGraph>;
    type Target = MinimumMatrixDomination;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a maximal matching of the source bipartite graph from a
    /// matrix-domination witness via the Yannakakis-Gavril (1980) polynomial
    /// EDS-to-IEDS transformation.
    ///
    /// The target witness identifies a set of 1-entries of `M`. Each selected
    /// 1-entry in the upper-right block `B*` corresponds bijectively to a
    /// source edge, so the selection induces an edge set `D` of `B` that is an
    /// edge dominating set (EDS). Arbitrary optimal MMD witnesses may select
    /// 1-entries whose corresponding source edges form a connected subgraph
    /// rather than a matching (e.g. two edges sharing a left endpoint), so
    /// `D` is not in general independent.
    ///
    /// The Yannakakis-Gavril transformation (Theorem 1 of @yannakakis1980)
    /// converts any EDS into an independent EDS (a maximal matching) of the
    /// same or smaller size by repeatedly applying one of the following
    /// reductions while `D` contains two adjacent edges `e1 = (u, v)` and
    /// `e2 = (v, w)`:
    ///
    /// - **Drop:** if every edge of `B` incident to `u` is already dominated
    ///   by `D \ {e1}`, set `D := D \ {e1}` (size strictly decreases).
    ///   Symmetric for `w` and `e2`.
    /// - **Swap:** otherwise, some edge `(u, x)` of `B` is currently dominated
    ///   only by `e1`. This `x` must lie outside `V(D \ {e1})` and is
    ///   therefore distinct from `w`, so `(u, x)` is not adjacent to `e2`.
    ///   Replace `e1` with `(u, x)`: `D := (D \ {e1}) \cup {(u, x)}`. Size is
    ///   preserved and the adjacent pair at `v` is resolved.
    ///
    /// Each iteration strictly decreases either `|D|` or the number of
    /// adjacent pairs, so the loop terminates in `O(|F|^2)` iterations. Each
    /// iteration scans `O(|F|)` edges to find an adjacent pair, an EDS check,
    /// and a swap candidate, for a total of `O(|F|^3)` time. The result is a
    /// matching that is an EDS, i.e. an independent EDS, which is precisely a
    /// maximal matching.
    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let graph = self.source.graph();
            let edges = graph.edges();
            let num_source_edges = edges.len();
            let m = graph.left_size();
            let target_ones = self.target.ones();

            // Step 1: map selected target 1-entries back to source edge indices.
            // The reduction places source edge `(l_i, r_j)` (in bipartite-local
            // form) at matrix cell `(i, m + j)`, which equals the global edge
            // `(i, m + j)` returned by `Graph::edges()`. Build the lookup from
            // matrix cell -> source edge index so we are robust to any ordering
            // discrepancy between `Graph::edges()` and row-major 1-entries.
            let cell_to_source_edge: std::collections::HashMap<(usize, usize), usize> = edges
                .iter()
                .enumerate()
                .map(|(idx, &(u, v))| {
                    // Source edge endpoints in bipartite global coords are
                    // (left_idx, m + right_idx); matrix cell is (row=left, col=m+right).
                    let (row, col) = if u < m { (u, v) } else { (v, u) };
                    ((row, col), idx)
                })
                .collect();
            let mut d: Vec<usize> = target_solution
                .iter()
                .zip(target_ones.iter())
                .filter_map(|(&sel, &cell)| {
                    if sel == 1 {
                        Some(cell_to_source_edge.get(&cell).copied().ok_or_else(|| {
                            crate::rules::ExtractionError::invalid(format!(
                                "selected matrix cell {cell:?} has no source edge"
                            ))
                        }))
                    } else {
                        None
                    }
                })
                .collect::<crate::rules::ExtractionResult<_>>()?;

            // Step 2: Yannakakis-Gavril EDS -> independent EDS (maximal matching).
            // Loop invariants: `d` is an EDS of the source graph; each iteration
            // strictly decreases either |d| or the number of (unordered) pairs of
            // adjacent edges inside `d`.
            loop {
                // Find an adjacent pair (e1_idx, e2_idx) inside `d`, sharing vertex v.
                let pair = find_adjacent_pair(&d, &edges);
                let Some((e1_idx, e2_idx, _shared)) = pair else {
                    break; // `d` is a matching; we are done.
                };

                // Try dropping e1_idx or e2_idx if the remainder is still an EDS.
                let mut without_e1 = d.clone();
                let e1_position = d.iter().position(|&x| x == e1_idx).ok_or_else(|| {
                    crate::rules::ExtractionError::invalid(
                        "edge-domination transformation lost its selected edge",
                    )
                })?;
                without_e1.swap_remove(e1_position);
                if is_edge_dominating_set(&without_e1, &edges) {
                    d = without_e1;
                    continue;
                }
                let mut without_e2 = d.clone();
                let e2_position = d.iter().position(|&x| x == e2_idx).ok_or_else(|| {
                    crate::rules::ExtractionError::invalid(
                        "edge-domination transformation lost its selected edge",
                    )
                })?;
                without_e2.swap_remove(e2_position);
                if is_edge_dominating_set(&without_e2, &edges) {
                    d = without_e2;
                    continue;
                }

                // Neither drop works -> perform a swap on one of e1 or e2.
                // Choose endpoint not shared with the other edge: for e1=(u, v),
                // e2=(v, w), the "non-shared" endpoint of e1 is u.
                let (e1_a, e1_b) = edges[e1_idx];
                let (e2_a, e2_b) = edges[e2_idx];
                let shared = if e1_a == e2_a || e1_a == e2_b {
                    e1_a
                } else {
                    e1_b
                };
                let u = if e1_a == shared { e1_b } else { e1_a };
                let w = if e2_a == shared { e2_b } else { e2_a };

                // Try to swap e1 := (u, x) where x ∉ V(d \ {e1}). The YG proof
                // guarantees such x exists when neither drop succeeded.
                if let Some(new_idx) = find_swap_edge(u, e1_idx, &d, &edges) {
                    d[e1_position] = new_idx;
                    continue;
                }
                // Symmetric swap on e2.
                if let Some(new_idx) = find_swap_edge(w, e2_idx, &d, &edges) {
                    d[e2_position] = new_idx;
                    continue;
                }

                // YG guarantees that for an EDS at least one of the four moves
                // above succeeds. Reaching this point implies the input was not
                // a valid EDS (i.e., not a feasible MMD witness on the constructed
                // instance), which violates the reduction's precondition.
                return Err(crate::rules::ExtractionError::invalid(
                    "target matrix entries do not encode an edge-dominating set",
                ));
            }

            // Step 3: encode the matching as a binary configuration over source edges.
            let mut config = vec![0usize; num_source_edges];
            for &idx in &d {
                config[idx] = 1;
            }
            config
        })
    }
}

/// Return `Some((i, j, v))` where `i`, `j` are indices in `d` of two edges that
/// share vertex `v`, or `None` if all edges in `d` are pairwise independent.
fn find_adjacent_pair(d: &[usize], edges: &[(usize, usize)]) -> Option<(usize, usize, usize)> {
    for (a_pos, &i) in d.iter().enumerate() {
        let (iu, iv) = edges[i];
        for &j in &d[a_pos + 1..] {
            let (ju, jv) = edges[j];
            if iu == ju || iu == jv {
                return Some((i, j, iu));
            }
            if iv == ju || iv == jv {
                return Some((i, j, iv));
            }
        }
    }
    None
}

/// Check whether the edge set `d` (indices into `edges`) dominates every edge
/// of `edges`. An edge `f` is dominated iff `f ∈ d` or `f` shares an endpoint
/// with some edge in `d`.
fn is_edge_dominating_set(d: &[usize], edges: &[(usize, usize)]) -> bool {
    // Vertex cover of the candidate EDS.
    let mut covered_vertices: std::collections::HashSet<usize> = std::collections::HashSet::new();
    for &i in d {
        let (u, v) = edges[i];
        covered_vertices.insert(u);
        covered_vertices.insert(v);
    }
    edges.iter().enumerate().all(|(f_idx, (u, v))| {
        d.contains(&f_idx) || covered_vertices.contains(u) || covered_vertices.contains(v)
    })
}

/// Find an edge index in `edges` that is (i) incident to vertex `endpoint`,
/// (ii) different from `excluded_idx`, and (iii) whose other endpoint lies
/// outside `V(d \ {excluded_idx})`.
///
/// This is the swap candidate `(u, x)` from the Yannakakis-Gavril argument
/// when the drop move is not available for `excluded_idx`.
fn find_swap_edge(
    endpoint: usize,
    excluded_idx: usize,
    d: &[usize],
    edges: &[(usize, usize)],
) -> Option<usize> {
    // Vertex cover of d \ {excluded_idx}.
    let mut other_cover: std::collections::HashSet<usize> = std::collections::HashSet::new();
    for &i in d {
        if i == excluded_idx {
            continue;
        }
        let (u, v) = edges[i];
        other_cover.insert(u);
        other_cover.insert(v);
    }
    for (k, &(u, v)) in edges.iter().enumerate() {
        if k == excluded_idx {
            continue;
        }
        let (e_endpoint, other) = if u == endpoint {
            (u, v)
        } else if v == endpoint {
            (v, u)
        } else {
            continue;
        };
        debug_assert_eq!(e_endpoint, endpoint);
        if !other_cover.contains(&other) {
            return Some(k);
        }
    }
    None
}

#[reduction(
    exact = {
        num_rows = "num_vertices",
        num_cols = "num_vertices",
        num_ones = "num_edges",
    }
)]
impl ReduceTo<MinimumMatrixDomination> for MinimumMaximalMatching<BipartiteGraph> {
    type Result = ReductionMMMToMatrixDomination;

    fn reduce_to(&self) -> Self::Result {
        let g = self.graph();
        let m = g.left_size();
        let n = g.right_size();
        let big_n = m + n;

        // Build the N x N matrix:
        //   upper-right m x n block = biadjacency matrix B*
        //   all other entries = 0
        // The matrix is upper triangular: 1-entries lie strictly in rows
        // 0..m and columns m..m+n.
        let mut matrix = vec![vec![false; big_n]; big_n];
        for &(left_idx, right_idx) in g.left_edges() {
            // Row = l_left_idx (in 0..m), Column = m + right_idx (in m..m+n).
            matrix[left_idx][m + right_idx] = true;
        }

        let target = MinimumMatrixDomination::new(matrix);

        ReductionMMMToMatrixDomination {
            target,
            source: self.clone(),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimummaximalmatching_to_minimummatrixdomination",
        build: || {
            // Canonical YES instance from the issue.
            //
            // Bipartite graph B with L = {l0, l1}, R = {r0, r1, r2} and edges
            // F = {(l0, r0), (l0, r1), (l0, r2), (l1, r1), (l1, r2)}.
            //
            // Source edge indices (in BipartiteGraph::edges() order):
            //   0: (l0, r0) = (0, 0)
            //   1: (l0, r1) = (0, 1)
            //   2: (l0, r2) = (0, 2)
            //   3: (l1, r1) = (1, 1)
            //   4: (l1, r2) = (1, 2)
            //
            // mm(B) = 2; one optimum is M = {(l0, r0), (l1, r1)} ->
            // source_config = [1, 0, 0, 1, 0].
            //
            // Constructed N x N matrix with N = 5; 1-entries in row-major
            // order (matching the source edge order above):
            //   idx 0: (0, 2)  <- (l0, r0)
            //   idx 1: (0, 3)  <- (l0, r1)
            //   idx 2: (0, 4)  <- (l0, r2)
            //   idx 3: (1, 3)  <- (l1, r1)
            //   idx 4: (1, 4)  <- (l1, r2)
            //
            // Selecting target_config = [1, 0, 0, 1, 0] picks 1-entries
            // {(0, 2), (1, 3)}, which together dominate every other 1-entry by
            // shared row 0 or row 1.
            let source = MinimumMaximalMatching::new(BipartiteGraph::new(
                2,
                3,
                vec![(0, 0), (0, 1), (0, 2), (1, 1), (1, 2)],
            ));
            crate::example_db::specs::rule_example_with_witness::<_, MinimumMatrixDomination>(
                source,
                SolutionPair {
                    source_config: vec![1, 0, 0, 1, 0],
                    target_config: vec![1, 0, 0, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimummaximalmatching_minimummatrixdomination.rs"]
mod tests;
