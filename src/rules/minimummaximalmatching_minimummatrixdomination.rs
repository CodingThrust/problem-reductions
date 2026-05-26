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
//! Yannakakis and Gavril (1980) prove that any edge dominating set can be
//! transformed in polynomial time into an independent edge dominating set
//! (a maximal matching) of the same size. We implement this conversion by a
//! direct search: enumerate maximal matchings of `B` and return one of size at
//! most `|EDS|`. Because every minimum maximal matching is also an EDS and the
//! two minima are equal, such a matching always exists when the target witness
//! is optimal.
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
    /// matrix-domination witness.
    ///
    /// The target witness identifies a set of 1-entries of `M`. Each selected
    /// 1-entry in the upper-right block `B*` corresponds bijectively to a
    /// source edge, so the selection induces an edge set `D` of `B` that is an
    /// edge dominating set. The minimum edge dominating set size of a graph
    /// equals the minimum maximal matching size [Yannakakis-Gavril 1980], so
    /// any optimal target witness yields `|D|` equal to `mm(B)`. We then
    /// recover a maximal matching `M` of `B` with `|M| <= |D|` by enumerating
    /// candidate source configurations.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let num_source_edges = self.source.graph().num_edges();
        let target_ones = self.target.ones();
        let bound: usize = target_solution
            .iter()
            .zip(target_ones.iter())
            .filter(|(&sel, _)| sel == 1)
            .count();

        // Search for any maximal matching of B with cardinality at most `bound`.
        // For an optimal target witness, |D| = mm(B), so such a matching
        // exists by Yannakakis-Gavril (1980). For canonical example sizes this
        // enumeration is fast; in the worst case it is 2^|E| which mirrors the
        // brute-force solve used elsewhere in the test infrastructure.
        //
        // Iterate by size from 0 upward so we always return a smallest-known
        // maximal matching.
        for target_size in 0..=bound {
            for mask in 0u64..(1u64 << num_source_edges) {
                if mask.count_ones() as usize != target_size {
                    continue;
                }
                let config: Vec<usize> = (0..num_source_edges)
                    .map(|i| ((mask >> i) & 1) as usize)
                    .collect();
                if self.source.is_valid_maximal_matching(&config) {
                    return config;
                }
            }
        }

        // Fallback: a zero configuration. This branch is unreachable when the
        // reduction is correct and the supplied target witness is feasible.
        vec![0; num_source_edges]
    }
}

#[reduction(
    overhead = {
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
