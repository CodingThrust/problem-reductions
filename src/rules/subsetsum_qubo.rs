//! Direct exact integer encoding of SubsetSum.
use crate::models::algebraic::QUBO;
use crate::models::decision::Decision;
use crate::models::misc::SubsetSum;
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
use num_bigint::BigInt;
use num_traits::ToPrimitive;
#[derive(Debug, Clone)]
pub struct ReductionSubsetSumToQUBO {
    target: Decision<QUBO<i64>>,
}
impl ReductionResult for ReductionSubsetSumToQUBO {
    type Source = SubsetSum;
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
impl crate::rules::AggregateReductionResult for ReductionSubsetSumToQUBO {}
#[crate::reduction(transform = { exact { num_vars = "num_elements" }, upper_bound { num_quadratic_terms = "(num_elements)^2" } })]
impl ReduceTo<Decision<QUBO<i64>>> for SubsetSum {
    type Result = ReductionSubsetSumToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<SubsetSum, Decision<QUBO<i64>>>;
        let mut q = Builder::new(self.num_elements());
        let target = BigInt::from(self.target().clone());
        q.add_constant(
            (&target * &target)
                .to_i128()
                .ok_or_else(Builder::overflow)?,
        )?;
        for (i, size) in self.sizes().iter().enumerate() {
            let w = BigInt::from(size.clone());
            let diagonal: BigInt = &w * &w - 2 * &target * &w;
            q.add(i, i, diagonal.to_i128().ok_or_else(Builder::overflow)?)?;
            for (j, other) in self.sizes().iter().enumerate().skip(i + 1) {
                let coupling: BigInt = 2 * &w * BigInt::from(other.clone());
                q.add(i, j, coupling.to_i128().ok_or_else(Builder::overflow)?)?;
            }
        }
        let (target, constant) = q.finish()?;
        let bound = constant.checked_neg().ok_or_else(Builder::overflow)?;
        Ok(ReductionSubsetSumToQUBO {
            target: Decision::new(target, bound),
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "subsetsum_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                SubsetSum::new(vec![1u32, 2, 4], 3u32),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([true, true, false]),
                    target_config: serde_json::json!([true, true, false]),
                },
            )
        },
    }]
}
