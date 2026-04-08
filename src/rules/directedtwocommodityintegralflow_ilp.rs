//! Reduction from DirectedTwoCommodityIntegralFlow to ILP<i32>.
//!
//! One non-negative integer variable per (commodity, arc):
//!   f1_a = a             for a in 0..num_arcs  (commodity 1 flow on arc a)
//!   f2_a = num_arcs + a  for a in 0..num_arcs  (commodity 2 flow on arc a)
//!
//! Constraints:
//! - Joint capacity: f1_a + f2_a ≤ cap[a] for each arc a
//! - Flow conservation: for each commodity, Σ f_out(v) - Σ f_in(v) = 0 at non-terminals
//! - Sink requirement: net inflow at sink_k ≥ R_k for each commodity k
//!
//! Objective: Minimize 0 (feasibility).
//! Extraction: Direct 2*|A| variables.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::DirectedTwoCommodityIntegralFlow;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing DirectedTwoCommodityIntegralFlow to ILP<i32>.
///
/// Variable layout:
/// - `f1_a` at index a for a in 0..num_arcs (commodity 1)
/// - `f2_a` at index num_arcs + a for a in 0..num_arcs (commodity 2)
#[derive(Debug, Clone)]
pub struct ReductionD2CIFToILP {
    target: ILP<i32>,
    num_arcs: usize,
}

impl ReductionResult for ReductionD2CIFToILP {
    type Source = DirectedTwoCommodityIntegralFlow;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Extract flow solution: all 2*|A| variables directly encode the flow.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..2 * self.num_arcs].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "2 * num_arcs",
        num_constraints = "num_arcs + 2 * num_vertices + 2",
    }
)]
impl ReduceTo<ILP<i32>> for DirectedTwoCommodityIntegralFlow {
    type Result = ReductionD2CIFToILP;

    fn reduce_to(&self) -> Self::Result {
        let arcs = self.graph().arcs();
        let m = arcs.len();
        let n = self.num_vertices();
        let num_vars = 2 * m;

        let f1 = |a: usize| a;
        let f2 = |a: usize| m + a;

        let mut constraints = Vec::new();

        // 1. Joint capacity: f1_a + f2_a ≤ cap[a]
        for a in 0..m {
            constraints.push(LinearConstraint::le(
                vec![(f1(a), 1.0), (f2(a), 1.0)],
                self.capacities()[a] as f64,
            ));
        }

        // 2. Flow conservation away from each commodity's own source and sink
        for vertex in 0..n {
            // Commodity 1: Σ_in f1 - Σ_out f1 = 0
            let mut terms_c1: Option<Vec<(usize, f64)>> = None;
            // Commodity 2: Σ_in f2 - Σ_out f2 = 0
            let mut terms_c2: Option<Vec<(usize, f64)>> = None;

            if vertex != self.source_1() && vertex != self.sink_1() {
                terms_c1 = Some(Vec::new());
            }
            if vertex != self.source_2() && vertex != self.sink_2() {
                terms_c2 = Some(Vec::new());
            }

            for (a, &(u, v)) in arcs.iter().enumerate() {
                if vertex == u {
                    // Arc leaves vertex: outgoing
                    if let Some(terms) = &mut terms_c1 {
                        terms.push((f1(a), -1.0));
                    }
                    if let Some(terms) = &mut terms_c2 {
                        terms.push((f2(a), -1.0));
                    }
                } else if vertex == v {
                    // Arc enters vertex: incoming
                    if let Some(terms) = &mut terms_c1 {
                        terms.push((f1(a), 1.0));
                    }
                    if let Some(terms) = &mut terms_c2 {
                        terms.push((f2(a), 1.0));
                    }
                }
            }

            if let Some(terms_c1) = terms_c1.filter(|terms| !terms.is_empty()) {
                constraints.push(LinearConstraint::eq(terms_c1, 0.0));
            }
            if let Some(terms_c2) = terms_c2.filter(|terms| !terms.is_empty()) {
                constraints.push(LinearConstraint::eq(terms_c2, 0.0));
            }
        }

        // 3. Net flow into sink_1 ≥ requirement_1
        let sink_1 = self.sink_1();
        let mut sink1_terms: Vec<(usize, f64)> = Vec::new();
        for (a, &(u, v)) in arcs.iter().enumerate() {
            if v == sink_1 {
                sink1_terms.push((f1(a), 1.0));
            } else if u == sink_1 {
                sink1_terms.push((f1(a), -1.0));
            }
        }
        constraints.push(LinearConstraint::ge(
            sink1_terms,
            self.requirement_1() as f64,
        ));

        // Net flow into sink_2 ≥ requirement_2
        let sink_2 = self.sink_2();
        let mut sink2_terms: Vec<(usize, f64)> = Vec::new();
        for (a, &(u, v)) in arcs.iter().enumerate() {
            if v == sink_2 {
                sink2_terms.push((f2(a), 1.0));
            } else if u == sink_2 {
                sink2_terms.push((f2(a), -1.0));
            }
        }
        constraints.push(LinearConstraint::ge(
            sink2_terms,
            self.requirement_2() as f64,
        ));

        ReductionD2CIFToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize),
            num_arcs: m,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::topology::DirectedGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "directedtwocommodityintegralflow_to_ilp",
        build: || {
            // 6-vertex network: s1=0, s2=1, t1=4, t2=5
            // Arcs: (0,2),(0,3),(1,2),(1,3),(2,4),(2,5),(3,4),(3,5)
            // f1 routes 0→2→4 (1 unit), f2 routes 1→3→5 (1 unit)
            let source = DirectedTwoCommodityIntegralFlow::new(
                DirectedGraph::new(
                    6,
                    vec![
                        (0, 2),
                        (0, 3),
                        (1, 2),
                        (1, 3),
                        (2, 4),
                        (2, 5),
                        (3, 4),
                        (3, 5),
                    ],
                ),
                vec![1; 8],
                0,
                4,
                1,
                5,
                1,
                1,
            );
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/directedtwocommodityintegralflow_ilp.rs"]
mod tests;
