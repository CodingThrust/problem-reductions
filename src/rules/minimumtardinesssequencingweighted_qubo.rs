//! Direct integer encoding of MinimumTardinessSequencing<i64>.
use crate::models::algebraic::QUBO;
use crate::models::misc::MinimumTardinessSequencing;
use crate::rules::qubo_helpers::ObjectiveEncoding;
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::start_time_qubo::StartChoices;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
use crate::types::Min;
#[derive(Debug, Clone)]
pub struct ReductionMinimumTardinessSequencingWeightedToQUBO {
    encoding: ObjectiveEncoding,
    starts: StartChoices,
}
impl ReductionResult for ReductionMinimumTardinessSequencingWeightedToQUBO {
    type Source = MinimumTardinessSequencing<i64>;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_solution(
        &self,
        bits: &Vec<bool>,
    ) -> ExtractionResult<<Self::Source as crate::Problem>::Solution> {
        self.encoding.validate(bits)?;
        self.starts.decode(bits)
    }
}
#[crate::aggregate_reduction]
impl crate::rules::AggregateReductionResult for ReductionMinimumTardinessSequencingWeightedToQUBO {
    type Source = MinimumTardinessSequencing<i64>;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_value(&self, value: Min<i64>) -> Min<i64> {
        self.encoding.value(value)
    }
}
#[crate::reduction(transform = unavailable { num_vars = "Start-grid size depends exponentially on processing magnitude bits; the contract supports only constant integer exponents", num_quadratic_terms = "Start-grid size depends exponentially on processing magnitude bits; the contract supports only constant integer exponents" })]
impl ReduceTo<QUBO<i64>> for MinimumTardinessSequencing<i64> {
    type Result = ReductionMinimumTardinessSequencingWeightedToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<MinimumTardinessSequencing<i64>, QUBO<i64>>;
        let n = self.num_tasks();
        let horizon = self
            .lengths()
            .iter()
            .try_fold(0i64, |s, &p| s.checked_add(p))
            .ok_or_else(Builder::overflow)?;
        let (starts, q, high) = StartChoices::encode::<Self, QUBO<i64>>(
            self.lengths(),
            &vec![0; n],
            &vec![horizon; n],
            self.precedences(),
            |i, end| Ok(i64::from(end > self.deadlines()[i])),
        )?;
        Ok(Self::Result {
            encoding: q.optimization(0, high)?,
            starts,
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumtardinesssequencingweighted_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                MinimumTardinessSequencing::with_lengths(vec![1, 1, 2], vec![1, 2, 3], vec![]),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([0, 1, 2]),
                    target_config: serde_json::json!([
                        true, false, false, false, false, true, false, false, false, false, true
                    ]),
                },
            )
        },
    }]
}
