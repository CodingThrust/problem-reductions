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
use crate::types::i64_to_exact_f64;

/// Result of reducing MinimumFeedbackArcSet to ILP.
///
/// The ILP uses integer variables (`ILP<i64>`) because it needs both
/// binary arc-removal variables (y_a) and integer ordering variables (o_v).
///
/// Variable layout:
/// - `y_a` at index `a` for `a in 0..m`: binary (0 or 1), arc removal indicator
/// - `o_v` at index `m + v` for `v in 0..n`: integer in {0, ..., n-1}, topological order
#[derive(Debug, Clone)]
pub struct ReductionFASToILP {
    target: ILP<i64>,
    /// Number of arcs in the source graph (needed for solution extraction).
    num_arcs: usize,
}

impl ReductionResult for ReductionFASToILP {
    type Source = MinimumFeedbackArcSet<i64>;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &ILP<i64> {
        &self.target
    }

    /// Extract solution from ILP back to MinimumFeedbackArcSet.
    ///
    /// The first m variables of the ILP solution are the binary y_a values,
    /// which directly correspond to the FAS configuration (1 = removed).
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution[..self.num_arcs]
            .iter()
            .map(|&value| value == 1)
            .collect())
    }
}

#[reduction(
    size = exact {
        num_vars = "num_arcs + num_vertices",
        num_constraints = "num_arcs + num_arcs + num_vertices",
    },
)]
impl ReduceTo<ILP<i64>> for MinimumFeedbackArcSet<i64> {
    type Result = ReductionFASToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let m = self.num_arcs();
        let arcs = self.graph().arcs();
        let num_vars = m + n;

        // Variable indices:
        // y_a = a         (binary: arc a removed?)
        // o_v = m + v     (integer: topological order of vertex v)

        let mut constraints = Vec::new();
        let n_i64 = Self::exact_i64(n, "encoding the topological order")?;

        // Binary bounds: y_a <= 1 for a in 0..m
        for a in 0..m {
            constraints.push(LinearConstraint::le(vec![(a, 1)], 1));
        }

        // Order bounds: o_v <= n - 1 for v in 0..n
        for v in 0..n {
            constraints.push(LinearConstraint::le(vec![(m + v, 1)], n_i64 - 1));
        }

        // Arc constraints: for each arc a = (u -> v):
        //   o_v - o_u >= 1 - n * y_a
        // Rearranged: o_v - o_u + n * y_a >= 1
        for (a, &(u, v)) in arcs.iter().enumerate() {
            let terms = vec![
                (m + v, 1),  // o_v
                (m + u, -1), // -o_u
                (a, n_i64),  // n * y_a
            ];
            constraints.push(LinearConstraint::ge(terms, 1));
        }

        // Objective: minimize sum w_a * y_a
        let objective: Vec<(usize, f64)> = self
            .weights()
            .iter()
            .enumerate()
            .map(|(arc, &weight)| Ok((arc, i64_to_exact_f64(weight)?)))
            .collect::<Result<_, crate::types::ExactI64ToF64Error>>()
            .map_err(|error| {
                crate::rules::ReductionError::inexact_float_conversion::<
                    MinimumFeedbackArcSet<i64>,
                    ILP<i64>,
                >(error)
            })?;

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;

        Ok(ReductionFASToILP {
            target,
            num_arcs: m,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::topology::DirectedGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumfeedbackarcset_to_ilp",
        build: || {
            // Simple cycle: 0 -> 1 -> 2 -> 0 (FAS = 1 arc)
            // 3 arcs, 3 vertices: 6 total variables
            // Remove arc 2 (2->0): source_config = [0, 0, 1]
            // ILP solution: y_0=0, y_1=0, y_2=1, o_0=0, o_1=1, o_2=2
            let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
            let source = MinimumFeedbackArcSet::new(graph, vec![1i64; 3]);
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumfeedbackarcset_ilp.rs"]
mod tests;
