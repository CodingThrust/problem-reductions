//! Tournament comparisons encode the vertices between each edge's endpoints.
use crate::models::algebraic::QUBO;
use crate::models::graph::OptimalLinearArrangement;
use crate::rules::qubo_helpers::{ObjectiveEncoding, QuboBuilder, pair_index, tournament_ranks};
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
        tournament_ranks(self.num_vertices, bits)
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
    exact { num_vars = "num_vertices * (num_vertices - 1) / 2" },
    upper_bound { num_quadratic_terms = "num_vertices * (num_vertices - 1) * (num_vertices - 2) / 2" },
})]
impl ReduceTo<QUBO<i64>> for OptimalLinearArrangement<SimpleGraph> {
    type Result = ReductionOptimalLinearArrangementToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<OptimalLinearArrangement<SimpleGraph>, QUBO<i64>>;
        let n = self.num_vertices();
        let mut q = Builder::new(Builder::dimension(n, n.saturating_sub(1))? / 2);
        let edges =
            <Self as ReduceTo<QUBO<i64>>>::exact_i64(self.num_edges(), "counting graph edges")?;
        let span = <Self as ReduceTo<QUBO<i64>>>::exact_i64(
            n.saturating_sub(1),
            "bounding rank distance",
        )?;
        let high = edges.checked_mul(span).ok_or_else(Builder::overflow)?;
        let penalty = high.checked_add(1).ok_or_else(Builder::overflow)?;
        q.add_constant(i128::from(edges))?;
        for (u, v) in self.graph().edges() {
            for w in 0..n {
                if w == u || w == v {
                    continue;
                }
                let a = pair_index(n, u.min(w), u.max(w));
                let b = pair_index(n, v.min(w), v.max(w));
                let ca = i128::from(u > w);
                let cb = i128::from(v > w);
                let sa = 1 - 2 * ca;
                let sb = 1 - 2 * cb;
                // XOR of the two comparisons identifies a vertex strictly between u and v.
                q.add_constant(ca + cb - 2 * ca * cb)?;
                q.add(a, a, sa * (1 - 2 * cb))?;
                q.add(b, b, sb * (1 - 2 * ca))?;
                q.add(a, b, -2 * sa * sb)?;
            }
        }
        q.tournament(n, &[], penalty)?;
        Ok(ReductionOptimalLinearArrangementToQUBO {
            encoding: q.optimization(0, high)?,
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
                    target_config: serde_json::json!([true, true, true]),
                },
            )
        },
    }]
}
