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
use crate::rules::traits::{recover_preserving_status, ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;
use crate::topology::{BipartiteGraph, Graph};

/// Result of reducing `MinimumMaximalMatching<BipartiteGraph>` to
/// `MinimumMatrixDomination`.
///
/// Holds the constructed target matrix-domination instance together with a copy
/// of the source bipartite-matching problem. The source copy is used by
/// `recover_result` to perform the Yannakakis-Gavril conversion from an edge
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
    /// - **Swap:** otherwise, some edge `(u, x)` of `B` is currently dominated
    ///   only by `e1`. This `x` must lie outside `V(D \ {e1})` and is
    ///   therefore distinct from `w`, so `(u, x)` is not adjacent to `e2`.
    ///   Replace `e1` with `(u, x)`: `D := (D \ {e1}) \cup {(u, x)}`. Size is
    ///   preserved and the adjacent pair at `v` is resolved.
    ///
    /// Each iteration strictly decreases either `|D|` or the number of
    /// adjacent pairs, so the loop terminates in `O(|F|^2)` iterations. Each
    /// iteration scans `O(|F|)` edges to find an adjacent pair and an
    /// undominated edge, for a total of `O(|F|^3)` time. The result is a
    /// matching that is an EDS, i.e. an independent EDS, which is precisely a
    /// maximal matching.
    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        recover_preserving_status(source, target, |solution| self.map_solution(solution))
    }
}

impl ReductionMMMToMatrixDomination {
    fn map_solution(
        &self,
        target_solution: &<<Self as ReductionResult>::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<
        <<Self as ReductionResult>::Source as crate::traits::Problem>::Solution,
    > {
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
                .zip(target_ones)
                .filter(|(selected, _)| **selected)
                .map(|(_, cell)| cell_to_source_edge[cell])
                .collect();

            // Remove one of two adjacent selected edges. Any edge no longer
            // dominated is private to its non-shared endpoint; replacing the
            // removed edge with it preserves domination and reduces adjacency.
            while let Some(position) = find_adjacent_pair(&d, &edges) {
                let mut remaining = d.clone();
                remaining.swap_remove(position);
                let covered: std::collections::HashSet<_> = remaining
                    .iter()
                    .flat_map(|&edge| [edges[edge].0, edges[edge].1])
                    .collect();
                match edges
                    .iter()
                    .position(|&(u, v)| !covered.contains(&u) && !covered.contains(&v))
                {
                    Some(edge) => d[position] = edge,
                    None => d = remaining,
                }
            }

            // Step 3: encode the matching as a binary configuration over source edges.
            let mut config = vec![false; num_source_edges];
            for &idx in &d {
                config[idx] = true;
            }
            config
        })
    }
}

/// Find the position of a selected edge adjacent to another selected edge.
fn find_adjacent_pair(d: &[usize], edges: &[(usize, usize)]) -> Option<usize> {
    let mut incident = std::collections::HashMap::new();
    for (position, &edge) in d.iter().enumerate() {
        for vertex in [edges[edge].0, edges[edge].1] {
            if let Some(previous) = incident.insert(vertex, position) {
                return Some(previous);
            }
        }
    }
    None
}

#[reduction(
    transform = exact {
        num_rows = "num_vertices",
        num_cols = "num_vertices",
        num_ones = "num_edges",
    }
)]
impl ReduceTo<MinimumMatrixDomination> for MinimumMaximalMatching<BipartiteGraph> {
    type Result = ReductionMMMToMatrixDomination;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
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

        Ok(ReductionMMMToMatrixDomination {
            target,
            source: self.clone(),
        })
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
                    source_config: serde_json::json!(vec![true, false, false, true, false]),
                    target_config: serde_json::json!(vec![true, false, false, true, false]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimummaximalmatching_minimummatrixdomination.rs"]
mod tests;
