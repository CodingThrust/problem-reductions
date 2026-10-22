//! Direct integer encoding of Betweenness.
use crate::models::algebraic::QUBO;
use crate::models::decision::Decision;
use crate::models::misc::Betweenness;
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::qubo_helpers::{pair_index, tournament_ranks};
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
#[derive(Debug, Clone)]
pub struct ReductionBetweennessToQUBO {
    target: Decision<QUBO<i64>>,
    size: usize,
}
impl ReductionResult for ReductionBetweennessToQUBO {
    type Source = Betweenness;
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
        tournament_ranks(self.size, bits)
    }
}
#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionBetweennessToQUBO {}
#[crate::reduction(transform = { exact { num_vars = "num_elements * (num_elements - 1) / 2" }, upper_bound { num_quadratic_terms = "(num_elements * (num_elements - 1) / 2)^2" } })]
impl ReduceTo<Decision<QUBO<i64>>> for Betweenness {
    type Result = ReductionBetweennessToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<Betweenness, Decision<QUBO<i64>>>;
        let n = self.num_elements();
        let mut q = Builder::new(Builder::dimension(n, n.saturating_sub(1))? / 2);
        q.tournament(n, &[], 1)?;
        for &(a, b, c) in self.triples() {
            let mut terms = Vec::with_capacity(2);
            let mut constant = 0;
            for (i, j, sign) in [(a, b, 1), (b, c, -1)] {
                if i < j {
                    terms.push((pair_index(n, i, j), sign));
                } else {
                    terms.push((pair_index(n, j, i), -sign));
                    constant += sign;
                }
            }
            q.affine_square(&terms, -constant)?;
        }
        let (target, constant) = q.finish()?;
        let bound = constant.checked_neg().ok_or_else(Builder::overflow)?;
        Ok(Self::Result {
            target: Decision::new(target, bound),
            size: n,
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "betweenness_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                Betweenness::new(3, vec![(0, 1, 2)]),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([0, 1, 2]),
                    target_config: serde_json::json!([true, true, true]),
                },
            )
        },
    }]
}
