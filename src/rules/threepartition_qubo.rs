//! Direct exact integer encoding of ThreePartition.
use crate::models::algebraic::QUBO;
use crate::models::decision::Decision;
use crate::models::misc::ThreePartition;
use crate::rules::qubo_helpers::decode_rows;
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
#[derive(Debug, Clone)]
pub struct ReductionThreePartitionToQUBO {
    target: Decision<QUBO<i64>>,
    rows: usize,
    groups: usize,
}
impl ReductionResult for ReductionThreePartitionToQUBO {
    type Source = ThreePartition;
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
        decode_rows(bits, std::iter::repeat_n(self.groups, self.rows))
    }
}
#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionThreePartitionToQUBO {}
#[crate::reduction(transform = { exact { num_vars = "num_elements * num_groups" }, upper_bound { num_quadratic_terms = "(num_elements * num_groups)^2" } })]
impl ReduceTo<Decision<QUBO<i64>>> for ThreePartition {
    type Result = ReductionThreePartitionToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<ThreePartition, Decision<QUBO<i64>>>;
        let n = self.num_elements();
        let k = self.num_groups();
        let mut q = Builder::new(Builder::dimension(n, k)?);
        q.assignment(n, k, false, 1)?;
        for g in 0..k {
            q.square(&(0..n).map(|i| i * k + g).collect::<Vec<_>>(), 3, 1)?;
            q.affine_square(
                &(0..n)
                    .map(|i| (i * k + g, self.sizes()[i]))
                    .collect::<Vec<_>>(),
                self.bound(),
            )?;
        }
        let (target, constant) = q.finish()?;
        let bound = constant.checked_neg().ok_or_else(Builder::overflow)?;
        Ok(ReductionThreePartitionToQUBO {
            target: Decision::new(target, bound),
            rows: n,
            groups: k,
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "threepartition_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                ThreePartition::new(vec![2, 2, 2], 6),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([0, 0, 0]),
                    target_config: serde_json::json!([true, true, true]),
                },
            )
        },
    }]
}
