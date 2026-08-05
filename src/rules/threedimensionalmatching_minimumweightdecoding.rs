//! Reduction from ThreeDimensionalMatching to MinimumWeightDecoding.
//!
//! This is the classical Berlekamp–McEliece–van Tilborg (1978) construction
//! (Garey & Johnson MS7) that establishes NP-hardness of the minimum-weight
//! codeword problem. Each triple `t_j = (a_j, b_j, c_j)` becomes a column of
//! a `3q × m` parity-check matrix `H` with exactly three 1s (one per row
//! block `W`, `X`, `Y`), and the syndrome is the all-ones vector `1^{3q}`.
//!
//! **Bridge.** This is a *witness* reduction from `ThreeDimensionalMatching`
//! (`Value = Or`) to `MinimumWeightDecoding` (`Value = Min<usize>`):
//!
//! `source.evaluate(S) == Or(true)` ⇔ `target.evaluate(x) == Min(Some(q))`,
//!
//! where `S = { t_j ∈ T : x_j = 1 }`. We rely on the witness-extraction
//! route `source.evaluate(extract_solution(x))` rather than comparing the
//! optimum value directly, mirroring `partition_sumofsquarespartition.rs`.
//!
//! **Sentinel branch.** `MinimumWeightDecoding::new` panics on zero-row or
//! zero-column matrices, so degenerate inputs (`q = 0` or `T = []`) emit a
//! fixed `1×1` sentinel `H = [[1]]` with syndrome `s = [0]`. The unique
//! feasible codeword `x = (0)` decodes to the empty subset `S = ∅`, and
//! `source.evaluate(∅)` correctly returns `Or(true)` iff `q = 0`.

use crate::models::algebraic::MinimumWeightDecoding;
use crate::models::set::ThreeDimensionalMatching;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing ThreeDimensionalMatching to MinimumWeightDecoding.
#[derive(Debug, Clone)]
pub struct ReductionThreeDimensionalMatchingToMinimumWeightDecoding {
    target: MinimumWeightDecoding,
    /// Number of triples in the original 3DM instance.
    /// Used to return a correctly-sized witness when the sentinel path is
    /// taken (i.e. `q == 0` or `num_triples == 0`).
    source_num_triples: usize,
}

impl ReductionResult for ReductionThreeDimensionalMatchingToMinimumWeightDecoding {
    type Source = ThreeDimensionalMatching;
    type Target = MinimumWeightDecoding;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: identity mapping in the main branch. The target
    /// codeword `x ∈ {0,1}^m` is the source subset indicator over the same
    /// triple index set. In the sentinel branch the target witness has length
    /// `1` (always `[0]`); we return the all-zero source-sized vector,
    /// which decodes to `S = ∅`. `ThreeDimensionalMatching::evaluate(∅)`
    /// then yields `Or(true)` iff `q == 0` (the correct answer for both
    /// sentinel sub-cases).
    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        Ok({
            if target_solution.len() == self.source_num_triples {
                target_solution.to_vec()
            } else {
                vec![0; self.source_num_triples]
            }
        })
    }
}

#[reduction(overhead = {
    num_rows = "3 * universe_size",
    num_cols = "num_triples",
})]
impl ReduceTo<MinimumWeightDecoding> for ThreeDimensionalMatching {
    type Result = ReductionThreeDimensionalMatchingToMinimumWeightDecoding;

    fn reduce_to(&self) -> Self::Result {
        let q = self.universe_size();
        let m = self.num_triples();

        if q == 0 || m == 0 {
            // Sentinel: MinimumWeightDecoding::new panics on empty matrices
            // or zero-column matrices. Build a fixed 1×1 instance whose only
            // feasible codeword is x = (0), decoding to S = ∅. The source's
            // own evaluate on the empty set gives the correct answer:
            //   q = 0 → Or(true)  (empty matching of empty universe)
            //   q ≥ 1 → Or(false) (no triples cannot cover non-empty universe).
            return ReductionThreeDimensionalMatchingToMinimumWeightDecoding {
                target: MinimumWeightDecoding::new(vec![vec![true]], vec![false]),
                source_num_triples: m,
            };
        }

        // Main branch: build H ∈ {0,1}^{3q × m} with row blocks W, X, Y and
        // one 1 per row block per column at the triple's coordinate.
        let num_rows = 3 * q;
        let mut matrix = vec![vec![false; m]; num_rows];
        for (j, &(a, b, c)) in self.triples().iter().enumerate() {
            matrix[a][j] = true;
            matrix[q + b][j] = true;
            matrix[2 * q + c][j] = true;
        }
        let syndrome = vec![true; num_rows];

        ReductionThreeDimensionalMatchingToMinimumWeightDecoding {
            target: MinimumWeightDecoding::new(matrix, syndrome),
            source_num_triples: m,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "threedimensionalmatching_to_minimumweightdecoding",
        build: || {
            // q = 2, T = [(0,0,0), (1,1,1), (0,1,0), (1,0,1)].
            // Perfect matchings: {t_0, t_1} and {t_2, t_3} -- both attain
            // target minimum weight = q = 2.
            crate::example_db::specs::rule_example_with_witness::<_, MinimumWeightDecoding>(
                ThreeDimensionalMatching::new(2, vec![(0, 0, 0), (1, 1, 1), (0, 1, 0), (1, 0, 1)]),
                SolutionPair {
                    source_config: vec![1, 1, 0, 0],
                    target_config: vec![1, 1, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/threedimensionalmatching_minimumweightdecoding.rs"]
mod tests;
