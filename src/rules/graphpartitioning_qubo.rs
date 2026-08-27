//! Reduction from GraphPartitioning to QUBO.
//!
//! Uses the penalty-method QUBO
//! H = sum_(u,v in E) (x_u + x_v - 2 x_u x_v) + P (sum_i x_i - n/2)^2
//! with P = |E| + 1 so any imbalanced partition is dominated by a balanced one.

use crate::models::algebraic::QUBO;
use crate::models::graph::GraphPartitioning;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::i64_to_exact_f64;

/// Result of reducing GraphPartitioning to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionGraphPartitioningToQUBO {
    target: QUBO<f64>,
}

impl ReductionResult for ReductionGraphPartitioningToQUBO {
    type Source = GraphPartitioning<SimpleGraph>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.to_vec())
    }
}

#[reduction(size = exact {
    num_vars = "num_vertices",
})]
impl ReduceTo<QUBO<f64>> for GraphPartitioning<SimpleGraph> {
    type Result = ReductionGraphPartitioningToQUBO;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let to_f64 = |value: usize, operation: &str| {
            let value = i64::try_from(value).map_err(|_| {
                crate::rules::ReductionError::integer_overflow::<Self, QUBO<f64>>(operation)
            })?;
            i64_to_exact_f64(value).map_err(|error| {
                crate::rules::ReductionError::inexact_float_conversion::<Self, QUBO<f64>>(error)
            })
        };
        let n_f64 = to_f64(n, "converting the vertex count to a QUBO coefficient")?;
        let penalty = to_f64(
            self.num_edges(),
            "converting the edge count to a QUBO coefficient",
        )? + 1.0;
        let mut matrix = vec![vec![0.0f64; n]; n];
        let mut degrees = vec![0usize; n];
        let edges = self.graph().edges();

        for &(u, v) in &edges {
            degrees[u] += 1;
            degrees[v] += 1;
        }

        for (i, row) in matrix.iter_mut().enumerate() {
            let degree = to_f64(
                degrees[i],
                "converting a vertex degree to a QUBO coefficient",
            )?;
            row[i] = degree + penalty * (1.0 - n_f64);
            for value in row.iter_mut().skip(i + 1) {
                *value = 2.0 * penalty;
            }
        }

        for (u, v) in edges {
            let (lo, hi) = if u < v { (u, v) } else { (v, u) };
            matrix[lo][hi] -= 2.0;
        }

        Ok(ReductionGraphPartitioningToQUBO {
            target: QUBO::from_matrix(matrix).map_err(|message| {
                crate::rules::ReductionError::construction::<
                    GraphPartitioning<SimpleGraph>,
                    QUBO<f64>,
                >(message)
            })?,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "graphpartitioning_to_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<f64>>(
                GraphPartitioning::new(SimpleGraph::new(
                    6,
                    vec![
                        (0, 1),
                        (0, 2),
                        (1, 2),
                        (1, 3),
                        (2, 3),
                        (2, 4),
                        (3, 4),
                        (3, 5),
                        (4, 5),
                    ],
                )),
                SolutionPair {
                    source_config: serde_json::json!(vec![false, false, false, true, true, true]),
                    target_config: serde_json::json!(vec![false, false, false, true, true, true]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/graphpartitioning_qubo.rs"]
mod tests;
