//! Reduction from KColoring to QUBO.
//!
//! One-hot encoding: x_{v,c} = 1 iff vertex v gets color c.
//! QUBO variable index: v * K + c.
//!
//! One-hot penalty: P1*sum_v (1 - sum_c x_{v,c})^2
//! Edge penalty: P2*sum_{(u,v) in E} sum_c x_{u,c}*x_{v,c}
//!
//! QUBO has n*K variables.

use crate::models::graph::KColoring;
use crate::models::optimization::QUBO;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::variant::{KValue, K2, K3, KN};

/// Result of reducing KColoring to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionKColoringToQUBO<K: KValue> {
    target: QUBO<f64>,
    num_vertices: usize,
    num_colors: usize,
    _phantom: std::marker::PhantomData<K>,
}

impl<K: KValue> ReductionResult for ReductionKColoringToQUBO<K> {
    type Source = KColoring<K, SimpleGraph>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Decode one-hot: for each vertex, find which color bit is 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let k = self.num_colors;
        (0..self.num_vertices)
            .map(|v| {
                (0..k)
                    .find(|&c| target_solution[v * k + c] == 1)
                    .unwrap_or(0)
            })
            .collect()
    }
}

/// Helper function implementing the KColoring to QUBO reduction logic.
fn reduce_kcoloring_to_qubo<K: KValue>(
    problem: &KColoring<K, SimpleGraph>,
) -> ReductionKColoringToQUBO<K> {
    let k = problem.num_colors();
    let n = problem.graph().num_vertices();
    let edges = problem.graph().edges();
    let nq = n * k;

    // Penalty must be large enough to enforce one-hot constraints
    // P1 for one-hot, P2 for edge conflicts; use same penalty
    let penalty = 1.0 + n as f64;

    let mut matrix = vec![vec![0.0; nq]; nq];

    // One-hot penalty: P1*sum_v (1 - sum_c x_{v,c})^2
    // Expanding: (1 - sum_c x_{v,c})^2 = 1 - 2*sum_c x_{v,c} + (sum_c x_{v,c})^2
    // = 1 - 2*sum_c x_{v,c} + sum_c x_{v,c}^2 + 2*sum_{c<c'} x_{v,c}*x_{v,c'}
    // Since x^2 = x for binary: = 1 - sum_c x_{v,c} + 2*sum_{c<c'} x_{v,c}*x_{v,c'}
    for v in 0..n {
        for c in 0..k {
            let idx = v * k + c;
            // Diagonal: -P1 (from the linear term -sum_c x_{v,c})
            matrix[idx][idx] -= penalty;
        }
        // Off-diagonal within same vertex: 2*P1 for each pair of colors
        for c1 in 0..k {
            for c2 in (c1 + 1)..k {
                let idx1 = v * k + c1;
                let idx2 = v * k + c2;
                matrix[idx1][idx2] += 2.0 * penalty;
            }
        }
    }

    // Edge penalty: P2*sum_{(u,v) in E} sum_c x_{u,c}*x_{v,c}
    let edge_penalty = penalty / 2.0;
    for (u, v) in &edges {
        for c in 0..k {
            let idx_u = u * k + c;
            let idx_v = v * k + c;
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
        num_vertices: n,
        num_colors: k,
        _phantom: std::marker::PhantomData,
    }
}

// Register only the KN variant in the reduction graph
#[reduction(
    overhead = { ReductionOverhead::new(vec![("num_vars", "num_vertices * num_colors")]) }
)]
impl ReduceTo<QUBO<f64>> for KColoring<KN, SimpleGraph> {
    type Result = ReductionKColoringToQUBO<KN>;

    fn reduce_to(&self) -> Self::Result {
        reduce_kcoloring_to_qubo(self)
    }
}

// Additional concrete impls for tests (not registered in reduction graph)
macro_rules! impl_kcoloring_to_qubo {
    ($($ktype:ty),+) => {$(
        impl ReduceTo<QUBO<f64>> for KColoring<$ktype, SimpleGraph> {
            type Result = ReductionKColoringToQUBO<$ktype>;
            fn reduce_to(&self) -> Self::Result { reduce_kcoloring_to_qubo(self) }
        }
    )+};
}

impl_kcoloring_to_qubo!(K2, K3);

#[cfg(test)]
#[path = "../unit_tests/rules/coloring_qubo.rs"]
mod tests;
