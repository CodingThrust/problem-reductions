//! Direct integer encoding of SequencingWithReleaseTimesAndDeadlines.
use crate::models::algebraic::QUBO;
use crate::models::decision::Decision;
use crate::models::misc::SequencingWithReleaseTimesAndDeadlines;
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::start_time_qubo::StartChoices;
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
#[derive(Debug, Clone)]
pub struct ReductionSequencingWithReleaseTimesAndDeadlinesToQUBO {
    target: Decision<QUBO<i64>>,
    starts: StartChoices,
}
impl ReductionResult for ReductionSequencingWithReleaseTimesAndDeadlinesToQUBO {
    type Source = SequencingWithReleaseTimesAndDeadlines;
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
    for ReductionSequencingWithReleaseTimesAndDeadlinesToQUBO
{
}
#[crate::reduction(transform = upper_bound { num_vars = "num_tasks * max_start_slots", num_quadratic_terms = "num_tasks * max_start_slots * (num_tasks * max_start_slots - 1) / 2" })]
impl ReduceTo<Decision<QUBO<i64>>> for SequencingWithReleaseTimesAndDeadlines {
    type Result = ReductionSequencingWithReleaseTimesAndDeadlinesToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<SequencingWithReleaseTimesAndDeadlines, Decision<QUBO<i64>>>;
        // Bound every source finish addition, including orders that miss deadlines.
        self.lengths()
            .iter()
            .try_fold(
                self.release_times().iter().copied().max().unwrap_or(0),
                |end, &length| end.checked_add(length),
            )
            .ok_or_else(Builder::overflow)?;
        let (starts, q, _) = StartChoices::encode::<Self, Decision<QUBO<i64>>>(
            self.lengths(),
            self.release_times(),
            self.deadlines(),
            &[],
            |_, _| Ok(0),
        )?;
        let (target, constant) = q.finish()?;
        let bound = constant.checked_neg().ok_or_else(Builder::overflow)?;
        Ok(Self::Result {
            target: Decision::new(target, bound),
            starts,
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "sequencingwithreleasetimesanddeadlines_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                SequencingWithReleaseTimesAndDeadlines::new(
                    vec![1, 2, 1],
                    vec![0, 0, 2],
                    vec![3, 3, 4],
                ),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([0, 1, 2]),
                    target_config: serde_json::json!([
                        true, false, false, false, true, false, true
                    ]),
                },
            )
        },
    }]
}
