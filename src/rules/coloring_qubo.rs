//! Reduction from KColoring to QUBO.
//!
//! One-hot encoding: x_{v,c} = 1 iff vertex v gets color c.
//! QUBO variable index: v * K + c.
//!
//! One-hot penalty: P₁·Σ_v (1 - Σ_c x_{v,c})²
//! Edge penalty: P₂·Σ_{(u,v)∈E} Σ_c x_{u,c}·x_{v,c}
//!
//! QUBO has n·K variables.

use crate::models::graph::KColoring;
use crate::models::optimization::QUBO;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::ProblemSize;

/// Result of reducing KColoring to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionKColoringToQUBO<const K: usize> {
    target: QUBO<f64>,
    source_size: ProblemSize,
    num_vertices: usize,
}

impl<const K: usize> ReductionResult for ReductionKColoringToQUBO<K> {
    type Source = KColoring<K, SimpleGraph, i32>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Decode one-hot: for each vertex, find which color bit is 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        (0..self.num_vertices)
            .map(|v| {
                (0..K)
                    .find(|&c| target_solution[v * K + c] == 1)
                    .unwrap_or(0)
            })
            .collect()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

#[reduction(
    source_graph = "SimpleGraph",
    overhead = { ReductionOverhead::new(vec![("num_vars", poly!(num_vertices * num_colors))]) }
)]
impl<const K: usize> ReduceTo<QUBO<f64>> for KColoring<K, SimpleGraph, i32> {
    type Result = ReductionKColoringToQUBO<K>;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let edges = self.edges();
        let nq = n * K;

        // Penalty must be large enough to enforce one-hot constraints
        // P1 for one-hot, P2 for edge conflicts; use same penalty
        let penalty = 1.0 + n as f64;

        let mut matrix = vec![vec![0.0; nq]; nq];

        // One-hot penalty: P₁·Σ_v (1 - Σ_c x_{v,c})²
        // Expanding: (1 - Σ_c x_{v,c})² = 1 - 2·Σ_c x_{v,c} + (Σ_c x_{v,c})²
        // = 1 - 2·Σ_c x_{v,c} + Σ_c x_{v,c}² + 2·Σ_{c<c'} x_{v,c}·x_{v,c'}
        // Since x² = x for binary: = 1 - Σ_c x_{v,c} + 2·Σ_{c<c'} x_{v,c}·x_{v,c'}
        for v in 0..n {
            for c in 0..K {
                let idx = v * K + c;
                // Diagonal: -P₁ (from the linear term -Σ_c x_{v,c})
                matrix[idx][idx] -= penalty;
            }
            // Off-diagonal within same vertex: 2·P₁ for each pair of colors
            for c1 in 0..K {
                for c2 in (c1 + 1)..K {
                    let idx1 = v * K + c1;
                    let idx2 = v * K + c2;
                    matrix[idx1][idx2] += 2.0 * penalty;
                }
            }
        }

        // Edge penalty: P₂·Σ_{(u,v)∈E} Σ_c x_{u,c}·x_{v,c}
        let edge_penalty = penalty / 2.0;
        for (u, v) in &edges {
            for c in 0..K {
                let idx_u = u * K + c;
                let idx_v = v * K + c;
                let (i, j) = if idx_u < idx_v {
                    (idx_u, idx_v)
                } else {
                    (idx_v, idx_u)
                };
                matrix[i][j] += edge_penalty;
            }
        }

        ReductionKColoringToQUBO {
            target: QUBO::from_matrix(matrix),
            source_size: self.problem_size(),
            num_vertices: n,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/coloring_qubo.rs"]
mod tests;
