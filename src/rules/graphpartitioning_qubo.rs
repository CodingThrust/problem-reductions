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

/// Result of reducing GraphPartitioning to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionGraphPartitioningToQUBO {
    target: QUBO<i64>,
}

impl ReductionResult for ReductionGraphPartitioningToQUBO {
    type Source = GraphPartitioning<SimpleGraph>;
    type Target = QUBO<i64>;

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

#[reduction(transform = exact {
    num_vars = "num_vertices",
})]
impl ReduceTo<QUBO<i64>> for GraphPartitioning<SimpleGraph> {
    type Result = ReductionGraphPartitioningToQUBO;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let overflow = |operation| {
            crate::rules::ReductionError::integer_overflow::<Self, QUBO<i64>>(operation)
        };
        let n_i64 = i64::try_from(n)
            .map_err(|_| overflow("converting the vertex count to a QUBO coefficient"))?;
        let edge_count = i64::try_from(self.num_edges())
            .map_err(|_| overflow("converting the edge count to a QUBO coefficient"))?;
        let penalty = edge_count
            .checked_add(1)
            .ok_or_else(|| overflow("computing the balance penalty"))?;
        let mut matrix = vec![vec![0i64; n]; n];
        let mut degrees = vec![0usize; n];
        let edges = self.graph().edges();

        for &(u, v) in &edges {
            degrees[u] += 1;
            degrees[v] += 1;
        }

        for (i, row) in matrix.iter_mut().enumerate() {
            let degree = i64::try_from(degrees[i])
                .map_err(|_| overflow("converting a vertex degree to a QUBO coefficient"))?;
            let balance_linear = penalty
                .checked_mul(
                    1i64.checked_sub(n_i64)
                        .ok_or_else(|| overflow("computing a balance coefficient"))?,
                )
                .ok_or_else(|| overflow("computing a balance coefficient"))?;
            row[i] = degree
                .checked_add(balance_linear)
                .ok_or_else(|| overflow("combining QUBO diagonal coefficients"))?;
            for value in row.iter_mut().skip(i + 1) {
                *value = penalty
                    .checked_mul(2)
                    .ok_or_else(|| overflow("computing a balance interaction coefficient"))?;
            }
        }

        for (u, v) in edges {
            let (lo, hi) = if u < v { (u, v) } else { (v, u) };
            matrix[lo][hi] = matrix[lo][hi]
                .checked_sub(2)
                .ok_or_else(|| overflow("adding a cut interaction coefficient"))?;
        }

        Ok(ReductionGraphPartitioningToQUBO {
            target: QUBO::from_matrix(matrix).map_err(|message| {
                crate::rules::ReductionError::construction::<
                    GraphPartitioning<SimpleGraph>,
                    QUBO<i64>,
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
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
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
