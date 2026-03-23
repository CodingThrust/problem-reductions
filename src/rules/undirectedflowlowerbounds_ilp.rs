//! Reduction from UndirectedFlowLowerBounds to ILP<i32>.
//!
//! For each undirected edge e = {u,v} (indexed by e), we introduce:
//!   f_{uv} = 2*e      (flow in u→v direction, ≥ 0)
//!   f_{vu} = 2*e + 1  (flow in v→u direction, ≥ 0)
//!   z_e    = 2*|E| + e (binary orientation: 1 if u→v, 0 if v→u)
//!
//! Constraints per edge (4 constraints):
//!   z_e ≤ 1  (force binary)
//!   f_{uv} ≤ cap[e] * z_e        (only if oriented u→v)
//!   f_{vu} ≤ cap[e] * (1 - z_e)  (only if oriented v→u)
//!   f_{uv} ≥ lower[e] * z_e      (must carry at least lower bound if oriented u→v)
//!   f_{vu} ≥ lower[e] * (1 - z_e)(must carry at least lower bound if oriented v→u)
//! Since we need all 4: linearized as:
//!   z_e ≤ 1
//!   f_{uv} - cap[e]*z_e ≤ 0
//!   f_{vu} + cap[e]*z_e ≤ cap[e]
//!   f_{uv} - lower[e]*z_e ≥ 0    (only lower bound if positive)
//!   f_{vu} - lower[e]*(1-z_e) ≥ 0 => f_{vu} + lower[e]*z_e ≥ lower[e]
//!
//! Flow conservation at non-terminal vertices.
//! Net flow into sink ≥ requirement.
//!
//! Overhead: 3*|E| variables, 4*|E| + |V| + 1 constraints (conservative for non-terminals).

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::UndirectedFlowLowerBounds;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::Graph;

/// Result of reducing UndirectedFlowLowerBounds to ILP<i32>.
///
/// Variable layout:
/// - `f_{uv}` at 2*e (flow u→v on edge e)
/// - `f_{vu}` at 2*e + 1 (flow v→u on edge e)
/// - `z_e` at 2*|E| + e (orientation indicator: 1 = u→v direction)
#[derive(Debug, Clone)]
pub struct ReductionUFLBToILP {
    target: ILP<i32>,
    num_edges: usize,
}

impl ReductionResult for ReductionUFLBToILP {
    type Source = UndirectedFlowLowerBounds;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Extract edge orientation from ILP: z_e values at indices [2*|E|..3*|E|).
    ///
    /// The model encodes orientation as config[e] = 0 for u→v, 1 for v→u.
    /// The ILP uses z_e = 1 for u→v, z_e = 0 for v→u.
    /// So we return 1 - z_e to match the model's convention.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let e = self.num_edges;
        target_solution[2 * e..3 * e]
            .iter()
            .map(|&z| 1 - z)
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "3 * num_edges",
        num_constraints = "4 * num_edges + num_vertices + 1",
    }
)]
impl ReduceTo<ILP<i32>> for UndirectedFlowLowerBounds {
    type Result = ReductionUFLBToILP;

    fn reduce_to(&self) -> Self::Result {
        let edges = self.graph().edges();
        let e = edges.len();
        let n = self.num_vertices();
        let num_vars = 3 * e;

        let f_uv = |edge: usize| 2 * edge;
        let f_vu = |edge: usize| 2 * edge + 1;
        let z = |edge: usize| 2 * e + edge;

        let mut constraints = Vec::new();

        for (edge_idx, _) in edges.iter().enumerate() {
            let cap = self.capacities()[edge_idx] as f64;
            let lower = self.lower_bounds()[edge_idx] as f64;

            // z_e ≤ 1 (binary)
            constraints.push(LinearConstraint::le(vec![(z(edge_idx), 1.0)], 1.0));

            // f_{uv} ≤ cap * z_e  =>  f_{uv} - cap*z_e ≤ 0
            constraints.push(LinearConstraint::le(
                vec![(f_uv(edge_idx), 1.0), (z(edge_idx), -cap)],
                0.0,
            ));

            // f_{vu} ≤ cap * (1 - z_e)  =>  f_{vu} + cap*z_e ≤ cap
            constraints.push(LinearConstraint::le(
                vec![(f_vu(edge_idx), 1.0), (z(edge_idx), cap)],
                cap,
            ));

            if lower > 0.0 {
                // f_{uv} ≥ lower * z_e  =>  f_{uv} - lower*z_e ≥ 0
                constraints.push(LinearConstraint::ge(
                    vec![(f_uv(edge_idx), 1.0), (z(edge_idx), -lower)],
                    0.0,
                ));

                // f_{vu} ≥ lower * (1 - z_e)  =>  f_{vu} + lower*z_e ≥ lower
                constraints.push(LinearConstraint::ge(
                    vec![(f_vu(edge_idx), 1.0), (z(edge_idx), lower)],
                    lower,
                ));
            }
        }

        // Flow conservation at non-terminal vertices
        for vertex in 0..n {
            if vertex == self.source() || vertex == self.sink() {
                continue;
            }

            let mut terms: Vec<(usize, f64)> = Vec::new();
            for (edge_idx, &(u, v)) in edges.iter().enumerate() {
                if vertex == u {
                    // f_{uv} leaves vertex u, f_{vu} enters
                    terms.push((f_uv(edge_idx), -1.0));
                    terms.push((f_vu(edge_idx), 1.0));
                } else if vertex == v {
                    // f_{uv} enters vertex v, f_{vu} leaves
                    terms.push((f_uv(edge_idx), 1.0));
                    terms.push((f_vu(edge_idx), -1.0));
                }
            }

            if !terms.is_empty() {
                constraints.push(LinearConstraint::eq(terms, 0.0));
            }
        }

        // Net flow into sink ≥ requirement
        let sink = self.sink();
        let mut sink_terms: Vec<(usize, f64)> = Vec::new();
        for (edge_idx, &(u, v)) in edges.iter().enumerate() {
            if v == sink {
                // f_{uv} flows into sink, f_{vu} flows out
                sink_terms.push((f_uv(edge_idx), 1.0));
                sink_terms.push((f_vu(edge_idx), -1.0));
            } else if u == sink {
                // f_{vu} flows into sink (from v side), f_{uv} flows out
                sink_terms.push((f_uv(edge_idx), -1.0));
                sink_terms.push((f_vu(edge_idx), 1.0));
            }
        }
        constraints.push(LinearConstraint::ge(sink_terms, self.requirement() as f64));

        ReductionUFLBToILP {
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
        id: "undirectedflowlowerbounds_to_ilp",
        build: || {
            // 3-vertex graph: edge (0,1) cap=2 lower=1, edge (1,2) cap=2 lower=1
            // source=0, sink=2, requirement=1
            // Route: 0→1→2: flow 1 unit, orientations z_0=1, z_1=1
            let source = UndirectedFlowLowerBounds::new(
                SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
                vec![2, 2],
                vec![1, 1],
                0,
                2,
                1,
            );
            // Route 0→1→2: orient both edges u→v, i.e. source config [0,0]
            // f_{01}=1, f_{10}=0, f_{12}=1, f_{21}=0, z_0=1, z_1=1
            // extract_solution converts z to model config: config[e] = 1 - z_e → [0,0]
            crate::example_db::specs::rule_example_with_witness::<_, ILP<i32>>(
                source,
                SolutionPair {
                    source_config: vec![0, 0],
                    target_config: vec![1, 0, 1, 0, 1, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/undirectedflowlowerbounds_ilp.rs"]
mod tests;
