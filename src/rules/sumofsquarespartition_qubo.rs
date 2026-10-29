//! Direct quadratic encoding of SumOfSquaresPartition.
use crate::models::algebraic::QUBO;
use crate::models::misc::SumOfSquaresPartition;
use crate::rules::qubo_helpers::{ObjectiveEncoding, QuboBuilder, decode_rows};
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
use crate::types::Min;

#[derive(Debug, Clone)]
pub struct ReductionSumOfSquaresPartitionToQUBO {
    encoding: ObjectiveEncoding,
    rows: usize,
    cols: usize,
}

impl ReductionResult for ReductionSumOfSquaresPartitionToQUBO {
    type Source = SumOfSquaresPartition;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_solution(&self, bits: &Vec<bool>) -> ExtractionResult<Vec<usize>> {
        self.encoding.validate(bits)?;
        decode_rows(bits, std::iter::repeat_n(self.cols, self.rows))
    }
}

#[crate::aggregate_reduction]
impl crate::rules::AggregateReductionResult for ReductionSumOfSquaresPartitionToQUBO {
    type Source = SumOfSquaresPartition;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_value(&self, value: Min<i64>) -> Min<i64> {
        self.encoding.value(value)
    }
}

#[crate::reduction(transform = {
    exact { num_vars = "num_elements * num_groups" },
    upper_bound { num_quadratic_terms = "num_elements * num_groups * (num_groups - 1) / 2 + num_groups * num_elements * (num_elements - 1) / 2" },
})]
impl ReduceTo<QUBO<i64>> for SumOfSquaresPartition {
    type Result = ReductionSumOfSquaresPartitionToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<SumOfSquaresPartition, QUBO<i64>>;
        let n = self.num_elements();
        let k = self.num_groups();
        let mut q = Builder::new(Builder::dimension(n, k)?);
        for g in 0..k {
            for i in 0..n {
                let s = i128::from(self.sizes()[i]);
                q.add(i * k + g, i * k + g, s * s)?;
                for j in i + 1..n {
                    q.add(i * k + g, j * k + g, 2 * s * i128::from(self.sizes()[j]))?;
                }
            }
        }
        let (penalty, low, high) = q.penalty()?;
        q.assignment(n, k, false, penalty)?;
        Ok(ReductionSumOfSquaresPartitionToQUBO {
            encoding: q.optimization(low, high)?,
            rows: n,
            cols: k,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "sumofsquarespartition_to_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                SumOfSquaresPartition::new(vec![1, 2, 3], 2),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([0, 0, 1]),
                    target_config: serde_json::json!([true, false, true, false, false, true]),
                },
            )
        },
    }]
}
