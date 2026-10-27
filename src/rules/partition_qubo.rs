//! Direct exact integer encoding of Partition.
use crate::models::algebraic::QUBO;
use crate::models::decision::Decision;
use crate::models::misc::Partition;
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionPartitionToQUBO {
    target: Decision<QUBO<i64>>,
}
impl ReductionResult for ReductionPartitionToQUBO {
    type Source = Partition;
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
impl crate::rules::AggregateReductionResult for ReductionPartitionToQUBO {}
#[crate::reduction(transform = { exact { num_vars = "num_elements" }, upper_bound { num_quadratic_terms = "(num_elements)^2" } })]
impl ReduceTo<Decision<QUBO<i64>>> for Partition {
    type Result = ReductionPartitionToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<Partition, Decision<QUBO<i64>>>;
        let mut q = Builder::new(self.num_elements());
        let total = i128::from(self.total_sum());
        q.add_constant(total * total)?;
        for (i, &size) in self.sizes().iter().enumerate() {
            let w = 2 * i128::from(size);
            q.add(i, i, w * w - 2 * total * w)?;
            for (j, &other) in self.sizes().iter().enumerate().skip(i + 1) {
                q.add(i, j, 2 * w * 2 * i128::from(other))?;
            }
        }
        let (target, constant) = q.finish()?;
        let bound = constant.checked_neg().ok_or_else(Builder::overflow)?;
        Ok(ReductionPartitionToQUBO {
            target: Decision::new(target, bound),
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                Partition::new(vec![1, 2, 3]).unwrap(),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([true, true, false]),
                    target_config: serde_json::json!([true, true, false]),
                },
            )
        },
    }]
}
