//! Reduction from BMF (exact Boolean Matrix Factorization) to BicliqueCover.
//!
//! Classical equivalence (Monson, Pullman, Rees 1995): an m x n boolean
//! matrix `A` is the biadjacency matrix of the bipartite graph `G_A`, and
//! each rank-1 factor of `A = B ⊙ C` is exactly a (complete) biclique of
//! `G_A` — its left side is `{i : B[i][r] = 1}` and its right side is
//! `{j : C[r][j] = 1}`. Hence an exact rank-`k` factorization corresponds
//! to a cover of `E(G_A)` by `k` sub-bicliques of `G_A`, and the total
//! factor weight `|B|_1 + |C|_1` equals the total biclique size (the
//! number of vertex memberships summed over all bicliques).
//!
//! Variable-layout mapping: BMF stores `B` row-major followed by `C`
//! row-major, while BicliqueCover stores vertex memberships vertex-major.
//! `extract_solution` transposes the right-vertex half so the extracted
//! BMF config matches `B` and `C`.

use crate::models::algebraic::BMF;
use crate::models::graph::BicliqueCover;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::BipartiteGraph;

/// Result of reducing BMF to BicliqueCover.
#[derive(Debug, Clone)]
pub struct ReductionBMFToBicliqueCover {
    target: BicliqueCover,
    m: usize,
    n: usize,
    k: usize,
}

impl ReductionResult for ReductionBMFToBicliqueCover {
    type Source = BMF;
    type Target = BicliqueCover;

    fn target_problem(&self) -> &BicliqueCover {
        &self.target
    }

    /// Map a BicliqueCover config (vertex-major) back to a BMF config (B row-major, then C row-major).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let m = self.m;
        let n = self.n;
        let k = self.k;
        let mut source = vec![0usize; m * k + k * n];
        // Left half: identical layout — BicliqueCover left vertex i, biclique l at index i*k + l
        // matches BMF B[i][l] at index i*k + l.
        for i in 0..m {
            for l in 0..k {
                source[i * k + l] = target_solution[i * k + l];
            }
        }
        // Right half: transpose from vertex-major to biclique-row-major.
        // BicliqueCover right vertex j, biclique l at index (m + j)*k + l.
        // BMF C[l][j] at index m*k + l*n + j.
        for l in 0..k {
            for j in 0..n {
                source[m * k + l * n + j] = target_solution[(m + j) * k + l];
            }
        }
        source
    }
}

#[reduction(
    overhead = {
        num_vertices = "rows + cols",
        num_edges = "rows * cols",
        rank = "rank",
    }
)]
impl ReduceTo<BicliqueCover> for BMF {
    type Result = ReductionBMFToBicliqueCover;

    fn reduce_to(&self) -> Self::Result {
        let m = self.rows();
        let n = self.cols();
        let k = self.rank();
        let mut edges = Vec::new();
        for (i, row) in self.matrix().iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if val {
                    edges.push((i, j));
                }
            }
        }
        let target = BicliqueCover::new(BipartiteGraph::new(m, n, edges), k);
        ReductionBMFToBicliqueCover { target, m, n, k }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "bmf_to_bicliquecover",
        build: || {
            // 2x2 all-ones, rank 1 — a single biclique covering both sides exactly.
            let source = BMF::new(vec![vec![true, true], vec![true, true]], 1);
            crate::example_db::specs::rule_example_with_witness::<_, BicliqueCover>(
                source,
                SolutionPair {
                    // BMF config (B row-major, C row-major): B = [[1],[1]], C = [[1,1]]
                    source_config: vec![1, 1, 1, 1],
                    // BicliqueCover config (vertex-major, k=1): v0, v1 (left), v2, v3 (right) all in biclique 0
                    target_config: vec![1, 1, 1, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/bmf_bicliquecover.rs"]
mod tests;
