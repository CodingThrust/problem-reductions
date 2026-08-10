//! Reduction from MaxCut to MinimumMatrixCover.
//!
//! Given a MaxCut instance `(G, w)` with nonnegative integer edge weights,
//! the target matrix `A` is the symmetric weighted adjacency matrix of `G`:
//!     `a_{ij} = w({i, j})` if `{i, j} ∈ E`, otherwise `0` (diagonal is `0`).
//!
//! The key identity is
//!     `Σ_{i,j} a_{ij} · f(i) · f(j) = 2 W − 4 · cut(S)`,
//! where `S = { i : f(i) = +1 }` and `W = Σ_{e ∈ E} w(e)`. Minimizing the
//! quadratic form is therefore equivalent to maximizing the cut, and the
//! reduction is witness-preserving via the identity map between MaxCut's
//! partition encoding (`config[i] = 1 ⇔ i ∈ S`) and MinimumMatrixCover's
//! sign encoding (`config[i] = 1 ⇔ f(i) = +1 ⇔ i ∈ S`).
//!
//! **Precondition:** all edge weights must be nonnegative. The reduction
//! panics on any negative weight, since `MinimumMatrixCover` requires a
//! nonnegative integer matrix. Negative-weight `MaxCut` instances are out
//! of scope and must use a different (preprocessing) reduction.
//!
//! Reference: Garey & Johnson, *Computers and Intractability* (1979),
//! Appendix A1.2, MS13 ("Transformation from MAXIMUM CUT").

use crate::models::algebraic::MinimumMatrixCover;
use crate::models::graph::MaxCut;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MaxCut to MinimumMatrixCover.
#[derive(Debug, Clone)]
pub struct ReductionMaxCutToMMC {
    target: MinimumMatrixCover,
}

impl ReductionResult for ReductionMaxCutToMMC {
    type Source = MaxCut<SimpleGraph, i32>;
    type Target = MinimumMatrixCover;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction is the identity.
    ///
    /// Both encodings agree that bit `i = 1` means vertex `i` is in `S`:
    /// MaxCut treats `config[i] = 1` as one side of the partition, and
    /// MinimumMatrixCover treats `config[i] = 1` as `f(i) = +1`, i.e.,
    /// vertex `i` in `S`. The complementary assignment is equally optimal
    /// because the quadratic form (and the cut) is invariant under
    /// `f -> -f`.
    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.to_vec())
    }
}

#[reduction(
    exact = {
        num_rows = "num_vertices",
    }
)]
impl ReduceTo<MinimumMatrixCover> for MaxCut<SimpleGraph, i32> {
    type Result = ReductionMaxCutToMMC;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();
        let mut matrix: Vec<Vec<i64>> = vec![vec![0i64; n]; n];

        for (u, v, w) in self.edges() {
            assert!(
                w >= 0,
                "MaxCut -> MinimumMatrixCover requires nonnegative edge weights, got w({u},{v}) = {w}"
            );
            let w64 = w as i64;
            matrix[u][v] = w64;
            matrix[v][u] = w64;
        }

        ReductionMaxCutToMMC {
            target: MinimumMatrixCover::new(matrix),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maxcut_to_minimummatrixcover",
        build: || {
            // Canonical example: C_4 (4-cycle) with unit weights.
            // W = 4, max cut = 4 (partition {0,2} vs {1,3} cuts all edges).
            // The target's minimum quadratic form value is 2W - 4 * cut = 8 - 16 = -8.
            let source = MaxCut::<SimpleGraph, i32>::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]),
                vec![1, 1, 1, 1],
            );
            crate::example_db::specs::rule_example_with_witness::<_, MinimumMatrixCover>(
                source,
                SolutionPair {
                    // S = {0, 2}: vertices 0 and 2 have f = +1.
                    source_config: vec![1, 0, 1, 0],
                    target_config: vec![1, 0, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maxcut_minimummatrixcover.rs"]
mod tests;
