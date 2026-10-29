//! Direct quadratic encoding of MaximumLikelihoodRanking.
use crate::models::algebraic::QUBO;
use crate::models::misc::MaximumLikelihoodRanking;
use crate::rules::qubo_helpers::{ObjectiveEncoding, QuboBuilder, pair_index, tournament_ranks};
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
use crate::types::Min;

#[derive(Debug, Clone)]
pub struct ReductionMaximumLikelihoodRankingToQUBO {
    encoding: ObjectiveEncoding,
    rows: usize,
}

impl ReductionResult for ReductionMaximumLikelihoodRankingToQUBO {
    type Source = MaximumLikelihoodRanking;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_solution(&self, bits: &Vec<bool>) -> ExtractionResult<Vec<usize>> {
        self.encoding.validate(bits)?;
        tournament_ranks(self.rows, bits)
    }
}

#[crate::aggregate_reduction]
impl crate::rules::AggregateReductionResult for ReductionMaximumLikelihoodRankingToQUBO {
    type Source = MaximumLikelihoodRanking;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_value(&self, value: Min<i64>) -> Min<i64> {
        self.encoding.value(value)
    }
}

#[crate::reduction(transform = {
    exact { num_vars = "num_items * (num_items - 1) / 2" },
    upper_bound { num_quadratic_terms = "num_items * (num_items - 1) * (num_items - 2) / 2" },
})]
impl ReduceTo<QUBO<i64>> for MaximumLikelihoodRanking {
    type Result = ReductionMaximumLikelihoodRankingToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<MaximumLikelihoodRanking, QUBO<i64>>;
        let n = self.num_items();
        let mut q = Builder::new(Builder::dimension(n, n.saturating_sub(1))? / 2);
        for i in 0..n {
            for j in i + 1..n {
                q.add_constant(i128::from(self.matrix()[i][j]))?;
                let v = pair_index(n, i, j);
                q.add(
                    v,
                    v,
                    i128::from(self.matrix()[j][i]) - i128::from(self.matrix()[i][j]),
                )?;
            }
        }
        let (penalty, low, high) = q.penalty()?;
        q.tournament(n, &[], penalty)?;
        Ok(ReductionMaximumLikelihoodRankingToQUBO {
            encoding: q.optimization(low, high)?,
            rows: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximumlikelihoodranking_to_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                MaximumLikelihoodRanking::new(vec![vec![0, 1, 2], vec![3, 0, 1], vec![2, 3, 0]]),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([2, 1, 0]),
                    target_config: serde_json::json!([false, false, false]),
                },
            )
        },
    }]
}
