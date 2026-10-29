//! Direct exact quadratic encoding of MinimumFeedbackArcSet.
use crate::models::algebraic::QUBO;
use crate::models::graph::MinimumFeedbackArcSet;
use crate::rules::qubo_helpers::{ObjectiveEncoding, QuboBuilder};
use crate::rules::qubo_helpers::{pair_index, tournament_ranks};
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
use crate::types::Min;
#[derive(Debug, Clone)]
pub struct ReductionMinimumFeedbackArcSetToQUBO {
    encoding: ObjectiveEncoding,
    source: MinimumFeedbackArcSet<i64>,
}
impl ReductionResult for ReductionMinimumFeedbackArcSetToQUBO {
    type Source = MinimumFeedbackArcSet<i64>;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_solution(
        &self,
        bits: &Vec<bool>,
    ) -> ExtractionResult<<Self::Source as crate::Problem>::Solution> {
        self.encoding.validate(bits)?;
        let ranks = tournament_ranks(self.source.num_vertices(), bits)?;
        Ok(self
            .source
            .graph()
            .iter_arcs()
            .zip(self.source.weights())
            .map(|((u, v), &w)| w < 0 || u == v || ranks[u] > ranks[v])
            .collect())
    }
}
#[crate::aggregate_reduction]
impl crate::rules::AggregateReductionResult for ReductionMinimumFeedbackArcSetToQUBO {
    type Source = MinimumFeedbackArcSet<i64>;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_value(&self, value: Min<i64>) -> Min<i64> {
        self.encoding.value(value)
    }
}
#[crate::reduction(transform = { exact { num_vars = "num_vertices * (num_vertices - 1) / 2" }, upper_bound { num_quadratic_terms = "num_vertices * (num_vertices - 1) * (num_vertices - 2) / 2" } })]
impl ReduceTo<QUBO<i64>> for MinimumFeedbackArcSet<i64> {
    type Result = ReductionMinimumFeedbackArcSetToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<MinimumFeedbackArcSet<i64>, QUBO<i64>>;
        let n = self.num_vertices();
        let mut negative = 0i64;
        let mut positive = 0i64;
        for &w in self.weights() {
            negative = negative
                .checked_add(w.min(0))
                .ok_or_else(Builder::overflow)?;
            positive = positive
                .checked_add(w.max(0))
                .ok_or_else(Builder::overflow)?;
        }
        let mut q = Builder::new(Builder::dimension(n, n.saturating_sub(1))? / 2);
        for ((u, v), &w) in self.graph().iter_arcs().zip(self.weights()) {
            if w < 0 || u == v {
                q.add_constant(i128::from(w))?;
            } else if u < v {
                q.add_constant(i128::from(w))?;
                let bit = pair_index(n, u, v);
                q.add(bit, bit, -i128::from(w))?;
            } else {
                let bit = pair_index(n, v, u);
                q.add(bit, bit, i128::from(w))?;
            }
        }
        let (penalty, low, high) = q.penalty()?;
        q.tournament(n, &[], penalty)?;
        Ok(ReductionMinimumFeedbackArcSetToQUBO {
            encoding: q.optimization(low, high)?,
            source: self.clone(),
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumfeedbackarcset_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                MinimumFeedbackArcSet::new(
                    crate::topology::MultiDigraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
                    vec![-2i64, 3, 1],
                ),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([true, false, false]),
                    target_config: serde_json::json!([false, false, true]),
                },
            )
        },
    }]
}
