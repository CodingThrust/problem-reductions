//! Reduction from MaxCut to QUBO.
//!
//! Maximize Σ_{(i,j)∈E} w_ij·(x_i ⊕ x_j)
//! = Maximize Σ w_ij·(x_i + x_j - 2·x_i·x_j)
//! = Minimize -Σ w_ij·(x_i + x_j - 2·x_i·x_j)
//!
//! Upper-triangular QUBO:
//! Q[i][i] = -Σ_{j:(i,j)∈E} w_ij, Q[i][j] = 2·w_ij for edges (i<j).
//! No penalty needed — MaxCut is unconstrained.

use crate::models::graph::MaxCut;
use crate::models::optimization::QUBO;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::ProblemSize;

/// Result of reducing MaxCut to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionMaxCutToQUBO {
    target: QUBO<f64>,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionMaxCutToQUBO {
    type Source = MaxCut<SimpleGraph, i32>;
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
impl ReduceTo<QUBO<f64>> for MaxCut<SimpleGraph, i32> {
    type Result = ReductionMaxCutToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let edges = self.edges();

        let mut matrix = vec![vec![0.0; n]; n];
        for (u, v, w) in &edges {
            let weight = *w as f64;
            let (i, j) = if u < v { (*u, *v) } else { (*v, *u) };
            matrix[i][i] -= weight;
            matrix[j][j] -= weight;
            matrix[i][j] += 2.0 * weight;
        }

        ReductionMaxCutToQUBO {
            target: QUBO::from_matrix(matrix),
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/maxcut_qubo.rs"]
mod tests;
