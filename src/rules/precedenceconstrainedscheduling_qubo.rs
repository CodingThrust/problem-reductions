//! Direct exact integer encoding of PrecedenceConstrainedScheduling.
use crate::models::algebraic::QUBO;
use crate::models::decision::Decision;
use crate::models::misc::PrecedenceConstrainedScheduling;
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionPrecedenceConstrainedSchedulingToQUBO {
    target: Decision<QUBO<i64>>,
    lengths: Vec<usize>,
}
impl ReductionResult for ReductionPrecedenceConstrainedSchedulingToQUBO {
    type Source = PrecedenceConstrainedScheduling;
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
impl crate::rules::AggregateReductionResult for ReductionPrecedenceConstrainedSchedulingToQUBO {}
#[crate::reduction(transform = upper_bound { num_vars = "2 * num_tasks * deadline", num_quadratic_terms = "(2 * num_tasks * deadline)^2" })]
impl ReduceTo<Decision<QUBO<i64>>> for PrecedenceConstrainedScheduling {
    type Result = ReductionPrecedenceConstrainedSchedulingToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        let deadlines = vec![self.deadline(); self.num_tasks()];
        let (target, lengths) = crate::rules::unit_schedule_qubo::encode::<Self>(
            &deadlines,
            self.num_processors(),
            self.precedences(),
        )?;
        Ok(ReductionPrecedenceConstrainedSchedulingToQUBO { target, lengths })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "precedenceconstrainedscheduling_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                PrecedenceConstrainedScheduling::new(3, 2, 2, vec![(0, 2)]),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([0, 0, 1]),
                    target_config: serde_json::json!([
                        true, false, true, false, false, true, false, false, true, false
                    ]),
                },
            )
        },
    }]
}
