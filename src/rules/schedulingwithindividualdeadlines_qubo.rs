//! Direct exact integer encoding of SchedulingWithIndividualDeadlines.
use crate::models::algebraic::QUBO;
use crate::models::decision::Decision;
use crate::models::misc::SchedulingWithIndividualDeadlines;
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionSchedulingWithIndividualDeadlinesToQUBO {
    target: Decision<QUBO<i64>>,
    lengths: Vec<usize>,
}
impl ReductionResult for ReductionSchedulingWithIndividualDeadlinesToQUBO {
    type Source = SchedulingWithIndividualDeadlines;
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
        crate::rules::qubo_helpers::decode_rows(bits, self.lengths.iter().copied())
    }
}
#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionSchedulingWithIndividualDeadlinesToQUBO {}
#[crate::reduction(transform = upper_bound { num_vars = "2 * num_tasks * max_deadline", num_quadratic_terms = "(2 * num_tasks * max_deadline)^2" })]
impl ReduceTo<Decision<QUBO<i64>>> for SchedulingWithIndividualDeadlines {
    type Result = ReductionSchedulingWithIndividualDeadlinesToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        let (target, lengths) = crate::rules::unit_schedule_qubo::encode::<Self>(
            self.deadlines(),
            self.num_processors(),
            self.precedences(),
        )?;
        Ok(ReductionSchedulingWithIndividualDeadlinesToQUBO { target, lengths })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "schedulingwithindividualdeadlines_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                SchedulingWithIndividualDeadlines::new(3, 2, vec![1, 1, 2], vec![(0, 2)]),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([0, 0, 1]),
                    target_config: serde_json::json!([true, true, false, true, false, false]),
                },
            )
        },
    }]
}
