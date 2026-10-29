//! Direct exact integer encoding of ThreeDimensionalMatching.
use crate::models::algebraic::QUBO;
use crate::models::decision::Decision;
use crate::models::set::ThreeDimensionalMatching;
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionThreeDimensionalMatchingToQUBO {
    target: Decision<QUBO<i64>>,
}
impl ReductionResult for ReductionThreeDimensionalMatchingToQUBO {
    type Source = ThreeDimensionalMatching;
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
        Ok(bits.clone())
    }
}
#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionThreeDimensionalMatchingToQUBO {}
#[crate::reduction(transform = { exact { num_vars = "num_triples" }, upper_bound { num_quadratic_terms = "(num_triples)^2" } })]
impl ReduceTo<Decision<QUBO<i64>>> for ThreeDimensionalMatching {
    type Result = ReductionThreeDimensionalMatchingToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<ThreeDimensionalMatching, Decision<QUBO<i64>>>;
        let mut q = Builder::new(self.num_triples());
        for axis in 0..3 {
            for u in 0..self.universe_size() {
                let bits = self
                    .triples()
                    .iter()
                    .enumerate()
                    .filter_map(|(j, &(a, b, c))| ([a, b, c][axis] == u).then_some(j))
                    .collect::<Vec<_>>();
                q.square(&bits, 1, 1)?;
            }
        }
        let (target, constant) = q.finish()?;
        let bound = constant.checked_neg().ok_or_else(Builder::overflow)?;
        Ok(ReductionThreeDimensionalMatchingToQUBO {
            target: Decision::new(target, bound),
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "threedimensionalmatching_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                ThreeDimensionalMatching::new(3, vec![(0, 1, 2), (1, 0, 1), (2, 2, 0)]),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([true, true, true]),
                    target_config: serde_json::json!([true, true, true]),
                },
            )
        },
    }]
}
