//! Direct exact quadratic encoding of StackerCrane.
use crate::models::algebraic::QUBO;
use crate::models::misc::StackerCrane;
use crate::rules::qubo_helpers::{decode_rows, invert_permutation};
use crate::rules::qubo_helpers::{ObjectiveEncoding, QuboBuilder};
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
use crate::types::Min;
#[derive(Debug, Clone)]
pub struct ReductionStackerCraneToQUBO {
    encoding: ObjectiveEncoding,
    size: usize,
}
impl ReductionResult for ReductionStackerCraneToQUBO {
    type Source = StackerCrane;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_solution(
        &self,
        bits: &Vec<bool>,
    ) -> ExtractionResult<<Self::Source as crate::Problem>::Solution> {
        self.encoding.validate(bits)?;
        invert_permutation(&decode_rows(
            bits,
            std::iter::repeat_n(self.size, self.size),
        )?)
    }
}
#[crate::aggregate_reduction]
impl crate::rules::AggregateReductionResult for ReductionStackerCraneToQUBO {
    type Source = StackerCrane;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_value(&self, value: Min<i64>) -> Min<i64> {
        self.encoding.value(value)
    }
}
#[crate::reduction(transform = { exact { num_vars = "num_arcs^2" }, upper_bound { num_quadratic_terms = "num_arcs^4" } })]
impl ReduceTo<QUBO<i64>> for StackerCrane {
    type Result = ReductionStackerCraneToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<StackerCrane, QUBO<i64>>;
        let n = self.num_arcs();
        let mut q = Builder::new(Builder::dimension(n, n)?);
        let adjacency = self.mixed_graph_adjacency();
        let mut distances = vec![vec![None; n]; n];
        let mut max_distance = 0i64;
        for (i, &(_, head)) in self.arcs().iter().enumerate() {
            for (j, &(tail, _)) in self.arcs().iter().enumerate() {
                let distance = self.shortest_path_length(&adjacency, head, tail);
                if let Some(d) = distance {
                    max_distance = max_distance.max(d);
                }
                distances[i][j] = distance;
            }
        }
        let base = self
            .arc_lengths()
            .iter()
            .try_fold(0i64, |s, &w| s.checked_add(w))
            .ok_or_else(Builder::overflow)?;
        i64::try_from(n)
            .ok()
            .and_then(|n| n.checked_mul(max_distance))
            .and_then(|travel| base.checked_add(travel))
            .ok_or_else(Builder::overflow)?;
        q.add_constant(i128::from(base))?;
        for (i, row) in distances.iter().enumerate() {
            for (j, &distance) in row.iter().enumerate() {
                if i == j && n > 1 {
                    continue;
                }
                if let Some(d) = distance {
                    for p in 0..n {
                        q.add(i * n + p, j * n + (p + 1) % n, i128::from(d))?;
                    }
                }
            }
        }
        let (penalty, low, high) = q.penalty()?;
        q.assignment(n, n, true, penalty)?;
        for (i, row) in distances.iter().enumerate() {
            for (j, distance) in row.iter().enumerate() {
                if i == j && n > 1 {
                    continue;
                }
                if distance.is_none() {
                    for p in 0..n {
                        q.add(i * n + p, j * n + (p + 1) % n, i128::from(penalty))?;
                    }
                }
            }
        }
        Ok(ReductionStackerCraneToQUBO {
            encoding: q.optimization(low, high)?,
            size: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "stackercrane_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                StackerCrane::new(3, vec![(0, 1), (1, 2), (2, 0)], vec![], vec![1; 3], vec![]),
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
