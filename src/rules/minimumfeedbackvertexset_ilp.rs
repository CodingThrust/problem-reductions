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

/// Result of reducing MinimumFeedbackVertexSet to ILP.
///
/// The ILP uses integer variables (`ILP<i32>`) because it needs both
/// binary selection variables (x_i) and integer ordering variables (o_i).
///
/// Variable layout:
/// - `x_i` at index `i` for `i in 0..n`: binary (0 or 1), vertex removal indicator
/// - `o_i` at index `n + i` for `i in 0..n`: integer in {0, ..., n-1}, topological order
#[derive(Debug, Clone)]
pub struct ReductionMFVSToILP {
    target: ILP<i32>,
    /// Number of vertices in the source graph (needed for solution extraction).
    num_vertices: usize,
}

impl ReductionResult for ReductionMFVSToILP {
    type Source = MinimumFeedbackVertexSet<i32>;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Extract solution from ILP back to MinimumFeedbackVertexSet.
    ///
    /// The first n variables of the ILP solution are the binary x_i values,
    /// which directly correspond to the FVS configuration (1 = removed).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_vertices].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "2 * num_vertices",
        num_constraints = "num_arcs + 2 * num_vertices",
    }
)]
impl ReduceTo<ILP<i32>> for MinimumFeedbackVertexSet<i32> {
    type Result = ReductionMFVSToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();
        let arcs = self.graph().arcs();
        let num_vars = 2 * n;

        // Variable indices:
        // x_i = i         (binary: vertex i removed?)
        // o_i = n + i     (integer: topological order of vertex i)

        let mut constraints = Vec::new();

        // Binary bounds: x_i <= 1 for i in 0..n
        for i in 0..n {
            constraints.push(LinearConstraint::le(vec![(i, 1.0)], 1.0));
        }

        // Order bounds: o_i <= n - 1 for i in 0..n
        for i in 0..n {
            constraints.push(LinearConstraint::le(vec![(n + i, 1.0)], (n - 1) as f64));
        }

        // Arc constraints: for each arc (u -> v):
        //   o_v - o_u >= 1 - n * (x_u + x_v)
        // Rearranged: o_v - o_u + n*x_u + n*x_v >= 1
        let n_f64 = n as f64;
        for &(u, v) in &arcs {
            let terms = vec![
                (n + v, 1.0),  // o_v
                (n + u, -1.0), // -o_u
                (u, n_f64),    // n * x_u
                (v, n_f64),    // n * x_v
            ];
            constraints.push(LinearConstraint::ge(terms, 1.0));
        }

        // Objective: minimize sum w_i * x_i
        let objective: Vec<(usize, f64)> = self
            .weights()
            .iter()
            .enumerate()
            .map(|(i, &w)| (i, w as f64))
            .collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionMFVSToILP {
            target,
            num_vertices: n,
        }
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
            let source = MinimumFeedbackVertexSet::new(graph, vec![1i32; 3]);
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumfeedbackvertexset_ilp.rs"]
mod tests;
