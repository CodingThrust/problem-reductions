//! Direct integer encoding of FeasibleRegisterAssignment.
use crate::models::algebraic::QUBO;
use crate::models::decision::Decision;
use crate::models::misc::FeasibleRegisterAssignment;
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::qubo_helpers::{pair_index, tournament_ranks};
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
#[derive(Debug, Clone)]
pub struct ReductionFeasibleRegisterAssignmentToQUBO {
    target: Decision<QUBO<i64>>,
    size: usize,
}
impl ReductionResult for ReductionFeasibleRegisterAssignmentToQUBO {
    type Source = FeasibleRegisterAssignment;
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
impl crate::rules::AggregateReductionResult for ReductionFeasibleRegisterAssignmentToQUBO {}
#[crate::reduction(transform = { exact { num_vars = "num_vertices * (num_vertices - 1) / 2" }, upper_bound { num_quadratic_terms = "(num_vertices * (num_vertices - 1) / 2)^2" } })]
impl ReduceTo<Decision<QUBO<i64>>> for FeasibleRegisterAssignment {
    type Result = ReductionFeasibleRegisterAssignmentToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<FeasibleRegisterAssignment, Decision<QUBO<i64>>>;
        let n = self.num_vertices();
        let mut q = Builder::new(Builder::dimension(n, n.saturating_sub(1))? / 2);
        let precedences: Vec<_> = self.arcs().iter().map(|&(v, u)| (u, v)).collect();
        q.tournament(n, &precedences, 1)?;
        for &(consumer, value) in self.arcs() {
            for overwriter in 0..n {
                if overwriter == value
                    || overwriter == consumer
                    || self.assignment()[overwriter] != self.assignment()[value]
                {
                    continue;
                }
                // Penalize value < overwriter < consumer, allowing consumption at overwrite.
                let (a, reverse_a) = (
                    pair_index(n, value.min(overwriter), value.max(overwriter)),
                    value > overwriter,
                );
                let (b, reverse_b) = (
                    pair_index(n, overwriter.min(consumer), overwriter.max(consumer)),
                    overwriter > consumer,
                );
                let (r, s) = (i128::from(reverse_a), i128::from(reverse_b));
                q.add_constant(r * s)?;
                q.add(a, a, (1 - 2 * r) * s)?;
                q.add(b, b, (1 - 2 * s) * r)?;
                q.add(a, b, (1 - 2 * r) * (1 - 2 * s))?;
            }
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
        id: "feasibleregisterassignment_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                FeasibleRegisterAssignment::new(3, vec![(1, 0), (2, 1)], 1, vec![0; 3]),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([0, 1, 2]),
                    target_config: serde_json::json!([true, true, true]),
                },
            )
        },
    }]
}
