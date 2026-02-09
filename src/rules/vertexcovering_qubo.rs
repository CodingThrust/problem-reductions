//! Reduction from VertexCovering to QUBO.
//!
//! Minimize Σ w_i·x_i s.t. x_i + x_j ≥ 1 for (i,j) ∈ E
//! = Minimize Σ w_i·x_i + P·Σ_{(i,j)∈E} (1-x_i)(1-x_j)
//!
//! Expanding: Q[i][i] = w_i - P·deg(i), Q[i][j] = P for edges.
//! P = 1 + Σ w_i.

use crate::models::graph::VertexCovering;
use crate::models::optimization::QUBO;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::ProblemSize;

/// Result of reducing VertexCovering to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionVCToQUBO {
    target: QUBO<f64>,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionVCToQUBO {
    type Source = VertexCovering<SimpleGraph, i32>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
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
    overhead = { ReductionOverhead::new(vec![("num_vars", poly!(num_vertices))]) }
)]
impl ReduceTo<QUBO<f64>> for VertexCovering<SimpleGraph, i32> {
    type Result = ReductionVCToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let edges = self.edges();
        let weights = self.weights_ref();
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
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/vertexcovering_qubo.rs"]
mod tests;
