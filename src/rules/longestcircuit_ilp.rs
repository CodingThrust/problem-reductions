//! Reduction from LongestCircuit to ILP (Integer Linear Programming).
//!
//! Direct cycle-selection formulation:
//! - Binary y_e for edge selection
//! - Binary s_v for vertex on circuit
//! - Degree: sum_{e : v in e} y_e = 2 s_v
//! - At least 3 edges selected
//! - Maximize: sum l_e y_e
//! - Multi-commodity flow connectivity from a root chosen on the circuit

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::LongestCircuit;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing LongestCircuit to ILP.
///
/// Variable layout (all binary):
/// - `y_e` for edge e, indices `0..m`
/// - `s_v` for vertex v, indices `m..m+n`
/// - `r_v` for the chosen root, indices `m+n..m+2n`
/// - `f^t_{e,dir}` flow to vertex t, indices `m+2n..m+2n+2mn`
#[derive(Debug, Clone)]
pub struct ReductionLongestCircuitToILP {
    target: ILP<bool>,
    num_edges: usize,
}

impl ReductionResult for ReductionLongestCircuitToILP {
    type Source = LongestCircuit<SimpleGraph, i64>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract: output the binary edge-selection vector (y_e).
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution[..self.num_edges]
            .iter()
            .map(|&value| value == 1)
            .collect())
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_edges + 2 * num_vertices + 2 * num_edges * num_vertices",
        num_constraints = "2 + num_vertices + 2 * num_vertices^2 + 2 * num_edges * num_vertices",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for LongestCircuit<SimpleGraph, i64> {
    type Result = ReductionLongestCircuitToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let m = self.num_edges();
        let edges = self.graph().edges();
        let lengths = self.edge_lengths();

        let num_vars = m
            .checked_mul(n)
            .and_then(|flow| flow.checked_add(n))
            .and_then(|flow_and_vertices| flow_and_vertices.checked_mul(2))
            .and_then(|auxiliary| auxiliary.checked_add(m))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<Self, ILP<bool>>(
                    "computing the number of cycle and flow variables",
                )
            })?;

        let y_idx = |e: usize| -> usize { e };
        let s_idx = |v: usize| -> usize { m + v };
        let r_idx = |v: usize| -> usize { m + n + v };
        let flow_idx = |commodity: usize, edge: usize, dir: usize| -> usize {
            m + 2 * n + commodity * 2 * m + 2 * edge + dir
        };
        let mut vertex_edges = vec![Vec::new(); n];
        for (edge, &(u, v)) in edges.iter().enumerate() {
            vertex_edges[u].push(edge);
            vertex_edges[v].push(edge);
        }

        let mut constraints = Vec::new();

        // Degree constraints: sum_{e : v in e} y_e = 2 s_v for all v
        for (v, incident_edges) in vertex_edges.iter().enumerate() {
            let mut terms: Vec<(usize, i64)> = Vec::new();
            for &edge in incident_edges {
                terms.push((y_idx(edge), 1));
            }
            terms.push((s_idx(v), -2));
            constraints.push(LinearConstraint::eq(terms, 0));
        }

        // At least 3 edges selected
        let all_edge_terms: Vec<(usize, i64)> = (0..m).map(|e| (y_idx(e), 1)).collect();
        constraints.push(LinearConstraint::ge(all_edge_terms, 3));

        // Choose exactly one root among the selected vertices.
        constraints.push(LinearConstraint::eq(
            (0..n).map(|v| (r_idx(v), 1)).collect(),
            1,
        ));
        for v in 0..n {
            constraints.push(LinearConstraint::le(vec![(r_idx(v), 1), (s_idx(v), -1)], 0));
        }

        // Each selected non-root vertex receives one unit from the chosen root.
        for t in 0..n {
            // Flow conservation at each vertex v
            for (v, incident_edges) in vertex_edges.iter().enumerate() {
                let mut terms = Vec::new();
                for &edge in incident_edges {
                    let (u, _) = edges[edge];
                    // Forward dir: u->w, reverse dir: w->u
                    if u == v {
                        terms.push((flow_idx(t, edge, 0), 1)); // outgoing
                        terms.push((flow_idx(t, edge, 1), -1)); // incoming
                    } else {
                        terms.push((flow_idx(t, edge, 0), -1)); // incoming
                        terms.push((flow_idx(t, edge, 1), 1)); // outgoing
                    }
                }

                if v == t {
                    // Target: outflow - inflow = r_t - s_t.
                    terms.push((s_idx(t), 1));
                    terms.push((r_idx(t), -1));
                    constraints.push(LinearConstraint::eq(terms, 0));
                } else {
                    // Only the root can supply flow: 0 <= outflow - inflow <= r_v.
                    constraints.push(LinearConstraint::ge(terms.clone(), 0));
                    terms.push((r_idx(v), -1));
                    constraints.push(LinearConstraint::le(terms, 0));
                }
            }

            // Capacity: f^t_{e,dir} <= y_e
            for e in 0..m {
                constraints.push(LinearConstraint::le(
                    vec![(flow_idx(t, e, 0), 1), (y_idx(e), -1)],
                    0,
                ));
                constraints.push(LinearConstraint::le(
                    vec![(flow_idx(t, e, 1), 1), (y_idx(e), -1)],
                    0,
                ));
            }
        }

        // Objective: maximize total edge length
        let objective: Vec<(usize, i64)> = lengths
            .iter()
            .enumerate()
            .map(|(e, &length)| (y_idx(e), length))
            .collect();
        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize)
            .map_err(Self::target_construction)?;

        Ok(ReductionLongestCircuitToILP {
            target,
            num_edges: m,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "longestcircuit_to_ilp",
        build: || {
            // Triangle with unit lengths
            let source = LongestCircuit::new(
                SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
                vec![1, 1, 1],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/longestcircuit_ilp.rs"]
mod tests;
