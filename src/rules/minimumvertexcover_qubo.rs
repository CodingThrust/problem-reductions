//! Reduction from MinimumVertexCover to QUBO.
//!
//! Minimize Σ w_i·x_i s.t. x_i + x_j ≥ 1 for (i,j) ∈ E
//! = Minimize Σ w_i·x_i + P·Σ_{(i,j)∈E} (1-x_i)(1-x_j)
//!
//! Expanding: Q[i][i] = w_i - P·deg(i), Q[i][j] = P for edges.
//! P = 1 + Σ w_i.

use crate::models::graph::MinimumVertexCover;
use crate::models::optimization::QUBO;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MinimumVertexCover to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionVCToQUBO {
    target: QUBO<f64>,
}

impl ReductionResult for ReductionVCToQUBO {
    type Source = MinimumVertexCover<SimpleGraph, i32>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = { ReductionOverhead::new(vec![("num_vars", poly!(num_vertices))]) }
)]
impl ReduceTo<QUBO<f64>> for MinimumVertexCover<SimpleGraph, i32> {
    type Result = ReductionVCToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();
        let edges = self.graph().edges();
        let weights = self.weights();
        let total_weight: f64 = weights.iter().map(|&w| w as f64).sum();
        let penalty = 1.0 + total_weight;

        let mut matrix = vec![vec![0.0; n]; n];

        // Compute degree of each vertex
        let mut degree = vec![0usize; n];
        for (u, v) in &edges {
            degree[*u] += 1;
            degree[*v] += 1;
        }

        // Diagonal: w_i - P * deg(i)
        for i in 0..n {
            matrix[i][i] = weights[i] as f64 - penalty * degree[i] as f64;
        }

        // Off-diagonal: P for each edge
        for (u, v) in &edges {
            let (i, j) = if u < v { (*u, *v) } else { (*v, *u) };
            matrix[i][j] += penalty;
        }

        ReductionVCToQUBO {
            target: QUBO::from_matrix(matrix),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_qubo.rs"]
mod tests;
