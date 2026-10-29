//! Reduction from MaxCut to MinimumMatrixCover.
//!
//! For nonnegative edge weights, the target matrix `A` is the symmetric
//! weighted adjacency matrix of `G`:
//!     `a_{ij} = w({i, j})` if `{i, j} ∈ E`, otherwise `0` (diagonal is `0`).
//!
//! The key identity is
//!     `Σ_{i,j} a_{ij} · f(i) · f(j) = 2 W − 4 · cut(S)`,
//! where `S = { i : f(i) = +1 }` and `W = Σ_{e ∈ E} w(e)`. Minimizing the
//! quadratic form is therefore equivalent to maximizing the cut, and the
//! original vertices keep their bits between MaxCut's
//! partition encoding (`config[i] = 1 ⇔ i ∈ S`) and MinimumMatrixCover's
//! sign encoding (`config[i] = 1 ⇔ f(i) = +1 ⇔ i ∈ S`).
//!
//! `MinimumMatrixCover` requires a nonnegative matrix, so each negative edge
//! `{u, v}` with weight `w < 0` becomes an auxiliary vertex `t` joined to `v`
//! by weight `|w| + 1` and to `u` by weight `|w|`. Moving `t` to the side
//! opposite `v` gains `|w| + 1` and loses at most `|w|`, so every maximum cut
//! separates `t` from `v`. Then `t` is cut from `u` exactly when `u` and `v`
//! share a side, and `|w| · [u, v same side] = w · [u, v cut] + |w|`: each
//! gadget shifts the cut by the constant `2|w| + 1`. Maximum cuts, and so
//! optimal sign assignments, correspond after dropping auxiliary vertices.
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
    num_source_vertices: usize,
}

impl ReductionResult for ReductionMaxCutToMMC {
    type Source = MaxCut<SimpleGraph, i64>;
    type Target = MinimumMatrixCover;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction keeps the source vertices' bits.
    ///
    /// Both encodings agree that bit `i = 1` means vertex `i` is in `S`:
    /// MaxCut treats `config[i] = 1` as one side of the partition, and
    /// MinimumMatrixCover treats `config[i] = 1` as `f(i) = +1`, i.e.,
    /// vertex `i` in `S`. The complementary assignment is equally optimal
    /// because the quadratic form (and the cut) is invariant under
    /// `f -> -f`.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution[..self.num_source_vertices].to_vec())
    }
}

#[reduction(
    transform = upper_bound {
        // One auxiliary row per negative-weight edge.
        num_rows = "num_vertices + num_edges",
    }
)]
impl ReduceTo<MinimumMatrixCover> for MaxCut<SimpleGraph, i64> {
    type Result = ReductionMaxCutToMMC;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.graph().num_vertices();
        let edges = self.edges();
        let overflow = || {
            crate::rules::ReductionError::integer_overflow::<Self, MinimumMatrixCover>(
                "bounding the matrix-cover objective",
            )
        };
        // All entries are nonnegative. Their sum is attained by equal signs and
        // bounds the absolute value of every signed evaluation prefix. Checking
        // this sum is therefore necessary and sufficient for i64 evaluation.
        let mut matrix_sum = 0_i64;
        for &(_, _, weight) in &edges {
            let link_sum = if weight < 0 {
                weight
                    .checked_neg()
                    .and_then(|magnitude| magnitude.checked_mul(2))
                    .and_then(|sum| sum.checked_add(1))
            } else {
                Some(weight)
            }
            .ok_or_else(overflow)?;
            matrix_sum = link_sum
                .checked_mul(2)
                .and_then(|sum| matrix_sum.checked_add(sum))
                .ok_or_else(overflow)?;
        }
        let size = n + edges.iter().filter(|&&(_, _, w)| w < 0).count();
        let mut matrix: Vec<Vec<i64>> = vec![vec![0i64; size]; size];
        let mut link = |a: usize, b: usize, weight: i64| {
            matrix[a][b] = weight;
            matrix[b][a] = weight;
        };

        let mut auxiliary = n;
        for (u, v, w) in edges {
            if w >= 0 {
                link(u, v, w);
                continue;
            }
            // anchor = |w| + 1 = 1 - w; then |w| = anchor - 1 cannot overflow.
            let Some(anchor) = 1_i64.checked_sub(w) else {
                return Err(crate::rules::ReductionError::integer_overflow::<
                    MaxCut<SimpleGraph, i64>,
                    MinimumMatrixCover,
                >("negating a negative edge weight"));
            };
            link(auxiliary, v, anchor);
            link(auxiliary, u, anchor - 1);
            auxiliary += 1;
        }

        Ok(ReductionMaxCutToMMC {
            target: MinimumMatrixCover::new(matrix),
            num_source_vertices: n,
        })
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
            let source = MaxCut::<SimpleGraph, i64>::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]),
                vec![1, 1, 1, 1],
            );
            crate::example_db::specs::rule_example_with_witness::<_, MinimumMatrixCover>(
                source,
                SolutionPair {
                    // S = {0, 2}: vertices 0 and 2 have f = +1.
                    source_config: serde_json::json!(vec![true, false, true, false]),
                    target_config: serde_json::json!(vec![true, false, true, false]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maxcut_minimummatrixcover.rs"]
mod tests;
