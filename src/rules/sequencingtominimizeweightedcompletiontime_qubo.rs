//! Direct quadratic encoding of SequencingToMinimizeWeightedCompletionTime.
use crate::models::algebraic::QUBO;
use crate::models::misc::SequencingToMinimizeWeightedCompletionTime;
use crate::rules::qubo_helpers::{
    invert_permutation, pair_index, tournament_ranks, ObjectiveEncoding, QuboBuilder,
};
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
use crate::types::Min;

#[derive(Debug, Clone)]
pub struct ReductionSequencingToMinimizeWeightedCompletionTimeToQUBO {
    encoding: ObjectiveEncoding,
    rows: usize,
}

impl ReductionResult for ReductionSequencingToMinimizeWeightedCompletionTimeToQUBO {
    type Source = SequencingToMinimizeWeightedCompletionTime;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_solution(&self, bits: &Vec<bool>) -> ExtractionResult<Vec<usize>> {
        self.encoding.validate(bits)?;
        invert_permutation(&tournament_ranks(self.rows, bits)?)
    }
}

#[crate::aggregate_reduction]
impl crate::rules::AggregateReductionResult
    for ReductionSequencingToMinimizeWeightedCompletionTimeToQUBO
{
    type Source = SequencingToMinimizeWeightedCompletionTime;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_value(&self, value: Min<i64>) -> Min<i64> {
        self.encoding.value(value)
    }
}

#[crate::reduction(transform = {
    exact { num_vars = "num_tasks * (num_tasks - 1) / 2" },
    upper_bound { num_quadratic_terms = "num_tasks * (num_tasks - 1) * (num_tasks - 2) / 2" },
})]
impl ReduceTo<QUBO<i64>> for SequencingToMinimizeWeightedCompletionTime {
    type Result = ReductionSequencingToMinimizeWeightedCompletionTimeToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<SequencingToMinimizeWeightedCompletionTime, QUBO<i64>>;
        let n = self.num_tasks();
        // Expansion must not hide overflow in source completion times or costs.
        let (mut earliest, mut latest) = (0i64, 0i64);
        for &length in self.lengths() {
            earliest = earliest
                .checked_add(length.min(0))
                .ok_or_else(Builder::overflow)?;
            latest = latest
                .checked_add(length.max(0))
                .ok_or_else(Builder::overflow)?;
        }
        let (mut negative, mut positive) = (0i64, 0i64);
        for (&length, &weight) in self.lengths().iter().zip(self.weights()) {
            let a = (earliest + length.max(0))
                .checked_mul(weight)
                .ok_or_else(Builder::overflow)?;
            let b = (latest + length.min(0))
                .checked_mul(weight)
                .ok_or_else(Builder::overflow)?;
            negative = negative
                .checked_add(a.min(b).min(0))
                .ok_or_else(Builder::overflow)?;
            positive = positive
                .checked_add(a.max(b).max(0))
                .ok_or_else(Builder::overflow)?;
        }
        let count = Builder::dimension(n, n.saturating_sub(1))? / 2;
        let mut q = Builder::new(count);
        for i in 0..n {
            q.add_constant(i128::from(self.lengths()[i]) * i128::from(self.weights()[i]))?;
            for j in i + 1..n {
                let reverse = i128::from(self.lengths()[j]) * i128::from(self.weights()[i]);
                q.add_constant(reverse)?;
                let v = pair_index(n, i, j);
                q.add(
                    v,
                    v,
                    i128::from(self.lengths()[i]) * i128::from(self.weights()[j]) - reverse,
                )?;
            }
        }
        let (penalty, low, high) = q.penalty()?;
        q.tournament(n, self.precedences(), penalty)?;
        Ok(ReductionSequencingToMinimizeWeightedCompletionTimeToQUBO {
            encoding: q.optimization(low, high)?,
            rows: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "sequencingtominimizeweightedcompletiontime_to_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                SequencingToMinimizeWeightedCompletionTime::new(vec![1; 3], vec![1, 2, 3], vec![]),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([2, 1, 0]),
                    target_config: serde_json::json!([false, false, false]),
                },
            )
        },
    }]
}
