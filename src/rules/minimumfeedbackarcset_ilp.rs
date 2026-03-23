//! Reduction from MinimumFeedbackArcSet to ILP (Integer Linear Programming).
//!
//! Uses MTZ-style topological ordering constraints on arcs:
//! - Variables: |A| binary y_a (arc removal) + |V| integer o_v (topological order)
//! - Constraints:
//!   - For each arc a=(u→v): o_v - o_u + n*y_a >= 1
//!   - Binary bounds: y_a <= 1 for all arcs
//!   - Order bounds: o_v <= n-1 for all vertices
//! - Objective: Minimize Σ w_a * y_a
//! - Variable layout: first |A| are y_a, next |V| are o_v

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MinimumFeedbackArcSet;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing MinimumFeedbackArcSet to ILP.
///
/// The ILP uses integer variables (`ILP<i32>`) because it needs both
/// binary arc-removal variables (y_a) and integer ordering variables (o_v).
///
/// Variable layout:
/// - `y_a` at index `a` for `a in 0..m`: binary (0 or 1), arc removal indicator
/// - `o_v` at index `m + v` for `v in 0..n`: integer in {0, ..., n-1}, topological order
#[derive(Debug, Clone)]
pub struct ReductionFASToILP {
    target: ILP<i32>,
    /// Number of arcs in the source graph (needed for solution extraction).
    num_arcs: usize,
}

impl ReductionResult for ReductionFASToILP {
    type Source = MinimumFeedbackArcSet<i32>;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Extract solution from ILP back to MinimumFeedbackArcSet.
    ///
    /// The first m variables of the ILP solution are the binary y_a values,
    /// which directly correspond to the FAS configuration (1 = removed).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_arcs].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_arcs + num_vertices",
        num_constraints = "num_arcs + num_arcs + num_vertices",
    }
)]
impl ReduceTo<ILP<i32>> for MinimumFeedbackArcSet<i32> {
    type Result = ReductionFASToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let m = self.num_arcs();
        let arcs = self.graph().arcs();
        let num_vars = m + n;

        // Variable indices:
        // y_a = a         (binary: arc a removed?)
        // o_v = m + v     (integer: topological order of vertex v)

        let mut constraints = Vec::new();

        // Binary bounds: y_a <= 1 for a in 0..m
        for a in 0..m {
            constraints.push(LinearConstraint::le(vec![(a, 1.0)], 1.0));
        }

        // Order bounds: o_v <= n - 1 for v in 0..n
        for v in 0..n {
            constraints.push(LinearConstraint::le(vec![(m + v, 1.0)], (n - 1) as f64));
        }

        // Arc constraints: for each arc a = (u -> v):
        //   o_v - o_u >= 1 - n * y_a
        // Rearranged: o_v - o_u + n * y_a >= 1
        let n_f64 = n as f64;
        for (a, &(u, v)) in arcs.iter().enumerate() {
            let terms = vec![
                (m + v, 1.0),  // o_v
                (m + u, -1.0), // -o_u
                (a, n_f64),    // n * y_a
            ];
            constraints.push(LinearConstraint::ge(terms, 1.0));
        }

        // Objective: minimize sum w_a * y_a
        let objective: Vec<(usize, f64)> = self
            .weights()
            .iter()
            .enumerate()
            .map(|(a, &w)| (a, w as f64))
            .collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionFASToILP {
            target,
            num_arcs: m,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::topology::DirectedGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumfeedbackarcset_to_ilp",
        build: || {
            // Simple cycle: 0 -> 1 -> 2 -> 0 (FAS = 1 arc)
            // 3 arcs, 3 vertices: 6 total variables
            // Remove arc 2 (2->0): source_config = [0, 0, 1]
            // ILP solution: y_0=0, y_1=0, y_2=1, o_0=0, o_1=1, o_2=2
            let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
            let source = MinimumFeedbackArcSet::new(graph, vec![1i32; 3]);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<i32>>(
                source,
                SolutionPair {
                    source_config: vec![0, 0, 1],
                    target_config: vec![0, 0, 1, 0, 1, 2],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumfeedbackarcset_ilp.rs"]
mod tests;
