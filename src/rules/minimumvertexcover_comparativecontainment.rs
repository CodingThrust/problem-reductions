//! Signed-weight decision vertex cover to comparative containment.
//!
//! Extends the complement-set construction cited in Garey--Johnson SP10.
//! Let W = sum w_v, U = sum max(w_v,0), B = min(K,U), and
//! P = 1 + sum |w_v|. Encode the signed containment expression
//! B - W + sum w_v [Y subset V\{v}] - P sum_e [Y subset V\e].
//! Positive terms go to R, negative terms to S, and zero terms are omitted.
//! Its value is B - w(Y) - P * uncovered(Y), so it is nonnegative exactly
//! for vertex covers meeting the bound. Extraction is the identity.
//! Stored weights and family totals must fit the repository's i64 contract.

use crate::models::decision::Decision;
use crate::models::graph::MinimumVertexCover;
use crate::models::set::ComparativeContainment;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Identity witness map for the signed-weight containment construction.
#[derive(Debug, Clone)]
pub struct ReductionDecisionMVCToComparativeContainment {
    target: ComparativeContainment<i64>,
}

impl ReductionResult for ReductionDecisionMVCToComparativeContainment {
    type Source = Decision<MinimumVertexCover<SimpleGraph, i64>>;
    type Target = ComparativeContainment<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        if !crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?
            .0
        {
            return Err(crate::rules::ExtractionError::invalid(
                "containment inequality is not satisfied",
            ));
        }
        Ok(target_solution.clone())
    }
}

#[reduction(
    transform = upper_bound {
        universe_size = "num_vertices",
        num_r_sets = "num_vertices + 1",
        num_s_sets = "num_vertices + num_edges + 1",
    }
)]
impl ReduceTo<ComparativeContainment<i64>> for Decision<MinimumVertexCover<SimpleGraph, i64>> {
    type Result = ReductionDecisionMVCToComparativeContainment;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let overflow = |operation| {
            crate::rules::ReductionError::integer_overflow::<Self, ComparativeContainment<i64>>(
                operation,
            )
        };
        let weights = self.inner().weights();
        let n = self.inner().graph().num_vertices();
        let mut positive_total = 0i64;
        let mut negative_total = 0i64;
        for &weight in weights {
            if weight > 0 {
                positive_total = positive_total
                    .checked_add(weight)
                    .ok_or_else(|| overflow("summing positive vertex weights"))?;
            } else {
                negative_total = negative_total
                    .checked_add(weight)
                    .ok_or_else(|| overflow("summing negative vertex weights"))?;
            }
        }
        // All subset sums lie in [negative_total, positive_total].
        // Opposite-sign operands cannot overflow.
        let total = positive_total + negative_total;
        let bound = (*self.bound()).min(positive_total);
        let constant = bound
            .checked_sub(total)
            .ok_or_else(|| overflow("computing the containment budget term"))?;
        let penalty = positive_total
            .checked_sub(negative_total)
            .and_then(|span| span.checked_add(1))
            .ok_or_else(|| overflow("computing a strict uncovered-edge penalty"))?;

        let mut r_sets = Vec::new();
        let mut r_weights = Vec::new();
        let mut s_sets = Vec::new();
        let mut s_weights = Vec::new();
        // A signed term is represented on the corresponding side of R >= S.
        for (set, coefficient) in weights
            .iter()
            .enumerate()
            .filter(|(_, w)| **w != 0)
            .map(|(v, &w)| (complement_singleton(n, v), w))
            .chain((constant != 0).then(|| ((0..n).collect(), constant)))
        {
            if coefficient > 0 {
                r_sets.push(set);
                r_weights.push(coefficient);
            } else if coefficient < 0 {
                s_sets.push(set);
                s_weights.push(
                    coefficient
                        .checked_neg()
                        .ok_or_else(|| overflow("negating a containment coefficient"))?,
                );
            }
        }
        for (u, v) in self.inner().graph().edges() {
            s_sets.push(complement_pair(n, u, v));
            s_weights.push(penalty);
        }
        // The empty subset is contained in every set. Checking both full sums
        // ensures every target evaluation stays in i64, including NO witnesses.
        for family in [&r_weights, &s_weights] {
            family
                .iter()
                .try_fold(0i64, |sum, &weight| sum.checked_add(weight))
                .ok_or_else(|| overflow("summing a containment weight family"))?;
        }
        let target = ComparativeContainment::with_weights(n, r_sets, s_sets, r_weights, s_weights)
            .map_err(
                crate::rules::ReductionError::construction::<Self, ComparativeContainment<i64>>,
            )?;
        Ok(ReductionDecisionMVCToComparativeContainment { target })
    }
}

fn complement_singleton(n: usize, v: usize) -> Vec<usize> {
    (0..n).filter(|&x| x != v).collect()
}

fn complement_pair(n: usize, u: usize, v: usize) -> Vec<usize> {
    (0..n).filter(|&x| x != u && x != v).collect()
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "decisionminimumvertexcover_to_comparativecontainment",
        build: || {
            // Path P_4: 0-1-2-3, bound K=2. Minimum cover {1,2} has size 2.
            let inner = MinimumVertexCover::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
                vec![1i64; 4],
            );
            let source = Decision::new(inner, 2);
            crate::example_db::specs::rule_example_with_witness::<_, ComparativeContainment<i64>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![false, true, true, false]),
                    target_config: serde_json::json!(vec![false, true, true, false]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_comparativecontainment.rs"]
mod tests;
