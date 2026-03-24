//! Reduction from UndirectedTwoCommodityIntegralFlow to ILP<i32>.
//!
//! For each undirected edge {u,v} (indexed by e), we introduce 4 flow variables:
//!   f1_{uv} = 4*e + 0  (commodity 1 flow u→v)
//!   f1_{vu} = 4*e + 1  (commodity 1 flow v→u)
//!   f2_{uv} = 4*e + 2  (commodity 2 flow u→v)
//!   f2_{vu} = 4*e + 3  (commodity 2 flow v→u)
//!
//! Additional binary indicator variables for capacity sharing:
//!   d1_e = 4*|E| + 2*e     (1 if commodity 1 uses forward direction on edge e)
//!   d2_e = 4*|E| + 2*e + 1 (1 if commodity 2 uses forward direction on edge e)
//!
//! For each edge e with capacity c_e, the joint capacity constraint is:
//!   max(f1_{uv}, f1_{vu}) + max(f2_{uv}, f2_{vu}) ≤ c_e
//!
//! Since this is ILP<i32>, we use direction indicators d1_e, d2_e ∈ {0,1} to linearize:
//!   f1_{uv} ≤ c_e * d1_e;  f1_{vu} ≤ c_e * (1 - d1_e)
//!   f2_{uv} ≤ c_e * d2_e;  f2_{vu} ≤ c_e * (1 - d2_e)
//!   f1_{uv} + f1_{vu} + f2_{uv} + f2_{vu} ≤ c_e  (joint capacity)
//!
//! Variable layout (6 variables per edge):
//!   [0..4*E): f1_{uv}, f1_{vu}, f2_{uv}, f2_{vu} per edge
//!   [4*E..6*E): d1_e, d2_e per edge
//!
//! Constraints per edge (7 per edge) + flow conservation (2 per non-terminal vertex) + net flow (2)

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::UndirectedTwoCommodityIntegralFlow;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::Graph;

/// Result of reducing UndirectedTwoCommodityIntegralFlow to ILP<i32>.
///
/// Variable layout:
/// - `f1_{uv}` at 4*e + 0, `f1_{vu}` at 4*e + 1 (commodity 1 flows on edge e)
/// - `f2_{uv}` at 4*e + 2, `f2_{vu}` at 4*e + 3 (commodity 2 flows on edge e)
/// - `d1_e` at 4*|E| + 2*e, `d2_e` at 4*|E| + 2*e + 1 (direction indicators)
#[derive(Debug, Clone)]
pub struct ReductionU2CIFToILP {
    target: ILP<i32>,
    num_edges: usize,
}

impl ReductionResult for ReductionU2CIFToILP {
    type Source = UndirectedTwoCommodityIntegralFlow;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Extract flow solution: first 4*|E| variables are the flow values.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..4 * self.num_edges].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "6 * num_edges",
        num_constraints = "7 * num_edges + 2 * num_nonterminal_vertices + 2",
    }
)]
impl ReduceTo<ILP<i32>> for UndirectedTwoCommodityIntegralFlow {
    type Result = ReductionU2CIFToILP;

