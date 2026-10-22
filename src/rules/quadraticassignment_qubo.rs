//! Direct quadratic encoding of QuadraticAssignment.
use crate::models::algebraic::QuadraticAssignment;
use crate::models::algebraic::QUBO;
use crate::rules::qubo_helpers::{decode_rows, ObjectiveEncoding, QuboBuilder};
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};
use crate::types::Min;

#[derive(Debug, Clone)]
pub struct ReductionQuadraticAssignmentToQUBO {
    encoding: ObjectiveEncoding,
    rows: usize,
    cols: usize,
}

impl ReductionResult for ReductionQuadraticAssignmentToQUBO {
    type Source = QuadraticAssignment;
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
impl crate::rules::AggregateReductionResult for ReductionQuadraticAssignmentToQUBO {
    type Source = QuadraticAssignment;
    type Target = QUBO<i64>;
    fn target_problem(&self) -> &Self::Target {
        &self.encoding.target
    }
    fn extract_value(&self, value: Min<i64>) -> Min<i64> {
        self.encoding.value(value)
    }
}

#[crate::reduction(transform = {
    exact { num_vars = "num_facilities * num_locations" },
    upper_bound { num_quadratic_terms = "num_facilities * num_locations * (num_facilities * num_locations - 1) / 2" },
})]
impl ReduceTo<QUBO<i64>> for QuadraticAssignment {
    type Result = ReductionQuadraticAssignmentToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<QuadraticAssignment, QUBO<i64>>;
        let n = self.num_facilities();
        let m = self.num_locations();
        let mut q = Builder::new(Builder::dimension(n, m)?);
        let (mut negative, mut positive) = (0i64, 0i64);
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let (mut low, mut high) = (0i64, 0i64);
                    for p in 0..m {
                        for s in 0..m {
                            let cost = self.cost_matrix()[i][j]
                                .checked_mul(self.distance_matrix()[p][s])
                                .ok_or_else(Builder::overflow)?;
                            low = low.min(cost);
                            high = high.max(cost);
                            q.add(i * m + p, j * m + s, i128::from(cost))?;
                        }
                    }
                    // Directed source costs are summed before opposite terms can cancel.
                    negative = negative.checked_add(low).ok_or_else(Builder::overflow)?;
                    positive = positive.checked_add(high).ok_or_else(Builder::overflow)?;
                }
            }
        }
        let (penalty, low, high) = q.penalty()?;
        q.assignment(n, m, false, penalty)?;
        for p in 0..m {
            for i in 0..n {
                for j in i + 1..n {
                    q.add(i * m + p, j * m + p, i128::from(penalty))?;
                }
            }
        }
        Ok(ReductionQuadraticAssignmentToQUBO {
            encoding: q.optimization(low, high)?,
            rows: n,
            cols: m,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "quadraticassignment_to_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                QuadraticAssignment::new(
                    vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]],
                    vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]],
                ),
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
