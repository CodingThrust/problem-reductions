//! Direct integer encoding of SequencingToMinimizeWeightedTardiness.
use crate::models::algebraic::QUBO;
use crate::models::decision::Decision;
use crate::models::misc::SequencingToMinimizeWeightedTardiness;
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::start_time_qubo::StartChoices;
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
#[derive(Debug, Clone)]
pub struct ReductionSequencingToMinimizeWeightedTardinessToQUBO {
    target: Decision<QUBO<i64>>,
    starts: StartChoices,
}
impl ReductionResult for ReductionSequencingToMinimizeWeightedTardinessToQUBO {
    type Source = SequencingToMinimizeWeightedTardiness;
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
        self.starts.decode(bits)
    }
}
#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult
    for ReductionSequencingToMinimizeWeightedTardinessToQUBO
{
}
#[crate::reduction(transform = unavailable { num_vars = "Start-grid size depends exponentially on processing magnitude bits; the contract supports only constant integer exponents", num_quadratic_terms = "Start-grid size depends exponentially on processing magnitude bits; the contract supports only constant integer exponents" })]
impl ReduceTo<Decision<QUBO<i64>>> for SequencingToMinimizeWeightedTardiness {
    type Result = ReductionSequencingToMinimizeWeightedTardinessToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<SequencingToMinimizeWeightedTardiness, Decision<QUBO<i64>>>;
        let n = self.num_tasks();
        let horizon = self
            .lengths()
            .iter()
            .try_fold(0i64, |s, &p| s.checked_add(p))
            .ok_or_else(Builder::overflow)?;
        // Bound source products and objective prefixes before target cancellation.
        self.weights()
            .iter()
            .zip(self.deadlines())
            .try_fold(0i64, |s, (&w, &d)| {
                w.checked_mul((horizon - d).max(0))
                    .and_then(|v| s.checked_add(v))
            })
            .ok_or_else(Builder::overflow)?;
        let (starts, q, high) = StartChoices::encode::<Self, Decision<QUBO<i64>>>(
            self.lengths(),
            &vec![0; n],
            &vec![horizon; n],
            &[],
            |i, end| {
                self.weights()[i]
                    .checked_mul((end - self.deadlines()[i]).max(0))
                    .ok_or_else(Builder::overflow)
            },
        )?;
        let (target, constant) = q.finish()?;
        let bound = self
            .bound()
            .min(high)
            .checked_sub(constant)
            .ok_or_else(Builder::overflow)?;
        Ok(Self::Result {
            target: Decision::new(target, bound),
            starts,
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "sequencingtominimizeweightedtardiness_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                SequencingToMinimizeWeightedTardiness::new(
                    vec![1, 0, 1],
                    vec![1, 2, 1],
                    vec![0, 0, 1],
                    2,
                ),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([1, 0, 2]),
                    target_config: serde_json::json!([
                        true, false, true, false, false, false, true
                    ]),
                },
            )
        },
    }]
}
