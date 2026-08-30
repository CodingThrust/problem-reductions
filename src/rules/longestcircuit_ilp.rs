//! Reduction from LongestCircuit to ILP (Integer Linear Programming).
//!
//! Direct cycle-selection formulation:
//! - Binary y_e for edge selection
//! - Binary s_v for vertex on circuit
//! - Degree: sum_{e : v in e} y_e = 2 s_v
//! - At least 3 edges selected
//! - Maximize: sum l_e y_e
//! - Multi-commodity flow connectivity

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::LongestCircuit;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::i64_to_exact_f64;

/// Result of reducing LongestCircuit to ILP.
///
/// Variable layout (all binary):
/// - `y_e` for edge e, indices `0..m`
/// - `s_v` for vertex v, indices `m..m+n`
/// - `f^t_{e,dir}` flow for commodity t, indices `m+n..m+n+2m*(n-1)`
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
        num_vars = "num_edges + num_vertices + 2 * num_edges * (num_vertices - 1)",
        num_constraints = "1 + num_vertices^2 + 2 * num_edges * (num_vertices - 1)",
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

        let y_idx = |e: usize| -> usize { e };
        let s_idx = |v: usize| -> usize { m + v };

        // Multi-commodity flow for connectivity
        let num_commodities = n.saturating_sub(1);
        let num_flow = 2 * m * num_commodities;
        let num_vars = m + n + num_flow;

        let flow_idx = |commodity: usize, edge: usize, dir: usize| -> usize {
            m + n + commodity * 2 * m + 2 * edge + dir
        };

        let mut constraints = Vec::new();

        // Degree constraints: sum_{e : v in e} y_e = 2 s_v for all v
        for v in 0..n {
            let mut terms: Vec<(usize, i64)> = Vec::new();
            for (e, &(u, w)) in edges.iter().enumerate() {
                if u == v || w == v {
                    terms.push((y_idx(e), 1));
                }
            }
            terms.push((s_idx(v), -2));
            constraints.push(LinearConstraint::eq(terms, 0));
        }

        // At least 3 edges selected
        let all_edge_terms: Vec<(usize, i64)> = (0..m).map(|e| (y_idx(e), 1)).collect();
        constraints.push(LinearConstraint::ge(all_edge_terms, 3));

        // Multi-commodity flow for connectivity
        // Root = vertex 0. For each non-root vertex t (commodity index = t-1):
        for t in 1..n {
            let commodity = t - 1;

            // Flow conservation at each vertex v
            for v in 0..n {
                let mut terms = Vec::new();
                for (e, &(u, w)) in edges.iter().enumerate() {
                    // Forward dir: u->w, reverse dir: w->u
                    if u == v {
                        terms.push((flow_idx(commodity, e, 0), 1)); // outgoing
                        terms.push((flow_idx(commodity, e, 1), -1)); // incoming
                    }
                    if w == v {
                        terms.push((flow_idx(commodity, e, 0), -1)); // incoming
                        terms.push((flow_idx(commodity, e, 1), 1)); // outgoing
                    }
                }

                if v == 0 {
                    // Root: outflow - inflow = s_t
                    terms.push((s_idx(t), -1));
                    constraints.push(LinearConstraint::eq(terms, 0));
                } else if v == t {
                    // Target: outflow - inflow = -s_t
                    terms.push((s_idx(t), 1));
                    constraints.push(LinearConstraint::eq(terms, 0));
                } else {
                    // Transit: outflow - inflow = 0
                    constraints.push(LinearConstraint::eq(terms, 0));
                }
            }

            // Capacity: f^t_{e,dir} <= y_e
            for e in 0..m {
                constraints.push(LinearConstraint::le(
                    vec![(flow_idx(commodity, e, 0), 1), (y_idx(e), -1)],
                    0,
                ));
                constraints.push(LinearConstraint::le(
                    vec![(flow_idx(commodity, e, 1), 1), (y_idx(e), -1)],
                    0,
                ));
            }
        }

        // Objective: maximize total edge length
        let objective: Vec<(usize, f64)> = lengths
            .iter()
            .enumerate()
            .map(|(e, &l)| {
                i64_to_exact_f64(l)
                    .map(|length| (y_idx(e), length))
                    .map_err(|error| {
                        crate::rules::ReductionError::inexact_float_conversion::<
                            LongestCircuit<SimpleGraph, i64>,
                            ILP<bool>,
                        >(error)
                    })
            })
            .collect::<Result<_, _>>()?;
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
