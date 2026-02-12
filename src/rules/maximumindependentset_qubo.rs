//! Reduction from MaximumIndependentSet to QUBO.
//!
//! Maximize Σ w_i·x_i s.t. x_i·x_j = 0 for (i,j) ∈ E
//! = Minimize -Σ w_i·x_i + P·Σ_{(i,j)∈E} x_i·x_j
//!
//! Q[i][i] = -w_i, Q[i][j] = P for edges. P = 1 + Σ w_i.

use crate::models::graph::MaximumIndependentSet;
use crate::models::optimization::QUBO;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
/// Result of reducing MaximumIndependentSet to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionISToQUBO {
    target: QUBO<f64>,
}

impl ReductionResult for ReductionISToQUBO {
    type Source = MaximumIndependentSet<SimpleGraph, i32>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    source_graph = "SimpleGraph",
    overhead = { ReductionOverhead::new(vec![("num_vars", poly!(num_vertices))]) }
)]
impl ReduceTo<QUBO<f64>> for MaximumIndependentSet<SimpleGraph, i32> {
    type Result = ReductionISToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let edges = self.edges();
        let weights = self.weights_ref();
        let total_weight: f64 = weights.iter().map(|&w| w as f64).sum();
        let penalty = 1.0 + total_weight;

        let mut matrix = vec![vec![0.0; n]; n];
        for i in 0..n {
            matrix[i][i] = -(weights[i] as f64);
        }
        for (u, v) in &edges {
            let (i, j) = if u < v { (*u, *v) } else { (*v, *u) };
            matrix[i][j] += penalty;
        }

        ReductionISToQUBO {
            target: QUBO::from_matrix(matrix),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumindependentset_qubo.rs"]
mod tests;
