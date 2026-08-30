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

/// Convert one vertex-membership row per biclique into BMF factors.
pub(crate) fn config_bc_to_bmf(
    bc: &[Vec<bool>],
    m: usize,
    n: usize,
    k: usize,
) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
    let mut b = vec![vec![false; k]; m];
    let mut c = vec![vec![false; n]; k];
    for i in 0..m {
        for l in 0..k {
            b[i][l] = bc[l][i];
        }
    }
    for l in 0..k {
        for j in 0..n {
            c[l][j] = bc[l][m + j];
        }
    }
    (b, c)
}

/// Inverse of [`config_bc_to_bmf`].
pub(crate) fn config_bmf_to_bc(
    bmf: &(Vec<Vec<bool>>, Vec<Vec<bool>>),
    m: usize,
    n: usize,
    k: usize,
) -> Vec<Vec<bool>> {
    let (b, c) = bmf;
    let mut bc = vec![vec![false; m + n]; k];
    for i in 0..m {
        for l in 0..k {
            bc[l][i] = b[i][l];
        }
    }
    for l in 0..k {
        for j in 0..n {
            bc[l][m + j] = c[l][j];
        }
    }
    bc
}

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
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(config_bc_to_bmf(target_solution, self.m, self.n, self.k))
    }
}

#[reduction(
    transform = exact {
        num_vertices = "rows + cols",
        num_edges = "rows * cols",
        rank = "rank",
    },
    unavailable = {
        left_size = "the exact target parameter is not represented by this reduction's symbolic transform",
        right_size = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<BicliqueCover> for BMF {
    type Result = ReductionBMFToBicliqueCover;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
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
        Ok(ReductionBMFToBicliqueCover { target, m, n, k })
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
                    source_config: serde_json::json!((
                        vec![vec![true], vec![true]],
                        vec![vec![true, true]]
                    )),
                    target_config: serde_json::json!(vec![vec![true, true, true, true]]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/bmf_bicliquecover.rs"]
mod tests;
