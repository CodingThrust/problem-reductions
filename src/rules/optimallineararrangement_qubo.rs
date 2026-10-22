//! Direct quadratic encoding of OptimalLinearArrangement<SimpleGraph>.
use crate::models::algebraic::QUBO;
use crate::models::graph::OptimalLinearArrangement;
use crate::rules::qubo_helpers::{decode_rows, ObjectiveEncoding, QuboBuilder};
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::Min;

#[derive(Debug, Clone)]
pub struct ReductionOptimalLinearArrangementToQUBO {
    encoding: ObjectiveEncoding,
    num_vertices: usize,
}

impl ReductionResult for ReductionOptimalLinearArrangementToQUBO {
    type Source = OptimalLinearArrangement<SimpleGraph>;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_solution(&self, bits: &Vec<bool>) -> ExtractionResult<Vec<usize>> {
        self.encoding.validate(bits)?;
        decode_rows(
            bits,
            std::iter::repeat_n(self.num_vertices, self.num_vertices),
        )
    }
}

#[crate::aggregate_reduction]
impl crate::rules::AggregateReductionResult for ReductionOptimalLinearArrangementToQUBO {
    type Source = OptimalLinearArrangement<SimpleGraph>;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_value(&self, value: Min<i64>) -> Min<i64> {
        self.encoding.value(value)
    }
}

#[crate::reduction(transform = {
    exact { num_vars = "num_vertices^2" },
    upper_bound { num_quadratic_terms = "num_vertices^2 * (num_vertices^2 - 1) / 2" },
})]
impl ReduceTo<QUBO<i64>> for OptimalLinearArrangement<SimpleGraph> {
    type Result = ReductionOptimalLinearArrangementToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<OptimalLinearArrangement<SimpleGraph>, QUBO<i64>>;
        let n = self.num_vertices();
        let mut q = Builder::new(Builder::dimension(n, n)?);
        for (u, v) in self.graph().edges() {
            for p in 0..n {
                for s in 0..n {
                    q.add(u * n + p, v * n + s, p.abs_diff(s) as i128)?;
                }
            }
        }
        let (penalty, low, high) = q.penalty()?;
        q.assignment(n, n, true, penalty)?;
        Ok(ReductionOptimalLinearArrangementToQUBO {
            encoding: q.optimization(low, high)?,
            num_vertices: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "optimallineararrangement_to_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                OptimalLinearArrangement::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)])),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([0, 1, 2]),
                    target_config: serde_json::json!([
                        true, false, false, false, true, false, false, false, true
                    ]),
                },
            )
        },
    }]
}