    fn reduce_to(&self) -> Self::Result {
        let edges = self.graph().edges();
        let e = edges.len();
        let n = self.num_vertices();
        // 4*e flow variables + 2*e direction indicators = 6*e total
        let num_vars = 6 * e;

        // Variable index helpers
        let f1_uv = |edge: usize| 4 * edge;
        let f1_vu = |edge: usize| 4 * edge + 1;
        let f2_uv = |edge: usize| 4 * edge + 2;
        let f2_vu = |edge: usize| 4 * edge + 3;
        let d1 = |edge: usize| 4 * e + 2 * edge;
        let d2 = |edge: usize| 4 * e + 2 * edge + 1;

        let mut constraints = Vec::with_capacity(7 * e + 2 * self.num_nonterminal_vertices() + 2);

        for (edge_idx, (_u, _v)) in edges.iter().enumerate() {
            let cap = self.capacities()[edge_idx] as f64;

            // Direction indicators are binary: d1_e ≤ 1, d2_e ≤ 1
            constraints.push(LinearConstraint::le(vec![(d1(edge_idx), 1.0)], 1.0));
            constraints.push(LinearConstraint::le(vec![(d2(edge_idx), 1.0)], 1.0));

            // Commodity 1 anti-parallel: f1_{uv} ≤ cap * d1_e
            // => f1_{uv} - cap * d1_e ≤ 0
            constraints.push(LinearConstraint::le(
                vec![(f1_uv(edge_idx), 1.0), (d1(edge_idx), -cap)],
                0.0,
            ));
            // f1_{vu} ≤ cap * (1 - d1_e) => f1_{vu} + cap*d1_e ≤ cap
            constraints.push(LinearConstraint::le(
                vec![(f1_vu(edge_idx), 1.0), (d1(edge_idx), cap)],
                cap,
            ));

            // Commodity 2 anti-parallel: f2_{uv} ≤ cap * d2_e
            constraints.push(LinearConstraint::le(
                vec![(f2_uv(edge_idx), 1.0), (d2(edge_idx), -cap)],
                0.0,
            ));
            // f2_{vu} ≤ cap * (1 - d2_e)
            constraints.push(LinearConstraint::le(
                vec![(f2_vu(edge_idx), 1.0), (d2(edge_idx), cap)],
                cap,
            ));

            // Joint capacity: f1_{uv} + f1_{vu} + f2_{uv} + f2_{vu} ≤ cap
            constraints.push(LinearConstraint::le(
                vec![
                    (f1_uv(edge_idx), 1.0),
                    (f1_vu(edge_idx), 1.0),
                    (f2_uv(edge_idx), 1.0),
                    (f2_vu(edge_idx), 1.0),
                ],
                cap,
            ));
        }

        // Flow conservation for each commodity at non-terminal vertices
        let terminals = [
            self.source_1(),
            self.sink_1(),
            self.source_2(),
            self.sink_2(),
        ];

        for vertex in 0..n {
            if terminals.contains(&vertex) {
                continue;
            }

            // Commodity 1 conservation: Σ_in f1 - Σ_out f1 = 0
            let mut terms_c1: Vec<(usize, f64)> = Vec::new();
            // Commodity 2 conservation: Σ_in f2 - Σ_out f2 = 0
            let mut terms_c2: Vec<(usize, f64)> = Vec::new();

            for (edge_idx, &(u, v)) in edges.iter().enumerate() {
                if vertex == u {
                    // outgoing from u: f1_{uv} goes out, f1_{vu} comes in
                    terms_c1.push((f1_uv(edge_idx), -1.0));
                    terms_c1.push((f1_vu(edge_idx), 1.0));
                    terms_c2.push((f2_uv(edge_idx), -1.0));
                    terms_c2.push((f2_vu(edge_idx), 1.0));
                } else if vertex == v {
                    // outgoing from v: f1_{vu} goes out, f1_{uv} comes in
                    terms_c1.push((f1_uv(edge_idx), 1.0));
                    terms_c1.push((f1_vu(edge_idx), -1.0));
                    terms_c2.push((f2_uv(edge_idx), 1.0));
                    terms_c2.push((f2_vu(edge_idx), -1.0));
                }
            }

            if !terms_c1.is_empty() {
                constraints.push(LinearConstraint::eq(terms_c1, 0.0));
            }
            if !terms_c2.is_empty() {
                constraints.push(LinearConstraint::eq(terms_c2, 0.0));
            }
        }

        // Net flow into sinks ≥ requirements
        // Commodity 1: net inflow at sink_1 ≥ requirement_1
        let sink_1 = self.sink_1();
        let mut sink1_terms: Vec<(usize, f64)> = Vec::new();
        for (edge_idx, &(u, v)) in edges.iter().enumerate() {
            if sink_1 == v {
                sink1_terms.push((f1_uv(edge_idx), 1.0));
                sink1_terms.push((f1_vu(edge_idx), -1.0));
            } else if sink_1 == u {
                sink1_terms.push((f1_uv(edge_idx), -1.0));
                sink1_terms.push((f1_vu(edge_idx), 1.0));
            }
        }
        constraints.push(LinearConstraint::ge(
            sink1_terms,
            self.requirement_1() as f64,
        ));

        // Commodity 2: net inflow at sink_2 ≥ requirement_2
        let sink_2 = self.sink_2();
        let mut sink2_terms: Vec<(usize, f64)> = Vec::new();
        for (edge_idx, &(u, v)) in edges.iter().enumerate() {
            if sink_2 == v {
                sink2_terms.push((f2_uv(edge_idx), 1.0));
                sink2_terms.push((f2_vu(edge_idx), -1.0));
            } else if sink_2 == u {
                sink2_terms.push((f2_uv(edge_idx), -1.0));
                sink2_terms.push((f2_vu(edge_idx), 1.0));
            }
        }
        constraints.push(LinearConstraint::ge(
            sink2_terms,
            self.requirement_2() as f64,
        ));

        ReductionU2CIFToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize),
            num_edges: e,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::topology::SimpleGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "undirectedtwocommodityintegralflow_to_ilp",
        build: || {
            // 4-vertex graph: edges (0,2),(1,2),(2,3); capacities [1,1,2]
            // s1=0, t1=3, s2=1, t2=3, R1=1, R2=1
            // f1 routes 0→2→3 (1 unit), f2 routes 1→2→3 (1 unit)
            let source = UndirectedTwoCommodityIntegralFlow::new(
                SimpleGraph::new(4, vec![(0, 2), (1, 2), (2, 3)]),
                vec![1, 1, 2],
                0,
                3,
                1,
                3,
                1,
                1,
            );
            let reduction: ReductionU2CIFToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
            let solver = crate::solvers::ILPSolver::new();
            let target_config = solver
                .solve(reduction.target_problem())
                .expect("canonical example should be feasible");
            let source_config = reduction.extract_solution(&target_config);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<i32>>(
                source,
                SolutionPair {
                    source_config,
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/undirectedtwocommodityintegralflow_ilp.rs"]
mod tests;
