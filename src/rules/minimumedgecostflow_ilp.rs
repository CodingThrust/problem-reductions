//! Reduction from MinimumEdgeCostFlow to ILP<i32>.
//!
//! Variables (2m total):
//!   f_a  (a = 0..m-1)  — integer flow on arc a, domain {0, ..., c(a)}
//!   y_a  (a = m..2m-1) — binary indicator: y_a = 1 iff f_a > 0
//!
//! Constraints:
//!   f_a ≤ c(a)          — capacity (m constraints)
//!   f_a ≤ c(a) · y_a    — linking: forces y_a = 1 when f_a > 0 (m constraints)
//!   y_a ≤ 1             — binary bound on indicators (m constraints)
//!   conservation at non-terminal vertices (|V|-2 equality constraints)
//!   net flow into sink ≥ R (1 constraint)
//!
//! Total: 3m + |V| - 1 constraints (but we omit redundant capacity since
//! linking already implies f_a ≤ c(a) when y_a ≤ 1).
//! Actually we keep all for clarity: 2m + |V| - 1 constraints.
//!
//! Objective: minimize Σ p(a) · y_a.
//! Extraction: first m variables are the flow values.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MinimumEdgeCostFlow;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing MinimumEdgeCostFlow to ILP<i32>.
///
/// Variable layout:
/// - `f_a` at index a for a in 0..num_edges (flow on arc a)
/// - `y_a` at index num_edges + a for a in 0..num_edges (binary indicator)
#[derive(Debug, Clone)]
pub struct ReductionMECFToILP {
    target: ILP<i32>,
    num_edges: usize,
}

impl ReductionResult for ReductionMECFToILP {
    type Source = MinimumEdgeCostFlow;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Extract flow solution: first m variables are the flow values.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_edges].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "2 * num_edges",
        num_constraints = "2 * num_edges + num_vertices - 1",
    }
)]
impl ReduceTo<ILP<i32>> for MinimumEdgeCostFlow {
    type Result = ReductionMECFToILP;

    fn reduce_to(&self) -> Self::Result {
        let arcs = self.graph().arcs();
        let m = arcs.len();
        let n = self.num_vertices();
        let num_vars = 2 * m;

        let f = |a: usize| a; // flow variable index
        let y = |a: usize| m + a; // indicator variable index

        let mut constraints = Vec::new();

        // 1. Linking: f_a - c(a) * y_a ≤ 0  (forces y_a = 1 when f_a > 0)
        for a in 0..m {
            constraints.push(LinearConstraint::le(
                vec![(f(a), 1.0), (y(a), -(self.capacities()[a] as f64))],
                0.0,
            ));
        }

        // 2. Binary bound: y_a ≤ 1
        for a in 0..m {
            constraints.push(LinearConstraint::le(vec![(y(a), 1.0)], 1.0));
        }

        // 3. Flow conservation at non-terminal vertices
        for vertex in 0..n {
            if vertex == self.source() || vertex == self.sink() {
                continue;
            }

            let mut terms: Vec<(usize, f64)> = Vec::new();
            for (a, &(u, v)) in arcs.iter().enumerate() {
                if vertex == u {
                    terms.push((f(a), -1.0)); // outgoing
                } else if vertex == v {
                    terms.push((f(a), 1.0)); // incoming
                }
            }

            if !terms.is_empty() {
                constraints.push(LinearConstraint::eq(terms, 0.0));
            }
        }

        // 4. Flow requirement: net flow into sink ≥ R
        let sink = self.sink();
        let mut sink_terms: Vec<(usize, f64)> = Vec::new();
        for (a, &(u, v)) in arcs.iter().enumerate() {
            if v == sink {
                sink_terms.push((f(a), 1.0));
            } else if u == sink {
                sink_terms.push((f(a), -1.0));
            }
        }
        constraints.push(LinearConstraint::ge(
            sink_terms,
            self.required_flow() as f64,
        ));

        // Objective: minimize Σ p(a) · y_a
        let objective: Vec<(usize, f64)> =
            (0..m).map(|a| (y(a), self.prices()[a] as f64)).collect();

        ReductionMECFToILP {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize),
            num_edges: m,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::topology::DirectedGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumedgecostflow_to_ilp",
        build: || {
            let source = MinimumEdgeCostFlow::new(
                DirectedGraph::new(5, vec![(0, 1), (0, 2), (0, 3), (1, 4), (2, 4), (3, 4)]),
                vec![3, 1, 2, 0, 0, 0],
                vec![2, 2, 2, 2, 2, 2],
                0,
                4,
                3,
            );
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumedgecostflow_ilp.rs"]
mod tests;
