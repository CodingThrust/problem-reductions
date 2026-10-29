//! Direct exact integer encoding of GroupingBySwapping.
use crate::models::algebraic::QUBO;
use crate::models::decision::Decision;
use crate::models::misc::GroupingBySwapping;
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::qubo_helpers::{pair_index, tournament_ranks};
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
#[derive(Debug, Clone)]
pub struct ReductionGroupingBySwappingToQUBO {
    target: Decision<QUBO<i64>>,
    source: GroupingBySwapping,
}
impl ReductionResult for ReductionGroupingBySwappingToQUBO {
    type Source = GroupingBySwapping;
    type Target = Decision<QUBO<i64>>;
    fn target_problem(&self) -> &Self::Target {
        &self.target
    }
    fn extract_solution(
        &self,
        bits: &Vec<bool>,
    ) -> ExtractionResult<<Self::Source as crate::Problem>::Solution> {
        validate_target_witness(
            &self.target,
            bits,
            |v| v.0,
            "QUBO does not certify a source witness",
        )?;
        let ranks = tournament_ranks(self.source.alphabet_size(), bits)?;
        let mut string = self.source.string().to_vec();
        let mut swaps = Vec::new();
        for i in 1..string.len() {
            let mut j = i;
            while j > 0 && ranks[string[j - 1]] > ranks[string[j]] {
                string.swap(j - 1, j);
                swaps.push(j - 1);
                j -= 1;
            }
        }
        if swaps.len() > self.source.budget() {
            return Err(crate::rules::ExtractionError::invalid(
                "inversion count exceeds the swap budget",
            ));
        }
        if !string.is_empty() {
            swaps.resize(self.source.budget(), string.len() - 1);
        }
        Ok(swaps)
    }
}
#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionGroupingBySwappingToQUBO {}
#[crate::reduction(transform = { exact { num_vars = "alphabet_size * (alphabet_size - 1) / 2" }, upper_bound { num_quadratic_terms = "alphabet_size * (alphabet_size - 1) * (alphabet_size - 2) / 2" } })]
impl ReduceTo<Decision<QUBO<i64>>> for GroupingBySwapping {
    type Result = ReductionGroupingBySwappingToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<GroupingBySwapping, Decision<QUBO<i64>>>;
        let n = self.alphabet_size();
        let mut q = Builder::new(Builder::dimension(n, n.saturating_sub(1))? / 2);
        for (p, &i) in self.string().iter().enumerate() {
            for &j in &self.string()[p + 1..] {
                if i < j {
                    q.add_constant(1)?;
                    let v = pair_index(n, i, j);
                    q.add(v, v, -1)?;
                } else if i > j {
                    let v = pair_index(n, j, i);
                    q.add(v, v, 1)?;
                }
            }
        }
        let max_swaps =
            Builder::dimension(self.string_len(), self.string_len().saturating_sub(1))? / 2;
        let budget =
            i64::try_from(self.budget().min(max_swaps)).map_err(|_| Builder::overflow())?;
        let max_swaps = i64::try_from(max_swaps).map_err(|_| Builder::overflow())?;
        q.tournament(
            n,
            &[],
            max_swaps.checked_add(1).ok_or_else(Builder::overflow)?,
        )?;
        let (target, constant) = q.finish()?;
        let bound = budget.checked_sub(constant).ok_or_else(Builder::overflow)?;
        Ok(ReductionGroupingBySwappingToQUBO {
            target: Decision::new(target, bound),
            source: self.clone(),
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "groupingbyswapping_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                GroupingBySwapping::new(3, vec![0, 1, 2, 0], 2),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([2, 1]),
                    target_config: serde_json::json!([true, true, true]),
                },
            )
        },
    }]
}
