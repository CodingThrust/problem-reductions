//! Reduction from MinimumFeedbackVertexSet to ILP (Integer Linear Programming).
//!
//! Uses MTZ-style topological ordering constraints:
//! - Variables: n binary x_i (vertex removal) + n integer o_i (topological order) = 2n total
//! - Constraints: For each arc (u->v): o_v - o_u >= 1 - n*(x_u + x_v)
//!   Plus binary bounds (x_i <= 1) and order bounds (o_i <= n-1)
//! - Objective: Minimize the weighted sum of removed vertices

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MinimumFeedbackVertexSet;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::i64_to_exact_f64;

/// Result of reducing MinimumFeedbackVertexSet to ILP.
///
/// The ILP uses integer variables (`ILP<i64>`) because it needs both
/// binary selection variables (x_i) and integer ordering variables (o_i).
///
/// Variable layout:
/// - `x_i` at index `i` for `i in 0..n`: binary (0 or 1), vertex removal indicator
/// - `o_i` at index `n + i` for `i in 0..n`: integer in {0, ..., n-1}, topological order
#[derive(Debug, Clone)]
pub struct ReductionMFVSToILP {
    target: ILP<i64>,
    /// Number of vertices in the source graph (needed for solution extraction).
    num_vertices: usize,
}

impl ReductionResult for ReductionMFVSToILP {
    type Source = MinimumFeedbackVertexSet<i64>;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &ILP<i64> {
        &self.target
    }

    /// Extract solution from ILP back to MinimumFeedbackVertexSet.
    ///
    /// The first n variables of the ILP solution are the binary x_i values,
    /// which directly correspond to the FVS configuration (1 = removed).
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution[..self.num_vertices]
            .iter()
            .map(|&value| value == 1)
            .collect())
    }
}

#[reduction(
    size = exact {
        num_vars = "2 * num_vertices",
        num_constraints = "num_arcs + 2 * num_vertices",
    },
)]
impl ReduceTo<ILP<i64>> for MinimumFeedbackVertexSet<i64> {
    type Result = ReductionMFVSToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.graph().num_vertices();
        let arcs = self.graph().arcs();
        let num_vars = 2 * n;

        // Variable indices:
        // x_i = i         (binary: vertex i removed?)
        // o_i = n + i     (integer: topological order of vertex i)

        let mut constraints = Vec::new();
        let n_i64 = <Self as ReduceTo<ILP<i64>>>::exact_i64(n, "encoding the topological order")?;

        // Binary bounds: x_i <= 1 for i in 0..n
        for i in 0..n {
            constraints.push(LinearConstraint::le(vec![(i, 1)], 1));
        }

        // Order bounds: o_i <= n - 1 for i in 0..n
        for i in 0..n {
            constraints.push(LinearConstraint::le(vec![(n + i, 1)], n_i64 - 1));
        }

        // Arc constraints: for each arc (u -> v):
        //   o_v - o_u >= 1 - n * (x_u + x_v)
        // Rearranged: o_v - o_u + n*x_u + n*x_v >= 1
        for &(u, v) in &arcs {
            let terms = vec![
                (n + v, 1),  // o_v
                (n + u, -1), // -o_u
                (u, n_i64),  // n * x_u
                (v, n_i64),  // n * x_v
            ];
            constraints.push(LinearConstraint::ge(terms, 1));
        }

        // Objective: minimize sum w_i * x_i
        let objective: Vec<(usize, f64)> = self
            .weights()
            .iter()
            .enumerate()
            .map(|(vertex, &weight)| Ok((vertex, i64_to_exact_f64(weight)?)))
            .collect::<Result<_, crate::types::ExactI64ToF64Error>>()
            .map_err(|error| {
                crate::rules::ReductionError::inexact_float_conversion::<
                    MinimumFeedbackVertexSet<i64>,
                    ILP<i64>,
                >(error)
            })?;

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
            .map_err(<Self as ReduceTo<ILP<i64>>>::target_construction)?;

        Ok(ReductionMFVSToILP {
            target,
            num_vertices: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::topology::DirectedGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumfeedbackvertexset_to_ilp",
        build: || {
            // Simple cycle: 0 -> 1 -> 2 -> 0 (FVS = 1 vertex)
            let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
            let source = MinimumFeedbackVertexSet::new(graph, vec![1i64; 3]);
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumfeedbackvertexset_ilp.rs"]
mod tests;
