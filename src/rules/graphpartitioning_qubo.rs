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
    target: QUBO<f64>,
}

impl ReductionResult for ReductionGraphPartitioningToQUBO {
    type Source = GraphPartitioning<SimpleGraph>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = { num_vars = "num_vertices" })]
impl ReduceTo<QUBO<f64>> for GraphPartitioning<SimpleGraph> {
    type Result = ReductionGraphPartitioningToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let penalty = self.num_edges() as f64 + 1.0;
        let mut matrix = vec![vec![0.0f64; n]; n];
        let mut degrees = vec![0usize; n];
        let edges = self.graph().edges();

        for &(u, v) in &edges {
            degrees[u] += 1;
            degrees[v] += 1;
        }

        for i in 0..n {
            matrix[i][i] = degrees[i] as f64 + penalty * (1.0 - n as f64);
        }

        for i in 0..n {
            for j in (i + 1)..n {
                matrix[i][j] = 2.0 * penalty;
            }
        }

        for (u, v) in edges {
            let (lo, hi) = if u < v { (u, v) } else { (v, u) };
            matrix[lo][hi] -= 2.0;
        }

        ReductionGraphPartitioningToQUBO {
            target: QUBO::from_matrix(matrix),
        }
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
                    source_config: vec![0, 0, 0, 1, 1, 1],
                    target_config: vec![0, 0, 0, 1, 1, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/graphpartitioning_qubo.rs"]
mod tests;
