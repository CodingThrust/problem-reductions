//! Reduction from BicliqueCover to BMF (exact Boolean Matrix Factorization).
//!
//! Inverse of [`crate::rules::bmf_bicliquecover`]: a bipartite graph
//! `G = (L, R, E)` is encoded by its `|L| × |R|` biadjacency matrix
//! `A_G` with `A_G[i][j] = 1` iff `(i, j) ∈ E`. Under the classical
//! sub-biclique semantics, a biclique cover of `G` by `k` sub-bicliques
//! is exactly an exact rank-`k` Boolean factorization of `A_G`, and the
//! total biclique size equals `|B|_1 + |C|_1`.
//!
//! Layout mapping is the inverse transpose from the forward reduction —
//! the shared helpers [`super::bmf_bicliquecover::config_bmf_to_bc`] and
//! `config_bc_to_bmf` live in the sibling module.

use crate::models::algebraic::BMF;
use crate::models::graph::BicliqueCover;
use crate::reduction;
use crate::rules::bmf_bicliquecover::config_bmf_to_bc;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing BicliqueCover to BMF.
#[derive(Debug, Clone)]
pub struct ReductionBicliqueCoverToBMF {
    target: BMF,
    m: usize,
    n: usize,
    k: usize,
}

impl ReductionResult for ReductionBicliqueCoverToBMF {
    type Source = BicliqueCover;
    type Target = BMF;

    fn target_problem(&self) -> &BMF {
        &self.target
    }

    /// Map a BMF config (B row-major, C row-major) to a BicliqueCover
    /// config (vertex-major) via the inverse transpose.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        config_bmf_to_bc(target_solution, self.m, self.n, self.k)
    }
}

#[reduction(
    overhead = {
        rows = "left_size",
        cols = "right_size",
        rank = "rank",
    }
)]
impl ReduceTo<BMF> for BicliqueCover {
    type Result = ReductionBicliqueCoverToBMF;

    fn reduce_to(&self) -> Self::Result {
        let m = self.left_size();
        let n = self.right_size();
        let k = self.k();
        let mut matrix = vec![vec![false; n]; m];
        for &(i, j) in self.graph().left_edges() {
            matrix[i][j] = true;
        }
        let target = BMF::new(matrix, k);
        ReductionBicliqueCoverToBMF { target, m, n, k }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::topology::BipartiteGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "bicliquecover_to_bmf",
        build: || {
            // Single K_{2,2} biclique at rank 1 — matches the forward example.
            let source = BicliqueCover::new(
                BipartiteGraph::new(2, 2, vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
                1,
            );
            crate::example_db::specs::rule_example_with_witness::<_, BMF>(
                source,
                SolutionPair {
                    // BicliqueCover (vertex-major, k=1): all 4 vertices in biclique 0
                    source_config: vec![1, 1, 1, 1],
                    // BMF (B row-major then C row-major): B=[[1],[1]], C=[[1,1]]
                    target_config: vec![1, 1, 1, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/bicliquecover_bmf.rs"]
mod tests;
