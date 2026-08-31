//! Reduction from KColoring to QUBO.
//!
//! One-hot encoding: x_{v,c} = 1 iff vertex v gets color c.
//! QUBO variable index: v * K + c.
//!
//! Integer-scaled one-hot penalty: 2P*sum_v (1 - sum_c x_{v,c})^2
//! Edge penalty: P*sum_{(u,v) in E} sum_c x_{u,c}*x_{v,c}
//!
//! QUBO has n*K variables.

use crate::models::algebraic::QUBO;
use crate::models::graph::KColoring;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::variant::{KValue, K2, K3, KN};

/// Result of reducing KColoring to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionKColoringToQUBO<K: KValue> {
    target: QUBO<i64>,
    num_vertices: usize,
    num_colors: usize,
    _phantom: std::marker::PhantomData<K>,
}

impl<K: KValue> ReductionResult for ReductionKColoringToQUBO<K> {
    type Source = KColoring<K, SimpleGraph>;
    type Target = QUBO<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Decode one-hot: for each vertex, find which color bit is 1.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        (0..self.num_vertices)
            .map(|vertex| {
                let mut selected = (0..self.num_colors)
                    .filter(|&color| target_solution[vertex * self.num_colors + color]);
                match (selected.next(), selected.next()) {
                    (Some(color), None) => Ok(color),
                    (None, _) => Err(crate::rules::ExtractionError::invalid(format!(
                        "assignment row {vertex} has no selected color"
                    ))),
                    (Some(_), Some(_)) => Err(crate::rules::ExtractionError::invalid(format!(
                        "assignment row {vertex} has multiple selected colors"
                    ))),
                }
            })
            .collect()
    }
}

/// Helper function implementing the KColoring to QUBO reduction logic.
fn reduce_kcoloring_to_qubo<K: KValue>(
    problem: &KColoring<K, SimpleGraph>,
) -> Result<ReductionKColoringToQUBO<K>, crate::rules::ReductionError> {
    let k = problem.num_colors();
    let n = problem.graph().num_vertices();
    let edges = problem.graph().edges();
    let overflow = |operation| {
        crate::rules::ReductionError::integer_overflow::<KColoring<K, SimpleGraph>, QUBO<i64>>(
            operation,
        )
    };
    let nq = n
        .checked_mul(k)
        .ok_or_else(|| overflow("computing the number of QUBO variables"))?;

    // Use P = n + 1, then scale the former half-integral objective by two.
    let n_i64 = i64::try_from(n)
        .map_err(|_| overflow("converting the vertex count to a QUBO coefficient"))?;
    let penalty = n_i64
        .checked_add(1)
        .ok_or_else(|| overflow("computing the coloring penalty"))?;
    let diagonal_penalty = penalty
        .checked_mul(-2)
        .ok_or_else(|| overflow("computing a coloring diagonal coefficient"))?;
    let one_hot_interaction = penalty
        .checked_mul(4)
        .ok_or_else(|| overflow("computing a one-hot interaction coefficient"))?;

    let mut matrix = vec![vec![0i64; nq]; nq];

    // Twice the former half-integral objective keeps every coefficient integral.
    // One-hot penalty: 2P*sum_v (1 - sum_c x_{v,c})^2
    // Expanding: (1 - sum_c x_{v,c})^2 = 1 - 2*sum_c x_{v,c} + (sum_c x_{v,c})^2
    // = 1 - 2*sum_c x_{v,c} + sum_c x_{v,c}^2 + 2*sum_{c<c'} x_{v,c}*x_{v,c'}
    // Since x^2 = x for binary: = 1 - sum_c x_{v,c} + 2*sum_{c<c'} x_{v,c}*x_{v,c'}
    for v in 0..n {
        for c in 0..k {
            let idx = v * k + c;
            // Diagonal: -2P
            matrix[idx][idx] = matrix[idx][idx]
                .checked_add(diagonal_penalty)
                .ok_or_else(|| overflow("adding a coloring diagonal coefficient"))?;
        }
        // Off-diagonal within same vertex: 4P for each pair of colors
        for c1 in 0..k {
            for c2 in (c1 + 1)..k {
                let idx1 = v * k + c1;
                let idx2 = v * k + c2;
                matrix[idx1][idx2] = matrix[idx1][idx2]
                    .checked_add(one_hot_interaction)
                    .ok_or_else(|| overflow("adding a one-hot interaction coefficient"))?;
            }
        }
    }

    // Edge penalty: P*sum_{(u,v) in E} sum_c x_{u,c}*x_{v,c}
    for (u, v) in &edges {
        for c in 0..k {
            let idx_u = u * k + c;
            let idx_v = v * k + c;
            let (i, j) = if idx_u < idx_v {
                (idx_u, idx_v)
            } else {
                (idx_v, idx_u)
            };
            matrix[i][j] = matrix[i][j]
                .checked_add(penalty)
                .ok_or_else(|| overflow("adding an edge-conflict coefficient"))?;
        }
    }

    Ok(ReductionKColoringToQUBO {
        target: QUBO::from_matrix(matrix).map_err(|message| {
            crate::rules::ReductionError::construction::<KColoring<K, SimpleGraph>, QUBO<i64>>(
                message,
            )
        })?,
        num_vertices: n,
        num_colors: k,
        _phantom: std::marker::PhantomData,
    })
}

// Register only the KN variant in the reduction graph
#[reduction(
    transform = exact {
        num_vars = "num_vertices * num_colors",
    }
)]
impl ReduceTo<QUBO<i64>> for KColoring<KN, SimpleGraph> {
    type Result = ReductionKColoringToQUBO<KN>;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        reduce_kcoloring_to_qubo(self)
    }
}

// Additional concrete impls for tests (not registered in reduction graph)
macro_rules! impl_kcoloring_to_qubo {
    ($($ktype:ty),+) => {$(
        impl ReduceTo<QUBO<i64>> for KColoring<$ktype, SimpleGraph> {
            type Result = ReductionKColoringToQUBO<$ktype>;
            fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
                reduce_kcoloring_to_qubo(self)
            }
        }
    )+};
}

impl_kcoloring_to_qubo!(K2, K3);

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::algebraic::QUBO;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "kcoloring_to_qubo",
        build: || {
            let (n, edges) = crate::topology::small_graphs::house();
            let source = KColoring::<KN, _>::with_k(SimpleGraph::new(n, edges), 3);
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![1, 2, 2, 1, 0]),
                    target_config: serde_json::json!(vec![
                        false, true, false, false, false, true, false, false, true, false, true,
                        false, true, false, false
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/coloring_qubo.rs"]
mod tests;
