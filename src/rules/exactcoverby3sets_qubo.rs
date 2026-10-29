//! Direct exact integer encoding of ExactCoverBy3Sets.
use crate::models::algebraic::QUBO;
use crate::models::decision::Decision;
use crate::models::set::ExactCoverBy3Sets;
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionExactCoverBy3SetsToQUBO {
    target: Decision<QUBO<i64>>,
}
impl ReductionResult for ReductionExactCoverBy3SetsToQUBO {
    type Source = ExactCoverBy3Sets;
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
impl crate::rules::AggregateReductionResult for ReductionExactCoverBy3SetsToQUBO {}
#[crate::reduction(transform = { exact { num_vars = "num_subsets" }, upper_bound { num_quadratic_terms = "(num_subsets)^2" } })]
impl ReduceTo<Decision<QUBO<i64>>> for ExactCoverBy3Sets {
    type Result = ReductionExactCoverBy3SetsToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<ExactCoverBy3Sets, Decision<QUBO<i64>>>;
        let mut q = Builder::new(self.num_subsets());
        for u in 0..self.universe_size() {
            let bits = self
                .subsets()
                .iter()
                .enumerate()
                .filter_map(|(j, t)| t.contains(&u).then_some(j))
                .collect::<Vec<_>>();
            q.square(&bits, 1, 1)?;
        }
        let (target, constant) = q.finish()?;
        let bound = constant.checked_neg().ok_or_else(Builder::overflow)?;
        Ok(ReductionExactCoverBy3SetsToQUBO {
            target: Decision::new(target, bound),
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "exactcoverby3sets_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([true, true, false]),
                    target_config: serde_json::json!([true, true, false]),
                },
            )
        },
    }]
}
